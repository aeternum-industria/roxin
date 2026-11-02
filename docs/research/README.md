# Research notes

Background material produced while designing Roxin. Not part of the language documentation —
nothing here is normative, and `docs/registry.md` overrides anything in this directory.

| File | What it is |
|---|---|
| [`gds-concept-matrix.md`](gds-concept-matrix.md) | Every concept in *Generalized Dynamical Systems Part I* (Zargham & Shorish, 2022), mapped across cadCAD, gds-core and Roxin, plus where Roxin falls short, what was deliberately not copied, and what has no counterpart in either |

**The numbering in `gds-concept-matrix.md` Table 1 is load-bearing.** The author's design notes
reference its rows by number — "point 10", "points 7-8" — so rows must not be renumbered,
reordered, or removed. Append only. Table 1 has 37 rows: 1–18 cover the paper's Section 2
(the definitional core), 19–24 Section 3, 25–33 Section 4, 34–37 Section 5.

## Where the matrix has been superseded

`gds-concept-matrix.md` is a dated snapshot (reviewed 30 Oct 2026) and is left unedited so it stays
faithful to what was found at the time. Working through the author's notes against it has already
changed the reading of some rows. The registry is authoritative; this table says where to look.

| Point | The matrix says | The current reading |
|---|---|---|
| 15 — Closure under composition | `◐` for Roxin, scored against gds-core's four-operator algebra | Scored against the **paper**, Roxin is a full mark: Part I §5 prescribes composing parts "within one state space", which is endomorphic composition. Three of the four gds-core operators reduce to one missing object, the output map. See [D21](../registry.md) and LANG-28 |
| 18 — Learning `g` from observed trajectories | `✗ — inverse problem, nobody attempts it` | Downstream, not impossible, and **not one problem**. Recovering `u` from an observed transition is the explorer's fan-out plus an equality test; only fitting a function to the recovered pairs is learning, and that is the export layer under D17. See CHK-12 |
| 19–24 — Section 3, contingent representation | `✗` across the board, "analysis machinery, not a language's job" | Correct, for a better reason. **§3 is the machinery for point 18** — it says so, noting that inferring which `g` produced a trajectory needs "more structure on the constraints imposed by the Input Map g". It is the *analytic* route, mandatory in continuous time with unenumerable inputs. With discrete time and an enumerable `allowed` the same question has a direct combinatorial answer, so §3 is a workaround for a difficulty Roxin does not have. See CHK-07 and CHK-12 |
| 22 — Uniform Lipschitz | `✗` | Degenerates usefully. In discrete time the smallest gap is one tick, so the condition collapses to a bound on how far the state may move per step — point 27 — which is checkable and invariant-shaped. See CHK-13 |
| 30, 33 — Local controllability, observability | `◐` for gds-core, `✗` for Roxin | gds-core satisfies both via the Kalman rank test and the observability matrix, which are linear time-invariant constructs — i.e. by reverting to the special case §2 was written to escape. **Matching them would be actively harmful**, not merely wasteful. See CHK-08 and CHK-15; explained for readers in [`../modeling.md`](../modeling.md) ch.26 |
| 32 — P-controllability | `✗` for Roxin | Largely already built, as a side effect of D20. Violating successors being terminal means the pruned fan-out *is* the P-controllable subtree. See CHK-14 |
| 26, 29 — Reachable set, configuration space | `◐`/`✓` for gds-core | Both are computed over gds-core's **sampled** graph, so they answer "what we visited" rather than "what is possible". Roxin's would be exact — a different claim, not a better number. See CHK-03 and CHK-04 |
| 35 — Parameters may be scalars, methods, or classes | `✗` rejected, "arbitrary Python objects" | **The sentence bundles opposite ends of the spectrum.** A class is a record with typed fields — Tier 1, nearly free, already designed as LANG-17. A method is an opaque callable — Tier 4, fatal. §5.2 is also *describing cadCAD*, not stating a GDS requirement. What Roxin rejects is opacity and impurity, not richness. See LANG-31 and the type-cost table in [`../registry.md`](../registry.md) |
