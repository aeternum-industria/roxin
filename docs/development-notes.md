# Roxin development notes

How the project is run: sequencing, risks, documentation strategy, and measurements. Everything about *what
the language is* — feature status, open questions, decision records — lives in
[`registry.md`](registry.md); everything about *how it is being developed* lives here.

Related documents: [`registry.md`](registry.md) — the source of truth for feature status, open questions and
every decision record — and [`core-semantics.md`](core-semantics.md) (precise semantics).

---

## Roadmap

| Milestone | Build | Done when |
|---|---|---|
| M0 | Read through `docs/core-semantics.md` (its decisions D1–D13 are settled); settle the questions in Open Questions at the end; collect 3–5 real models from GDS practitioners | the draft has been reviewed and the real models are in hand |
| M1 | Rust project, AST, and a tree-walking evaluator for one step (the oracle) | the thermostat with k = 0.5 goes `10 → 15 → 17.5 → 18.75` |
| M2 | Trajectory loop, stop rule, output table; s-expression reader and a minimal REPL | all three k values match the closed-form values below; predator–prey holds still at equilibrium; the marbles model reaches `(5,5)`; a model can be typed into the REPL |
| M3 | Typed IR and a bytecode compiler and VM | every M1–M2 benchmark gives identical results on the VM and the tree-walker |
| M4 | `allowed` sets with runtime checks; seeded sampling | the thermostat with k = 1.5 and `allowed: heat in 0..5` raises an error naming the state and input; a seed repeats a run on both engines |
| M5 | Sweeps, Monte Carlo, and A/B variants with run metadata; parallel jobs | a 3×2 sweep yields exactly 6 jobs; `Proportional` versus `BangBang` runs side by side |
| M6 | Pre-run checker: wiring, types, mechanism purity | an impure mechanism is rejected before running |
| M7 | Reachability, metrics, and invariant checking | the market model's reachable states all satisfy its invariant |
| M8 | The first surface syntax | all benchmark models are written as source text |
| M9 | Governance model; composing subsystems | the governance model runs with parameters in state |

The VM becomes the primary engine at M3. The tree-walker stays as the oracle. Backends beyond the VM are not scheduled.

### Benchmark models

Moved. The benchmark values live in the fixture headers under `../tests/fixtures/`, which are what
`tests/fixtures/tools/reference.py` actually checks against, and the expected outputs are the `.expected` files
beside them. Feature status for each benchmark is in [`registry.md`](registry.md) Part 1.

## Documentation plan

- **Core reference and tutorial:** domain-neutral, using a balanced mix of examples. The thermostat, marbles, predator–prey, SIR, and market models already span control, games, biology, and economics. A rough guard: no field supplies more than about a third of the examples in the core docs.
- **Audience guides:** same chapters, same order, same terms, different example families. The test for a guide is that a model written from one guide is readable by someone who learned from another. The guides must say plainly that unbounded `int` can diverge from a real contract at overflow boundaries, and show how to state the bound as an invariant (for example `invariant NoOverflow(s: Pool) = s.balance < 2^256`).
- **Terminology rule:** a guide may introduce a concept through a familiar word once, then switches to the Roxin term for good. Roxin ships no domain aliases.
- **Defaults reference:** every default listed in one page (see the defaults table), kept in sync with the implementation.
- **Decision log:** `docs/registry.md` records every decision with its alternatives and reasoning; new decisions are appended, and changes are added as dated notes, never silent edits.

---

---

## Risks

- **Scope creep** toward general-purpose or analysis tooling. The biggest risk.
- **Unwritten semantics** causing endless syntax churn. Mitigation: write the formal core first (drafted in `docs/core-semantics.md`).
- **Floating-point rounding** silently breaking invariant checks.
- **State explosion** in reachability, especially with joint `allowed` sets over several fields.
- **VM speed** on large agent-based models. Mitigation: parallel jobs first, then a faster backend only if measured timings justify it.
- **Learning Rust and building the compiler at the same time.** Mitigation: the tree-walking oracle comes first, and the VM is built second against known-good results.
- **Size of the build:** compiler, VM, checker, runtime, and result store, all from scratch. Mitigation: staged milestones that are each useful alone.
- **Adoption:** the cadCAD community faded, so the niche may be small. The project's value does not depend on adoption, but its direction might.
- **Partial evidence base:** only part of the cadCAD demo repository was reviewed (see below).

Specific open gaps are Part 1 rows in [`registry.md`](registry.md) with status `open` or `accidental`.

## Performance notes (informal, prototype)

These numbers come from a throwaway prototype of the evaluator, not from the repository build, and from one 2-vCPU
Xeon (Rust 1.97 release build, Python 3.13, cadCAD 0.5.3, pandas 3.1.0). They are here to say *where to look*, not
to make claims. Re-measure on the real build before quoting any of them.

| Workload (thermostat model) | cadCAD 0.5.3 (execute only) | Roxin tree-walker prototype |
|---|---|---|
| 1 job, 8 steps | 0.014 s | 27 µs |
| 1 job, 200,000 steps | 3.1 s | 0.46 s |
| 1,000 jobs, 200 steps each | 25.4 s | 0.35 s |

Caveats that matter: the model is as small as a model gets; cadCAD does much more per step (state history, deep
copies, substeps, a results table), so this measures "framework overhead on a trivial model", not "modeling speed";
jobs ran sequentially on both sides (cadCAD in its local mode); and a tree-walker's advantage shrinks as models get
heavier. Both engines produced identical final values for the thermostat.
