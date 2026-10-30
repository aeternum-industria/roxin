# GDS for collective decision-making and commons

*For people who design, run, or study shared institutions: councils, co-ops, commons, committees, mutual aid funds,
municipal budgets, DAOs. No programming or mathematics background assumed.*

A word on stance. Roxin is a modeling language, not a position. Every institution is a set of rules acting on a shared
situation, and the interesting arguments are usually about *assumptions*: how many people, how much they take, what
they do when a rule is broken. A model makes those assumptions visible and lets people with different goals run the
same rules and read the same numbers. This guide shows what a rule set does under stated assumptions. It does not say
which rules are better.

Tags used below: **[runs today]**, **[designed]** (specified, not built), **[exploring]**. Runnable models are files in
[`examples/`](../../examples/).

---

## 1. Why someone who designs institutions should care

Rules about shared resources have consequences that are hard to see from the rule text alone:

- **The same rule behaves differently at different sizes.** A custom that works for 20 members may fail at 200.
- **"What if people don't comply?" is a modeling question, not a taboo.** The honest versions of a rule say what
  happens when it is followed *and* when it is not.
- **Shared funds can run out slowly.** The fund looks fine for months and then is insolvent. A model finds the date.
- **Comparing rule sets needs the same situation under each rule.** Changing one rule and keeping everything else the
  same is hard to do in a meeting and easy to do in a model.
- **What must never happen can be stated.** "The fund never goes negative." "The stock never falls below the
  regeneration threshold."

GDS gives you three separate pieces: what each participant **chooses**, what the rules **allow**, and what actually
**happens** when choices combine. That separation is the point: you can change the rules without rewriting the
behavior, or change the behavior without rewriting the rules.

## 2. The one idea: state, input, mechanism

Meet each word once; from here the guide uses the Roxin word.

| In an institution you say | In Roxin | What it is |
|---|---|---|
| the shared resource, the treasury, the register | **state** | all the quantities that change |
| membership size, regeneration rate, dues, payout size | **param** | values fixed during a run |
| a member's action: harvest, contribute, claim, vote | **input** | what is chosen or arrives this step |
| what the combined actions do to the resource | **mechanism** | next state from old state and input |
| season, month, meeting cycle, round | **step** | one pass of the rules |
| "you may take at most the quota" | **allowed** | which actions are legal in this state |
| "the fund must never be negative" | **invariant** | a rule that must hold on every state |
| how members decide: custom, habit, strategy, procedure | **input map** | how the input is produced from the state |
| the charter or constitution, in the part that can be modeled | the model | state, rules, and parameters together |

**An input is not a change to state.** A member *decides* to harvest at some rate (input). The ecology decides what
that does to the stock (mechanism). Keeping them apart lets you ask "what if members behaved differently?" without
touching the ecology, and "what if the ecology were different?" without touching the members.

## 3. Your first model **[runs today]**

A shared fishery. The stock regrows logistically (fast when low, slow near capacity). Every member takes the same
fraction of the stock each season. The only thing that changes between jobs is **how many members there are**.

<!-- example: commons.rox -->
```
-- A shared fishery. Each member takes the same fraction of the stock every step.
-- The only thing that changes between jobs is how many members there are.

param growth: float = 0.2       -- regrowth rate of the stock
param capacity: float = 1000    -- the most the water can hold
param effort: float = 0.05      -- fraction of the stock one member takes per step
param members: int = 1

state Fishery { stock: float, caught: float }

step Season:
    input:
        catch: float = members * effort * s.stock
    mechanism:
        stock: s.stock + growth * s.stock * (1 - s.stock / capacity) - u.catch
        caught: s.caught + u.catch

system Commons: Fishery = Season

run Commons from Fishery { stock: 500, caught: 0 } for 100 steps
    sweep members in [1, 2, 4]
```

`input:` is the members' collective action; `mechanism:` is the ecology. Starting from a stock of 500, the first rows of
the one-member job:

<!-- output: commons.rox -->
```
job 0  members = 1
    0  { stock: 500, caught: 0 }
    1  { stock: 525, caught: 25 }
    2  { stock: 548.625, caught: 51.25 }
```

At step 100:

| members | total take rate | stock at step 100 | total caught over 100 seasons | what happened |
|---|---|---|---|---|
| 1 | 5% | about 750 | 3,647 | the stock climbs to 750 and settles there |
| 2 | 10% | 500 | 5,000 | the stock never moves: 50 caught every season, regrowth replaces it exactly |
| 4 | 20% | 44.5 | 2,373 | the stock falls below 100 by season 39 and keeps shrinking |

Every member's behavior is identical in the three jobs: each takes 5% of the stock per season. Only the headcount
differs. Two members catch more in total than one or four. This is the arithmetic behind the commons dilemma, and it
does not depend on anyone's motives: the regrowth is 0.2 × stock × (1 − stock/capacity), and taking 20% a season is
exactly as large as the regrowth rate.

## 4. Separate the decision from the consequence **[runs today]**

Holding the ecology fixed, change the *rule* that sets each member's effort. With four members and the effort per
member swept over `[0.025, 0.05, 0.10]` (change the `sweep` line to `sweep effort in [0.025, 0.05, 0.10]` and the
default `members` to 4), the stock and the total caught at season 100 are:

| effort per member | total take rate | stock | total caught |
|---|---|---|---|
| 0.025 | 10% | 500.0 | 5,000 |
| 0.05 | 20% | 44.5 | 2,373 |
| 0.10 | 40% | about 6×10^-8 | 789 |

Four members at 0.025 each reproduce the two-member outcome exactly: the ecology only sees the *total*. Which
institution produces a total take rate of 10% (a custom, a quota, a rotation, a market) is a separate question from
whether 10% is sustainable here.

**[designed]** Rules will be nameable and swappable, so an open-access rule and a quota rule can sit side by side and
run against the same ecology:

```
input map OpenAccess(s: Fishery) -> { catch: float } =
    { catch: members * effort * s.stock }

input map Quota(s: Fishery) -> { catch: float } =
    { catch: min(members * effort * s.stock, 50) }
```

With four members and a flat cap of 50 per season, the stock stays at 500 and 5,000 is caught over 100 seasons, against
2,373 under open access. Tighten the cap to 30 and the stock recovers to about 816 while 3,000 is caught. (These two
results come from the Python reference; `min` is next in the build.)

## 5. What is legal **[designed]**

A *rule* in an institution can mean two different things, and Roxin makes you say which:

- **A rule about what happens**: "catch is capped at 50". It sits in the input map; the cap simply applies.
- **A rule about what is permitted**: "members may not take more than their share." It sits in `allowed`. A member who
  tries to take more than the allowance is outside the legal set, and the job **stops with an error** that names the
  state and the input.

```
step Season:
    input:
        catch: float = members * effort * s.stock
    allowed: catch <= 50
    mechanism: ...
```

These are different models of enforcement. The first assumes the rule binds perfectly. The second says "this model
cannot proceed if a member breaks the rule", which tells you that *the model does not yet say what happens on a
violation*. To study non-compliance and sanctions, make the violation part of the model: an input field for the
overage, a state field for a penalty or reputation, and a mechanism that applies it. Both are legitimate; the language
only insists that you notice which one you are doing.

## 6. Rules that must always hold **[designed]**

```
invariant Sustained(s: Fishery) = s.stock >= 100
invariant Solvent(s: Fund) = s.balance >= 0
```

Checked on the starting state and after every step. If the rule breaks, the failing state is recorded as the last row,
so you can read how it happened, and other jobs in the same sweep carry on. In the fishery above, `Sustained` first
fails in the four-member job at season 39 (the stock is 100 or more through season 38), and holds throughout for one
and two members. The invariant turns "it looked fine for a while" into "it breaks here".

## 7. Choosing numbers

- **`int` for people and votes**: member counts, ballots, whole items. Integers are exact. Division of two integers is
  an error, on purpose: share-splitting has to say how remainders are handled (`//` and `%`).
- **`decimal(N)` for money** **[designed]**: dues, payouts, budgets, with rounding that you write down. A shared fund
  that rounds dues up and payouts down is a different institution from one that rounds the other way.
- **`float` for quantities that are really continuous**: a fish stock, a water level, a forest.
- Splitting a quantity among members is where fairness lives. Use integer division and name what happens to the
  remainder, for example: the vesting model in the tokenomics guide spreads the remainder across the schedule so the
  total is exact.

## 8. Randomness and time **[designed]**

**Time.** `step` is the season or meeting number, starting at 1, readable in `input:` and `allowed:`. A rotation
(only one member harvests each season) or a seasonal ban is `step % N` in the input.

**Randomness.** Attendance at a meeting, who needs help this month, a bad harvest: all are random draws, legal only in
`input:`. A run says `runs 1000` and `seed 42`; the generator is specified, so the same seed reproduces the same run on
any machine. A fund that is solvent on the average month can still fail on a bad run, and 1,000 runs tell you how often.

## 9. A bigger model, walked through **[runs today]**

A mutual aid fund. Every member pays in each month. A fraction of members make a claim, and each claim pays a fixed
amount. The fund is sustainable only if contributions cover the claims on average.

<!-- example: mutual-aid.rox -->
```
-- A mutual aid fund. Every member contributes each step; some members claim a payout.
-- The fund is sustainable only if contributions cover the expected payouts.

param members: int = 100
param contribution: float = 1      -- paid by each member per step
param claim_rate: float = 0.1      -- fraction of members who claim per step
param payout: float = 10           -- paid per claim

state Fund { balance: float }

step Month:
    input:
        paid_in: float = members * contribution
        paid_out: float = members * claim_rate * payout
    mechanism:
        balance: s.balance + u.paid_in - u.paid_out

system Aid: Fund = Month

run Aid from Fund { balance: 500 } for 24 steps
    sweep claim_rate in [0.05, 0.1, 0.2]
```

Both flows are inputs (what comes in and what goes out); the mechanism applies them. The sweep tries three claim rates.
Paid in is always 100 per month. Paid out is `100 × claim_rate × 10`, so 50, 100, and 200:

| `claim_rate` | paid out per month | balance after 24 months | what happened |
|---|---|---|---|
| 0.05 | 50 | 1,700 | grows by 50 a month |
| 0.10 | 100 | 500 | exactly break-even; the balance never moves |
| 0.20 | 200 | -1,900 | falls by 100 a month; negative from month 6 |

The break-even claim rate is `contribution / payout`, here 0.1. The model is deliberately plain so that every number is
traceable. Real questions come next, each a small change: set `contribution` to 1.2 and ask what claim rate it
survives; add a reserve target as an invariant; make claims random (section 8) and ask how large the opening balance
must be for the fund to survive 99 runs in 100. Because the balance in the third job goes negative and nothing stops
it, the missing piece is exactly `invariant Solvent(s: Fund) = s.balance >= 0` from section 6.

## 10. Honest limits

- **A model cannot tell you which rules are good.** It tells you what a rule does under your assumptions. Goals,
  fairness, legitimacy, and values are not in the model unless you write them in as quantities you choose to measure.
- **Behavior is assumed.** Members who respond to each other, learn, coordinate, or defect are not in these models.
  Adaptive behavior needs collections of agents, which are **[exploring]**. Today a state is a record of numbers, so
  "a list of 200 members each with their own history" cannot be written yet.
- **Aggregates hide distribution.** The fishery model tracks a total. Who gets what, and who is excluded, needs
  per-member state.
- **Simple ecology is simple.** Logistic regrowth is a textbook model, not a measurement of any real fishery.
- **No voting systems yet.** Choices with payloads and tallying are **[exploring]**.

## 11. Glossary: your word, the Roxin term

| Institutional word | Roxin |
|---|---|
| the resource, the treasury, the ledger, the register | state |
| season, month, cycle, meeting round | step |
| action, harvest, contribution, claim, vote | input |
| custom, procedure, strategy, decision process | input map |
| what happens to the resource, the outcome | mechanism |
| permission, quota, entitlement, "may not" | allowed |
| "must never happen", solvency, sustainability threshold | invariant |
| membership size, dues, payout, regrowth rate | param |
| scenario, comparison of rule sets, stress test | run, with `sweep` |
| chance events (attendance, claims, shocks) | random draw in an input; `runs N` with `seed` |
| the charter, bylaws, constitution (modelable part) | the model |
| reform, amendment | an edit to the model (a different `input map`, `allowed`, or `param`) |

## 12. Exercises

1. **Break-even.** In the mutual aid fund, find the `claim_rate` at which the balance never moves. (Answer: 0.1, which
   is `contribution / payout`. Sweep `claim_rate` over `[0.09, 0.1, 0.11]` and watch the balance at month 24.)
2. **Dues.** Set `claim_rate` to 0.2 and raise `contribution` until the balance stops falling. What is the minimum
   contribution? (Answer: 2.0.)
3. **A different headcount.** In the fishery, change the sweep to `[3, 5]` members. For which is the stock higher at
   season 100? Which has the larger total catch? (Answer: 3 members leave a stock of about 251 and catch 4,262; 5 members
   leave under 1 and catch 1,338. Compare each total take rate, 15% and 25%, to the regrowth rate 0.2.)
4. **Postpone it.** In the fund model, the `claim_rate = 0.2` job first goes negative in month 6. Find the opening
   balance that postpones that to month 12. (Answer: 1,100. The deficit is 100 a month, so the balance reaches exactly
   0 in month 11 and is first negative in month 12. Check it.)
5. **Your own.** State one rule from an institution you know, then say whether it belongs in the input (what happens),
   in `allowed` (what is permitted), or in an `invariant` (what must never happen), and why.
