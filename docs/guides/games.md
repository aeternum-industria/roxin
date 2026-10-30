# GDS for game design and development

*For game and economy designers, systems designers, and gameplay programmers. No math background assumed.*

Tags used below: **[runs today]**, **[designed]** (specified, not built), **[exploring]**. Runnable models are files in
[`examples/`](../../examples/); run one with `cargo run --release -- examples/game-economy.rox`.

---

## 1. Why a game person should care

You already build generalized dynamical systems. A save file, a rule book, and a player who makes moves *is* one. The
trouble is that in a game codebase the three parts are tangled together, so the questions designers actually ask are
hard to answer:

- **"Does this economy stabilize, or does gold pile up forever?"** The answer is hiding in a faucet, a sink, and a
  player behavior, all in different files.
- **"What if the player does something dumb, or something clever?"** Playtesting finds a few behaviors. A model can
  try thousands.
- **"Can the player ever end up with negative gold, or a duplicated item?"** Today that is a bug report. A model can
  state it as a rule and check it.
- **"Which tuning knob matters?"** A sweep answers it in one run.

GDS gives you three separate pieces: what the **player (or bot, or dice) chooses**, what is **allowed**, and what the
**rules do** with a choice. Keeping them apart is what makes the questions above cheap to ask.

## 2. The one idea: state, input, mechanism

Think of one turn (or tick, or frame) of a game. Roxin has a word for each part. You will meet each game word once;
from here on the guide uses the Roxin word.

| In a game you say | In Roxin | What it is |
|---|---|---|
| the save file, the game world right now | **state** | all the values that change |
| the tuning sheet, difficulty sliders | **param** | values that stay fixed during a run |
| the player's move, an AI decision, a loot roll | **input** | what is chosen or arrives this turn |
| the rule book: what a move does | **mechanism** | next state from old state and input |
| turn, tick, frame, round | **step** | one pass of the rules |
| "you can't buy that, you have 0 coins" | **allowed** | which inputs are legal in this state |
| "gold must never be negative" | **invariant** | a rule that must hold on every state |

**An input is not a change to state.** The player *chooses* to spend 20 gold (input). The rules decide what that does
to the wallet (mechanism). Keeping them separate is the whole trick.

## 3. Your first model **[runs today]**

An idle game's gold economy: a faucet adds gold each step, and the player spends a fraction of their wallet.

<!-- example: game-economy.rox -->
```
-- A gold faucet and a spending rule in an idle game.
-- The player decides how much to spend; the mechanism says what that does to the wallet.

param faucet: float = 100    -- gold earned per step
param spend_rate: float = 0.05  -- fraction of the wallet the player spends per step

state Wallet { gold: float }

step Economy:
    input:
        spend: float = spend_rate * s.gold     -- the decision
    mechanism:
        gold: s.gold + faucet - u.spend        -- the consequence

system Game: Wallet = Economy

run Game from Wallet { gold: 0 } for 60 steps
    sweep spend_rate in [0.02, 0.05, 0.10]
```

Inside a step, `s` is the state at the start and `u` is the input. The `input:` block holds the player's decision; the
`mechanism:` block says what it does. The last line runs the game for 60 steps, three times, once for each spending
habit. Three jobs come out, each printing 61 rows (row 0 is the start). Rows 0, 1, and 2 of the first job:

<!-- output: game-economy.rox -->
```
job 0  spend_rate = 0.02
    0  { gold: 0 }
    1  { gold: 100 }
    2  { gold: 198 }
```

Gold at step 60, by spending habit:

| `spend_rate` | gold at step 10 | gold at step 60 | where it is heading |
|---|---|---|---|
| 0.02 | 914.64 | 3512.23 | 5000 (= 100 / 0.02) |
| 0.05 | 802.53 | 1907.86 | 2000 |
| 0.10 | 651.32 | 998.20 | 1000 |

The economy settles where the faucet equals the spend: `faucet / spend_rate`. A designer can now answer "how much gold
does a typical player hold at level 30?" by choosing the knob, not by guessing.

## 4. Separate the decision from the consequence **[runs today]**

Change only the decision. Suppose the game sells a fixed-price item instead of letting the player spend a fraction.
Replace the `input:` line with `spend: float = 50` and leave the mechanism alone. The wallet now grows by exactly 50
every step: 3000 gold at step 60, and it never settles. A fixed sink is a *different kind* of economy than a
proportional one, and the model shows it without any other change.

Because the mechanism did not change, you know the difference came from the decision rule alone. That is what makes
A/B comparisons honest: swap one thing, keep the rest.

**[designed]** Decision rules will be nameable and swappable, so a "hoarder" and a "spender" can sit side by side:

```
input map Spender(s: Wallet) -> { spend: float } =
    { spend: 0.10 * s.gold }

input map Hoarder(s: Wallet) -> { spend: float } =
    { spend: 0.01 * s.gold }
```

## 5. What is legal **[designed]**

Games are full of legality rules: you can't cast a spell without the mana, buy without the coins, or move into a wall.
In Roxin those live in `allowed:`, separate from what the move does. A mana-limited combat step:

```
state Duel { mana: int, enemy_hp: int }

step Cast:
    input:
        spell: int = 3
    allowed: spell in 0..s.mana
    mechanism:
        mana: s.mana - u.spell + 1
        enemy_hp: s.enemy_hp - 2 * u.spell
```

The player casts a 3-mana spell every step and regains 1 mana. Mana goes 10, 8, 6, 4, 2. At step 5 the spell costs 3
but mana is 2, so the move is not legal and the job stops with an error that names the state and the input. The first
five rows are kept:

```
    0  { mana: 10, enemy_hp: 100 }
    1  { mana: 8, enemy_hp: 94 }
    2  { mana: 6, enemy_hp: 88 }
    3  { mana: 4, enemy_hp: 82 }
    4  { mana: 2, enemy_hp: 76 }
  stopped at step 5: admissibility violation: `spell` = 3 is not in 0..2, ...
```

(Expected output computed by the Python reference; error wording is provisional.) This is a feature, not a
limitation: a bot that tries an illegal move is a *bug in the bot*, and the model finds it.

## 6. Rules that must always hold **[designed]**

State a rule once and it is checked on the starting state and after every step. If it ever fails, the failing state is
recorded as the last row so you can inspect it, and other jobs keep running.

```
invariant NoDebt(s: Wallet) = s.gold >= 0
invariant Conserved(s: Boxes) = s.box_a + s.box_b == 10
```

The second is the marble-box rule from the periodic-robots model in the README's tour: no matter how the two robots
move marbles, the total never changes. A *duplication exploit* is a violation of exactly this kind of rule, and a sweep over
tuning values can find the setting where it first appears.

## 7. Choosing numbers

Every number in Roxin says what kind it is. For games:

- **`int` for anything you count**: coins, items, hit points, cooldown turns. Integers are exact and never overflow
  in the language (the current build stops at about 1.7×10^38 instead; see the development notes).
- **`float` for physics and rates**: speeds, probabilities, the `spend_rate` above. Floats round, so two ways of
  computing "the same" value can differ in the last digits.
- **`decimal(N)` for money-like values** when you need exact cents. **[designed]**
- Integer division is `//` (floor) and `%` is the matching modulo; `a / b` on two integers is an error, on purpose.
  Level thresholds and cooldowns are usually `//` and `%` anyway.

Do not store currency in a float. In the idle-game model above it is fine because the economy is a simulation of
tendencies; in a shop that charges real money, use `int` cents or `decimal(2)`.

## 8. Randomness and time **[designed]**

**Time.** `step` is the number of the transition being computed, starting at 1. It is readable in `input:` and
`allowed:`, which is where a periodic actor belongs. Two abilities, one every 4th step and one every 3rd:

```
step Abilities:
    input:
        fireball: int = if step % 4 == 0 then 1 else 0
        heal: int = if step % 3 == 0 then 1 else 0
    mechanism:
        fireballs: s.fireballs + u.fireball
        heals: s.heals + u.heal
```

After 24 steps: 6 fireballs and 8 heals; both fire together on steps 12 and 24. The marble-robots model has two
robots acting every 2nd and 3rd step, and the boxes reach `(5, 5)` and stay there. Each robot's decision is its own
input field, and the mechanism combines them explicitly. Nothing is merged behind your back.

**Randomness.** Dice and loot are only legal in `input:`, never in the mechanism, so the rules stay pure and
checkable. A run says how many repetitions and which seed:

```
run Banner from Gacha { since: 0, rares: 0 } for 1000 steps
    until s.rares >= 1
    runs 5000
    seed 3
```

A seed makes the run reproducible on any machine. A 0.6% drop rate with a guaranteed rare on the 90th pull needs, on
average, about 69.7 pulls to the first rare. Without the guarantee it would be about 167 (that is 1/0.006). Designers
argue about "pity" in the abstract; the model puts a number on it.

## 9. A bigger model, walked through **[designed]**

An idle clicker: each step earns 1 coin plus 1 per clicker owned. A clicker costs 10 plus 5 per clicker already owned,
and the player buys one as soon as it is affordable.

```
state Game { coins: int, clickers: int }

step Play:
    input:
        buy: int = if s.coins >= 10 + 5 * s.clickers then 1 else 0
    mechanism:
        coins: s.coins + 1 + s.clickers - u.buy * (10 + 5 * s.clickers)
        clickers: s.clickers + u.buy
```

Read it in three lines. The input is the decision rule (a greedy buyer). The mechanism is the economy. The state is
two integers. Run for 60 steps, the player makes purchases at steps 11, 19, 26, 33, 39, 45, 51 and 57, ending with 8
clickers and 39 coins. (Expected values from the Python reference; this model needs `if` and comparisons, which are
next in the build.)

Everything a designer would then ask is a one-line change: let the buyer wait until they have a 20% cushion (change the
input), make clickers cost 4 more instead of 5 (a `param`), sweep the base price. The three pieces stay separate.

## 10. Honest limits

What a game developer would expect, and what Roxin does not do:

- **It is not a game engine.** There is no rendering, no real-time input, no physics simulation. It models the
  *rules and economy* of a game, not the game.
- **Inventories, grids, and crowds need collections**, which are **[exploring]**. Today a state is a record of
  numbers, so "a list of 200 NPCs" cannot be written yet.
- **Randomness needs many runs.** One seed is one playthrough; balance questions need `runs 1000` or more.
- **Model assumptions are yours.** A model with a greedy buyer tells you about greedy buyers. Real players vary; sweep
  the behavior, not just the numbers.

## 11. Glossary: your word, the Roxin term

| Game word | Roxin |
|---|---|
| save file, world state | state |
| turn, tick, frame, round, wave | step |
| player action, command, AI decision | input |
| bot, AI policy, player profile | input map |
| rule, system, update function | mechanism |
| can't do that, illegal move, requirement | allowed |
| tuning knob, difficulty slider, balance sheet value | param |
| "never happens" bug, exploit check | invariant |
| faucet, sink, source, drain | (not a keyword: just terms in the mechanism) |
| cooldown, spawn timer, periodic event | `step % N` in an input |
| loot roll, crit chance | random draw in an input |
| playtest session, simulation batch | run, with `sweep` and `runs` |

## 12. Exercises

1. **Find the knob.** With `faucet = 100`, what `spend_rate` makes gold settle at 400? (Answer: 0.25, from
   `faucet / spend_rate`. Check with a `sweep`.)
2. **Reach the target faster.** With `spend_rate = 0.05`, gold at step 60 is 1907.86. Raise `faucet` until it passes
   2500 and see whether the shape of the curve changes.
3. **Make a sink that overshoots.** Replace the proportional spend with `spend: float = 150`. What happens to the wallet
   after the first step, and why does the job not stop? (Hint: nothing in the model forbids negative gold. Which
   section tells you how to add that rule?)
4. **Write the invariant.** In your own words, state the rule that would catch a "free item" exploit in a model you
   know. Which fields does it mention?
