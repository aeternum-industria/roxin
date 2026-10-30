# Guides: GDS for your field

A generalized dynamical system is a simple idea that is usually explained in the vocabulary of one particular
research community. These guides teach the same idea in the vocabulary of **your** field, with models that are
recognizable to people who work in it.

| Guide | Written for | Models that run today | Models written ahead of their features |
|---|---|---|---|
| [Game design and development](games.md) | game and economy designers, gameplay programmers | gold faucet and sink | cooldowns, idle-game upgrades, mana-limited combat, gacha with pity |
| [Quant finance and tokenomics](tokenomics-quant.md) | quants, protocol and mechanism designers | Bitcoin issuance schedule, constant-product market maker | token vesting, bonding curve, the float-rounding trap |
| [Life sciences](life-sciences.md) | epidemiologists, pharmacologists, ecologists | epidemic (SIR), one-pill pharmacokinetics, enzyme kinetics | conservation checks, repeated dosing |
| [Control and engineering](control-engineering.md) | control, mechanical and electrical engineers | thermostat, PID controller, divergence catch | actuator limits, bang-bang with hysteresis |
| [Collective decision-making and commons](governance-commons.md) | designers and students of councils, co-ops, commons, committees, DAOs | shared fishery, mutual aid fund | quota rules, solvency invariant |

## How the guides work

Every guide has the same twelve sections in the same order, so a reader who has finished one can skim any other.

1. Why someone in this field should care
2. The one idea: state, input, mechanism
3. Your first model
4. Separate the decision from the consequence
5. What is legal
6. Rules that must always hold
7. Choosing numbers
8. Randomness and time
9. A bigger model, walked through
10. Honest limits
11. Glossary: your word, the Roxin term
12. Exercises

**The terminology rule.** A guide may introduce a concept through a familiar word once (a game's *turn*, a chain's
*block*, a lab's *time point*) and then switches to the Roxin term for good. Roxin ships no domain aliases. This is
what lets a model written from one guide be read by someone who learned from another.

**Status tags.** Every feature is tagged where it first appears:

- **[runs today]** works in the current build. Every such block is a file in [`examples/`](../../examples/), and
  the output shown is what the build prints.
- **[designed]** specified in the project docs but not built yet. Expected outputs for these were computed
  independently in Python (see `tools/reference.py` in the fixtures package), not by a Roxin run, and are labeled so.
- **[exploring]** an open question.

**Neutrality.** The guides show what a rule set *does* under stated assumptions. They do not say which rules are
better, whose goals are right, or which design is preferable. The numbers are the numbers.

## Writing a new guide

A guide for a field that is not listed is welcome, and the format is meant to be copied. Checklist:

- [ ] Same twelve sections, same order.
- [ ] At least two models that **run today**, as files in `examples/`, with the output shown verbatim.
- [ ] Every number in the text can be traced to a run, a reference script, or a stated formula.
- [ ] Planned features are tagged **[designed]** and any expected output says where it came from.
- [ ] A glossary mapping at least ten words from the field to Roxin terms.
- [ ] An "Honest limits" section that says what the field would expect Roxin to do and it cannot yet.
- [ ] No claims about which outcome, rule, or design is better.
