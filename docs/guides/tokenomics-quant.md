# GDS for quant finance and tokenomics

*For quants, protocol and mechanism designers, token economists, and anyone who has had to ask "what happens to this
system if people behave badly?" No math background beyond arithmetic assumed.*

Tags used below: **[runs today]**, **[designed]** (specified, not built), **[exploring]**. Runnable models are files in
[`examples/`](../../examples/).

---

## 1. Why a quant or mechanism designer should care

Token systems and market mechanisms are made of rules, and rules fail in surprising ways:

- **A mechanism is deployed, then attacked.** Specifying the rule as a model first lets you ask "what if everyone
  does the worst legal thing?" before money is at stake.
- **Exactness matters.** An issuance schedule, a vesting plan, or a pool invariant is an *integer* statement. A
  spreadsheet in floating point drifts.
- **Invariants are the audit.** "Supply is conserved." "The pool's product never shrinks." "Locked plus released equals
  total." You want those written down and checked on every step, not remembered.
- **Parameters are governance.** Fees, emission rates, and caps are the knobs. A sweep tells you which ones matter and
  where the cliffs are.

GDS (generalized dynamical systems) is the framework behind cadCAD, the Python library that popularized this style of
simulation for token engineering. Roxin is a language built around the same ideas.

## 2. The one idea: state, input, mechanism

Meet each finance word once; from here the guide uses the Roxin word.

| In finance you say | In Roxin | What it is |
|---|---|---|
| balances, reserves, positions, the ledger | **state** | all the values that change |
| fee, emission rate, cap, halving interval | **param** | values fixed during a run |
| a trade, a transaction, an order, an arrival | **input** | what is chosen or arrives this step |
| the protocol rules, the contract | **mechanism** | next state from old state and input |
| block, epoch, trading day, tick | **step** | one pass of the rules |
| `require(...)`, margin limit, "valid transaction" | **allowed** | which inputs are legal in this state |
| `x * y >= k`, "supply is conserved" | **invariant** | a rule that must hold on every state |
| agent strategy, trader behavior | **input map** | how the input is produced from the state |

**An input is not a change to state.** A trader *submits* a swap (input). The protocol decides how pools and balances
change (mechanism). Separating the two lets you swap trader behavior without touching the protocol, and change the
protocol without touching the trader.

## 3. Your first model **[runs today]**

Bitcoin's issuance schedule, in whole satoshis. One step is one halving era of 210,000 blocks. The subsidy starts at
50 BTC and is halved by *integer* division, so it reaches zero.

<!-- example: bitcoin-halving.rox -->
```
-- Bitcoin's issuance schedule in whole satoshis. One step is one halving era (210,000 blocks).

param blocks_per_era: int = 210000
param initial_subsidy: int = 5_000_000_000    -- 50 BTC

state Chain { era: int, supply: int }

step Era:
    input:
        subsidy: int = initial_subsidy // 2 ^ s.era
    mechanism:
        era: s.era + 1
        supply: s.supply + u.subsidy * blocks_per_era

system Bitcoin: Chain = Era

run Bitcoin from Chain { era: 0, supply: 0 } for 40 steps
```

`//` is floor division, `^` is power, and `2 ^ s.era` binds tighter than `//`. Underscores in numbers are for reading.
The input is the era's subsidy (the rule, read off the state); the mechanism adds one era's issuance to the supply.

<!-- output: bitcoin-halving.rox -->
```
job 0  
    0  { era: 0, supply: 0 }
    1  { era: 1, supply: 1050000000000000 }
    2  { era: 2, supply: 1575000000000000 }
    3  { era: 3, supply: 1837500000000000 }
   32  { era: 32, supply: 2099999997480000 }
   33  { era: 33, supply: 2099999997690000 }
   34  { era: 34, supply: 2099999997690000 }
   40  { era: 40, supply: 2099999997690000 }
```

After era 33 the subsidy is 0 and the supply is fixed at **2,099,999,997,690,000 satoshis, which is
20,999,999.9769 BTC**, exactly. (The cap is not 21 million flat; integer halving leaves a small remainder unissued.
That detail is invisible in a floating-point model, and it is precisely the kind of thing exact integers are for.)
The number was also computed independently in Python; they agree digit for digit.

## 4. Separate the decision from the consequence **[runs today]**

A constant-product market maker (`x * y = k`) with a trading fee. Each step, a trader pays 10 of token `x` into the
pool, and `y` is paid out so that the fee stays in the pool.

<!-- example: amm-swap.rox -->
```
-- A constant-product pool (x * y = k) with a trading fee, one swap per step.
-- The trader pays `amount` of x into the pool; y is paid out so that the fee stays in the pool.

param fee: float = 0.003
param trade: float = 10

state Pool { x: float, y: float }

step Swap:
    input:
        amount: float = trade
    mechanism:
        x: s.x + u.amount
        y: s.x * s.y / (s.x + u.amount * (1 - fee))

system Amm: Pool = Swap

run Amm from Pool { x: 1000, y: 1000 } for 50 steps
    sweep fee in [0, 0.003, 0.01]
```

The trader's behavior is the input (here, a constant 10). The protocol is the mechanism. Change the trader without
touching the protocol, or sweep the protocol parameter without touching the trader. After 50 steps:

| `fee` | `x` | `y` | product `x * y` |
|---|---|---|---|
| 0 | 1500 | 666.666666666667 | 1,000,000.0000000005 |
| 0.003 | 1500 | 667.4747721062278 | 1,001,212.158 |
| 0.01 | 1500 | 669.364240335036 | 1,004,046.361 |

With no fee the product stays at its starting value (to rounding); with a fee it grows, and the growth is what liquidity
providers earn. The numbers are the same as an independent Python implementation of the formula.

## 5. What is legal **[designed]**

The pool can't pay out more than it holds, and a trade can't be negative. In Roxin that is `allowed`, separate from the
rule of what a swap does:

```
step Swap:
    input:
        amount: float = trade
    allowed: amount in 0..s.x
    mechanism:
        x: s.x + u.amount
        y: s.x * s.y / (s.x + u.amount * (1 - fee))
```

`allowed` can also constrain several inputs jointly, for example `alice + bob <= s.balance`. An input outside the set
stops the job with an error that names the state and the input. Because the legal set is explicit, a later version can
*enumerate* it, and ask what the worst legal action is.

## 6. Rules that must always hold **[designed]**

The market maker's rule is `x * y >= k0`:

```
invariant NotShrinking(s: Pool) = s.x * s.y >= k0
```

It is checked on the starting state and after every step. A violation stops that job, records the violating state as
its last row, and lets other jobs in the sweep continue.

**The float trap.** Run the same pool with *no fee* and trade 7 per step. In exact arithmetic the product never moves.
In floating point it falls to `999999.9999999999` at step 11 purely from rounding, and the invariant fails:

```
   10  { x: 1070, y: 934.5794392523364 }
   11  { x: 1077, y: 928.5051067780872 }
  stopped at step 11: invariant violated
```

(From the Python reference, which uses plain IEEE-754 double precision like the interpreter.) This is why Roxin has no
default float tolerance: whether `1 - 1e-16` counts as "equal" is the author's decision, stated with `within`, not a
silent default. For money-like values, `decimal(N)` with explicit rounding avoids the problem entirely (section 7).

Conservation checks matter just as much. Token vesting over twelve steps, with the integer remainder spread across the
schedule:

```
param total: int = 1_000_003
param months: int = 12

state Vest { locked: int, released: int }

invariant Conserved(s: Vest) = s.locked + s.released == total

step Unlock:
    input:
        amount: int = total * step // months - total * (step - 1) // months
    mechanism:
        locked: s.locked - u.amount
        released: s.released + u.amount
```

Step 1 releases 83,333, step 12 ends with `locked: 0, released: 1000003`, and the sum is exactly `total` on every row. Using
`step` in an input is how a schedule is written. (This model needs `step` and `invariant`; both are next in the build.)

## 7. Choosing numbers

Roxin has no default numeric type; each field says what it is.

- **`int` for token units and anything that must be exact.** `int` has no size limit, so a 256-bit balance is just an
  integer. The current build uses 128-bit integers as a stand-in and stops with an error beyond about 1.7×10^38.
- **`decimal(N)` for fiat and fixed-point quantities** **[designed]**. `decimal(2)` keeps exactly two places. `+` and `-` are
  exact; a product has more places than its factors and must be rounded back with `round(e, mode)` (`down`, `up`, or
  `nearest`). Division is only legal inside `round`.
- **`float` for models of continuous things**: price processes, volatility, the pool above.

Rounding direction is a **design decision with a visible effect.** A pool with 110 apples that must keep
`apples * oranges >= 10000`: rounding the oranges **up** gives `90.909091` and a product of `10000.000010` (the rule
holds); rounding **down** gives `90.909090` and `9999.999900` (it fails). Round against the user. The language will not
choose for you.

**Overflow is real in contracts and absent in the language.** A real contract can wrap or revert at 2^256. Roxin's
`int` does not, so state the bound yourself and let the checker hold you to it: `invariant NoOverflow(s: Pool) =
s.balance < 2^256`.

## 8. Randomness and time **[designed]**

**Time.** `step` is the number of the transition being computed, starting at 1, readable in `input:` and `allowed:`.
That is how an emission schedule or a vesting cliff is written (the vesting model above).

**Randomness** is only legal in `input:`, never in the mechanism. A random price return:

```
step Market:
    input:
        shock: float = normal(0, 0.02)
    mechanism:
        price: s.price * (1 + u.shock)
```

A run says `runs 1000` and `seed 42`. The generator is specified (xoshiro256++ seeded by SplitMix64, Box–Muller for
normal draws), so a seed reproduces the same run on any machine. Without a seed, one is drawn at random, saved with the
results, and printed, so any run can be replayed. For a quant, this is the difference between "I ran it and it looked
fine" and "here is the seed".

## 9. A bigger model, walked through **[designed]**

A linear bonding curve. Price rises with supply: `price = base + slope × supply`. Each step a buyer buys 10 tokens, and
the reserve must always equal the area under the price curve.

```
param base: float = 1
param slope: float = 0.01
param amount: float = 10

state Curve { supply: float, reserve: float }

invariant Backed(s: Curve) =
    s.reserve == base * s.supply + slope * s.supply ^ 2 / 2 within 0.000001

step Buy:
    mechanism:
        supply: s.supply + amount
        reserve: s.reserve + amount * (base + slope * (s.supply + amount / 2))
```

The mechanism says buying `amount` tokens costs `amount × price at the midpoint`, which is the exact integral for a
linear price. The invariant says the reserve equals the closed-form integral `base × supply + slope × supply² / 2`, to
within a tolerance you chose. After 50 steps, supply is 500 and the reserve is 1,750 (= 500 + 0.01 × 500² / 2). A change
to the mechanism that breaks the integral breaks the invariant on the first step it differs, not in a post-mortem.

## 10. Honest limits

- **Roxin is not a smart-contract language** and does not model gas, transaction ordering, or mempool dynamics. A step
  is atomic; if ordering matters (front-running, MEV), model the ordering as part of the input.
- **Agents and order books need collections**, which are **[exploring]**. Today the state is a record of numbers, so
  "one thousand traders each with a balance" cannot be written yet.
- **Floats are IEEE-754.** Real contracts use fixed-point integers. Model them as `int` or `decimal`, not `float`.
- **Behavior is your assumption.** A model with a constant trader tells you about constant traders. Use the input to
  test adversarial and random behavior, and use `allowed` to define what an adversary can legally do.
- **Nothing here is financial advice**, and no model result is a prediction of a real market.

## 11. Glossary: your word, the Roxin term

| Finance / tokenomics word | Roxin |
|---|---|
| ledger, balances, reserves, positions | state |
| block, epoch, day, tick | step |
| transaction, order, trade, swap, claim | input |
| trader, agent, strategy, bot | input map |
| protocol rule, contract logic, settlement | mechanism |
| `require`, margin limit, validity condition | allowed |
| conservation law, solvency, `x*y >= k`, audit check | invariant |
| fee, emission rate, cap, halving interval | param |
| scenario analysis, stress test, backtest sweep | run, with `sweep` and `runs` |
| Monte Carlo, price path | `runs N` with `seed` |
| rounding mode | `round(e, down\|up\|nearest)` |
| time series replay (historical prices) | `data` declaration **[designed]** |

## 12. Exercises

1. **Check the cap.** Change `initial_subsidy` to `5_000_000_001` and run. Does the final supply change? By how
   much? (Answer: it rises by exactly 210,000 satoshis, to 2099999997900000. Only era 0 sees the extra satoshi,
   because every later era's floor division absorbs it. Why?)
2. **Find the fee.** In the market maker, which fee in the sweep makes the product grow fastest, and by how much per
   step? (Answer: the largest, 0.01; the product rises about 0.4% over 50 steps versus 0.12% at 0.003. Check by
   multiplying the final `x` and `y`.)
3. **Break it.** Set `trade` to 2000, more than the 1000 of `x` the pool starts with. The pool still runs; is
   anything wrong? Which section shows how to forbid it?
4. **Write the invariant.** State, in one line, a conservation rule for a token with a burn and a mint. Which fields
   does it mention?
