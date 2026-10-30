# GDS for life sciences

*For epidemiologists, pharmacologists, ecologists, and systems biologists. No programming background assumed.*

A scope note up front: this guide is about **models of how quantities change over time** (compartment models,
reactions, dosing, populations). It is not about sequence analysis, alignment, or variant calling; those are data
pipelines, and Roxin is not one. See section 10.

Tags used below: **[runs today]**, **[designed]** (specified, not built), **[exploring]**. Runnable models are files in
[`examples/`](../../examples/).

---

## 1. Why a life scientist should care

A compartment model is already a generalized dynamical system. You keep a few quantities (susceptible, infected,
plasma concentration, prey), you have flows between them, and you step the system forward. What GDS adds is structure
around the parts that most often go wrong:

- **Mass balance is written down, not remembered.** "Susceptible plus infected plus recovered is the population." "Gut
  plus plasma plus eliminated is the dose." In a script, a typo in a flow silently creates or destroys mass. Here it is
  a checkable rule.
- **Interventions are inputs, not edits to the equations.** A vaccination program, a dosing schedule, or a harvest is
  something *chosen* each step. Keeping it separate from the biology lets you compare programs against the same
  biology.
- **Constraints are explicit.** You cannot vaccinate more people than are susceptible, or give more drug than the
  patient has tolerated. Those limits live in the model.
- **Sweeps replace spreadsheets of scenarios.** One run, many parameter values, results labeled by parameter.

## 2. The one idea: state, input, mechanism

Meet each word once; from here the guide uses the Roxin word.

| In the life sciences you say | In Roxin | What it is |
|---|---|---|
| compartments, populations, concentrations | **state** | all the quantities that change |
| rate constants, R0, half-life, carrying capacity | **param** | values fixed during a run |
| flows, infections this day, a dose, a harvest | **input** | what is chosen or arrives this step |
| the equations, the reaction law | **mechanism** | next state from old state and input |
| time point, day, generation, Euler step | **step** | one pass of the rules |
| "can't vaccinate more than the susceptible" | **allowed** | which inputs are legal in this state |
| mass balance, conservation of population | **invariant** | a rule that must hold on every state |
| intervention strategy, treatment protocol | **input map** | how the input is produced from the state |

**An input is not a change to state.** The infection flow is computed (input); the mechanism applies it, subtracting
from susceptible and adding to infected. Because the flow is computed once and used in three places, the same quantity
cannot be accidentally different in each.

**A note on time.** Roxin steps in discrete steps. One step is whatever you decide: a day, an hour, 0.1 of a time unit.
It is not an adaptive ODE solver; you choose the step size and are responsible for it being small enough (section 10).

## 3. Your first model **[runs today]**

An epidemic on population fractions. `s` is the susceptible fraction, `i` the infected, `r` the recovered.

<!-- example: sir.rox -->
```
-- An epidemic on population fractions: susceptible, infected, recovered.

param beta: float = 0.2     -- infections per infected person per step
param gamma: float = 0.1    -- recoveries per infected person per step

state Epidemic { s: float, i: float, r: float }

step Spread:
    input:
        infections: float = beta * s.s * s.i
        recoveries: float = gamma * s.i
    mechanism:
        s: s.s - u.infections
        i: s.i + u.infections - u.recoveries
        r: s.r + u.recoveries

system Outbreak: Epidemic = Spread

run Outbreak from Epidemic { s: 0.999, i: 0.001, r: 0 } for 300 steps
    sweep beta in [0.1, 0.2, 0.4]
```

Inside the step, `s` is the whole state at the start of the step, and `s.s` is its field `s` (the susceptible
fraction). The two flows are the inputs; the mechanism applies them. All three mechanism lines read the *old* state, so
the three updates are simultaneous and do not contaminate each other.

With `gamma = 0.1`, the three values of `beta` are R0 = 1, 2, and 4. Results at step 300:

| `beta` (R0) | peak infected | at step | final recovered |
|---|---|---|---|
| 0.1 (1) | 0.0010 (the starting value; it only declines) | 0 | 0.0259 |
| 0.2 (2) | 0.1577 | 70 | 0.8028 |
| 0.4 (4) | 0.4242 | 30 | 0.9846 |

Row 70 of the R0 = 2 job:

<!-- output: sir.rox -->
```
   70  { s: 0.4910898297840742, i: 0.15765906536006657, r: 0.35125110485585886 }
```

For R0 = 2, the continuous-time model's final size is 0.7968. The one-day Euler step used here gives 0.8028. The
gap is the step size, not a bug; shrink the step (and rescale `beta` and `gamma`) and the two agree. This is a
discrete-time model, and it says so.

## 4. Separate the decision from the consequence **[runs today]**

The biology is the mechanism. What you *do* about it is the input. In the model above the "decision" is just the
disease's own flows; the structure becomes useful the moment you add a human choice. A vaccination program with a daily
rate `v` would add a third input:

```
vaccinated: float = v * s.s
```

and the mechanism would subtract it from `s` and add it to `r`. The disease equations did not change. To compare
programs, sweep `v`, or **[designed]** name them as swappable `input map`s and run `NoVaccination` against `Targeted`
side by side. Sweeps over any parameter already work: the `beta` sweep above is three scenarios in one run.

## 5. What is legal **[designed]**

Constraints from the real world go in `allowed`, separate from the biology:

```
allowed: vaccinated in 0..s.s
```

A program that tries to vaccinate more people than remain susceptible is a *bug in the program*, and the model stops
with an error that names the state and the input rather than quietly producing a negative compartment. Capacity limits
(daily doses available, maximum tolerated dose) read the same way.

## 6. Rules that must always hold **[designed]**

Mass balance becomes an invariant:

```
invariant Conserved(s: Epidemic) = s.s + s.i + s.r == 1 within 0.000000001
```

The `within` is an absolute tolerance, and it is required because these are floating-point numbers: floats round, so
exact equality is meaningless and Roxin makes you state how close is close enough. The check runs on the starting state
and after every step; a violation stops that job and records the violating state as its last row.

The same applies to the pharmacokinetic model in the next section. At step 40 the three compartments sum to 100.000000000
for every value of the absorption rate. If you rewrote a flow wrongly (say, the cleared drug was subtracted from plasma
but never added to `eliminated`), the sum would drift and the invariant would catch it on the first step.

## 7. Choosing numbers

- **`float` for continuous quantities**: concentrations, fractions of a population, rates. This is most of your models.
- **`int` for counts**: cells, individuals in a small population, event counts. Integers are exact. Integer division
  is `//`; plain `/` on two ints is an error on purpose.
- **`decimal(N)` for lab quantities you want to round in a stated way** **[designed]**: a concentration reported to three
  places, a dose in whole milligrams.
- Float equality needs a tolerance (`within`), because "equal" for a rounded number is a decision. `<`, `<=`, `>`, `>=`
  never need one.

## 8. Randomness and time **[designed]**

**Time.** `step` is the step being computed, starting at 1, readable in `input:` and `allowed:`. A repeating dose is
`step % 6`:

```
step Dose:
    input:
        amount: float = if step % 6 == 1 then dose else 0
    mechanism:
        conc: s.conc * (1 - clearance) + u.amount
```

With a 100 mg dose on steps 1, 7, 13, ... and 10% cleared per step, the concentration settles into a cycle whose peak
(right after a dose) is 100 / (1 - 0.9^6) = 213.4 mg and whose trough (just before the next) is 126.0 mg. The peak and
trough are reached to four digits by step 120 (peak 213.4196). Expected values come from the Python reference.

**Randomness.** A stochastic version of the epidemic draws each infection event from the random generator, only in
`input:`, and a run says `runs 1000` and `seed 42`. The generator is specified so the same seed reproduces the same
epidemic on any machine.

## 9. A bigger model, walked through **[runs today]**

A single pill. The drug starts in the gut, is absorbed into plasma at rate `ka`, and is cleared from plasma at rate
`ke`. Three compartments, two flows.

<!-- example: oral-dose.rox -->
```
-- One 100 mg pill: gut, then bloodstream, then eliminated.

param ka: float = 0.3     -- absorption rate (gut to blood)
param ke: float = 0.1     -- elimination rate

state Body { gut: float, plasma: float, eliminated: float }

step Absorb:
    input:
        absorbed: float = ka * s.gut
        cleared: float = ke * s.plasma
    mechanism:
        gut: s.gut - u.absorbed
        plasma: s.plasma + u.absorbed - u.cleared
        eliminated: s.eliminated + u.cleared

system Pill: Body = Absorb

run Pill from Body { gut: 100, plasma: 0, eliminated: 0 } for 40 steps
    sweep ka in [0.1, 0.3, 0.6]
```

Read it in three lines: the two inputs are the flows, the mechanism moves mass between compartments, and the sweep tries
three absorption speeds. Faster absorption means a higher, earlier peak:

| `ka` | peak plasma (mg) | at step | plasma at step 40 | eliminated at step 40 |
|---|---|---|---|---|
| 0.1 | 38.742 | 9 | 6.569 | 91.953 |
| 0.3 | 63.363 | 5 | 2.217 | 97.783 |
| 0.6 | 79.800 | 3 | 1.774 | 98.226 |

In every row `gut + plasma + eliminated = 100`. Fast absorption (`ka = 0.6`) produces a peak of 79.8 mg three steps in;
slow absorption flattens and delays it. A toxicity threshold (say 60 mg) is crossed by the faster two and not by the
slow one, which is what a formulation scientist wants to see.

A second runnable model is enzyme kinetics: substrate converts to product at the saturating rate
`vmax * S / (km + S)`, and substrate plus product is conserved.

<!-- example: michaelis-menten.rox -->
```
-- An enzyme converts substrate to product at a rate that saturates: vmax * S / (km + S).
-- Substrate plus product is conserved. One step is dt = 0.1 time units.

param vmax: float = 1
param km: float = 2
param dt: float = 0.1

state Reaction { substrate: float, product: float }

step React:
    input:
        rate: float = vmax * s.substrate / (km + s.substrate)
    mechanism:
        substrate: s.substrate - u.rate * dt
        product: s.product + u.rate * dt

system Enzyme: Reaction = React

run Enzyme from Reaction { substrate: 10, product: 0 } for 100 steps
    sweep km in [0.5, 2, 8]
```

After 100 steps (10 time units), a low `km` (0.5, a tight-binding enzyme) has converted 8.91 of the 10 units; `km = 2`
converts 7.36; `km = 8` converts 4.79. Here `dt` is a parameter you can halve to check the answer does not depend on the
step size.

## 10. Honest limits

What a life scientist might expect, and what Roxin does not do:

- **It is not a bioinformatics toolkit.** No sequences, alignments, assemblies, or expression matrices. If your work is
  a data pipeline, you want a different tool.
- **It is not an ODE solver.** There is no adaptive step, no stiff solver, no error control. You choose the step size,
  and a model with fast and slow dynamics may need a very small one. A model that blows up numerically stops with an
  error (see the control guide) rather than producing `nan` quietly.
- **Spatial and age-structured models need arrays**, which are **[exploring]**. Today a state is a record of numbers, so
  a grid of 100 patches or 20 age classes cannot be written compactly yet.
- **Parameter fitting and statistics are outside the language.** Export the results and fit them in R or Python.
- **The model is your assumption.** A result says what *this* model implies, not what a population will do.

## 11. Glossary: your word, the Roxin term

| Life-science word | Roxin |
|---|---|
| compartment, population, pool, concentration | field of the state |
| state vector, system state | state |
| time step, day, generation, dt | step |
| flow, transition, event rate | input (computed in `input:`) |
| intervention, protocol, treatment arm | input map |
| rate equations, reaction law, update rule | mechanism |
| capacity, tolerance limit, "can't exceed" | allowed |
| mass balance, conservation law | invariant |
| rate constant, R0, carrying capacity | param |
| scenario analysis, sensitivity analysis | run, with `sweep` |
| stochastic replicates | `runs N` with `seed` |
| trajectory, time course | the rows of a job |

## 12. Exercises

1. **Threshold.** In the epidemic model, find a `beta` for which the infected fraction never rises above its starting
   value of 0.001. (Answer: any `beta <= gamma`, that is R0 <= 1; try `beta = 0.1`, and check that row 0 holds the
   maximum.)
2. **Mass balance by hand.** Pick any row of the `oral-dose` output and add the three compartments. Do you get 100?
3. **Step size.** Halve `dt` and double the number of steps in the enzyme model. How much does the final product move
   for `km = 2`? (It should change by well under 1%; a larger change means the step is too coarse.)
4. **Break the balance.** Edit `oral-dose.rox` so `eliminated` is never increased. Sum the compartments at step 40. How
   much of the drug has vanished, and which rule would have caught it on step 1?
