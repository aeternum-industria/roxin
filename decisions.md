# Roxin decision log

Each entry records what was decided, what else was considered, why, what it costs, and what would make us revisit it. The goal is that anyone (including the author, later) can explain and defend each choice without having been in the room.

Entries are numbered to match `docs/core-semantics.md` (D1, D2, ...). New decisions are appended. A decision is never silently edited; if it changes, add a "Revisited" note with the date and reason.

Standing principles these entries rely on (see the README):

- **Boundary rule.** A default is acceptable where a wrong default is visible and harmless. Where a wrong default could silently weaken a guarantee, the author must decide and the checker refuses to guess.
- **Verification rule.** Every feature must state how it appears to a solver, or be marked unverifiable.
- **Neutrality.** No field's vocabulary or conventions are privileged.

---

## D1. `int` is an arbitrary-precision integer

**Decision.** `int` is a mathematical integer with no fixed width. Overflow does not exist. Implementations may use a fast small-integer path with a big-integer fallback. Range-refined integers (`type U256 = int in 0..2^256-1`, `type Percent = int in 0..100`), reusing the existing `in a..b` range form, are a planned later addition; a value outside its type's range is a run-time error. They are not in v1.

**Alternatives considered.**
- Fixed width (for example 64-bit) with wraparound or error on overflow.
- Several fixed-width types (`i32`, `u64`, `u256`, and so on) from the start.

**Why.**
- Models describe mathematics, and the integers of a model should behave like integers. Overflow in a model would be an accident of the implementation, not a property of the system being modeled.
- Verification: unbounded integers map directly onto the integer theory of SMT solvers. Fixed width would force bit-vector reasoning about wraparound, which is much harder for solvers and for people.
- Tokenomics work involves values near and above 2^64 and 2^256. A fixed-width default would either break such models or force a type zoo on everyone.
- Range-refinement reuses a form the language already has (`in a..b` from `allowed`) instead of inventing a second constraint syntax, and it leaves the door open to model fixed-width contracts exactly, when that matters.

**Costs and caveats.**
- An unbounded `int` can diverge from a real contract at overflow boundaries. This is stated plainly in the guides. A model can state the bound as an invariant: `invariant NoOverflow(s: Pool) = s.balance < 2^256`.
- Big-integer fallback costs some speed; the fast path keeps the common case cheap.
- Range-refined types sit in mild tension with the boundary rule (the type silently constrains values). It is acceptable because the author writes the range, the violation is a loud error, and unbounded behavior is the exact mathematical one.

**Revisit if.** Benchmarks show the big-integer fallback matters, or real models need wraparound arithmetic (then add explicit modular operations, not a default).

---

## D2. A mechanism must account for every state field; `..s` copies the rest

**Decision.** A `mechanism:` record must set or copy every field of the state type. A field it neither sets nor copies is a check-time error. `..s` copies every unmentioned field from the old state. All expressions read the old state, so updates are simultaneous.

```
mechanism:
  temp = s.temp + k * (heat - s.temp)
  ..s
```

**Alternatives considered.** Unmentioned fields stay unchanged with no marker (implicit keep).

**Why.**
- Forgetting to update a field is exactly the silent mistake the boundary rule exists for. Under implicit keep, a typo or omission is indistinguishable from "I meant it to stay the same", and the model runs and produces plausible wrong numbers.
- `..s` is one short line, and it records an author decision ("these are passthroughs"). It forces the author to consider the whole state.
- Verification: the mechanism is a total function from old state to new state with nothing filled in by hidden defaults.

**Costs.** One extra line in models where some fields do not change.

**Revisit if.** Real models show the line is a recurring annoyance. Even then the fix would be sugar, not removal of the check.

---

## D3. Input fields are independent

**Decision.** Input fields do not read each other (they are evaluated in declaration order, but an expression may not refer to a sibling field). The admissible set is the Cartesian product of per-field membership sets, filtered by any joint predicates. Any coupling between fields is an explicit predicate.

**Alternatives considered.** Coupled from the start: one combined rule defines the admissible set, with no per-field structure.

**Why.**
- Enumeration: reachability must list every admissible input. A product of finite per-field sets is directly walkable, and predicates just filter it. A single combined rule would require a search for valid combinations.
- Proofs: per-field ranges and predicates become plain linear constraints.
- Readability and determinism: the common case ("each decision has its own limits") reads directly, random-draw order is simple, and coupling is visible when it exists.
- It matches how the GDS paper treats the input as one space with an admissible subset.

**Costs.** A coupled rule such as "buy and sell cannot both be nonzero" needs a predicate on top of the per-field sets.

**Revisit if.** Composition of subsystems (milestone M9) needs inputs that depend on each other across subsystems.

---

## D4. NaN, infinity, and division by zero stop the job

**Decision.** A NaN or infinite float result, or division by zero of any numeric type, is an arithmetic error. The job stops, the error is recorded with the step, field, and expression, and other jobs in the sweep continue. There is no propagate mode.

**Alternatives considered.**
- Follow IEEE: let NaN and infinity flow through later steps.
- Configurable: stop by default, opt in to propagate.

**Why.**
- NaN spreads silently. One early bad division turns every later value into NaN, and every comparison with NaN is false, so an invariant such as `x >= 0` can pass or fail for the wrong reason. That is a wrong default that is invisible.
- Verification: solvers cannot reason about NaN the way they reason about numbers. If the checker can prove no division by zero is reachable, that is a clear guarantee.
- `int` and `decimal` have no IEEE fallback, so division by zero is an error for them under every option. Making floats consistent is simpler to explain.

**Costs.** A model that deliberately uses infinity (for example a "never" sentinel) must encode it differently.

**Revisit if.** A real model needs infinity as a value. Add an explicit opt-in then; do not build it speculatively.

---

## D5. Integer division is `//` (floor) and `%` (floored modulo); `/` on ints is a type error

**Decision.** `//` is floor division and `%` is floored modulo, so `a == (a // b) * b + a % b` always holds and `%` takes the sign of the divisor. `7 // 2 = 3`, `-7 // 2 = -4`, `-7 % 2 = 1`. Plain `/` on two `int` values is a check error ("use `//`, or convert to `decimal` or `float`"). `//` and `%` by zero are errors (D4).

**Alternatives considered.**
- Truncated division (C, Rust, Java), where `-7 / 2 = -3` and remainders can be negative.
- `/` on ints silently floors, truncates, or returns a float or rational.

**Why.**
- Languages disagree on `/` for integers, so any silent choice is a trap, and in tokenomics the rounding direction is money. This is the same reasoning as the decimal rule that division needs an explicit `round`.
- Floored modulo keeps results non-negative for a positive modulus, which is what "step mod 7" and wraparound indexing want. It matches Python, which many modelers already know.
- Verification: both operations are expressible in integer arithmetic, and floor division by a constant stays linear.

**Costs.** Slightly different from the hardware default. Not meaningful here.

**Revisit if.** There is a strong reason to match Rust's truncation (for example interoperating with generated Rust code). The author noted this can be revisited.

---

## D6. Relative tolerance is measured against the larger magnitude

**Decision.** `a == b within p%` means `|a - b| <= (p / 100) * max(|a|, |b|)`. A purely relative tolerance never makes a nonzero number equal to 0.

**Alternatives considered.** Relative to the second operand (`|a - b| <= tol * |b|`) or to the first.

**Why.**
- Symmetry: equality must not depend on argument order. Measuring against one operand makes `a == b` and `b == a` disagree near the boundary. With the symmetric test, reachability does not depend on which state was found first.
- No division, so no divide-by-zero case to define.
- Zero is deliberately exact under a relative tolerance: relative error against zero is meaningless (every nonzero number is infinitely far away in relative terms). Where "near zero" is meant, the author states an absolute tolerance. This follows the boundary rule: the author decides.

**Costs and caveats.**
- Tolerance equality is not transitive (`a ≈ b` and `b ≈ c` do not imply `a ≈ c`). Reachability therefore needs a deterministic rule for merging near-equal states (for example, keep the first one found). That rule is **not yet decided**.
- Near-equal states on opposite sides of a rounding boundary may fail to merge. That costs search size, never correctness.

**Revisit if.** The merge rule for reachability is designed, or users ask for a combined relative-or-absolute form as a built-in.

---

## D7. Random numbers use pinned algorithms so runs reproduce exactly

**Decision.**
- **Generator:** xoshiro256++.
- **Seeding:** SplitMix64 expands the 64-bit job seed into the generator's four state words. Job j's seed is `splitmix64(base ^ j)`, so parallel jobs never share a stream and results do not depend on thread scheduling.
- **Base seed:** if the run gives none, the runtime draws one from the operating system and records it in the results.
- **Uniform floats:** `u01 = (next_u64 >> 11) * 2^-53`, exactly representable.
- **Normal draws:** Box–Muller with a fresh pair on each call and no caching: `z = sqrt(-2 ln(1 - u1)) * cos(2π u2)`, then `mu + sigma * z`.
- **Math functions** (`ln`, `cos`, `sqrt`, and so on): a pinned pure-Rust `libm`, not the platform's.

**Alternatives considered.** Use whatever generator and math library the platform or a convenient crate provides; allow the implementation to choose and merely record the name.

**Why.**
- "Same seed, same result" is only a promise if it holds across machines, across Roxin versions, and across the bytecode VM and the tree-walking oracle (which must agree on every test).
- Platform math libraries can differ in the last bit between operating systems and CPUs, which would silently break reproducibility across machines.
- xoshiro256++ is fast, statistically strong, and short enough to specify exactly. SplitMix64 is the standard way to seed it. Box–Muller is simple to specify exactly.
- Per-job derived streams make parallel runs reproducible regardless of thread count.

**Costs.** Box–Muller (two uniforms per pair of normals) and a pinned `libm` are slower than the platform's best.

**Revisit if.** Benchmarks show sampling is a bottleneck. A different generator or sampler would be a versioned change, recorded in the results file so old results remain interpretable.

---

## D8. A violating state is recorded as the job's final row, flagged

**Decision.** When an invariant is false after a step, the job stops and the violating state is saved as the job's last row, with a flag and the name of the violated invariant. Nothing after it is recorded. For admissibility, arithmetic, or data errors no new state exists, so the last row is the previous state and the error record carries the offending state and input.

**Alternatives considered.** Leave the violating state out: the last saved row is the last good state and the violation appears only in the error record.

**Why.**
- Debugging: the first question after a violation is "what did the state look like when it broke?" With the row saved you can inspect it and compare it with the row before. Without it, you only know the previous state and the step number and must rerun to see the bad state.
- Honesty: the state exists, because the mechanism produced it. The flag marks it as not a legitimate part of the trajectory, so analysis can filter it with one condition.
- Invariant violations are the one failure where a new state exists, so recording it is consistent with how the other failures are treated.

**Costs.** Analysis must know about the flag, for example when averaging a metric across jobs. The flag is a column in every row, so filtering is one condition. A results file with a flagged final row must be read with the error table in mind.

**Revisit if.** Users routinely find the flagged row gets in the way of analysis, in which case the default could change while keeping the row available through an option.

---

## D9. Decimal multiplication grows precision; division only appears inside `round`

**Decision.**
- `decimal(N) * decimal(M)` is exact and has type `decimal(N+M)`. Example: `1.25 * 0.5` is `decimal(2) * decimal(1)`, giving `decimal(3)` and the value `0.625`.
- `decimal(N) * int` is exact and stays `decimal(N)`.
- `+` and `-` on `decimal(N)` are exact and stay `decimal(N)`.
- To store a product in a `decimal(N)` field, the author writes `round(e, mode)` with `down`, `up`, or `nearest`.
- `/` on decimals is legal only as the direct argument of `round`, as in `round(a / b, down)`. The result takes the precision of the field or annotation it is stored in.

**Alternatives considered.**
- Multiplication silently rounds back to a fixed precision with a default mode.
- Decimals carry unlimited precision and division produces a repeating or rational value.
- Allow `/` anywhere and round at the end.

**Why.**
- Exactness is the point of `decimal`. If multiplication silently rounded, `decimal` would offer little over a float in the cases it exists for (money, token amounts).
- Rounding direction is a policy: it decides who gets the leftover fraction, and over thousands of steps it adds up. Writing it where it happens puts the policy in the model and the audit trail. This is the boundary rule.
- Most divisions have no finite exact result (`1 / 3`), so the rounding mode has to be part of the operation, not applied afterwards.
- Verification: a decimal is an integer scaled by a power of ten, so decimal arithmetic maps onto the solver's integer theory.

**Costs.** Verbosity. Chains of multiplications grow precision quickly (three products of `decimal(2)` are `decimal(6)`), so models often write `round` after each product. This is judged the right default for this audience because the alternative hides the rounding.

**Revisit if.** Real models show a pattern of `round` boilerplate worth a named helper, which would be sugar over the same rules.

---

## D10. Order of checks in a run

**Decision.**
1. Check invariants on the initial state before step 1. A violation stops the job and records the start state as the flagged row (D8).
2. Before each step, test `until` on the current state. If it holds, the run ends cleanly and that state is the final row.
3. After each step, check invariants on the new state.

If a state both violates an invariant and satisfies `until`, the violation wins. After the last step of `for N steps`, the run ends without testing `until` again. A step that raises an arithmetic, admissibility, or data error never reaches the post-step invariant check, because no new state exists.

**Alternatives considered.** Skip the initial-state check and only check after steps; test `until` after each step instead of before.

**Why.**
- A bad start should fail loudly. A model that starts outside its own invariants (a typo in a parameter or initial value) would otherwise run on, or never report the problem. It also matches the proof pattern: an inductive invariant must hold at the start before it can be shown to be preserved.
- Testing `until` before each step matches the paper's stopping rule (`s = A(x_{k-1}) ∨ (k ≥ K)`): the decision to stop is made on the state already in hand. An `until` already true at the start does zero steps, which is what makes a state absorbing.
- Invariants take priority over `until` so a stopping rule cannot hide a broken state.
- `for N steps` is the `k ≥ K` part of the same rule; once step N has run, no further step would have run anyway.

**Costs.** One extra invariant evaluation per job on the initial state. Negligible.

**Revisit if.** A real model needs `until` to be evaluated on the post-step state for a different reason (for example, stopping after a state that triggers but does not break an invariant).

---

## Project policies

### P1. The implementation is written entirely by hand

**Policy.** All code in the Roxin repository (the Rust implementation, tests, and tooling) is written by the author, by hand. AI assistance is used for documentation, brainstorming, design review, and explanation. AI may show illustrative snippets and examples in conversation, including Rust language and library examples, but does not write code that goes into the repository. This may change in the future; if it does, it will be recorded here with a date.

**Why.**
- The primary purpose of the project is learning how languages are designed and implemented. Many properties of the design only become clear when the author writes the code and formalizes the relationships.
- The author wants to be able to state plainly that the language was implemented entirely by hand.

**What this means in practice.**
- Documents in `docs/` and the README may be drafted with AI help. Code never is.
- The "illustrative Rust types" in `docs/core-semantics.md` are documentation of the intended shape, not source to copy; the implementation is the author's own.
- Review is welcome: the author may paste their code and ask for explanations, bug hunting, or Rust idiom advice, and then make the changes themselves.

---

## Open items that came out of these decisions

- Reachability: deterministic rule for merging near-equal float states (from D6).
- Range-refined integer syntax and checking details (from D1).
- Solver-facing verification story table for the README (from the verification rule).
- All of D1 to D10 in `docs/core-semantics.md` are settled; the semantics draft still needs a full read-through by the author (milestone M0).
