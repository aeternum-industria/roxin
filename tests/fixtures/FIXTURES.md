# Roxin test fixtures

52 models: **32 run on the interpreter you have today** (`today/`) and **20 are written ahead of the
features they need** (`pending/`). Each `.rox` has a header that states its role, status, and the oracle (the
independently derived right answer). Each `.expected` file holds the output in the same format `print_job` prints.

Unzip this into `tests/fixtures/`. If your existing `thermostat.rox` and `predator-prey.rox` are byte-identical
to `today/thermostat.rox` and `today/predator-prey.rox` (`diff` them), delete your top-level copies; `marbles.rox`
lives in `pending/` for the same reason.

## How the expected outputs were produced and checked

- `today/*.expected` came from running the interpreter. The 21 numeric models were then compared against
  `tools/reference.py`, a separate Python implementation written from each model's mathematics (not from the
  Roxin source); all agree (ints exactly, floats to 1e-9 relative). Rerun any time:
  `python3 tools/reference.py check .`. The error-message fixtures (`missing_mechanism_field`, `int_slash`, and
  so on) are checked by their stop step and by reading the message against the spec.
- `pending/*.expected` came from the Python reference alone. They are what a correct Roxin should print once the
  feature lands. Nothing has run them yet, so treat a disagreement as "investigate", not "Roxin is wrong".
- `tools/gen_stress.py today` regenerates the two stress models.

## Comparison rules for a harness

| Header says | Compare |
|---|---|
| `exact` | text-for-text, or parse and compare values exactly |
| `float tolerance 1e-9 relative` | same job/row structure; ints exact; floats within 1e-9 relative (and 1e-12 absolute) |
| `rows exact; message provisional` | same rows and same stop step; do not compare the error wording yet |
| `statistical` | no `.expected`; run and check the headline number in the header |
| `none` | stress model: no output comparison; time it |

Error wording in the `today/` negative fixtures is the interpreter's current wording. Change it freely, then
regenerate the `.expected` file on purpose (review the diff).

## Things worth knowing before you trust a file

- **`factorial_overflow`** documents a divergence from the spec: D1 says `int` has no overflow, but the current
  evaluator stops at 34! (i128). When big integers land, swap in `factorial_overflow.expected-d1`.
- **`precedence`** rows `b` and `c` encode a rule not yet in `docs/core-semantics.md` (`^` is right-associative
  and binds tighter than unary minus). Record that decision before relying on them.
- **`until`, `runs`, `seed`, and `uniform(...)`** spellings in `pending/` are guesses; the spec names the ideas but
  not the syntax. Edit the `.rox` files when the syntax is fixed.
- **`random_walk`** depends on how D7 is read. The reference assumes `job_seed(j)` is the first SplitMix64 output
  started at `base xor j`, and that xoshiro256++ takes the first four outputs of a SplitMix64 stream started at
  the job seed. If your implementation reads it differently, decide which reading is intended and record it.
- **`amm_fee_zero`** is meant to fail: it pins a case where float rounding alone breaks `x * y >= k0`.
- Error text in `pending/*.expected` (admissibility, invariant) is a placeholder.

## `today/` (run now)

| File | Role | Status | Compare | Golden output |
|---|---|---|---|---|
| `bad_step_count_float.rox` | negative | runnable today | exact (error text) | yes |
| `bad_step_count_negative.rox` | negative | runnable today | exact (error text) | yes |
| `big_sweep.rox` | stress | runnable today | none | no (timing only) |
| `bitcoin_halving.rox` | golden | runnable today | exact | yes |
| `compound_interest.rox` | golden (sweep) | runnable today | float tolerance 1e-9 relative | yes |
| `doubling.rox` | golden | runnable today | exact | yes |
| `factorial.rox` | golden | runnable today | exact | yes |
| `factorial_overflow.rox` | negative (documents a KNOWN DIVERGENCE from the spec) | runnable today | exact | yes |
| `fibonacci.rox` | golden | runnable today | exact | yes |
| `field_set_twice.rox` | negative | runnable today | exact (error text) | yes |
| `float_div_zero.rox` | negative | runnable today | exact (error text) | yes |
| `float_into_int.rox` | negative | runnable today | exact (error text) | yes |
| `float_overflow.rox` | negative | runnable today | exact (error text included) | yes |
| `input_reads_input.rox` | negative | runnable today | exact (error text) | yes |
| `int_slash.rox` | negative | runnable today | exact (error text) | yes |
| `logistic_map.rox` | golden (sweep) | runnable today | float tolerance 1e-9 relative (chaotic job: compare only a few rows, errors grow) | yes |
| `long_run.rox` | stress | runnable today | none | no (timing only) |
| `michaelis_menten.rox` | conservation | runnable today | float tolerance 1e-9 relative | yes |
| `missing_mechanism_field.rox` | negative | runnable today | exact (error text) | yes |
| `newton_cooling.rox` | golden | runnable today | float tolerance 1e-9 relative | yes |
| `oral_dose.rox` | conservation | runnable today | float tolerance 1e-9 relative | yes |
| `pid.rox` | golden (sweep) | runnable today | float tolerance 1e-9 relative | yes |
| `precedence.rox` | torture + golden | runnable today | exact | yes |
| `predator-prey.rox` | ? | ? | ? | yes |
| `radioactive_decay.rox` | golden (sweep) | runnable today | float tolerance 1e-9 relative | yes |
| `sir.rox` | conservation + golden (sweep) | runnable today | float tolerance 1e-9 relative | yes |
| `syntax_torture.rox` | torture | runnable today | exact | yes |
| `thermostat.rox` | ? | ? | ? | yes |
| `unknown_mechanism_field.rox` | negative | runnable today | exact (error text) | yes |
| `unknown_sweep_param.rox` | negative | runnable today | exact (error text) | yes |
| `unstable_pendulum.rox` | negative (D4 divergence) | runnable today | exact rows up to the stop step, error kind only | yes |
| `water_tank.rox` | golden | runnable today | float tolerance 1e-9 relative | yes |

## `pending/` (written ahead of the features)

| File | Role | Needs | Compare | Golden output |
|---|---|---|---|---|
| `amm_constant_product.rox` | conservation | `allowed`, `invariant`, `>=` | float tolerance 1e-9 relative | yes |
| `amm_fee_zero.rox` | negative (float rounding vs invariant) | `allowed`, `invariant`, `>=` | exact | yes |
| `bang_bang.rox` | golden | if/then/else, comparisons, bool state, `true`/`false` | float tolerance 1e-9 relative, bool exact | yes |
| `bonding_curve.rox` | conservation | `invariant`, `within` | float tolerance 1e-9 relative | yes |
| `collatz.rox` | golden | if/then/else, ==, %, `until` | exact | yes |
| `combat_allowed.rox` | negative (admissibility) | `allowed` and its error | rows exact; error text provisional (match on "admissib" / step number) | yes |
| `cooldowns.rox` | golden | if/then/else, ==, %, `step` | exact | yes |
| `gacha_pity.rox` | golden (statistical) | `uniform`, `runs`, `seed`, `until`, `or`, if/then/else | statistical | no (timing only) |
| `gamblers_ruin.rox` | golden (statistical) | `uniform`, `runs`, `seed`, `until`, `or` | statistical | no (timing only) |
| `idle_clicker.rox` | golden | if/then/else, >= | exact | yes |
| `invariant_beats_until.rox` | negative (D10) | `invariant`, `until`, comparisons | rows exact; message provisional | yes |
| `invariant_violation.rox` | negative (D8) | `invariant`, `<` | rows exact; message provisional | yes |
| `marbles.rox` | ? | ? | ? | yes |
| `mm1_queue.rox` | golden (statistical) | `uniform`, `runs`, `seed`, `max` | statistical | no (timing only) |
| `random_walk.rox` | golden (RNG, D7) | `uniform`, `runs`, `seed`, if/then/else | exact | yes |
| `repeated_dosing.rox` | golden | if/then/else, ==, %, `step` | float tolerance 1e-9 relative | yes |
| `sir_invariant.rox` | conservation | `invariant`, `within`, `metric` | float tolerance 1e-9 relative | yes |
| `thermostat_allowed.rox` | negative (admissibility), M4 acceptance test | `allowed` and its error | rows exact; error text provisional | yes |
| `token_vesting.rox` | conservation | `step`, `invariant`, int `//`, `==` | exact | yes |
| `until_already_true.rox` | golden (D10) | `until`, `>=` | exact | yes |

## Suggested order for turning `pending/` on

1. Part 4e (`if/then/else`, comparisons, `and`/`or`, calls, `step`): unlocks `marbles`, `collatz` (with `until`),
   `cooldowns`, `idle_clicker`, `repeated_dosing`, `bang_bang`.
2. `allowed` and its error: `thermostat_allowed` (your M4 acceptance test), `combat_allowed`.
3. `invariant` / `until` / `metric`: `invariant_violation`, `until_already_true`, `invariant_beats_until`,
   `token_vesting`, `sir_invariant`, `bonding_curve`, `amm_*`.
4. RNG: `random_walk` (bit-exact), then the three statistical models.
