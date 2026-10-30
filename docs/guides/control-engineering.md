# GDS for control and engineering

*For control, mechanical, and electrical engineers, and anyone who has tuned a loop. No new math required: if you have
written `x[k+1] = f(x[k], u[k])`, you already know the shape.*

Tags used below: **[runs today]**, **[designed]** (specified, not built), **[exploring]**. Runnable models are files in
[`examples/`](../../examples/).

---

## 1. Why a control engineer should care

Discrete-time state-space is a generalized dynamical system with the pieces named differently. The reason to use
Roxin for it is what you get around the loop:

- **The controller is a replaceable part.** P, PI, PID, bang-bang, and an LQR gain are all just ways of producing the
  input from the state. Swapping one for another without touching the plant is the point of the structure.
- **Actuator limits are part of the model.** "The heater can't exceed 5" is a statement about which inputs are legal,
  and a controller that asks for 15 is flagged, not silently clipped.
- **Safety envelopes can be written down.** "Temperature never exceeds 30" is a rule checked on every step.
- **Sweeps are loop tuning.** One run, many gains, results labeled by gain.
- **Reachability is verification.** The same model that simulates can be asked what the *worst legal* sequence of inputs
  does, which is the question a safety case asks. (Reachability is **[designed]**; this guide says when it matters.)

## 2. The one idea: state, input, mechanism

Meet each word once; from here the guide uses the Roxin word.

| In control you say | In Roxin | What it is |
|---|---|---|
| state vector `x`, plant state | **state** | all the quantities that change |
| gains, setpoint, plant constants | **param** | values fixed during a run |
| control input `u`, actuator command | **input** | what is chosen or arrives this step |
| plant dynamics `x[k+1] = f(x, u)` | **mechanism** | next state from old state and input |
| sample period, tick, iteration `k` | **step** | one pass of the rules |
| actuator limits, saturation bounds | **allowed** | which inputs are legal in this state |
| safety envelope, stability margin | **invariant** | a rule that must hold on every state |
| feedback law, controller `u = g(x)` | **input map** | how the input is produced from the state |

**An input is not a change to state.** The controller *commands* a heat of 5 (input); the plant's physics decides what
that does to the temperature (mechanism). Control engineers know this as the controller/plant split; Roxin makes the
split the structure of the language.

**A note on time.** Roxin steps in discrete steps: your sample period. It is not a continuous-time solver. For a
continuous plant you discretize it yourself (Euler, zero-order hold) and choose the step; the model below does.

## 3. Your first model **[runs today]**

A proportional thermostat. The setpoint is 20 degrees; the controller commands `k` times the error; the plant adds the
commanded heat.

<!-- example: thermostat.rox -->
```
-- Thermostat. Provisional syntax.
-- Start 10, setpoint 20, 8 steps. All values are exact binary fractions,
-- so float arithmetic reproduces them exactly (tolerance 1e-12 is still fine).
--
--   k      rows 0-3                        row 8
--   0.25   10, 12.5, 14.375, 15.78125      18.998870849609375
--   0.5    10, 15, 17.5, 18.75             19.9609375
--   1.5    10, 25, 17.5, 21.25             19.9609375
--
-- Variant with `allowed: heat in 0..5` on the step (add it under `input:`):
--   k = 0.5: first heat is 5 (boundary, legal).
--   k = 1.5: first heat is 15, an admissibility error at step 1 with temp = 10.

param setpoint: float = 20
param k: float = 0.5

state Room { temp: float }

step Heating:
    input:
        heat: float = k * (setpoint - s.temp)    -- the decision
    mechanism:
        temp: s.temp + u.heat                    -- the physics

system Thermostat: Room = Heating

run Thermostat from Room { temp: 10 } for 8 steps
    sweep k in [0.25, 0.5, 1.5]
```

`input:` is the controller; `mechanism:` is the plant. One sweep, three gains. Row 8 of each job:

| `k` | row 8 | behavior |
|---|---|---|
| 0.25 | 18.998870849609375 | slow monotone approach |
| 0.5 | 19.9609375 | monotone, error halves every step |
| 1.5 | 19.9609375 | overshoots (10, 25, 17.5, 21.25, 19.375, 20.3125, ...) then rings down |

Why: the error `e = setpoint - temp` becomes `(1 - k) · e` each step. For `0 < k < 1` it shrinks without changing
sign; for `1 < k < 2` it flips sign and still shrinks; at `k = 2` it flips forever; above 2 it grows. You can see all
four regimes by sweeping `k`.

## 4. Separate the decision from the consequence **[runs today]**

Replace the proportional rule with a PID controller and leave the plant alone. The plant here just adds the force to
the position; the controller reads the error, its running sum, and its last change:

<!-- example: pid.rox -->
```
-- A PID controller driving a position to the setpoint 10, at three proportional gains.

param kp: float = 0.2
param ki: float = 0.05
param kd: float = 0.1
param setpoint: float = 10

state Loop { pos: float, integral: float, prev_err: float }

step Control:
    input:
        force: float = kp * (setpoint - s.pos) + ki * s.integral + kd * ((setpoint - s.pos) - s.prev_err)
    mechanism:
        pos: s.pos + u.force
        integral: s.integral + (setpoint - s.pos)
        prev_err: setpoint - s.pos

system Pid: Loop = Control

run Pid from Loop { pos: 0, integral: 0, prev_err: 0 } for 60 steps
    sweep kp in [0.2, 0.5, 1.2]
```

The controller's three terms live in the `input:` expression. The mechanism both moves the position and updates the
controller's memory (the integral and the last error are part of the state). Because an input cannot read another
input, the error is recomputed where it is needed; that is deliberate, and it keeps the legal-input set enumerable.

Peak position and the value at step 60, by proportional gain:

| `kp` | peak position | position at step 60 |
|---|---|---|
| 0.2 | 14.01 | 9.895 |
| 0.5 | 11.37 | 10.0005 |
| 1.2 | 13.00 | 10.027 |

In this setup the middle gain overshoots least. The sweep shows the effect in one run; explaining it (integral
windup at low gain, a large first kick at high gain) is a good exercise.

## 5. What is legal **[designed]**

Actuator limits belong in `allowed`, separate from the controller:

```
step Heating:
    input:
        heat: float = k * (setpoint - s.temp)
    allowed: heat in 0..5
    mechanism:
        temp: s.temp + u.heat
```

For `k = 0.5` the first heat is 5, exactly on the boundary (ranges include their endpoints), and the run finishes. For
`k = 1.5` the first heat would be 15, so the job stops at step 1 with an error that names the state (`temp: 10`) and the
input (`heat = 15`).

This is a deliberate choice. In practice an actuator *saturates*, and you may want the controller to clamp its
command. The clamp belongs in the controller (`input:`, once `min` and `max` exist). `allowed` says what is *legal*, so
a controller that asks for more than the actuator can do is flagged as a bug rather than quietly masked. Use whichever
your model means.

## 6. Rules that must always hold **[designed]**

```
invariant Safe(s: Room) = s.temp < 30
invariant Warm(s: Room) = s.temp > 0
```

Checked on the starting state and after every step; a violation records the violating state as the job's last row and
stops that job only. Add a safety envelope and sweep the gain: the first gain that breaches it is the answer to "what
is the largest gain I can use here?".

## 7. Choosing numbers

- **`float` for plant states and gains**: it is IEEE double precision, the same as most control software on a PC. Float
  equality needs a tolerance (`within`), because "is the output at the setpoint?" is a question about how close.
- **`int` for fixed-point and embedded controllers.** If your firmware uses 16.16 fixed point, model it as `int` with
  `//` for the shift and see the quantization directly. Integers do not overflow in the language, so state the word
  size as an invariant (`invariant Fits(s: Loop) = s.pos < 2^31`).
- **`decimal(N)`** is rarely what you want here **[designed]**.
- Division of two ints is an error on purpose; use `//` for fixed-point shifts.

## 8. Randomness and time **[designed]**

**Time.** `step` is the step number, starting at 1, readable in `input:` and `allowed:`: use it for a setpoint ramp or
a scheduled disturbance.

**Noise and disturbances** are random draws, legal only in `input:`:

```
measured: float = s.temp + normal(0, 0.1)
```

A run with `runs 1000` and `seed 42` repeats the experiment; the generator is specified, so the same seed reproduces
the run on any machine. Monte Carlo over noise realizations is how you estimate a loop's tail behavior.

**Bang-bang with hysteresis.** A heater that switches on below 18 and off above 22, and otherwise keeps its setting.
`on` is a boolean state field:

```
state House { temp: float, on: bool }

step Thermostat:
    mechanism:
        on: if s.temp < low then true else if s.temp > high then false else s.on
        temp: s.temp + (if s.on then heat_gain else 0.0) - loss * (s.temp - outside)
```

With `low = 18`, `high = 22`, `heat_gain = 2`, `loss = 0.1`, `outside = 5`, starting at 15 with the heater off, the
heater switches 13 times in 100 steps and the temperature swings between about 16.4 and 22.6 once it is cycling. The
swing is wider than the 18 to 22 band because the switching decision uses the *previous* state. That is the discrete
delay every sampled controller has, and here it is visible. (Expected values from the Python reference.)

## 9. A bigger model, walked through **[runs today]**

A numerical failure caught early. An inverted pendulum, linearized, with a step size that is far too large:

<!-- example: unstable-pendulum.rox -->
```
-- An inverted pendulum stepped with a far-too-large step size: the numerics blow up.
-- The growth factor per step is about 1000, so the angle passes the largest float and the job stops.

param dt: float = 1
param g_over_l: float = 1_000_000

state Pendulum { theta: float, omega: float }

step Fall:
    mechanism:
        theta: s.theta + dt * s.omega
        omega: s.omega + dt * g_over_l * s.theta

system Inverted: Pendulum = Fall

run Inverted from Pendulum { theta: 0.01, omega: 0 } for 400 steps
```

An unstable plant with an explicit Euler step grows geometrically. The first rows:

<!-- output: unstable-pendulum.rox -->
```
    0  { theta: 0.01, omega: 0 }
    1  { theta: 0.01, omega: 10000 }
    2  { theta: 10000.01, omega: 20000 }
```

By step 102 the angle is about 10^304; at step 103 the next value would pass the largest representable float, and the
job stops with:

<!-- output: unstable-pendulum.rox -->
```
  stopped at step 103: arithmetic error: the result is not a finite number
```

Rows 0 to 102 are kept for inspection, and in a sweep the other jobs would continue. In Python this silently becomes
`inf`, then `nan`, and a plot that is blank for no obvious reason; here the failure is stated and located. The fix a
numerical analyst would reach for (a smaller `dt`, or an implicit step) is an edit to one parameter or one expression.

## 10. Honest limits

- **It is not a continuous-time simulator.** No ODE solvers, no adaptive step, no event detection. You discretize the
  plant and own the step size.
- **No frequency-domain tools.** Bode plots, root loci, and transfer functions are outside the language. Export the
  trajectories and analyze them with your usual tools.
- **Vectors and matrices need collections**, which are **[exploring]**. Today a state is a record of scalars, so an
  LQR gain applied to a 6-state plant is six lines, not one matrix product.
- **No hardware loop, no real-time guarantees.** It is a modeling language.
- **A passing model is not a safety case.** Invariants checked on simulated runs show the absence of violations in the
  runs you did. Exhaustive checking over all legal inputs (reachability) is **[designed]** and, for floating-point
  plants, comes with stated caveats.

## 11. Glossary: your word, the Roxin term

| Control word | Roxin |
|---|---|
| plant state, state vector | state |
| sample, tick, iteration `k` | step |
| control input, command, actuation | input |
| controller, feedback law, policy | input map |
| plant dynamics, process model | mechanism |
| actuator limits, constraints, saturation bounds | allowed |
| safety envelope, stability margin, "never exceeds" | invariant |
| gain, setpoint, plant constant | param |
| gain sweep, parameter study | run, with `sweep` |
| noise, disturbance realization | random draw in an input; `runs N` with `seed` |
| step response, trajectory | the rows of a job |
| divergence, instability | a job stopped by a non-finite number, or an invariant |

## 12. Exercises

1. **Four regimes.** In the thermostat, sweep `k` over `[0.5, 1.5, 2.0, 2.5]` and look at row 8. For which `k` is the
   error the same size at row 8 as at row 0? Which grow? (Answer: error is multiplied by `(1 - k)` per step, so `k = 2`
   keeps its size and `k = 2.5` grows.)
2. **Tune it.** Using the PID model, find a `kp` in `[0.2, 1.2]` that keeps the peak under 11.5. (The sweep points tried
   are 0.2, 0.5, and 1.2; 0.5 gives a peak of 11.37.) Try values around it.
3. **Catch it.** Change `g_over_l` in the pendulum to `100` and `dt` to `0.1`. Does the job still fail? (Answer: no. The
   growth is about 2 per step instead of 1000, so 400 steps reach only about 10^118, which is finite. The plant is just
   as unstable; a smaller step only delays the blow-up. What does that tell you about reading "the run finished"?)
4. **Write the envelope.** For a model you know, write the safety rule as one line. Which state fields does it name,
   and what tolerance, if any, does it need?
