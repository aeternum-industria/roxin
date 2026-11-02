# GDS Part I concepts across cadCAD, gds-core, and Roxin

Every concept in the paper, with how each of the three implementations handles it. Built from the sources:
`cadCAD-org/cadCAD@master` (4,869 lines), `DynamicalSystemsGroup/gds-core@dev` (~103k lines), the Roxin working
tree (1,934 lines), and the Part I preprint.

Legend: **✓** implemented · **◐** partial or indirect · **[d]** designed, not built · **✗** absent · **—** not
applicable

---

## Framing: the thesis this table supports

Your diagnosis — that a framework wanting rigidity inside a language that lets modellers do anything is a
losing position — is visible in gds-core's source as a structural compromise, not just a performance one. Three
pieces of evidence:

1. **`constraints.py` splits every construct in two.** The dependency graph is a typed Pydantic model; the
   actual function is `Callable | None`, documented as "lossy in serialization." The rigidity stops exactly
   where Python begins.
2. **`gds-proof` cannot read the model it proves things about.** It re-derives behaviour in SymPy through an
   adapter, because the `Mechanism` callable is opaque. The prover and the simulator run on two different
   representations of the same model, with nothing enforcing that they agree.
3. **Reachability is sampling, not enumeration.** `gds_analysis/reachability.py` cites Definition 4.1 correctly,
   then says it "operates on the sampled graph, not the true transition structure," with "no coverage
   guarantees" for continuous spaces. It cannot enumerate `U_x` because `U_x` is a Python predicate it cannot
   invert.

None of these are mistakes by the author. They are what Definition 4.1 costs when `f` is a closure. Roxin's
`allowed:` is enumerable precisely because it is syntax rather than a callable — which is the whole argument
for a language, stated in one row of the table below.

*One caution on the speed claim:* your own development notes say the prototype numbers measure "framework
overhead on a trivial model" and should be re-measured on the real build before anyone quotes them. The 1,000-job
sweep figure (25.4 s → 0.35 s) is the one closest to 60×. I would keep that caveat attached until the harness
can reproduce it, because it is the first number a skeptical reader will attack.

---

## Table 1 — Every concept in the paper

### Section 2: Notation and definitions

| # | Concept (paper ref) | cadCAD | gds-core | Roxin |
|---|---|---|---|---|
| 1 | **State Space** `X` (2.1) | ◐ `State = Dict[str, object]`, untyped | ✓ `Entity` + variables, `Space` of `TypeDef` | ✓ `state` record, every field typed |
| 2 | **State** `x` (2.2) | ◐ a dict instance | ✓ entity instance | ✓ the record value |
| 3 | **Trajectory** (2.3) | ✓ result DataFrame rows | ✓ `gds_sim.Results` | ✓ job rows, N+1 incl. initial |
| 4 | **Input Space** `U` (2.4) | ◐ `PolicyOutput = Dict[str, object]`, implicit | ✓ typed signal spaces | ✓ declared `input:` fields |
| 5 | **Input** `u` (2.4) | ◐ merged policy dict | ✓ signal value | ✓ the `u` record |
| 6 | **Stochastic influence as input** (2.4 note) | ◐ `env_process`, plus ad-hoc `np.random` in policies | ✗ stochastic extensions are issue #169, Tier 3, unbuilt | **[d]** `sample` legal only in `input:` (D7) |
| 7 | **Admissible Input Space** `U_x` (2.5) | ✗ zero occurrences of admissib/legal/feasible/constraint | ◐ `AdmissibleInputConstraint` — structural; the predicate is a lossy callable | **[d]** `allowed:` as syntax |
| 8 | **Admissible Input Map** `U : X → ℘(U)` (2.5) | ✗ | ◐ registered + validated structurally | **[d]** `allowed:` clauses, enumerable |
| 9 | **State Update Map** `f : X × U_x → X` (2.6) | ◐ returns **one** `(key, value)` pair per function | ◐ `Mechanism` + `updates` list; body is opaque Python | ✓ `mechanism:` returns the whole next state (D2) |
| 10 | **Restricted State Update Map** `f\|_x` (2.7) | ✗ | ✗ (one doc mention) | ✗ — **all three lack it** |
| 11 | **Input Map** `g : X → U_x` (2.8) | ◐ `PolicyFunction`, but receives full history | ✓ `Policy` role | ✓ `input:` expressions; **[d]** named `input map` |
| 12 | **State Transition Map** `h = f ∘ g` (2.9) | ✓ PSUB execution | ✓ compiled wiring / canonical form | ✓ `step` |
| 13 | **Markov property of `h`** (2.9) | ✗ **violated** — `StateHistory` passed everywhere | ✓ blocks read `prev_state` | ✓ enforced; mechanism sees only `s` |
| 14 | **Autonomous GDS** `{h, X}` (2.10) | ◐ a configuration object | ✓ `GDSSpec` | ✓ `system` |
| 15 | **Closure under composition** (abstract) | ◐ list of PSUBs | ✓ `>>`, `\|`, `.feedback()`, `.loop()` | ◐ `>>` over steps, state→state only |
| 16 | **Canonical form** `u = g(x)`, `x+ = f(x,u)` (eq 1–2) | ✓ | ✓ | ✓ — the "decide, then happen" split |
| 17 | **Posterior state** `x+` (Table 1) | ◐ next row | ✓ `x_next` symbols | ✓ next recorded state |
| 18 | **Learning `g` from observed trajectories** (2.10 disc.) | ✗ | ✗ | ✗ — inverse problem, nobody attempts it |

### Section 3: Contingent representation

| # | Concept (paper ref) | cadCAD | gds-core | Roxin |
|---|---|---|---|---|
| 19 | **Attainability correspondence** `F` (3.1) | ✗ | ✗ (docs only) | ✗ |
| 20 | **Metric space** `(X, d_X)` (Assumption 3.2) | ✗ | ✓ `StateMetric`, with `metric_type` | ✗ — **needed for D6's float-merge rule** |
| 21 | **Contingent / Dini derivative** (3.3) | ✗ | ✗ (docs only) | ✗ |
| 22 | **Uniform Lipschitz property** (3.4) | ✗ | ✗ | ✗ |
| 23 | **Constraint set** `C(x₀,t₀;g)`, compact/convex/continuous (3.5) | ✗ | ✗ | ✗ |
| 24 | **Existence theorem** (3.6) | ✗ | ✗ | ✗ |

### Section 4: Differential inclusion representation

| # | Concept (paper ref) | cadCAD | gds-core | Roxin |
|---|---|---|---|---|
| 25 | **Differential inclusion** `ẋ ∈ F(x)` | ✗ | ◐ `gds-continuous` ODE runtime, not the inclusion | ✗ (out of scope) |
| 26 | **Immediately reachable set** `R(x) = ∪_{u∈U_x} f(x,u)` (4.1) | ✗ | ◐ cites 4.1; **sampling-based**, "no coverage guarantees" | **[d]** enumerate `allowed`, ignore `g` — exact |
| 27 | **State change** `Δx = d_X(x⁺, x)` | ✗ | ✓ via `StateMetric` | ✗ |
| 28 | **Tangent cone** `∂R(x)` (eq 6) | ✗ | ✗ | ✗ |
| 29 | **Configuration Space** `X_C` (4.2) | ✗ | ✓ largest SCC of the sampled reachability graph | ✗ — cheap to add once `R(x)` exists |
| 30 | **x-controllability set / local controllability** (4.3, Thm 4.4) | ✗ | ◐ classical Kalman rank tests, not Theorem 4.4 | ✗ |
| 31 | **Property** `P(x_i) = TRUE` **along a trajectory** (4.4) | ✗ | ✓ `gds_proof.Invariant` + inductive safety | **[d]** `invariant` (D8, D10) |
| 32 | **P-controllability** (4.4) | ✗ | ◐ Lyapunov / inductive safety approximate it | ✗ |
| 33 | **Observability** (4.4) | ✗ | ◐ `observability_matrix`, `is_observable` (classical) | ✗ |

### Section 5: Implementations

| # | Concept (paper ref) | cadCAD | gds-core | Roxin |
|---|---|---|---|---|
| 34 | **Optional input stage** (5.2) | ✓ `policies` may be empty | ✓ standalone `Mechanism` | ✓ `step` may omit `input:` |
| 35 | **Parameters may be scalars, methods, or classes** (5.2) | ✓ any Python object | ✓ any Python object | ✗ **deliberately** typed scalars only |
| 36 | **Scientific controlled experiments** (5.2) | ✓ sweeps, Monte Carlo, subsets | ✓ `psuu` search + sensitivity | ✓ `sweep`; **[d]** `runs`, `seed` |
| 37 | **Repeated stage game semantics** (5.2, described as legacy) | ✓ this *is* cadCAD | ✗ replaced by block algebra | ✗ replaced by `>>` composition |

**Scoreboard, counting ✓ only.** Of 37 concepts: cadCAD 8, gds-core 17, Roxin 12 (plus 6 `[d]`). But the
distribution matters more than the count — gds-core's lead is entirely in Sections 3–4 analysis machinery,
while Roxin's ✓s cluster in Section 2, the definitional core. The six concepts nobody implements (10, 18,
21–24, 28) are the paper's hardest mathematics, and 21–24 require convexity assumptions that general data
structures cannot satisfy anyway.

---

## Table 2a — Where Roxin falls short

| Gap | Who has it | Cost of not having it | Priority |
|---|---|---|---|
| **Arbitrary data structures** — lists, maps, graphs, agent populations | cadCAD (any Python), gds-core (any Python) | This is the paper's *motivating* claim: state spaces of "abstract classes", changing node and edge sets, per-node behavioural models. Roxin's flat scalar records can't express the models GDS exists for. | **Highest** — it's a gap between the name and the capability |
| **Output map / subsystem composition** | gds-core (`ControlAction`, `\|`, feedback) | No systems-of-systems, which §5 frames as the MBSE purpose | High (your M9) |
| **State metric `d_X`** (Assumption 3.2) | gds-core (`StateMetric`) | D6's open item — merging near-equal float states — *is* this | Medium, blocks reachability |
| **KPI level** (per-run, per-configuration) | gds-core (`psuu.Metric`/`KPI`/`Objective`) | `runs 1000` is meaningless without "the mean final balance across runs" | Medium, arrives with Monte Carlo |
| **Configuration space `X_C`** (4.2) | gds-core | A natural reachability output; cheap once `R(x)` exists | Low |
| **Restricted state update map `f\|_x`** (2.7) | nobody | The object an inductive proof quantifies over; the paper ties it to differential invariants | Low now, central if proofs get real |
| **Interchange format** | gds-core (OWL/SHACL/RDF, IR) | No interop with the ecosystem; also the obvious collaboration seam | Low |
| **Published docs site, visualisation** | gds-core (MkDocs, Mermaid) | Adoption friction | Low |

## Table 2b — Deliberately not copied, and why

| Feature | Source | Why not |
|---|---|---|
| **`StateHistory` passed to every function** | cadCAD | Breaks the Markov property of Definition 2.9 outright. History belongs in state explicitly. |
| **`policy_ops` merging of policy outputs** | cadCAD | `U_x` is a set and `g` selects *one* `u`; summing parallel outputs has no formal counterpart. D3 makes coupling an explicit predicate. |
| **Substeps as first-class execution semantics** | cadCAD | Bookkeeping the paper has no concept of. Recording them stays opt-in. |
| **Arbitrary Python objects as state and parameters** | cadCAD, gds-core, and paper §5.2 | The single decision that makes checking, enumeration, and serialisation impossible. Rejecting it is Roxin's reason to exist — even though the paper explicitly permits it. |
| **`deepcopy`-based state isolation** | cadCAD | Purity by construction instead of by defensive copying. |
| **`env_process` as a separate construct** | cadCAD | Definition 2.4 says exogenous influence *is* an input. Subsumed, correctly. |
| **`policy` as a keyword** | cadCAD, gds-core (`Policy` role) | Paper says Input Map. Keep the rule — but stop calling `policy` cadCAD-only vocabulary, since gds-core uses it under Zargham's direction. |
| **Structural/behavioural split** | gds-core | The compromise Roxin exists to refuse. Worth naming explicitly in the comparison docs. |
| **Sampling-based reachability** | gds-core | Roxin can enumerate `allowed` exactly. Don't trade that for Monte Carlo coverage. |
| **Contingent derivative, Dini limits, Lipschitz, existence theory** | paper §3 | Analysis machinery, not a language's job. Nobody implements it. |
| **Differential inclusions, tangent cones, local controllability** | paper §4.3, Thm 4.4 | Requires `X ⊆ ℝⁿ` — the paper abandons its own generality to get these. Roxin's mixed-type records live in the general regime. |
| **17-package ecosystem** | gds-core | Their own issue #143 wants 6. Scope creep is your top risk. |
| **Continuous time** | paper §3–4, gds-core | Already declared out of scope. Note Definition 2.6 itself calls `f` a discrete transition. |

## Table 2c — Roxin-original: no counterpart in either

Neither the paper nor gds-core nor cadCAD addresses any of these. This is the part of Roxin that is not a
reimplementation of anything.

| Feature | Status | Why it matters |
|---|---|---|
| **No default numeric type** — every field declares `int`/`decimal`/`float` | **runs today** (partly) | The paper is silent on numerics; gds-core defers arithmetic to its DSL layer |
| **`/` rejected on two ints; floored `//` and `%`** (D5) | runs today | Rounding direction is a modelling decision, not a language accident |
| **`decimal(N)` with mandatory explicit rounding** (D9) | [designed] | The AMM example — round up and the invariant holds, round down and it fails — is unexpressible in the other two |
| **Float equality requires a stated tolerance** (D6) | [designed] | Silent tolerance is how invariant checks produce false results |
| **NaN / infinity / divide-by-zero stops the job** (D4) | runs today | cadCAD propagates or throws depending on the library |
| **Mechanism totality** — every state field set exactly once (D2) | runs today | cadCAD's dict-key collision silently drops updates |
| **Positioned error messages** | runs today | Neither alternative can point at a line in a model |
| **Pre-run checker** | [designed] | Static rejection before any job runs |
| **Bit-exact reproducible RNG, pinned algorithms** (D7) | [designed] | gds-core has no stochastic layer at all yet |
| **Decision log with alternatives, costs, revisit triggers** | in place | gds-core is only now adding ADRs (issue #206) |
| **Fixture corpus with independently derived oracles** | in place | Their Tier 0 critical gaps are exactly this (T0-1, T0-2) |
| **Execution speed** | measured on a prototype | Real, but re-measure before quoting — see the caution above |
