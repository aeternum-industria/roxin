# Modeling with Roxin

*A from-zero guide to describing things that change.*

This is the on-ramp. It assumes you have never modeled anything, never programmed, and
have not thought about mathematics beyond arithmetic in a long time. It ends by handing you
to one of the [field guides](guides/README.md), which teach the same ideas in the vocabulary
of a particular line of work.

Written to [P4](registry.md) — ideas are stated, watched in motion, deliberately broken,
repaired, and poked somewhere else. If something gets explained and then immediately taken
apart, that is the method working.

---

## Contents

**Act I — What you are actually doing** *(no syntax at all)*

1. [Who this is for](#1-who-this-is-for)
2. [What a model is](#2-what-a-model-is)
3. [Things that change, and rules that change them](#3-things-that-change-and-rules-that-change-them)
4. [You write the rule, the machine does the repeating](#4-you-write-the-rule-the-machine-does-the-repeating)
5. [Other ways people model](#5-other-ways-people-model)

**Act II — Your first models**

6. [The four parts of every model](#6-the-four-parts-of-every-model)
7. [What is state, really?](#7-what-is-state-really)
8. [Reading the dots](#8-reading-the-dots)
9. [Reading what came out](#9-reading-what-came-out)
10. [Everything happens at once](#10-everything-happens-at-once)
11. [Numbers you can trust](#11-numbers-you-can-trust)

**Act III — Decisions, consequences, and rules**

12. [Numbers you can change without rewriting](#12-numbers-you-can-change-without-rewriting)
13. [Asking several questions at once](#13-asking-several-questions-at-once)
14. [The decision and the consequence](#14-the-decision-and-the-consequence)
15. [Other people's words for the decision](#15-other-peoples-words-for-the-decision)
16. [What is legal](#16-what-is-legal)
17. [Narrowing, never widening](#17-narrowing-never-widening)
18. [What goes in the model, what goes in the question](#18-what-goes-in-the-model-what-goes-in-the-question)

**Act IV — Being sure**

19. [Claims you are making](#19-claims-you-are-making)
20. [The limits of running it](#20-the-limits-of-running-it)
21. [Trying a lot of times is not the same as being sure](#21-trying-a-lot-of-times-is-not-the-same-as-being-sure)
22. [Proving instead of running](#22-proving-instead-of-running)
23. [True, but not provable yet](#23-true-but-not-provable-yet)
24. [When the machine says no](#24-when-the-machine-says-no)
25. [What could have happened](#25-what-could-have-happened)
26. [Questions mathematicians ask that Roxin doesn't](#26-questions-mathematicians-ask-that-roxin-doesnt)

**Act V — Doing it for real**

27. [Chance](#27-chance)
28. [More than one rule, in order](#28-more-than-one-rule-in-order)
29. [Agents, populations, and networks](#29-agents-populations-and-networks)
30. [Time that flows instead of ticking](#30-time-that-flows-instead-of-ticking)
31. [Starting from a blank page](#31-starting-from-a-blank-page)
32. [Signs your model is wrong](#32-signs-your-model-is-wrong)
33. [Why there's only one way to say things](#33-why-theres-only-one-way-to-say-things)
34. [Honest limits](#34-honest-limits)
35. [Talking to other people](#35-talking-to-other-people)
36. [Where to go next](#36-where-to-go-next)



---

# Act I — What you are actually doing

There is no Roxin in this act. Not one line. Before any of the writing makes sense you need
the handful of ideas underneath it, and those ideas have nothing to do with Roxin — they are
the same whether you end up using Roxin, a spreadsheet, or a notepad.

---

## 1. Who this is for

You, if there is something you want to understand and it changes over time.

A small business whose money moves in and out. A game where players earn gold and spend it. A
disease moving through a town. A thermostat fighting a cold room. A fishery that can be fished
until there is nothing left. A savings account. A queue at a clinic.

Those sound like six unrelated subjects. They are six instances of one thing, and noticing
that is the single most useful idea in this document.

### What you need

Addition, subtraction, multiplication, division. Patience. That is the entire list.

### What you do not need

You do not need mathematics past that arithmetic. You do not need to have written a line of
code. You do not need to know what a function, a variable, a state machine, a type, or a
differential equation is — and if you have heard those words and quietly felt stupid, that
ends here, because every word that matters gets defined before it is used.

It does not matter if your last classroom was thirty years ago, or if you never liked the
classroom much to begin with.

### What you will be able to do

Build a working description of something you care about. Run it and read what comes out.
Say which choices are legal and which are not. Write down the things that must never happen,
and have a machine check them for you. Ask what *could* have happened rather than only what
did. And talk about all of it with people who learned this vocabulary somewhere else, which
matters more than it sounds like it does.

### Three things about how this is written

**Ideas get broken on purpose.** An idea will be explained, shown working, and then
deliberately taken apart so you can see what it was holding up. Then it gets put back
together and poked somewhere else. This is not padding. Seeing an idea work tells you very
little; seeing what fails without it tells you what it is *for*.

**About a third of this is about things Roxin does not do.** Ideas it rejects get explained
properly, including why a reasonable person would want them. You cannot have any feel for why
something is shaped the way it is if you have never seen the shapes it is not. Every rejection
here says what is genuinely lost, not only what is gained.

**Some of this describes a language that is not finished.** Roxin is being built. Where a
feature is designed but not yet working, the chapter says so plainly and the examples are
marked. Ideas appear where they belong in the learning, not in the order they happened to get
built.

### If you feel lost

You will, in places. Understanding a new idea almost never arrives in one piece. The first
time through you get a rough shape — enough to sketch something, but lumpy, and wrong in
places you cannot yet see. Each further example you work smooths an edge, and enough edges
make the shape round.

This document assumes that is how it goes, which is why the same ideas keep coming back
wearing different clothes. If a later chapter feels like it is covering an earlier one again,
it is not. It is showing you a side of it you could not see from where you were standing.

---

## 2. What a model is

You already make models. Constantly. Here is one.

> You put **$1,000** into a savings account. Each year the bank adds a tenth of the balance —
> that is what people mean by 10% interest. And each year you put in another **$100** of your
> own.

How much is in there after eight years?

You could work that out on paper. But before you could start, you had to answer a question
the sentence never answered.

### The hole

**Does the bank add its tenth before or after your $100 goes in?**

The sentence does not say. It reads perfectly well either way, and two careful people will
read it two different ways. Let us do both, for one year.

**Reading A — the bank's tenth first, then your deposit.**
The balance is $1,000. A tenth of that is $100, so the bank makes it $1,100. Then you add your
$100. You end the year with **$1,200**.

**Reading B — your deposit first, then the bank's tenth.**
The balance is $1,000. You add your $100, making $1,100. Then the bank adds a tenth of
*that*, which is $110. You end the year with **$1,210**.

Ten dollars apart. Easy to shrug at. Keep going.

| After year | Reading A | Reading B | Difference |
|---|---|---|---|
| 1 | $1,200.00 | $1,210.00 | $10.00 |
| 2 | $1,420.00 | $1,441.00 | $21.00 |
| 3 | $1,662.00 | $1,695.10 | $33.10 |
| 4 | $1,928.20 | $1,974.61 | $46.41 |
| 5 | $2,221.02 | $2,282.07 | $61.05 |
| 6 | $2,543.12 | $2,620.28 | $77.16 |
| 7 | $2,897.43 | $2,992.31 | $94.87 |
| 8 | $3,287.18 | $3,401.54 | $114.36 |

By year eight they are **$114.36** apart, and the gap grows every single year. Same sentence.
Same bank. Same $100.

### What just happened

The English sentence was not a model. It was a description with a hole in it, and the hole was
invisible until somebody tried to carry it out. Both readings are faithful to the words.
Neither reader was careless. The sentence simply did not contain enough to settle it.

So:

> **A model is a description of how something changes, with no holes — precise enough that it
> can be carried out without you in the room to settle arguments.**

That is the whole definition. Not "a mathematical representation of a system." Not "an
abstraction of reality." A description precise enough to carry out.

### Why not just write better English?

A fair question, and the obvious move. Try it:

> "Each year, you deposit $100, and then the bank adds a tenth of the resulting balance."

That is better. It genuinely fixes the hole. But notice three things about the fix.

**You only fixed the hole you happened to notice.** What if you also take money out some
years? Before or after the interest? New hole, same shape, and nothing warned you.

**The holes are found by confusion, later.** Usually by someone reading your description
months afterwards, often after the numbers have already been used to decide something.

**Nothing checks you.** This is the real problem, and it is not that English is imprecise —
English can be made very precise. It is that **English never tells you when you have left
something out.** It reads just as smoothly with the hole as without.

A description written in a language that insists on completeness will refuse to proceed until
the hole is filled. That refusal is the service being offered. It is also, in the beginning,
the most annoying thing about it, and it is worth knowing in advance that the annoyance and
the usefulness are the same feature.

### Poke it: can a model with no holes still be wrong?

Yes, completely, and it is worth being clear about this early so you never confuse the two.

Reading B has no holes. It is also probably wrong about your actual bank, which likely pays
monthly, rounds to the cent, and has a limit on deposits. A model can be perfectly precise and
a poor description of the real thing.

Precision is not accuracy. What precision buys you is that your model says **exactly one
thing**, so when it disagrees with reality you can find out where. A vague description cannot
be wrong in any useful way, because it never committed to anything in the first place. It just
quietly means whatever the reader needed it to mean.

---

## 3. Things that change, and rules that change them

Every model in this document — and every model you will ever write in Roxin — is made of two
things.

**Things that change.** In the savings account: the balance.

**A rule for changing them.** Add $100, then add a tenth.

That is it. Everything else in this guide is detail hung off those two nouns. They have proper
names, which you will meet in Act II; the plain words are enough for now.

### Which things change?

Look at the savings model again and sort everything in it into two piles.

The **balance** changes. It is $1,000, then $1,210, then $1,441.

The **$100** does not. You add the same amount every year.

The **tenth** does not either. The rate is the rate.

So one thing changes and two are settings — fixed numbers the rule uses but never alters.

### Poke it: is that sorting fixed?

No, and this is the first genuinely important thing in this chapter.

Suppose you decide to increase your deposit by $10 every year. The deposit has just become a
thing that changes. Nothing about a deposit makes it inherently a setting; it was a setting
because *in that model* it never moved.

> **Nothing is inherently a setting or a changing thing. It depends on which model you are
> building.**

Deciding which is which is not bookkeeping. It is most of the skill, and it is the first thing
you will do every time you sit down to model something.

### Break it: a rule that mentions something you never wrote down

Here is a perfectly sensible-sounding rule.

> "Each year I add $100, unless I am short that year, in which case I add nothing."

Now try to carry it out. Year one. Am I short?

You cannot answer. Not because the question is hard, but because **nothing in the model says.**
There is a balance and there is a deposit, and neither of them tells you whether you were
short. The rule reaches for something that does not exist.

This is not a trick. It is the single most common thing that happens when someone writes their
first model, and there are three honest ways out — all legitimate, each meaning something
different.

**Write the missing thing down.** Add *income* to your list of things that change, and say
plainly what "short" means — income below $2,000, say. The model gets bigger and more
truthful.

**Make it a setting instead.** "In years three and seven I add nothing." You have replaced a
reason with a schedule. Less insightful, perfectly valid, and sometimes exactly right.

**Decide it is out of scope.** You are modeling the account, not your career. Drop it, and
know that you dropped it.

What you may never do is leave the rule as it stands, because it cannot be carried out.

> **A rule may only mention things that exist in the model. No exceptions, ever.**

That sounds restrictive. It is the opposite, and it is worth sitting with: it means that when
you read a rule, **everything it depends on is right there in front of you.** Nothing is
hiding in someone's head, or in a footnote, or in an assumption everybody made and nobody
wrote down. If the rule mentions it, it is in the model. If it is not in the model, the rule
cannot mention it.

### Poke it again: is "total interest earned so far" a thing that changes?

It goes up every year, so it certainly changes. But notice something: if you knew the balance
and knew how much of your own money you had put in, you could work it out. You do not
*have* to track it — you could calculate it whenever you wanted.

So is it a thing that changes, or a thing you calculate?

Both answers are defensible, and the choice has real consequences. Hold the question; it gets
a chapter of its own in Act II.

---

## 4. You write the rule, the machine does the repeating

Here is the shift that makes everything else possible, and it is easy to read past.

**You never write the story. You write the rule, and where to start.**

The savings model's story is eight numbers: $1,210, $1,441, $1,695.10, and so on out to
$3,401.54. You did not write those. You wrote one sentence — *add $100, then add a tenth* —
and said where to begin. The eight numbers were produced.

### Why that is worth caring about

Because it leaves you holding two knobs that turn independently.

**Turn the starting point.** Same rule, different story.

**Turn the rule.** Same start, different story.

| After year | Start $1,000, add $100 | Start $5,000, add $100 | Start $1,000, add $200 |
|---|---|---|---|
| 1 | $1,210.00 | $5,610.00 | $1,320.00 |
| 2 | $1,441.00 | $6,281.00 | $1,672.00 |
| 3 | $1,695.10 | $7,019.10 | $2,059.20 |
| 4 | $1,974.61 | $7,831.01 | $2,485.12 |
| 5 | $2,282.07 | $8,724.11 | $2,953.63 |
| 6 | $2,620.28 | $9,706.52 | $3,469.00 |
| 7 | $2,992.31 | $10,787.17 | $4,035.89 |
| 8 | $3,401.54 | $11,975.89 | $4,659.48 |

Three stories. One rule, written once, never edited. Everything that differs between those
columns is a number you changed in a single place.

### Break it: write the story instead

Imagine you had written out those eight numbers directly, by hand, in a list. You would have
an answer — a correct one — for starting at $1,000 and adding $100.

Now a friend asks what happens if she starts with $5,000.

You have nothing. Not a shortcut, not a partial answer. You would start over, from scratch,
doing all eight years again. And when she asks about $200 a year, you start over again.

> **A story is an answer. A rule is a machine that produces answers.**

Everything in the rest of this guide — asking what-if, sweeping across a hundred settings,
proving something can never happen — exists because you kept the machine instead of only its
output. None of it is available to someone holding a list of numbers.

### Poke it: can the rule know which year it is?

Say you get a $1,000 bonus in year five, and only year five. Can the rule say "if it is year
five, add $1,000"?

Think about chapter 3 before reading on. A rule may only mention things that exist in the
model. Is "which year it is" one of the things in your model?

It is not. You wrote down a balance. Nothing in there counts years.

So by the rule of chapter 3, you cannot — **unless you write it down.** Add a *year* to your
list of things that change, have it go up by one each time, and now the rule can mention it,
because now it exists.

Roxin does hand you one shortcut here: a reserved name for the step count, so you do not have
to add the counter yourself. But look at how it is defined — it is **shorthand for exactly the
counter you would have written.** The shortcut saves typing; it does not bend the rule. And it
is deliberately *not* available inside the part of a step that changes things, only in the part
that makes decisions — for reasons that will make sense in Act III and would be noise now.
*[designed, not yet built]*

This is the first time you will see a pattern worth naming, because it repeats: where Roxin
offers a convenience, the convenience is defined as shorthand for something you could have
written longhand. It is never an exception to a rule you were taught.

### Poke it once more: can a rule produce the same thing forever?

Yes. A thermostat that reaches the right temperature and holds it. A population that settles.
A balance that stops growing because what goes out matches what comes in.

And notice what kind of question "does it settle?" is. It is not a question about year eight,
or about any particular story. It is a question about the **rule itself** — about every story
the rule could ever produce.

You cannot ask that of a list of numbers. You can only ask it of a machine. Keep that in your
pocket; Act IV is built on it.

---

## 5. Other ways people model

You now have enough to compare. This chapter is here because you cannot judge whether a tool
suits you without seeing what else was available, and because some of these are better choices
than Roxin for work you may actually have in front of you.

### In your head

What everyone does. Instant, free, and you already know how.

It breaks quietly. Three or four things changing at once is past most people's limit, and the
failure mode is not "I cannot do this" — it is confident wrongness. You get an answer and no
signal that it is wrong.

### On paper

Precise, and it makes the hole from chapter 2 visible, because you cannot carry out a
calculation without settling the order.

It does not repeat well. Eight years is fine. Eight hundred is not, and "what if I had started
with $5,000" means doing the whole thing again.

### A spreadsheet

The honest competition, and for a great deal of real work it is the right answer. You see
every number at once, you change a cell and everything updates, you already have one, and
there is nothing to install or learn. For a ten-row calculation a spreadsheet beats Roxin and
you should use it.

Here is the thing worth noticing, though:

> **A spreadsheet is a model where the story is written out and the rule is hidden inside the
> cells.**

That is chapter 4 turned exactly backwards. Copy a formula down five hundred rows and you have
made five hundred copies of your rule. The rule is not written anywhere as a single thing you
can read — it is distributed across the grid, and the only way to see it is to click a cell
and look at the bar.

What follows from that is not theoretical:

- If one cell out of five hundred is wrong — a typo, a reference that slipped — **nothing tells
  you.** The column looks like a column. This is the single most common serious error in
  professional work, and it has moved markets and ended careers.
- Asking "what if" means copying the whole sheet. A hundred what-ifs means a hundred sheets.
- There is no way to ask "could this balance ever go negative?" except to look at the numbers
  that happen to be there and squint.
- There is nowhere to write down "this must never go negative" such that anything checks it.

### A script

Write it in a programming language and you can do literally anything.

Including things you did not mean. Nothing stops you reading a value you meant to write, using
last year's number where you meant this year's, or quietly mixing up dollars and cents. The
rule and the bookkeeping end up tangled together — loops, counters, output formatting — so the
thing you actually care about is a few lines buried in machinery. And you have to know a
programming language, which is a year of your life if you do not.

### Equations on paper

The approach a mathematician reaches for, and it can do something no simulation can: answer
questions about *all* time, in closed form. Not "what is it at year eight" but "what does it
approach forever." That is a genuine superpower and Roxin gives it up.

What it struggles with is the ordinary messiness of real systems. "Unless the balance is below
$500, in which case the fee is waived" is a sentence anyone understands and a nuisance to
express as an equation. Conditions, thresholds, and discrete choices are where the paper
starts to fight you.

### A simulation kit

Libraries exist that give you a box of parts for exactly this. They save real time, and the
good ones encode hard-won knowledge about how these models go wrong.

The cost is that you inherit the kit's assumptions, including ones nobody wrote down, and most
of them ask for a page of setup before anything runs. If you cannot explain why a line of your
own setup is there, you are no longer modeling — you are copying, and copying does not survive
contact with the next model.

### A modeling language

Which is where Roxin sits, and the trade is narrow and specific.

You give up seeing numbers as you type. You give up a window, in favour of a file and a
command. You give up being able to do anything at all, in exchange for not being able to do
things you did not mean. For a quick sum, that trade is bad and you should open a spreadsheet.

What you get is that **the rule is the document.** Written once, in one place, readable start
to finish. What is legal and what must never happen are part of that same description rather
than notes beside it — which means they can be checked, and later, proved.

---

That is the act. Everything from here is one of those two nouns from chapter 3, dressed in
increasingly specific clothes. Let us build one.

---

# Act II — Your first models

Syntax starts here. Every mark on the page gets explained — braces, colons, dots, indentation.
Nothing is assumed to be obvious.

Three kinds of marking appear from now on:

- **[runs today]** — you can type it and it works.
- **[designed]** — decided, not built yet. The syntax shown is what it will be.
- **[undecided]** — the idea is real and the shape is not chosen. You will see candidate
  syntax labelled as a candidate, along with what is actually being weighed.

---

## 6. The four parts of every model

*runs today*

Act I left you with two nouns: things that change, and rules that change them. A Roxin model
has four parts, and they are those two nouns plus two pieces of plumbing.

Here is a whole model. It is nine lines. We will take it apart one line at a time.

```
state Player { gold: int, potions: int }

step Shop:
    mechanism:
        gold: s.gold - 30
        potions: s.potions + 1

system Shopping: Player = Shop

run Shopping from Player { gold: 100, potions: 0 } for 4 steps
```

A player has gold and potions. Every turn they buy one potion for thirty gold. Start with a
hundred gold and nothing else, and watch four turns.

### Line 1: the things that change

```
state Player { gold: int, potions: int }
```

`state` announces the things that change. Everything after it is the list.

`Player` is a name **you** chose. It is not special, Roxin has never heard of players, and you
could have written `Hamster` and the model would behave identically. Names you choose start
with a capital letter in Roxin; that is a convention, not a rule you need to think about.

The curly braces `{ }` hold the list, the way brackets hold a list anywhere.

`gold: int` is one entry. The part before the colon is the name of the thing that changes. The
part after is **what kind of number it is**. `int` means a whole number — 5, 0, -20, never 3.5.

Two entries, separated by a comma. Done.

Notice what you were made to do: you had to say *what kind of number* gold is. Roxin will not
guess. There is a whole chapter on why at the end of this act, and for now `int` for countable
things is a safe habit.

### Lines 3–6: the rule

```
step Shop:
    mechanism:
        gold: s.gold - 30
        potions: s.potions + 1
```

`step` announces a rule. `Shop` is the name you gave it — again, yours, and again meaningless
to Roxin.

The colon at the end of `step Shop:` means "what follows, indented underneath, belongs to
this." Indentation is how Roxin knows what belongs to what. Four spaces is the convention.

`mechanism:` is the part of the rule that says what actually changes. Right now, with nothing
else inside the step, it looks like pointless nesting — a word whose only job is to hold two
lines. That is a fair reaction and you should keep it. In Act III a step grows a second part,
and `mechanism:` starts earning its place by distinguishing *what someone decided* from *what
follows from the decision*. Until then it is a word you are typing on credit.

The two lines underneath are the heart of everything.

```
gold: s.gold - 30
```

Read it right to left. `s.gold` is the gold as it was at the start of this turn — the `s.`
part gets a chapter of its own, two chapters from here. Subtract 30. The thing before the
colon, `gold`, is where the answer goes: the **new** gold.

So: *the new gold is the old gold minus thirty.*

And the other line: *the new potions count is the old one plus one.*

### Line 8: the plumbing

```
system Shopping: Player = Shop
```

This says: there is a thing called `Shopping`, it operates on a `Player`, and the rule it uses
is `Shop`.

With a single rule, this line is hard to justify, and it is worth saying so rather than
pretending. It is three names you already wrote, arranged to point at each other. It earns its
existence in Act III, when there is more than one rule and the system says which ones run and
in what order. With one rule it is ceremony.

*(Whether Roxin should simply work this out for you when there is only one rule is an open
question in the language's own notes. If that changes, this paragraph gets shorter.)*

### Line 10: the question

```
run Shopping from Player { gold: 100, potions: 0 } for 4 steps
```

This is **not part of the model.** It is a question you are asking it: start a `Player` with a
hundred gold and no potions, and apply the rule four times.

That distinction matters more than it looks like it does, and it gets a chapter in Act III.
For now: the three lines above describe how shopping works. This line describes one experiment.

### Run it

```
job 0
    0  { gold: 100, potions: 0 }
    1  { gold: 70, potions: 1 }
    2  { gold: 40, potions: 2 }
    3  { gold: 10, potions: 3 }
    4  { gold: -20, potions: 4 }
```

Three turns of shopping, and then the player buys a fourth potion **with ten gold**, ending at
minus twenty.

### Sit with that for a second

Nothing went wrong. The machine did exactly what you wrote. You said "every turn, gold goes
down by thirty," and on the fourth turn gold went down by thirty.

You did not say *you cannot buy a potion you cannot afford.* That sentence is not anywhere in
the model. You know it. The model does not.

This is the gap that the next two acts exist to close, and it is worth meeting it here, in a
nine-line model, where it is obvious. In a ninety-line model it is not obvious at all.

### Break it: leave a line out

Delete the `potions:` line, so the mechanism only sets gold.

```
    mechanism:
        gold: s.gold - 30
```

```
    0  { gold: 100, potions: 0 }
  stopped at step 1: the mechanism does not set field `potions`
```

It refuses. A rule must say what happens to **everything** that changes — all of it, every
time.

Your first reaction is probably that this is obnoxious, because obviously potions is meant to
stay the same. But look at what you just did: you decided that silence means "unchanged." That
is a guess about what you meant, and it is exactly the hole from chapter 2 — a description
that reads fine either way.

### Why not the other way?

Plenty of tools do it the other way: anything you do not mention stays as it was. It is
genuinely more convenient, and that convenience is not nothing.

Here is what it costs. Six months from now you add a third thing to your `Player` — say
`reputation`. You have nine rules. Under "unmentioned means unchanged," all nine rules
silently decide that reputation never changes. Some of those nine are right. At least one is a
bug. **Nothing tells you which**, and nothing ever will, because a rule that forgot a field and
a rule that deliberately left it alone look exactly the same.

Under Roxin's rule, adding `reputation` breaks all nine rules immediately, and you go through
them one at a time saying what should happen. That is more work on a Tuesday afternoon and
less work forever afterwards.

And when the answer genuinely is "nothing happens to the rest," you say so out loud:

```
    mechanism:
        gold: s.gold - 30
        ..s
```

`..s` means "and everything else stays as it was." Still a decision, still written down, still
visible to the next person. **[designed — not yet built; the example above will currently
refuse]**

---

## 7. What is state, really?

Chapter 6 told you `state` holds the things that change. True, and not enough. Deciding *what
goes in there* is the hardest part of modeling and the part nobody teaches, so this chapter is
about nothing else.

### The question that answers it

> **What would someone need to be handed in order to carry on from where you left off?**

Not "what is interesting." Not "what do I want to see." What would they **need**.

For the shop: hand someone `gold: 10` and `potions: 3` and they can keep going forever. Hand
them only `gold: 10` and they can also keep going, because the rule does not consult potions —
but then potions was never part of the model, it was just something you wanted to watch.

That question is the whole chapter. Everything below is it, poked from different sides.

### Poke it: what is the player's total worth?

Potions are worth thirty gold each, so total worth is `gold + potions * 30`. It changes every
turn. Should it be in the state?

Try it and see what happens.

```
state Player { gold: int, potions: int, worth: int }

step Shop:
    mechanism:
        gold: s.gold - 30
        potions: s.potions + 1
        worth: s.worth
```

Now `worth` never updates, because the person writing the rule forgot. Fine — fix it:

```
        worth: (s.gold - 30) + (s.potions + 1) * 30
```

Now it is right. And now look at what you have built: the number thirty appears **three
times**, the buying rule is written **twice**, and `worth` is only correct for as long as
nobody ever edits one copy without editing the others.

Change the potion price to forty next month. Three edits, and if you miss one, the model keeps
running and quietly reports a wrong number. Nothing crashes. Nothing warns you. You get a
plausible answer that is false.

> **If you can work it out from the other things, do not store it.**

Stored-and-derivable means two sources of truth, and two sources of truth means one of them is
eventually wrong.

### So where does it go?

You still want to see it. It just is not state — it is something **calculated from** state,
fresh, every turn. Roxin will have a way to say that, and what it will be called is genuinely
not decided yet.

**[undecided]** Here is the same line under the names being weighed:

```
derived Worth(s: Player): int = s.gold + s.potions * 30   -- what it is: worked out, not stored
observe Worth(s: Player): int = s.gold + s.potions * 30   -- why: something you want to watch
track   Worth(s: Player): int = s.gold + s.potions * 30   -- what happens: it gets recorded
metric  Worth(s: Player): int = s.gold + s.potions * 30   -- what the build calls it today
```

The shape is new, so read it once slowly. `Worth` is the name you are giving this calculation.
`(s: Player)` says "given a `Player`, which I will call `s`" — which is where the `s.` in the
rest of the line comes from. `: int` says the answer is a whole number. Everything after the
`=` is the calculation.

The argument against `metric`, which is the oldest of the four, is that the word already means
something precise in this field — a way of measuring the distance between two states — and
using it for "a number I want in the output" quietly burns a word that is needed elsewhere.
The argument for `derived` is that a newcomer can guess what it means without being told. The
argument for `observe` is that it says why you wrote it.

This guide writes **`derived`** from here on, as a placeholder. If the decision lands
elsewhere, these examples change and the idea does not. *(None of the four is implemented
today.)*

What matters is not the word. It is that the number is computed from state and not kept
alongside it, so it cannot drift out of step with the thing it describes.

### Poke it: can a rule see three turns ago?

A natural thing to want. "Spend more if gold has been falling."

**No.** A rule sees the state as it is now. That is all there is.

Before deciding that is a limitation, notice that you can model memory perfectly well — you
just have to say you are doing it:

```
state Player { gold: int, potions: int, gold_last_turn: int }

step Shop:
    mechanism:
        gold: s.gold - 30
        potions: s.potions + 1
        gold_last_turn: s.gold
```

Now the rule can compare `s.gold` against `s.gold_last_turn`, because the memory is a thing
that changes, written down, like everything else.

And look at what that bought: anyone reading this state declaration can see **at a glance that
this model remembers exactly one turn.** Not two. Not all of them. One. That fact is in the
state, where it can be read, instead of in the rules, where it would have to be hunted for.

### Why not the other way?

Some tools hand every rule the entire history — every state the model has ever been in — and
let the rule rummage through it. It is convenient. You never have to decide in advance what to
remember, and you can change your mind later without touching the state.

What it costs is almost everything this guide is building toward.

If a rule can reach into the whole past, then "what does this rule do?" has no answer that
does not involve the entire run. You cannot look at a rule and a state and work out what comes
next, because the rule might be consulting turn seventeen. Act IV — where a machine checks
that something can never happen — becomes impossible, because there is no longer any such
thing as "a state"; there is only "a state plus everything that led to it," and that is a
different, much larger thing to reason about.

It also makes models dishonest in a small way. Under history-passing, a model that secretly
depends on turn one looks identical to a model that does not. Under Roxin's rule, a model that
remembers has a field called `gold_last_turn` sitting in plain sight.

What is genuinely lost: convenience, and the ability to change your mind about what to
remember without editing the state declaration.

### Poke it: what is a field allowed to hold?

Numbers. True-or-false. Soon, a fixed set of named choices like `Buy` or `Sell`.

Not: a connection to a website. Not a file. Not a spreadsheet. Not a lump of code from another
language that you call into.

Several tools in this space do allow exactly that, and it is extremely convenient — you fetch
a live price mid-model, or call a library that already does the hard sum.

Here is why Roxin does not. Everything in Act IV depends on three things being possible: you
can **write a state down** completely, you can **tell whether two states are the same**, and
you can **list the possibilities**. An opaque object from somewhere else breaks all three at
once. You cannot write it down, so results are not reproducible. You cannot compare it, so you
cannot tell if the model has returned somewhere it has been. You cannot enumerate it, so you
cannot ask what might have happened.

The loss is real. You cannot call out to a pricing library in the middle of a step. The
exchange is that everything you *can* write is something a machine can reason about.

---

## 8. Reading the dots

*runs today*

A short chapter about two characters, because they carry more than their size suggests.

```
gold: s.gold - 30
```

`s` means **the state, as it was when this step began.**

`.` means "the field called."

So `s.gold` is *the gold, as it was at the start of this step.* The `s.` is not decoration and
it is not a prefix you get used to ignoring. It is a timestamp.

### Why can you not just write `gold`?

Try it.

```
    mechanism:
        gold: gold - 30
```

```
  stopped at step 1: unknown name `gold`
```

It refuses, and the reason is that `gold` would have to mean two different things in one line.

On the **left** of the colon, `gold:` means the new gold — the one being produced. On the
**right**, you want the old one. Written as `gold: gold - 30`, the line says "the new gold is
the old gold minus thirty" only if you already know which side means which. It would read
equally well as a statement that is never true, the way `x = x - 30` is never true in
ordinary arithmetic.

Roxin removes the ambiguity by making the two look different. Left of the colon: the new
value, written bare. Right of the colon: old values, which must carry their timestamp.

### Why not the other way?

Other tools distinguish them in other ways — a prime mark, a `next.` prefix, an arrow. One
common choice is to let you write `gold` on the right and have it mean the old value, since
the new one cannot be read anyway.

That works, and it is less typing. What it costs is that reading becomes positional: you have
to notice which side of the colon you are on to know what a name means. With `s.`, the name
carries its own meaning wherever it appears — including halfway through a long expression with
brackets in it, where "which side am I on" stops being obvious.

There is a second `.` prefix, `u.`, which you will meet in Act III. It is the same idea: a
name that says where its value came from.

---

## 9. Reading what came out

*runs today*

```
job 0
    0  { gold: 100, potions: 0 }
    1  { gold: 70, potions: 1 }
    2  { gold: 40, potions: 2 }
    3  { gold: 10, potions: 3 }
    4  { gold: -20, potions: 4 }
```

### Count the rows

You asked for four steps. There are five rows.

Row 0 is not a step. It is where you said to start — the `from Player { gold: 100, potions: 0 }`
part of your question, printed back at you before anything happened. Rows 1 through 4 are the
four applications of the rule.

Everyone trips on this once. **Four steps, five rows**, because the beginning is not something
the rule produced.

It is worth seeing why the alternative is worse. If row 0 were left out, the output would
begin at `{ gold: 70, potions: 1 }` and you would have no way to tell, from the output alone,
what the model started from. The first row would be an answer with its question thrown away.

### What the first line means

`job 0` is nearly empty, and it is supposed to be. A job is one run with one set of settings.
You only asked one question, so there is one job, numbered zero, with nothing to report about
how it differs from any other. In Act III you will ask several questions at once and that line
starts carrying the differences.

### Now the interesting part

Gold is negative.

Here is the habit worth forming on your very first model, because it is the habit that makes
modeling useful rather than decorative. A surprising result is **one of three things**, and
your job is to work out which.

**The rule is wrong.** You meant "buy a potion if you can afford one" and you wrote "buy a
potion." The model is a faithful description of something you did not mean. Fix the model.

**The question is wrong.** The rule is right — shopping does cost thirty gold — but asking for
four turns from a hundred gold is asking what happens past the point where the model stops
describing anything real. Fix the question.

**The model is right and you have learned something.** A player with a hundred gold buying
thirty-gold potions goes broke on the fourth purchase. If you did not know that, you do now,
and the model has already paid for itself.

Those three look identical in the output. Telling them apart is modeling, and nothing automates
it. What a good language can do is make the first one rarer and the third one trustworthy.

### What is not in there

**Anything you worked out rather than stored.** The player's total worth from chapter 7 is
nowhere, because you never asked for it.

**Anything in between.** Later your model will have several rules running in order within one
turn. The states in between are computed and then dropped; only the end of the turn is
recorded, unless you ask otherwise.

**Why it stopped.** Here it stopped because you asked for four steps and got four. In Act IV
a run can also stop because something you said must never happen, happened — and then this
output gains a line telling you so.

---

## 10. Everything happens at once

*runs today*

### A model that should not work

Two numbers that trade places.

```
state Pair { a: int, b: int }

step Swap:
    mechanism:
        a: s.b
        b: s.a

system Swapper: Pair = Swap

run Swapper from Pair { a: 1, b: 2 } for 3 steps
```

In plain English: *the new `a` is the old `b`; the new `b` is the old `a`.*

If you have ever written anything down as a list of instructions — a recipe, directions to
your house — you should be suspicious. Follow it like a list:

> **Line 1.** Put `b`'s value into `a`. So `a` becomes 2.
> **Line 2.** Put `a`'s value into `b`. But `a` is 2 now — we just changed it. The 1 is gone,
> written over. So `b` becomes 2 as well.

Both end up 2. The 1 is destroyed. That is what happens in almost every tool you have used,
and it is why anyone who has programmed will tell you a swap needs a third variable to hold
the value you are about to lose.

Run it:

```
    0  { a: 1, b: 2 }
    1  { a: 2, b: 1 }
    2  { a: 1, b: 2 }
    3  { a: 2, b: 1 }
```

They swapped. The 1 survived. Nothing was written over, and there is no third variable
anywhere.

### The idea

**Every line of a mechanism looks at the state as it was when the step began.**

Not as it is partway through. There is no partway through. The step takes the state at one
moment and produces the state at the next moment, and every line is answering the same
question about the same single instant.

That is what chapter 8's `s.` was for. `s.a` is not "a" — it is *"a, as it was at the start of
this step."* The `s` is a timestamp, and this is the chapter where that matters.

### Let us watch it happen

The step begins. The state is `{ a: 1, b: 2 }`. Freeze it. This is what every line will see,
no matter what any other line does.

| The line | `s` is frozen at | So it reads | It produces |
|---|---|---|---|
| `a: s.b` | `{ a: 1, b: 2 }` | `s.b` = **2** | new `a` = 2 |
| `b: s.a` | `{ a: 1, b: 2 }` | `s.a` = **1** | new `b` = 1 |

Both lines read the same frozen picture. Neither can see what the other produced, because
neither has produced anything yet — they both produce at the end, together. The new state
`{ a: 2, b: 1 }` comes into existence all at once, and *then* the next step freezes that.

### Now break it

That is a small difference on a swap. Here is one where it decides the entire answer.

```
state Fib { a: int, b: int }

step Next:
    mechanism:
        a: s.b
        b: s.a + s.b

system Fibonacci: Fib = Next

run Fibonacci from Fib { a: 0, b: 1 } for 6 steps
```

```
    0  { a: 0, b: 1 }
    1  { a: 1, b: 1 }
    2  { a: 1, b: 2 }
    3  { a: 2, b: 3 }
    4  { a: 3, b: 5 }
    5  { a: 5, b: 8 }
    6  { a: 8, b: 13 }
```

0, 1, 1, 2, 3, 5, 8, 13 — the Fibonacci numbers, each the sum of the two before it.

**Now break the rule.** Suppose the lines ran in order, each seeing the previous one's result.
Trace it by hand from `{ a: 0, b: 1 }`:

> `a: s.b` — `a` becomes 1.
> `b: s.a + s.b` — but `a` is 1 now. So `b` becomes 1 + 1 = **2**. State: `{ a: 1, b: 2 }`.
>
> Next time: `a` becomes 2. Then `b` becomes 2 + 2 = **4**. State: `{ a: 2, b: 4 }`.

1, 2, 4, 8, 16. **Doubling.** Not Fibonacci — powers of two.

You do not have to take my word for it, because Roxin can be made to behave that way
deliberately, which we will get to in a moment:

```
    0  { a: 0, b: 1 }
    1  { a: 1, b: 2 }
    2  { a: 2, b: 4 }
    3  { a: 4, b: 8 }
    4  { a: 8, b: 16 }
```

Same two lines of arithmetic. Same starting numbers. **Fibonacci or powers of two**, decided
entirely by whether the lines look at the same moment or at different ones.

This is why the rule gets its own chapter. It is not a technicality.

### Poke it: does the order of the lines matter at all?

If both lines read the same frozen picture, writing them the other way round should change
nothing.

```
    mechanism:
        b: s.a
        a: s.b
```

```
    0  { a: 1, b: 2 }
    1  { a: 2, b: 1 }
    2  { a: 1, b: 2 }
    3  { a: 2, b: 1 }
```

Identical. So here is a fact you can lean on: **you may sort the lines of a mechanism
alphabetically and your model is unchanged.** Order carries no meaning inside a mechanism.

If you have worked with instructions before, that is a genuinely strange thing to be told, and
it is worth sitting with. In the list-of-instructions version, order would matter enormously —
`a` then `b` gives `{2, 2}`, and `b` then `a` gives `{1, 1}`. Two different answers from the
same two lines in a different order.

### Why it has to be this way

Three reasons, and none of them is "because that was easier to build."

**Because a rule describes a moment, not a procedure.** You are not writing down what to *do*.
You are writing down what the world *looks like next*. "Next" is a single moment, and
everything true at that moment is true together. The question "which happened first, the new
`a` or the new `b`?" has no answer, the same way there is no answer to which hand of a clock
moved first.

**Because order would be meaning you never wrote down.** If moving a line changed what your
model does, then part of your model's behaviour would live in the arrangement of text — not
stated anywhere, not explainable, not something you believe about the system. Someone tidying
your file alphabetically would silently change your results. Every line in a model should be a
claim you could defend out loud, and "this line is second" is not a claim about gold or
potions or temperature.

**Because it is what makes the rest of this book possible.** Later you will ask a machine
questions like "could this gold ever go negative?" To answer, it has to understand what one
step does as a *single* thing. If the lines ran in sequence, "what this step does" would mean
tracing text line by line, and the machine would be reduced to trying it and seeing — which is
exactly what you came here to stop doing.

### Poke it again: what if I genuinely want it in sequence?

Sometimes you do. Settle the trades, *then* charge the fee on the settled amount. Apply
damage, *then* check whether anyone died. The second thing truly must see the first thing's
result.

The answer is not to reorder lines. It is to say plainly that there are **two moments**, which
means two rules:

```
step ShiftA:
    mechanism:
        a: s.b
        b: s.b

step ThenB:
    mechanism:
        a: s.a
        b: s.a + s.b

system Sequential: Fib = ShiftA >> ThenB
```

`>>` means "do this one, then do that one." That is the model that produced the doubling
numbers above. Each rule is still internally simultaneous; `ThenB` sees a frozen picture, it
is just a *later* frozen picture than `ShiftA` saw.

And notice what you got by being made to write it this way: the sequencing is now **visible**.
It is a `>>` on the system line, where anyone can see it, instead of being an invisible
consequence of which line you happened to type first. This is also the moment the `system`
line from chapter 6 stops being ceremony.

### Where this bites in the wild

The mistake almost never looks like Fibonacci. It looks like this:

```
    mechanism:
        gold:    s.gold - 30
        potions: s.potions + (s.gold - 30) / 30
```

Someone wants the second line to use "the gold we have left," so they write the subtraction
out a second time. That is correct, and it is a warning sign — they have written the same idea
twice because they expected line two to see line one's answer.

The real damage comes later, when the two copies drift apart during an edit. Someone changes
the first line and not the second, and now the model quietly means two different things about
the same quantity.

**If you catch yourself repeating an expression so a later line can "see" an earlier one,
stop.** Either it is genuinely one moment and you should name the shared quantity once, or it
is genuinely two moments and you want two rules.

---

## 11. Numbers you can trust

*mixed*

Chapter 6 made you write `int` and did not say why. Here is why, and it is the chapter most
likely to save you from a wrong answer that looks right.

### Roxin will not pick for you

Every number you declare says what kind it is. There are three kinds.

**`int` — whole numbers.** Counts of things. Potions, players, turns. Exact, always.
No fractions, ever.

**`float` — approximate numbers.** Temperatures, speeds, concentrations. Fast, and what nearly
every scientific tool uses. The word is short for "floating point," which describes how it
works inside and tells you nothing useful.

**`decimal` — exact fractional numbers.** Money. Anything where being off by a hundredth of a
cent is unacceptable. **[designed — not yet built]**

### Break it: watch a float go wrong

Add a tenth, five times, starting from zero.

```
state F { x: float }

step Add:
    mechanism:
        x: s.x + 0.1

system S: F = Add

run S from F { x: 0.0 } for 5 steps
```

```
    0  { x: 0 }
    1  { x: 0.1 }
    2  { x: 0.2 }
    3  { x: 0.30000000000000004 }
    4  { x: 0.4 }
    5  { x: 0.5 }
```

Look at step 3.

That is not a bug in Roxin, and it is not a bug in your computer. Every tool you have ever
used does this; most of them hide it by rounding the display, which is worse, because it
happens anyway and you cannot see it.

Here is the reason, and it needs no mathematics. Write a third as a decimal: 0.3333... and it
never finishes. You have to stop somewhere, and when you stop you are slightly wrong. A
computer stores numbers in twos rather than tens, and **in twos, a tenth is the number that
never finishes.** So 0.1 is stored slightly wrong, and when you add slightly-wrong numbers
together the wrongness piles up until it surfaces in the display.

### What follows from that

**Never use `float` for money.** Not for prices, not for balances, not for anything you would
be embarrassed to be a hundredth of a cent out on. Add a tenth of a dollar ten thousand times
and you will not have a thousand dollars.

**Never check two floats for being exactly equal.** `x == 0.3` can be false when x looks
exactly like 0.3, as step 3 above shows. Roxin will make you say how close is close enough
rather than letting you ask the question that has no good answer. **[designed]**

Floats are not bad. They are the right tool for a temperature, where the thermometer is less
accurate than the number anyway. They are the wrong tool for a ledger.

### Break it: divide two whole numbers

```
    mechanism:
        a: s.a / 2
```

```
    0  { a: 100 }
  stopped at step 1: `/` is not allowed on two ints; use `//` for floor division
```

It refuses, and this one catches people out because every calculator on earth allows it.

Think about what you would want from `7 / 2` when both are whole numbers. Three? Three and a
half? Both are defensible and they are different answers. Most tools pick one silently. If
yours picks 3, then splitting seven potions between two players loses half a potion every
time, and nothing ever tells you.

Roxin makes you say which you meant: `//` throws away the remainder on purpose, and if you
wanted three and a half you were not working with whole numbers and should say so.

### Honest about the current build

Two things do not yet match the plan, and you should know before they confuse you.

**Whole numbers are supposed to be unlimited** — no largest value, so a count can never
silently wrap around to a negative number the way it does in most languages. The build today
has a ceiling, and a big enough number stops the job rather than lying about it. Stopping is
the right failure, and the ceiling is temporary. **[divergence]**

**Mixing kinds currently works and is not supposed to.** If `x` is a float, `s.x + 1` is
accepted today — the whole number 1 is quietly turned into 1.0:

```
    0  { x: 0.5 }
    1  { x: 1.5 }
    2  { x: 2.5 }
```

The decided rule is that this should refuse, for the same reason as the division: a silent
conversion is a decision made on your behalf that you never see. Expect this to stop working.
**[divergence]**

One display note while you are here: a float holding zero prints as `0`, not `0.0`, so the
output does not tell you which kind a column is. The declaration does.

### Why not the other way?

Nearly every tool in this space has one number type, usually a float, and converts between
things as needed. It is much friendlier on the first day. You write `0.1` and `3` and `7/2`
and everything works.

What it costs is that the failures are invisible and arrive late. The money that is off by a
cent after ten thousand transactions. The count that wrapped around to negative. The equality
test that is false for no visible reason. None of those announce themselves; they show up as a
number that is slightly wrong in a report that somebody trusted.

The exchange Roxin offers is that you are made to decide, once, per field, at the moment you
know the answer — and in return you are not quietly wrong later.

---

---

# Act III — Decisions, consequences, and rules

Act II built a model that does what you said. This act is about the difference between what
*happens* and what someone *chose* — which is the idea the whole field is built on — and about
writing down which choices exist at all.

---

## 12. Numbers you can change without rewriting

*runs today*

Look at the shop model again and count how many times the number thirty appears.

```
step Shop:
    mechanism:
        gold: s.gold - 30
        potions: s.potions + 1
```

Once here. Twice if you wrote the total-worth calculation from chapter 7. Three times if you
also wrote it into a note for yourself. And every one of those has to change together or the
model starts lying.

Name it instead.

```
param price: int = 30

state Player { gold: int, potions: int }

step Shop:
    mechanism:
        gold: s.gold - price
        potions: s.potions + 1
```

`param` announces a **setting** — a number that is fixed for the whole run. `price` is the
name, `int` is the kind, and `= 30` is what it is unless something says otherwise.

That is the first noun from chapter 3 — the things that *do not* change — finally getting its
own word.

### Poke it: why not just make it a state field that never changes?

You could. `state Player { gold: int, potions: int, price: int }`, started at 30, and every
mechanism copies it across unchanged.

It works, and it is worse in three ways that are worth seeing.

**It shows up in every row of output.** Five rows, five copies of `price: 30`, every one of
them noise.

**Every rule has to mention it.** Chapter 6 taught that a mechanism must set every field, so
each one needs a line saying `price: s.price`. Ten rules, ten lines of bookkeeping, and the
reader has to check each one to confirm nothing sneaky is happening.

**Nothing stops a rule from changing it.** It is a thing that changes, sitting in the list of
things that change. The only reason it stays at 30 is that nobody wrote a line moving it.

A `param` cannot change. Not "does not" — **cannot**. That guarantee is the entire point, and
it is a guarantee you can read off the declaration without checking a single rule.

### Break it: what if the price *should* change?

Sale days. Inflation. A merchant who raises prices as you get richer.

Then it is not a setting. It is a thing that changes, and it belongs in the state, and you
should move it there.

That is not a failure of `param` — it is `param` doing its job. The question *"does this change
during a run?"* is the entire test, it has exactly one right answer per model, and being made
to answer it is how you find out what you actually believe about the system.

### Why not the other way?

Some tools let settings be adjusted part-way through a run, usually through a callback or a
scheduling mechanism. It is flexible, and it saves you from having to decide up front.

The cost is that "what was the price in this run?" stops having a single answer. To know what
the model did, you need the model *and* the schedule of interventions *and* the order they
fired in. The run is no longer described by the file plus a starting point.

Roxin's exchange: a setting is one number for the whole run, and anything that varies has to
say so in the state, where it is visible.

---

## 13. Asking several questions at once

*runs today*

Naming the price bought something immediately. You can now ask what happens at other prices
without touching the model.

```
run Shopping from Player { gold: 100, potions: 0 } for 4 steps
    sweep price in [20, 30, 40]
```

```
job 0 price = 20
    0  { gold: 100, potions: 0 }
    1  { gold: 80, potions: 1 }
    2  { gold: 60, potions: 2 }
    3  { gold: 40, potions: 3 }
    4  { gold: 20, potions: 4 }
job 1 price = 30
    0  { gold: 100, potions: 0 }
    1  { gold: 70, potions: 1 }
    2  { gold: 40, potions: 2 }
    3  { gold: 10, potions: 3 }
    4  { gold: -20, potions: 4 }
job 2 price = 40
    0  { gold: 100, potions: 0 }
    1  { gold: 60, potions: 1 }
    2  { gold: 20, potions: 2 }
    3  { gold: -20, potions: 3 }
    4  { gold: -60, potions: 4 }
```

Three runs. One command. And the `job` line from chapter 9 — the one that was conspicuously
empty — is now carrying its weight: it tells you which question each block of rows answers.

### What just changed, and what did not

**The model did not change.** Not one character. The four lines describing how shopping works
are identical across all three jobs. Only a number you named was swapped.

**You just did comparative analysis.** At twenty gold a potion the player finishes comfortably.
At thirty they go broke on the last turn. At forty they are broke a turn earlier and twice as
deep. That is a finding, and it cost you one line.

This is chapter 4's two knobs made real. The rule is a machine; a sweep turns the dial and
keeps the machine.

### Poke it: what can you sweep?

Settings. Only settings.

You cannot sweep a state field, because a state field already changes — "try it at 100 and
also at 200" is a question about where to *start*, which is what the `from` part of the run
line is for. You cannot sweep a rule, because a different rule is a different model.

A sweep varies the things that chapter 3 put in the "does not change" pile. That is the whole
category, and it is why chapter 12 had to come first.

### Poke it: more than one sweep

You can sweep several settings at once, and you get **every combination**.

Three prices and three starting golds is nine jobs. Add a third sweep of three and it is
twenty-seven. Add a fourth and you are at eighty-one.

That growth is not a flaw — it is what you asked for — but it is worth meeting early, because
it is the same growth that makes Act IV's harder questions expensive. Four settings with ten
values each is ten thousand runs, and nobody intends to ask for ten thousand runs.

---

## 14. The decision and the consequence

*runs today*

This is the chapter the rest of the book leans on. Everything so far has been machinery; this
is the idea.

### Two different claims, mashed together

Look at the rule once more.

```
step Shop:
    mechanism:
        gold: s.gold - price
        potions: s.potions + 1
```

Read it carefully and you will find it is making **two claims of completely different kinds**.

**"The player buys one potion."** That is a claim about how the player behaves.

**"Buying a potion costs `price` gold and yields one potion."** That is a claim about how the
shop works.

They are tangled into the same two lines, and they are not the same sort of thing at all. The
first could be wrong because you misjudged the player. The second could be wrong because you
misread the shop's price list. They change for different reasons, at different times, usually
for different people.

### Pull them apart

```
step Shop:
    input:
        buy: int = 1
    mechanism:
        gold: s.gold - price * u.buy
        potions: s.potions + u.buy
```

`input:` is the new part. It holds **the decision** — what somebody chose this turn. Here the
decision is called `buy`, it is a whole number, and the way it gets decided is `= 1`: always
buy one.

`u.buy` in the mechanism is how the consequence refers to the decision. Same dot idea as
chapter 8: `s.` means "from the state, as it was," and `u.` means "from the input, this turn."

Run it:

```
    0  { gold: 100, potions: 0 }
    1  { gold: 70, potions: 1 }
    2  { gold: 40, potions: 2 }
    3  { gold: 10, potions: 3 }
    4  { gold: -20, potions: 4 }
```

**Identical to before.** Not a single number changed.

That is the point, and it is worth not rushing past. You have not changed what the model does.
You have changed what the model *says* — two claims that were tangled are now in separate
places with separate names.

### Now change only the decision

The player in that run is an idiot: they buy a potion with ten gold in their pocket. Let us
make them sensible. Buy as many as you can afford, and no more.

```
    input:
        buy: int = s.gold // price
```

That is the only edit. The mechanism is untouched.

```
    0  { gold: 100, potions: 0 }
    1  { gold: 10, potions: 3 }
    2  { gold: 10, potions: 3 }
    3  { gold: 10, potions: 3 }
    4  { gold: 10, potions: 3 }
```

Three potions on the first turn, ten gold left, and then nothing — there is never enough for a
fourth. **Gold never goes negative.**

Stop and look at what just happened. The shop did not change. The price did not change. The
arithmetic of a purchase did not change. One line about *how somebody decides* changed, and
the entire character of the outcome changed with it.

### Two dials that do not interfere

That is what the split buys you, and it is the reason this idea is the foundation of a whole
field.

**You can ask "what if players behaved differently?"** without touching the physics. Greedy,
cautious, random, following a budget — all of those are edits to `input:` alone.

**You can ask "what if the world worked differently?"** without touching behaviour. Bulk
discounts, a transaction fee, potions that sometimes break — all edits to `mechanism:` alone.

In a tangled model, every one of those questions means rewriting lines that do both jobs, and
you cannot be sure you changed only the thing you meant to.

These have proper names, which you will hear from anyone who has studied this. The part that
produces a decision is the **input map**. The part that works out what follows is the **state
update map**. You now know what both of them are.

### The only ordering inside a step

Chapter 10 said everything in a mechanism happens at once, and that is still true. But a step
now has two parts, and between those two parts there is an order:

> **The decision is made first, from the state as it was. Then the consequence is worked out,
> from that same state and the decision.**

Which answers three questions you might have:

**Can the mechanism see the decision?** Yes. That is `u.`.

**Can the decision see the state?** Yes. That is `s.` — and it is how `s.gold // price` works.

**Can the decision see the new state?** No. It does not exist yet. The decision is one of the
things that brings it into being.

This is also the moment `mechanism:` stops being, as chapter 6 put it, a word you were typing
on credit. It has a name because there is now something else it needs to be distinguished
from.

### Break it: put the decision back in the mechanism

You do not have to split them. This works today and gives exactly the same numbers:

```
    mechanism:
        gold: s.gold - price * (s.gold // price)
        potions: s.potions + (s.gold // price)
```

No `input:` anywhere. Same output. So what did you lose?

**You cannot swap the decision.** To make the player greedy you must now edit the two lines
that also describe what a purchase costs, and hope you only changed the half you meant to.

**You cannot tell by reading which half is which.** Someone opening this file sees four
arithmetic expressions. Nothing says which of them is a claim about the shop and which is a
guess about the shopper — and those have very different standards of evidence.

**The decision is written twice.** `s.gold // price` appears in both lines, which is the exact
warning sign chapter 10 ended on.

**And the question Act IV is built on has nowhere to attach.** "What else could the player have
done?" needs there to be a *place* where the player decides. In the split version that place
is `input:`, and it has a name. In the tangled version there is no such place — only arithmetic
that happens to encode one particular choice.

That last one is not a style preference. It is the difference between a model you can only run
and a model you can interrogate.

---

## 15. Other people's words for the decision

You now understand the idea. This chapter is so that you can recognise it when somebody else
uses a different word for it, which they will.

### The same thing, six names

**Policy.** What cadCAD calls it, and what a good deal of the surrounding literature calls it.
If you have seen cadCAD, a policy function is an input map.

**Action.** Common in reinforcement learning and in game theory. An agent observes a state and
takes an action.

**Control input.** Engineering. A controller observes the plant and produces a control signal.
The thermostat deciding how hard to run the heater is producing a control input.

**Decision rule.** Economics, and most formal modelling of people.

**Strategy.** Game theory, usually meaning the whole rule rather than one turn's output.

**Behaviour.** Agent-based modelling, usually meaning the same thing bundled with some state.

They are not identical — each community packs slightly different assumptions in — but if you
hold "the part that chooses, separated from the part that follows," you will read all six
correctly.

Roxin says **input** for the decision and **input map** for the thing producing it, because
those are the words in the formalism it is built on.

### A real difference worth knowing: merged policies

cadCAD does something Roxin deliberately does not, and the reason it does it is good.

In cadCAD you may write several policies for the same step. One handles taxes, one handles
fees, one handles rewards. Each one looks at the state and proposes a number. The framework
then **combines them** — by default adding them up — and hands the combined result to the part
that updates state.

Why that is attractive is obvious the moment you have a decision with four unrelated
contributions. You write four small, separately-understandable rules instead of one big one.
That is real modularity and it is a genuine loss to give up.

Here is the cost. The number that actually drives your model — the combined one — **is written
nowhere.** You can read all four policies and still not know what the step received, because
that depends on a combining rule that lives in the framework rather than in your file. Change
the combining rule and every model's behaviour changes with no edit to any model.

Roxin's position is that the input a step receives must be written in the model, in one place,
as one thing. The cost is that a complicated decision is a complicated expression rather than
four tidy ones. The benefit is that you can point at the line that decided.

*(Worth knowing: the word "policy" is not only cadCAD's. It appears as a role in the newer
tools built directly on this formalism too. Roxin's objection is to the merging, not the
vocabulary.)*

---

## 16. What is legal

*designed — not yet built; the `allowed:` examples below will currently refuse*

The sensible player from chapter 14 never overspends. Ask yourself why not.

Because the *decision* was careful. `s.gold // price` works out what is affordable and never
asks for more. Change one character and the carefulness evaporates.

So here is the question this chapter is about:

> **"You cannot buy a potion you cannot afford" — is that a fact about the shopper, or a fact
> about the shop?**

It is a fact about the shop. It is true for a careful shopper and a reckless one. It is true
for a shopper who has not been written yet. A shop that will not sell you what you cannot
afford has that property no matter who walks in.

And a fact about the shop does not belong inside the shopper.

### Saying it

```
step Shop:
    input:
        buy: int = 1
    allowed: buy in 0..(s.gold // price)
    mechanism:
        gold: s.gold - price * u.buy
        potions: s.potions + u.buy
```

`allowed:` sits between the decision and the consequence, which is exactly where it belongs:
after something has been chosen, before anything follows from it.

It reads as English. *The number bought must be between zero and however many you can afford.*

`0..(s.gold // price)` is a range, written from-to. It depends on the state, which means the
set of legal choices is **different on every turn** — three on the first turn, zero later.
That is normal and it is the usual case, not an advanced one.

### What happens when a decision is illegal

The run stops, and says which step, which value, and which rule it broke. It does not clamp
the number to the nearest legal one, and it does not skip the turn.

Both of those would be *decisions about what happens when the impossible happens*, and making
one quietly is how a model ends up describing something nobody intended.

### Poke it: why not just write a careful decision?

Because then being legal is a property of **whoever wrote the decision**, and it has to be
re-established every time anyone writes a new one.

Swap in a greedy player, a random player, a player following a budget from a spreadsheet —
each one is a fresh opportunity to overspend, and the only protection is that whoever wrote it
remembered. With `allowed:`, the shop's rule is stated once and holds against every decision
anybody ever writes.

### Break it: put the rule in the mechanism

This is what nearly every other tool does, and it works:

```
    mechanism:
        gold:    if u.buy * price <= s.gold then s.gold - price * u.buy else s.gold
        potions: if u.buy * price <= s.gold then s.potions + u.buy else s.potions
```

*"If they can afford it, sell; otherwise nothing happens."* Perfectly sensible. Perfectly
common. And look at what it has quietly done.

**The illegal case now has behaviour.** You have specified what happens when someone buys
something they cannot afford — nothing happens — which means buying what you cannot afford is
no longer impossible. It is a thing that occurs, and has consequences, and the model has an
opinion about it.

**The rule is now in two places.** Both lines test the same condition. Change the rule and you
change it twice, or you change it once and the model becomes incoherent.

**You can no longer ask what the player could have done.** Every number is now a valid input
to a mechanism with branches in it. "What were the legal purchases on turn two?" has no answer
in the file, because legality was dissolved into arithmetic.

That last point is the whole reason `allowed:` exists as its own thing rather than as a
convention. The set of legal choices has a name in this field — the **admissible set** — and
it is the object that every question in Act IV is asked against. If it is not written down
somewhere a machine can read it, none of those questions can be asked.

### Why not the other way?

What `allowed:` costs you is honest and worth stating. It is another line to write. It is a
second place where you can be wrong. And when a decision is illegal your run stops, rather
than limping on and giving you partial output — which is genuinely annoying when you are
exploring and just want to see something.

What it buys is that "what is possible here" becomes a question with an answer, and Act IV
is entirely built out of that answer.

---

## 17. Narrowing, never widening

*designed*

A rule about how `allowed:` may be written, which looks arbitrary and is not.

### The situation

Sale days. Normally you may buy at most three potions; on a sale day, as many as you like.

Your instinct will be to write it like this:

```
    allowed: s.sale or (buy in 0..3)
```

*"Either it is a sale day, or you may buy up to three."* That is correct English, correct
logic, and Roxin will not accept it.

The right way:

```
    allowed: buy in 0..(if s.sale then 99 else 3)
```

The condition moved **inside** the range. Same meaning. Different shape.

### Why, without any mathematics

Picture the legal purchases as a row of numbered boxes — 0, 1, 2, 3, and so on.

Each `allowed:` line is a **filter**. It crosses some boxes out. Whatever is still standing at
the end is legal.

That works beautifully because crossing out is simple and independent. You can run the lines
in any order, one at a time, and the answer is the same. Ten rules, ten passes, done.

Now put an `or` in.

*"Either it is a sale day, or buy is at most three."* On a Tuesday this crosses out 4 and
upward. On a sale day it crosses out nothing. So far, fine.

But add a second `or` line, and a third. Now you cannot take them one at a time, because
whether the first line crosses anything out can depend on which half of the second line is
doing the work. You have to consider **every combination of which side of every `or` is true**.

Three `or` lines is eight combinations. Ten is more than a thousand. Each combination is a
different set of legal purchases, and working out the real answer means doing all of them and
combining the results.

**Crossing out is cheap. Branching is not.** The rule exists so that the legal set can always
be found by filtering, which is a thing you can do, rather than by searching, which is a thing
that gets away from you.

### What it costs

Some rules are genuinely more natural with `or`, and you will hit one eventually and have to
rearrange it. The rearrangement is always possible — any condition can move inside a bound —
but it is sometimes uglier than the sentence you had in your head.

That is a real cost, paid in awkwardness, once per awkward rule. What it buys is that Act IV
can always list the possibilities. The trade was made deliberately.

### A note on the hundred

`0..(if s.sale then 99 else 3)` has a 99 in it that nothing in your system justifies. "As many
as you like" is not a number, and you were made to invent one.

This is a known rough edge rather than a deep feature, and whether there is a better way to
say "no upper limit here" without losing the ability to list possibilities is an open question
in the language's own notes. If it is resolved, this paragraph changes.

---

## 18. What goes in the model, what goes in the question

*runs today*

Here is the whole file from this act.

```
param price: int = 30

state Player { gold: int, potions: int }

step Shop:
    input:
        buy: int = s.gold // price
    mechanism:
        gold: s.gold - price * u.buy
        potions: s.potions + u.buy

system Shopping: Player = Shop

run Shopping from Player { gold: 100, potions: 0 } for 4 steps
    sweep price in [20, 30, 40]
```

Two kinds of line are in there, and telling them apart is the last idea in this act.

**The model.** `param`, `state`, `step`, `system`. How shopping works. True on Tuesday, true
next year, true for anyone.

**The question.** `run`. Where to start, how many turns, which prices to try. What *you*
happened to want to know today.

### Poke it: what about the `= 30`?

`param price: int = 30` looks like a question — it is a specific number — but it is in the
model. Which is it?

It is the model's **own answer**, for when the question does not supply one. The model is
complete on its own: hand it to someone with no run line and it still describes a shop where
potions cost thirty. The sweep then overrides it for a particular investigation.

That is why a param has a default and a state field does not. A state field's starting value is
genuinely a property of the question — "start them with a hundred gold" is something you
chose to ask — so it lives on the run line, in the `from` part.

### Break it: bake the question into the model

Imagine the number of steps were part of the model. Not an absurd design — plenty of tools
work that way.

Now your friend wants to know what happens over eight turns. She edits the model.

And at that moment you no longer have one model with two questions asked of it. You have
**two models**, which must be kept in step with each other by hand, and anything either of you
established about one does not transfer to the other. Did the eight-turn version get the price
fix? Nobody knows without reading both.

### Why this matters more than it sounds like it does

A model is a thing that can be shared, reviewed, argued about, published, and — in the next
act — **proved**. A question is what you happened to ask on a Tuesday afternoon.

If they are mixed, you can never hand anybody "the model." You can only hand them your run of
it, and they have to reverse-engineer which parts were the thing and which parts were your
curiosity.

Keeping them separate also makes the next act possible. A proof is a statement about the
**model** — about every question anyone could ask of it, including questions nobody has thought
of yet. If the question were part of the model, a proof could only ever be about one run, and
a proof about one run is just a run.

---

---

# Act IV — Being sure

Everything so far produces numbers. This act is about knowing things.

Almost nothing in it runs yet, so the markings matter more than usual. The ideas are settled;
several of the spellings are not.

---

## 19. Claims you are making

*designed — not yet built*

Act III's shop, with a player who buys one potion a turn regardless, ends at minus twenty gold.
You know that is wrong. The model does not.

So tell it.

```
param price: int = 25
param purse: int = 100

state Player { gold: int, potions: int }

invariant Solvent(s: Player) = s.gold >= 0

step Shop:
    input:
        buy: int = 1
    mechanism:
        gold: s.gold - price * u.buy
        potions: s.potions + u.buy

system Shopping: Player = Shop

run Shopping from Player { gold: purse, potions: 0 } for 5 steps
```

`invariant` announces **a claim you are making about every state this model can be in.**
`Solvent` is a name you chose — and naming it matters, because when it fails the failure will
say `Solvent` rather than quoting an expression back at you.

`(s: Player)` is the shape from chapter 7: given a `Player`, which I will call `s`. After the
`=` is the claim itself, which must come out true or false.

### What it does

Nothing, during a normal turn. An invariant does not prevent anything, does not change
anything, and does not appear in the arithmetic. It sits there being checked.

The starting state is checked before anything runs. Then every state the model produces is
checked as it is produced. The moment one comes out false, that job stops, **the offending
state is recorded as the final row**, flagged with the invariant's name, and the other jobs in
your sweep carry on.

That last detail is deliberate and worth noticing: the bad state is kept, not thrown away. The
thing you most want to look at is the thing that went wrong, so it is the last row of the
output rather than something you have to reconstruct.

### Poke it: how is this different from `allowed`?

They are the two halves of a pair, and mixing them up is the most common confusion in this
act.

| | `allowed` | `invariant` |
|---|---|---|
| Is about | a **decision** | a **state** |
| Asks | may this be chosen? | is this an acceptable place to be? |
| When | before the consequence | after the consequence |
| On failure | the choice was never legal | the model reached somewhere it should not |

`allowed: buy in 0..(s.gold // price)` says *you may not choose to overspend.*
`invariant Solvent(s: Player) = s.gold >= 0` says *gold is never negative.*

### So why have both?

Because they catch different mistakes, and here is the one that shows why you want the second
even when you have the first.

Suppose the shop is generous and gives a discount on Tuesdays, and you got the discount
arithmetic slightly wrong, so on Tuesdays the price comes out negative. Now a perfectly legal
purchase — one the `allowed` rule happily permits, because it is well within what the player
can afford — produces a state with impossible gold.

`allowed` cannot catch that. Nothing was chosen wrongly. The **consequence** was wrong, and
only something looking at the resulting state will see it.

> `allowed` guards the door. `invariant` checks the room.

### What makes a good claim

Three kinds cover nearly everything, and all three are things you already believe before you
start.

**Conservation.** Something is neither created nor destroyed. Money moves but the total holds.
`s.gold + s.potions * price == purse`. These are the most valuable invariants you can write,
for a reason that becomes clear in chapter 23.

**Bounds.** Something never goes past a limit. Gold is never negative. Infected people never
exceed the population. A fraction stays between zero and one.

**Relationships.** Two things that must stay in step. Every potion sold was paid for. The
ledger balances.

### Poke it: what may a claim look at?

The state, and the settings. That is all.

It may not look at which turn it is, and it may not look at what the decision was. Both
restrictions have the same reason: an invariant is a claim about **a state**, and a state is
not accompanied by a turn number or a history. If your claim needs to know the turn, then the
turn is part of the state and belongs in the state — which is chapter 7, again, from a new
angle.

### Break it: use an invariant where you meant `allowed`

Write `invariant Solvent(s: Player) = s.gold >= 0` and no `allowed` rule, and run the
one-potion-a-turn player.

The job stops at the turn gold goes negative, and the final row shows you minus twenty. You
caught it.

But look at *when*. The overspend happened, the state was produced, and only then did anything
object. Compare with `allowed`, which refuses the purchase before the money moves.

Neither is wrong. They answer different questions: "is this choice available?" and "did we end
up somewhere impossible?" Use the one that matches what you actually believe. If a shop
genuinely cannot sell on credit, that is `allowed`. If gold genuinely should never be negative
no matter how it got there, that is an invariant. Usually both are true and you write both.

---

## 20. The limits of running it

*runs today — the limitation is the point*

You wrote the claim. You ran the model. The job stopped at turn four with minus twenty gold.

**What exactly do you now know?**

### Less than it feels like

You know that *this* model, from *this* starting point, with *this* decision rule, at *this*
price, goes negative on turn four.

Change the purse to 120 and run it again. It survives all five turns. Have you fixed it?

Run six turns. No.

### Count what you did not test

The shop is nine lines. Suppose the purse could be anything from 0 to 200, the price anything
from 10 to 50, and the player may buy between zero and three potions each turn.

- Starting purses: 201
- Prices: 41
- Decisions per turn: 4
- Over ten turns, decision sequences: 4 × 4 × ... ten times = **1,048,576**

Multiply: 201 × 41 × 1,048,576 — about **8.6 billion** distinct runs.

That is a nine-line model with two settings and one decision. Your real model will have six
things in its state and three decisions, and the number stops being worth writing down.

You ran one of them.

### This is not a tooling problem

It is tempting to think a faster computer fixes this. It does not, and it is worth being
precise about why.

The number of possible runs grows by **multiplication**, and your ability to run them grows by
**addition**. A computer ten times faster runs ten times as many. Adding one more turn to the
model multiplies the count by four. You lose that race immediately and permanently, and the
model does not have to be large — it has to be larger than tiny.

### What running is good for

Running is still how you find out most things, and this chapter is not an argument against it.

It shows you **what actually happens**, which is often surprising and always concrete. It
produces numbers you can plot, compare, and show somebody. It catches the mistake you made
twenty minutes ago, quickly, which is most mistakes.

What it cannot do is tell you about the run you did not do. Every run is a sample of one, and
the next three chapters are about what else is available.

---

## 21. Trying a lot of times is not the same as being sure

*mixed — sampling runs today, the rest is designed*

Here are three sentences about the shop. They look like weak, medium, and strong versions of
one claim. They are **three different claims**, and telling them apart is the most useful
thing in this act.

> **One.** "I ran ten thousand random players and gold never went negative."
>
> **Two.** "I checked every legal sequence of purchases out to ten turns and gold never went
> negative."
>
> **Three.** "Gold can never go negative."

### What each one actually says

**One is about the ten thousand.** It is a statement about the trials you ran, and it says
nothing whatsoever about the one you did not. Worse, random trials are drawn toward the
ordinary — that is what random means — and bugs are usually not ordinary.

Imagine the discount bug from chapter 19, where the price goes wrong only when gold is exactly
25 and it is a sale day. What are the odds a random player lands exactly there? Perhaps one in
a million. Ten thousand trials will almost certainly miss it, and will report success with
total confidence.

**Two is genuinely exhaustive, and genuinely bounded.** Every possibility, no exceptions —
*out to ten turns*. It says nothing at all about turn eleven. If the bad state takes twelve
turns to reach, this finds nothing and is not wrong; you asked about ten.

**Three is about all starting points, all decisions, and all time.** It is not an improved
version of the other two. It is a different kind of statement, and it is established a
different way — which is chapter 22.

### Why this is worth labouring

Because the three are routinely reported in the same words. "We tested it and it's fine" is
said by people who mean one, two, and three, and the sentence does not distinguish them.

If you take one thing from this chapter: **a sample never becomes a proof by getting bigger.**
A million trials is a better sample than ten thousand. It is not closer to certainty in any
way that matters, because the thing it misses is not rare by accident — it is rare *because* it
depends on a coincidence, and coincidences are what sampling is worst at.

### Poke it: so when is sampling the right answer?

Often. There is a whole class of question where proof is useless and sampling is the only
tool, and it is easy to lose sight of that in a chapter like this one.

Proof answers **"can it ever?"** Sampling answers **"how often, and how much?"**

- *Can gold ever go negative?* → proof
- *What is the average number of potions after ten turns?* → sampling
- *How often does the player go broke?* → sampling
- *What is the worst case?* → proof, or exhaustive search
- *What does the distribution of outcomes look like?* → sampling, and nothing else

"How often" questions are not weaker questions. For most real decisions — how much inventory
to hold, how to price something, whether a buffer is big enough — they are the questions you
actually have. Nobody wants to know whether the queue *can* reach a thousand. They want to
know how often it exceeds twenty.

### Where the three live

**Sampling** is what you have been doing all along, and what most tools in this area do
exclusively — including the mainstream ones. It runs today.

**Exhaustive search to a bounded depth** is chapter 25. It needs the legal choices to be
listable, which is exactly why chapters 16 and 17 fussed so much about `allowed`.

**Proof** is chapter 22.

A complete tool offers all three and tells you which one answered your question. The mistake is
not using sampling. The mistake is using sampling and reporting it as if it were three.

---

## 22. Proving instead of running

*open — the idea is settled, the shape is not*

Chapter 20 showed that you cannot check 8.6 billion runs. This chapter is about checking
**two** things instead, and getting all of them.

### Dominoes

A thousand dominoes stand in a line. You want to know that all of them fall.

You do not watch. You check two things:

> **The first one falls.**
>
> **Any domino that falls knocks over the next one.**

If both are true, every domino falls. You did not watch a thousand events. You checked two
facts, and the thousand followed.

Nothing deeper is going on. That is the entire idea, and it is called **induction**.

### The same two checks, for a model

Take the shop with the careful player and the `allowed` rule, and the claim that gold is never
negative.

```
param price: int = 25
param purse: int = 100

invariant Solvent(s: Player) = s.gold >= 0

step Shop:
    input:
        buy: int = s.gold // price
    allowed: buy in 0..(s.gold // price)
    mechanism:
        gold: s.gold - price * u.buy
        potions: s.potions + u.buy
```

**Check one: the first domino.** Does the starting state satisfy the claim? The player starts
with `purse` gold, which is 100, and 100 is not negative. ✓

**Check two: any falling domino knocks over the next.** If a state satisfies the claim, does
the state after it?

Assume `gold >= 0`. The `allowed` rule says `buy` is between 0 and `gold // price` — at most
however many potions the gold stretches to. So the amount spent, `price * buy`, is at most
`price * (gold // price)`, which is at most `gold`. Spending at most what you have leaves at
least nothing. ✓

Both hold. Gold is never negative — from any starting purse, under any legal decision, for any
number of turns, forever. **No runs were performed.**

Read that argument again if you like. It is arithmetic. There is no step in it a careful
person cannot follow, and that is not a property of this example being easy — it is a property
of what Roxin models are.

### Why this is reasonable to ask of a machine

If you have heard that proving things about programs is a specialist discipline requiring
years of training, you have heard correctly, and it is worth knowing why that does not apply
here.

In an ordinary programming language, the hard part is almost never the claim you care about.
It is that to prove anything about a loop you must first **invent** a statement that is true
every time round it — usually a statement about counters and partial results and pieces of
array, a statement that has nothing to do with what you were trying to say, and that is often
harder to find than the program was to write. That invention step is where people give up, and
it is the reputation the whole field has.

A Roxin model has no loops. It has no memory to rummage through, no two names pointing at the
same thing, nothing that calls itself. It is a flat list of things that change and a rule that
says what they become.

So there is nothing to invent. **The claim you wanted to make is already the claim the proof
needs.** You wrote "gold is never negative" because that is what you care about, and that is
exactly the sentence the two checks are run against.

The hard part of verification was removed by the shape of the thing being verified, before
anybody wrote any of it.

### Poke it: is it really that easy?

No, and chapter 23 is the honest version. Sometimes the claim you care about is true but the
second check fails anyway, and understanding why is the last genuinely new idea in this book.

---

## 23. True, but not provable yet

*open*

Set the price to 25 and the purse to 100, so the arithmetic stays tidy: each potion costs
exactly a quarter of the starting money.

Here is a claim that is plainly true.

```
invariant FewPotions(s: Player) = s.potions <= 4
```

The player starts with 100 gold. Potions cost 25. They can afford four, ever, and after that
there is nothing left to buy with. The player will never hold five potions.

Watch the second check fail anyway.

### Running the check

*If a state satisfies the claim, does the state after it?*

The machine assumes **only the claim** — `potions <= 4` — and then looks for any state
satisfying it that leads somewhere breaking it. Nothing else is assumed, because nothing else
was said.

So it picks:

```
    from    Player { gold: 1000, potions: 4 }
```

A thousand gold. Four potions. Does that satisfy `potions <= 4`? Yes — four is not more than
four. So it is a state the machine is entitled to consider.

A legal purchase: `buy in 0..(1000 // 25)`, so anything up to forty. Take forty.

```
    gives   Player { gold: 0, potions: 44 }
```

Forty-four potions. The claim is false. **Not inductive.**

### The machine is not being stupid

Look at the state it picked: a thousand gold and four potions. Your model never reaches that.
The player started with a hundred and gold only ever goes down.

So the machine chose a state that cannot happen, and broke your claim with it — which looks
unfair until you see what it is telling you, which is precise and correct:

> **Your claim is true for a reason you did not write down.**

Why can't the player hold five potions? Not because of anything in `FewPotions`. Because
**the total is a hundred**. Gold plus potions-worth never changes — buying converts one into
the other at exactly 25 a time. Four potions is the most a hundred buys.

You knew that. You have known it the whole time. It is not written anywhere.

### Write it down

```
invariant Worth(s: Player) = s.gold + s.potions * price == purse
invariant FewPotions(s: Player) = s.potions <= 4
```

Now run the check again. The machine may assume **both**, so it is no longer allowed to pick a
thousand gold and four potions — that state has a worth of 1100, and the first claim rules it
out.

Every state it may now pick has `gold + potions * 25 == 100`. Buying `b` more needs
`25 × b` to be no more than `gold`, which is `100 - 25 × potions`. So `potions + b` is at most
four. ✓

Inductive. And the first check is easy: the player starts with a hundred gold and no potions,
so the worth is a hundred, and nought is not more than four. ✓

### The move you just made

It has a name — **strengthening the invariant** — and it is the entire job.

You did not change the model. You did not change what you were claiming. You added **the
reason your claim was true**, and the reason happened to be a conservation law.

That is not an accident, and it is why chapter 19 said conservation claims are the most
valuable ones you can write. A conservation law is the usual missing ingredient, because it is
the thing a modeller knows so well they forget it is knowledge.

### Why this is not the Dafny problem

Chapter 22 claimed there is nothing to invent, and this chapter has you inventing something.
Both are true, and the difference is in **what kind of thing**.

In an ordinary programming language the missing statement is about machinery — an index, a
partial sum, which half of an array has been processed. It has nothing to do with what you
were trying to say. You have to reverse-engineer it from code, and it will be different for
every loop you write.

Here, the missing statement was *"gold plus potions-worth is always a hundred."* That is a fact
about shopping. You believed it before you opened the file. It would be the first thing you
said if somebody asked you to explain the model out loud.

The work is not the same work. One is bookkeeping about an implementation; the other is saying
out loud something you already thought. The second is still work — but it is work that makes
your model more honest, rather than work that only serves the tool.

### A habit worth forming

When a check fails this way, do not immediately weaken your claim. The instinct is to write
`potions <= 44` and move on, and it is wrong — it makes the claim true and useless.

Ask instead: **why did I believe this?** The answer is nearly always a sentence you can write
down, and writing it down makes the model better whether or not the proof ever runs.

---

## 24. When the machine says no

*undecided — the shape of this output has not been designed*

A proof that fails has to tell you something you can act on, and this is where most tools in
this area lose people. The standard experience is a message like *"assertion might not hold,"*
possibly with a line number, and then you are on your own.

Roxin can do considerably better, and the reason is structural rather than a matter of effort.

### What a failure looks like

Here is the chapter 23 failure, written out. **This format is a candidate, not a decision.**

```
invariant FewPotions is not inductive on step Shop

  counterexample:
    from    Player { gold: 1000, potions: 4 }
    input   buy = 40                 -- legal: buy in 0..(s.gold // price)
    gives   Player { gold: 0, potions: 44 }

    FewPotions says  s.potions <= 4
    but              44 <= 4         is false

  reproduce:
    run Shopping from Player { gold: 1000, potions: 4 } for 1 steps
```

### Why it can always be this concrete

Two reasons, and both come from decisions made much earlier in this book.

**The check only ever looks at three things.** One state, one decision, the resulting state.
That is its whole field of view — it never looks at a trajectory, because induction does not
involve one. So when it fails, the failure *is* those three things. There is nothing longer to
report, and no history to summarise.

**All three fit on a line.** Chapter 7 said a state field holds a number, a true-or-false, or a
named choice — nothing opaque, nothing from another language. The cost was that you cannot call
out to a pricing library. The benefit is right here: every state can be written down
completely, so the machine can hand you one.

In a general-purpose language the state is a sprawl of objects pointing at other objects,
sometimes in circles. There is often no way to print it that a person could read. That is the
real reason those tools say "assertion might not hold" — frequently they have nothing
comprehensible to show.

### Compare with finding it by running

If this bug surfaced during a run, you would get:

```
  stopped at step 437: invariant FewPotions violated
```

Now read backwards through four hundred rows working out when it really went wrong, because
the step where the invariant *failed* is rarely the step where the mistake *was*.

The inductive check cannot produce that message, because it never ran four hundred steps. It
hands you the shortest possible object: the step itself.

### The last line is the point

```
  reproduce:
    run Shopping from Player { gold: 1000, potions: 4 } for 1 steps
```

That is valid Roxin. Paste it into your file and run it, and you watch the failure happen with
all the tools you already know how to use.

A failed proof that hands you back a runnable model is not a verification experience. It is a
**debugging** experience — the one you already have, with the one piece you were missing
supplied.

### When no answer is possible

Sometimes the machine will say neither yes nor no. You should know in advance when, so it does
not feel like a defect.

**Curvy arithmetic.** Growth and decay curves, anything with a rate compounding continuously.
These involve functions where "is this ever true?" has no general answer — not "is hard,"
**has no answer**, provably. There are good techniques that get close with a stated margin, and
when one is used, the honest report is "proved, up to this margin," not "proved."

**Floats.** Chapter 11 showed `0.1 + 0.1 + 0.1` landing on `0.30000000000000004`. A proof
about float arithmetic is a proof about *that*, not about the arithmetic you meant, and it is
slow besides. Where exactness matters, exact types matter.

**Questions about forever.** "Does gold never go negative" is a safety question and the two
checks answer it. "Does the player eventually go broke" is a different shape — something must
*happen*, not merely never happen — and it needs a different argument, roughly "here is a
number that goes down every turn and cannot go down forever."

**Unlimited amounts of stuff.** A model with a list that can grow to any length raises
questions about all possible lengths at once, which is a step up in difficulty from anything
in this book.

None of these needs mathematics that does not exist. They are known territory with known
costs, and a tool that is honest about which one you are in is doing its job.

---

## 25. What could have happened

*designed*

One more way of being sure, sitting between a run and a proof.

### The actual and the potential

Every run you have done shows **what happened** — one path, one decision each turn, one row
after another.

But at every turn there was a set of legal choices and the player took one. The others did not
stop existing. They just did not happen.

A run shows the path. This chapter is about the fan.

### Fanning out

From a state, list every legal decision and apply the rule to each. One state in, several
states out.

Take the shop at `{ gold: 100, potions: 0 }` with price 25, and `allowed: buy in 0..(s.gold //
price)`. The legal purchases are 0, 1, 2, 3, 4 — five of them. So five successors:

```
  { gold: 100, potions: 0 }
      ├─ buy 0 →  { gold: 100, potions: 0 }
      ├─ buy 1 →  { gold:  75, potions: 1 }
      ├─ buy 2 →  { gold:  50, potions: 2 }
      ├─ buy 3 →  { gold:  25, potions: 3 }
      └─ buy 4 →  { gold:   0, potions: 4 }
```

Do it again from each of those, and again, and you have every state the model can reach in
three turns. Not a sample — **all of them**.

That is chapter 21's second kind of statement, and this is the machinery that produces it.

### This is why chapters 16 and 17 mattered

Look at what the first step needed: *list every legal decision.* 

If there is no `allowed` rule, the legal decisions are every number of that kind — and there is
no listing those. If the rule is written with `or` in it, working out what is legal means
considering every combination of branches rather than filtering a range.

**The fan-out is only possible because the legal set is written down and listable.** Those two
chapters were not fussiness. They were buying this.

### What it is good for

**Finding the bad state fastest.** If something awful is reachable in four turns, searching
the fan finds it in four turns. A random run might take a million attempts, because the path
there may require an unusual-looking sequence of choices that no sensible player would make —
and "no sensible player would" is not a guarantee.

**Answering "can it?" within a horizon.** Can the player end with five potions and change? Can
the pool be drained? Fan out and look.

**Showing someone the shape.** The picture above says more about the model in five lines than
a table of numbers does.

### What it costs

The fan multiplies. Five choices per turn and ten turns is nine and a half million end states.
Add a second decision with five options and it is that number squared.

Two things help, and neither is a cure. Many paths **arrive at the same state** — buying two
then one lands exactly where buying one then two does — so you keep each state once rather than
once per path, which collapses a great deal. And a model with no decision at all has exactly
one successor per state, so it fans out not at all.

But the honest statement is that this works to a depth, the depth depends on your model, and a
deep enough question needs chapter 22 instead.

### The names people use

The set of states you can reach is the **reachable set**. Searching it to a fixed depth is
**bounded model checking**, a phrase you will meet in the wider literature. The states a model
can get to *and get back from* form the **configuration space**, and that is a more
interesting object than it sounds: it is where a system genuinely lives, as opposed to places
it passes through on the way to being stuck.

---

## 26. Questions mathematicians ask that Roxin doesn't

*rejected — the ideas, taught anyway*

The formalism Roxin is built on comes with a body of theory, and a good deal of that theory is
not implemented and not going to be. You should still know what it asks, both so the papers
make sense and so you understand what the shape of Roxin's state costs you.

There is one thread running through these, so watch for it.

### Where can we get to, as a function of where we are?

The theory has an object that maps each state to the whole set of states reachable from it —
every future, bundled up and attached to the present.

As a definition it is clean and it makes the theorems work. As something to compute it is
hopeless for any model with real branching, because it is the entire fan from chapter 25, to
infinite depth, from every starting point at once. Chapter 25's bounded version is the
practical shadow of it.

Nobody implements this, including the tools written by the people who defined it.

### How fast is the state changing, right now?

If states are points in space, you can ask which way the system is heading and how quickly —
the direction of travel at an instant.

To ask that, "a little bit later" has to mean something. The state must sit somewhere you can
move through smoothly, by arbitrarily small amounts.

Now look at your shop. The state is gold and potions, both whole numbers. What is "a little
bit more potions"? There is no such thing. You have three potions or four. **The question does
not fail to have an answer; it fails to have a meaning.**

### Do small differences stay small?

A question about whether a system is well behaved: if two runs start nearly in the same place,
do they stay near each other, or does the gap explode?

It is a genuinely useful thing to know, and it underpins the theorems that say a system has a
future at all.

Same obstacle. "Nearly the same place" requires a notion of how far apart two states are, and
for gold and potions there is no obvious one. Is `{gold: 100, potions: 0}` near
`{gold: 0, potions: 4}`? They are worth the same. They are nothing alike. Both answers are
defensible, which means the distance is a thing **you** would have to choose, and the theorems
that assumed one were not assuming yours.

### Can we steer it wherever we want?

Given where you are, can you reach any nearby state by choosing the right inputs? This is a
central question in control engineering and there are strong, genuinely useful results about
it.

Those results assume the state is a list of ordinary measurable quantities — position, speed,
temperature. Under that assumption you can ask about moving a little in any direction and the
mathematics has grip.

Roxin's whole premise is that a state is **whatever structure your problem needs** — counts,
flags, choices, eventually lists and graphs. The moment that is true, the assumption is gone.

### Can we tell what happened, from what we can see?

The last one, and it fails twice over — which makes it the most useful of the five.

Suppose you cannot see the whole state. You see a few numbers coming out of a real system and you
want to work backwards to what was going on inside. Can you? That question is **observability**, and
it is half of control engineering.

The classical answer is a construction that, again, assumes the state is a list of ordinary numbers
related linearly. Same obstacle as the last three.

But there is a second obstacle, particular to Roxin, and it is worth seeing because it is a choice
rather than a limitation. **Roxin assumes you can see everything.** A run records the whole state
every step. There is no notion of a system that shows you some numbers and hides the rest, because
there is no separate "what this system reports" distinct from "what this system is". So the question
*can I work out the hidden part?* has no hidden part to ask about.

That changes the moment you point a model at real data. Real measurements are partial, and then the
question comes straight back — not as "can I reconstruct the state", but as something sharper and
genuinely answerable:

> You have two consecutive observations. You know the rules. **Could more than one decision have
> produced that change?**

If exactly one could, you have learned what the decision was. If several could, then what you can
see does not determine what happened — and *that is observability*, arriving as a result you can
compute rather than a property you have to assume.

Roxin can answer that one exactly, by fanning out every legal decision from the first observation
and counting how many land on the second. It needs no output map and no linear algebra. It is the
same machinery as chapter 25, pointed at data instead of at curiosity. **[designed]**

### The thread

Four of the five want the same thing: **states as points in a space you can measure distance in and
move smoothly through.**

The fifth, observability, fails on that count too — but also because Roxin deliberately shows you
the whole state, which is why its useful version reappears as a question about data rather than a
question about hidden variables.

Grant the assumption, and a large, beautiful, decades-old body of theory applies. It is not junk. For a
pendulum or a heat exchanger it is exactly right, and if your problem is one of those, use the
tools built for it — they will tell you things Roxin cannot.

Refuse it — as Roxin does, on purpose, because gold and potions and infection counts and
proposals and queue lengths are not points in a smooth space — and the theory stops applying.
Not because it was wrong. Because its assumption was not met.

### What is lost, honestly

Real theorems, with real content, that Roxin cannot offer: that a solution exists and is
unique; that a controller can reach any nearby target; that a small error stays small; closed
expressions for where a system ends up after infinite time.

What is kept in exchange is the thing Act IV has been about. A state you can **write down
completely**, **compare exactly**, and **list the possibilities of** — which is what makes the
two checks of chapter 22 and the fan of chapter 25 possible at all, for states that are not
points in space and never could be.

That is the trade. It is not a small one in either direction, and you should know you made it.

---

---

# Act V — Doing it for real

You can now build a model, say what is legal, say what must be true, and know the difference
between running it and being sure of it. This act is about the rest: chance, order, what to do
when the thing you want to model is a crowd, how to start from nothing, and what Roxin cannot
do.

---

## 27. Chance

*designed — not yet built; function calls do not parse in the current build*

Not everything that arrives each turn was chosen. Demand fluctuates. Dice land. Weather
happens. A patient either does or does not transmit the infection.

### Where chance goes

In the same slot as a decision. The `input:` stage.

```
step Season:
    input:
        catch: float = quota
        weather: float = uniform(0.8, 1.2)
    mechanism:
        fish: s.fish * weather + growth * s.fish - u.catch
```

That is worth pausing on, because it looks like a shortcut and is not. A deliberate choice and
a random draw sit in exactly the same place in the structure, with the same kind of name, read
the same way by the mechanism.

Why they belong together: from the model's point of view they are the same thing. Both are
**something that arrived this turn which the state did not determine.** Whether it arrived
because a person decided or because the world rolled dice is a fact about the world, not about
the shape of the model. The formalism Roxin is built on makes exactly this identification, and
it is one of its better ideas.

### Chance may only happen in `input:`

Never in a mechanism. This is a hard rule and the reason matters.

A mechanism must be a **function of the state and the input**: the same state and the same
input always produce the same next state. If a mechanism could roll dice, that stops being
true — and the moment it stops being true, every single thing in Act IV collapses. The two
checks of chapter 22 ask "for any state and any legal input, does the claim survive?" If the
answer can vary between askings, the question is meaningless. Chapter 25's fan-out has no
defined branches. Chapter 24's counterexample is not reproducible.

Put the randomness in the input, where it is one of the things being quantified over, and all
of that machinery keeps working. Put it in the mechanism and none of it does.

### Reproducible randomness

Every run gets a **seed** — a number that determines the entire sequence of draws. If you do
not supply one, Roxin picks one and **writes it down in the results.**

```
run Fishery from Sea { fish: 200.0 } for 20 steps
    seed 42
    runs 1000
```

`seed 42` pins it. `runs 1000` does the whole thing a thousand times with different draws,
which is how you answer the "how often" questions from chapter 21.

### Poke it: is pinned randomness really random?

No, and that is the point.

What you want from a model is not unpredictability. It is a sequence of draws that has no
pattern you accidentally relied on, and that you can **get back**. A run you cannot reproduce
is a run you cannot debug, cannot show anybody, and cannot put in a paper.

The failure this prevents is specific and miserable: something goes wrong once, in a run of a
thousand, and you never see it again. With recorded seeds, "run 617 did something strange"
is a thing you can return to for as long as the file exists.

### Why not the other way?

Most tools just ask the operating system for random numbers. It is less work and it is
genuinely random, if that is what you want.

What it costs is everything in the paragraph above. It also costs you the ability to compare:
if two runs differ, you cannot tell whether that is because you changed the model or because
the dice fell differently.

Roxin's draws also use **pinned algorithms** — the exact generator and the exact method for
turning its output into numbers are part of the language definition, not whatever the machine
happened to provide. So the same model and the same seed give the same answer on your laptop
and on somebody else's server. That is a stronger promise than most tools make, and it only
works if nobody is allowed to be clever about it.

---

## 28. More than one rule, in order

*runs today*

Chapter 10 introduced `>>` to make a point about simultaneity. Here it is for its own sake.

### When order is real

Sometimes a turn genuinely has stages, and the second one must see the first one's result.

**Settle, then charge.** Trades clear, and the fee is a percentage of what actually cleared.
You cannot compute the fee until settlement is done.

**Act, then check.** Damage is applied, and then anyone on zero health is removed. The removal
must see the post-damage health.

**Take turns.** Player one acts, then player two acts knowing what player one did.

```
system Market: Book = Settle >> Charge >> Accrue
```

One **tick** is one pass through all three. Each rule is still internally simultaneous, as
chapter 10 insisted; the chain is what puts one frozen picture after another.

### What gets recorded

The end of the tick. The states in between are computed and dropped.

That is usually what you want — a results table with three rows per turn, two of them
half-finished, is noise. When you do want them, you can ask.

### Poke it: when do you *not* need this?

More often than you would guess, and it is worth saying clearly because the instinct from
other tools is to reach for chaining immediately.

Chapter 6 established that a mechanism must set **every** field. So one rule can already write
the whole state at once. If three things change and none of them depends on another having
changed first, that is **one rule with three lines**, not three rules.

```
    mechanism:
        gold:       s.gold - price * u.buy
        potions:    s.potions + u.buy
        reputation: s.reputation + u.buy
```

One tick, three changes, no chaining. Reach for `>>` when you find yourself wanting a rule to
see what another rule just did — which is the exact situation chapter 10 told you to recognise.

### Break it: use `>>` for things that are not ordered

Split those three lines into three rules chained together. It will produce the same answer,
because nothing in any of them depends on the others.

What you have lost: the reader now has to check all three rules to confirm that the order does
not matter, because the chain *claims* it does. You have written down a dependency that is not
there, and every future reader has to disprove it.

**Order is a claim.** Do not make it unless you mean it.

---

## 29. Agents, populations, and networks

*open — not built; the two shapes of multiplicity are an open design question*

Everything so far has modelled one of something. One player, one sea, one account. Most
interesting systems are made of **many**, and they interact.

A market with a thousand traders. A town with ten thousand people and an infection. A network
where each node forwards what it receives. A game with four players taking turns.

### What this is called

**Agent-based modelling.** Many similar things, each with its own state, each following rules,
each affecting the others. It is one of the most popular ways to model anything social, and
with good reason: it matches how you already think about crowds.

### Roxin cannot do it yet

Plainly: there are no lists. A state field holds one number, not a thousand. Writing a
thousand traders is not awkward in Roxin today, it is impossible.

That is a genuine, large gap, and the field guides that mention agent models are describing
things that do not work yet.

### The two shapes it could take

Worth understanding, because it is being decided and the choice has consequences you now have
the background to see.

**Multiplicity from data.** One field holding a list of a thousand traders. Ergonomic — add a
trader by making the list longer, and the rules are written once for "each trader."

**Multiplicity from structure.** A thousand declared copies, or a model parameterised by a
count known before anything runs. Clunkier to write, and the number is fixed when you start.

The reason this is not obvious: Act IV. A list of unknown length means every claim becomes a
claim about **all possible lengths at once**, which is a real step up in difficulty — a
different kind of question, not a bigger one. Structural copies stay finite and known, so the
two checks of chapter 22 see something they can handle.

Ergonomics pulls one way; everything in Act IV pulls the other.

### What to do in the meantime, and a lesson worth having anyway

Here is the thing nobody tells beginners, and it is true regardless of which tool you use:

> **You usually do not need agents.**

You want to model an epidemic in a town of ten thousand. The instinct is ten thousand people,
each infected or not, each meeting others.

You do not need them. Track **how many** are in each category:

```
state Town { susceptible: float, infected: float, recovered: float }
```

Three numbers. Not thirty thousand. And the model tells you nearly everything the agent version
would — how fast it spreads, whether it dies out, what a measure would change — because for
most questions the individuals were never the point. The *counts* were.

This is called a compartment model, and it is what epidemiologists actually use.

When do you genuinely need individuals? When they **differ in ways that matter**, and the
differences drive the outcome. A network where who-knows-whom determines spread. A market where
one large trader moves the price. A game where players have distinct positions. Then the crowd
is not summarisable and you need the real thing.

Learning to tell those apart will save you more time than any feature in this book.

### For a handful of things

If you need four players rather than ten thousand, you can write them out:

```
state Table { p1_gold: int, p2_gold: int, p3_gold: int, p4_gold: int }
```

It is ugly, it does not scale, and adding a fifth player means editing every rule. It also
works today, and for a four-player game it is a perfectly reasonable thing to do while the
language grows.

---

## 30. Time that flows instead of ticking

*rejected — continuous time is out of scope; this is what it means and what you lose*

Everything in Roxin happens in steps. Turn one, turn two. A fish population jumps from 200 to
195 with nothing in between.

The world is not obviously like that, and this chapter is about the difference.

### Two ways to say how something changes

**How much per step.** "The population grows by a tenth each season." That is what Roxin says,
and it is a complete instruction: take the number, work out the next one.

**How fast, right now.** "The population is growing at a rate proportional to its size." That
is not an instruction. It describes the *tendency* at an instant, and the actual trajectory has
to be worked out from it.

The second is a **differential equation**, and it is how most of physics, chemistry,
pharmacology and ecology is written down. If you take a model from a paper in any of those
fields, this is probably what you will find.

### What you do about it

You turn the rate into a step, by pretending the rate holds steady for a short interval:

> new value = old value + (rate at the old value) × (length of the interval)

That is it. Every fish, thermostat and infection model in this book is already doing it.

```
    fish: s.fish + growth * s.fish - u.catch
```

`growth * s.fish` is the rate. The interval is one season, so multiplying by one is invisible.
If you wanted months, you would use a twelfth of the growth and run twelve times as many steps.

### Poke it: how wrong is that?

Somewhat, always, and the amount depends on the step size.

The rate does not really hold steady across the whole interval — the population is growing the
entire time, so the rate is growing too, and treating it as fixed undershoots. Smaller steps
mean less error and more steps to run.

**The practical test: halve the step size and run it again.** If the answer barely moves, your
steps were small enough. If it moves a lot, they were not, and you should keep halving until it
settles. This is worth doing once on any model you care about, and almost nobody does it.

### Why Roxin does not do continuous time

Two reasons, and the second is the real one.

The formalism it is built on defines its central object as a **discrete transition** — a rule
from one state to the next. Continuous time is not a limitation of the implementation; it is
outside what the thing is.

And more importantly: continuous time has no "next state." Act IV's two checks ask *"if the
claim holds here, does it hold in the state after?"* With flowing time there is no state after
— there is a state at every instant between here and there, infinitely many of them. The
question has to be asked completely differently, and all the machinery that answers it is
different machinery.

### What is lost

Real things, and you should know them.

**Exact solutions.** Some equations can be solved outright, giving a formula for the value at
any time, including infinity. Stepping only ever gives you a table.

**Stiff systems.** When one part of a model moves vastly faster than another, fixed steps are
either hopelessly slow or badly wrong. Proper solvers handle this; stepping does not.

**Adaptive steps.** Good solvers take small steps where things are changing fast and large ones
where they are not. Roxin takes the step you specified, always.

If those matter to your problem, use a differential equation solver. It is the right tool and
there are excellent ones. Many people use both: a solver for the continuous physics, and
something like Roxin for the decisions and rules wrapped around it.

---

## 31. Starting from a blank page

*runs today*

Every chapter so far has handed you a model. This one is about the hardest part, which nobody
writes about: you have something you care about, and an empty file.

### Five questions, in order

**One. What would someone need to be handed to carry on from where you left off?**
That is your `state`. Not what is interesting — what is *needed*.

**Two. What happens each turn?**
That is your `step`, and its `mechanism`.

**Three. Who chooses, and from what?**
That is your `input`. If nothing chooses, skip it.

**Four. What is forbidden?**
That is `allowed`.

**Five. What should never happen, no matter what?**
Those are your invariants.

You will not get them right first time. That is fine. The order matters more than the accuracy,
because each answer constrains the next.

### Let us do one

A fishery. Boats catch fish; fish breed; catch too many and the fishery collapses. You want to
know what a sustainable quota is.

**What would someone need to carry on?** How many fish are in the sea. That is genuinely all —
hand someone that number and they can continue.

Tempting additions, and why they are not state: *how many boats* — only if it changes;
otherwise it is a setting. *Total caught so far* — you could add it up; chapter 7 says do not
store it. *Last year's population* — only if a rule needs it, and none does yet.

```
state Sea { fish: float }
```

One field. Resist making it three.

**What happens each season?** Fish breed in proportion to how many there are, and boats take
their catch.

```
    mechanism:
        fish: s.fish + growth * s.fish - u.catch
```

**Who chooses?** The fleet chooses how much to catch. For now, the quota.

```
    input:
        catch: float = quota
```

**What is forbidden?** You cannot catch fish that are not there. *(Written here for
completeness; `allowed` does not run yet.)*

```
    allowed: catch in 0..s.fish
```

**What should never happen?** The sea never holds a negative number of fish.

```
invariant Stocked(s: Sea) = s.fish >= 0
```

### The whole thing

```
param growth: float = 0.1
param quota: float = 25

state Sea { fish: float }

step Season:
    input:
        catch: float = quota
    mechanism:
        fish: s.fish + growth * s.fish - u.catch

system Fishery: Sea = Season

run Fishery from Sea { fish: 200.0 } for 12 steps
```

```
    0  { fish: 200 }
    1  { fish: 195 }
    2  { fish: 189.5 }
    3  { fish: 183.45 }
    4  { fish: 176.795 }
    5  { fish: 169.47449999999998 }
    6  { fish: 161.42194999999998 }
    7  { fish: 152.56414499999997 }
    8  { fish: 142.82055949999997 }
    9  { fish: 132.10261544999997 }
   10  { fish: 120.31287699499998 }
   11  { fish: 107.34416469449997 }
   12  { fish: 93.07858116394996 }
```

A quota of 25 against a population of 200 growing at ten per cent. It declines — slowly at
first, then faster, because a smaller population breeds less and the quota does not care.

*(The ragged `169.47449999999998` is chapter 11 arriving on schedule. This is a float model, so
that is what floats do. If this were money you would have used `decimal`.)*

### Now ask the question you came for

You wanted a sustainable quota. You have one line for that.

```
run Fishery from Sea { fish: 200.0 } for 8 steps
    sweep quota in [15, 20, 25]
```

```
job 0 quota = 15
    0  { fish: 200 }
    1  { fish: 205 }
    ...
    8  { fish: 257.1794405 }
job 1 quota = 20
    0  { fish: 200 }
    1  { fish: 200 }
    ...
    8  { fish: 200 }
job 2 quota = 25
    0  { fish: 200 }
    1  { fish: 195 }
    ...
    8  { fish: 142.82055949999997 }
```

Fifteen grows. Twenty-five collapses. **Twenty holds exactly flat, forever.**

That is a knife-edge, and it fell out of a ten-line model and one line of question. A tenth of
two hundred is twenty; take exactly what grows and the population never moves. One fish more
and it unwinds, slowly at first, which is precisely why real fisheries collapse while everyone
involved is being careful.

Notice what you were not told. Nobody said "look for the break-even point." You asked three
questions at once and the structure appeared.

### Then grow it

The model above is wrong in interesting ways, and each fix is one of the five questions asked
again.

Fish cannot grow forever — the sea has a capacity. *That is a change to the mechanism.* The
fleet should catch less when stocks are low. *That is a change to the input, and the mechanism
does not move.* Weather varies. *That is chapter 27.* Several fleets with separate quotas.
*That is chapter 29, and you will have to wait.*

**Start with one field and one rule.** Run it. Be surprised. Add the next thing. A model that
runs and is too simple is worth more than a model that is complete and does not run, because
the simple one tells you something today.

### When you have got the state wrong

Three symptoms, each pointing back at a chapter.

**A rule needs something that is not there.** You are writing a mechanism and you want a number
that does not exist. Chapter 3 — either write it down, make it a setting, or decide it is out
of scope.

**You are storing something you could work out.** Chapter 7 — take it out.

**You need to know what happened before.** Chapter 7 again — add the specific memory you need,
as a field, and discover how much you actually needed.

---

## 32. Signs your model is wrong

A checklist. Each item is something earlier in this book, condensed into the shape it takes
when you meet it in your own file at eleven at night.

### A decision is hiding in a mechanism

```
    gold: s.gold - price * (s.gold // price)
```

Something in there is a *choice*, and it has no name. You cannot swap it, cannot point at it,
and chapter 25 cannot fan it out. **Give it a name and move it to `input:`.** (Chapter 14.)

### You are storing something you could work out

A field that is always equal to some combination of other fields. Two sources of truth, and one
of them will eventually be wrong — silently, with the model still running. **Compute it instead.**
(Chapter 7.)

### An expression appears twice

```
    gold:    s.gold - price * u.buy
    potions: s.potions + (s.gold - price * u.buy) / price
```

You wrote it twice because you wanted the second line to see the first. Either it is one moment
and the shared quantity needs a name, or it is two moments and you want two rules. **Do not
leave it duplicated** — the two copies will drift. (Chapter 10.)

### Money in a `float`

If a field is currency and its type is `float`, you have a bug that has not surfaced yet.
(Chapter 11.)

### A bound you invented to satisfy the tool

```
    allowed: buy in 0..999999
```

Nothing in your system says 999999. You wrote it because something demanded a number. That line
is not something you believe, and the next person to read it will copy the habit rather than
the reasoning. **Either find the real bound or question why you need one here.** (Chapters 16
and 17.)

### An invariant that survives only because nothing has tried

If your claim is true but you could not say *why* in one sentence, chapter 23 is waiting for you.
The sentence you cannot produce is usually the conservation law that makes the claim inductive —
and it is usually the most interesting thing about your model.

### The model only works from one starting point

Change the starting state to something else sensible and watch it do something absurd. If it
does, the model has your particular scenario baked into it somewhere. A model is a machine, not
an answer. (Chapter 4.)

### Modelling the answer instead of the system

The subtlest one, and the hardest to see in your own work.

You want to know whether the fishery collapses, so you add a field called `collapsed`, and a
rule that sets it when stocks fall below some number you picked.

You have not modelled collapse. You have **written down your belief about when collapse
happens** and arranged for the model to agree with you. It will now confirm that belief under
every parameter you try, because you installed it.

The model should contain **fish and boats and breeding**. Whether that collapses is something
you find out by looking — chapter 31's sweep, where the break-even fell out of three lines that
knew nothing about break-even points.

> If a result you were hoping to find is written into the model, you will find it.

### A rule you cannot explain

The last one, and it covers the rest. Point at any line and ask what it says about the world.
If the honest answer is "it has to be there" or "it was in the example I copied," that line is
a defect — not in you, in the model or the language. (This one is a rule the language itself is
held to.)

---

## 33. Why there's only one way to say things

You may have noticed that Roxin rarely offers a choice. There is one way to write a rule, one
way to say what is legal, one way to state a claim. No synonyms, no shorthand that means the
same as the long form, no alternative spellings.

That is deliberate, and the argument against it is good enough to state properly.

### The case for many ways

General-purpose languages celebrate having several ways to do a thing, and they are right to.

A general-purpose language serves every domain at once and cannot know yours. It cannot know
whether you are writing a device driver or a spreadsheet, whether your reader is an expert or a
beginner, whether this code will be read a thousand times or deleted tomorrow. Faced with that,
offering a range and trusting the author is the correct design. Expressiveness is a feature
when you cannot know the use.

### Why that reverses here

Roxin knows exactly what you are doing. You are describing a thing that changes over time, and
you may want to prove something about it.

Under that constraint, most of the ways to write something become **wrong** — not in bad taste,
wrong, because they break something you wanted. Putting the decision in the mechanism works
until you want to swap it. Writing legality with `or` works until you want to list the
possibilities. History-passing works until you want the two checks of chapter 22. Each of those
is a perfectly reasonable thing to offer, and each one quietly removes a capability that the
whole rest of the design was built to provide.

So the question is not "should authors have freedom?" It is: **if nine ways out of ten break
something, is offering all ten a kindness?**

It is not, and the reason is that the costs do not announce themselves. Nobody discovers at the
moment of writing that they have given up provability. They discover it two months later, when
they ask a question and are told it cannot be answered, and they have no idea which of three
hundred lines is responsible.

### No domain aliases, either

A game designer would rather write `turn` than `step`. An epidemiologist would rather write
`compartment`. A trader would rather write `tick`. Roxin offers none of them, and will not.

The reason is not tidiness. It is that **a model written by the game designer should be
readable by the epidemiologist**, with no translation. One vocabulary means the five field
guides are five sets of *examples* rather than five dialects — you can read a model from a
field you know nothing about and understand its structure immediately, even if you do not
understand what it is modelling.

That is a real asset and it is lost the moment there are two words for a step.

### What it costs

Your field's word is better than Roxin's word, for you. It carries meaning you already have,
and you are being asked to give it up and learn a less evocative one.

The guides soften it — each introduces a concept through the familiar word once and then
switches for good — but the cost is real and it is paid by everybody except whichever community
happened to supply the vocabulary.

### This is not minimalism

Worth being clear, because "one way to do it" can be a slogan for leaving things out.

Roxin is not trying to be small. It will grow lists, choices, proofs, and more. The rule is not
*few features*; it is **one spelling per idea**. A new capability is welcome. A second way to
say something that can already be said is not.

---

## 34. Honest limits

What Roxin cannot do. No softening, because finding these out by surprise is worse than reading
them here.

### Not built yet

**Lists, collections, agents, networks.** One of something, not many. Chapter 29.

**Proof.** The two checks of chapter 22 are designed and not implemented. Invariants are
checked while running, which catches real bugs, but nothing is proved yet.

**The explorer.** Chapter 25's fan-out is designed, not built.

**`allowed`.** Designed, not built.

**Exact decimals.** Designed, not built — so today there is no correct type for money.

**Randomness.** Designed, not built.

**Conditionals.** `if / then / else` is designed, not built, which is why several examples in
this book are marked.

That is a lot, and it is the honest state of a young language. What runs today is roughly Acts
II and III minus `allowed`: state, rules, settings, sweeps, the decision-consequence split, and
arithmetic that refuses to guess.

### Not going to be built

**Continuous time.** Chapter 30.

**Calling out to other languages mid-model.** Chapter 7 — it would cost everything in Act IV.

**History handed to every rule.** Chapter 7 — memory goes in the state, deliberately.

**The smooth-space mathematics.** Chapter 26.

**Several policies merged into one input.** Chapter 15.

### Hard even once built

**Curvy arithmetic and floats** put proofs out of reach or into approximation. Chapter 24.

**Questions about forever** — "does it eventually settle?" — need a different argument than
safety does.

**The fan-out explodes.** Chapter 25 works to a depth, and the depth is smaller than you want.

### When to use something else

A short list, offered sincerely.

**A spreadsheet**, for anything small enough to see at once. Chapter 5 meant it.

**A differential equation solver**, if your model came out of physics or pharmacology and is
written as rates. Chapter 30.

**An agent-based framework**, if you genuinely need thousands of distinct individuals and the
differences between them drive the result. Chapter 29 — and check first whether you can count
instead.

**A general-purpose language**, for anything that is not modelling. Roxin does not read files,
serve requests, draw charts, or talk to anything. It describes systems and answers questions
about them.

---

## 35. Talking to other people

You have the ideas. This is the vocabulary, so that you can read a paper, follow a seminar, or
argue with somebody who learned this elsewhere.

| What it is | Roxin's word | What you will hear elsewhere |
|---|---|---|
| The things that change | `state` | state, state vector, system state; *space* and *point* in older cadCAD |
| One particular set of values | a state | a point, a configuration, a state |
| A number fixed for the whole run | `param` | parameter, constant, setting, hyperparameter |
| A rule for what happens next | `step` / `mechanism` | state update function, transition function, dynamics, mechanism; *f* or *h* in papers |
| What someone chose this turn | `input` | action, control, control input, decision, policy output, behaviour |
| The thing that produces the choice | input map | policy, decision rule, strategy, controller, agent behaviour |
| What choices are legal here | `allowed` | admissible set, feasible set, constraint set, action space |
| Something that must always be true | `invariant` | invariant, safety property, constraint, conservation law |
| A value computed from state | `derived` *(name not final)* | observable, output, metric, KPI, measurement |
| One pass through all the rules | a tick | time step, period, round, epoch, iteration |
| The whole sequence of states | a trajectory | trajectory, path, run, history, time series |
| Everywhere you could get to | reachable set | reachable set, reachability, attainable set |
| Trying many random runs | `runs N` | Monte Carlo, simulation, sampling |
| Checking every possibility to a depth | the explorer | bounded model checking, exhaustive search, state-space search |
| The two checks of chapter 22 | proving an invariant | inductive invariant, induction, formal verification, safety proof |
| Adding the reason a claim is true | strengthening | strengthening the invariant, finding an inductive invariant |

### Three warnings

**"Metric" is a trap.** In this field it usually means a way of measuring the **distance
between two states** — a precise mathematical object. When somebody says a model has good
metrics they mean something else entirely. Ask.

**"Policy" means both things.** In cadCAD a policy is a function producing an input. In
reinforcement learning it is a rule mapping states to actions, which is the same idea. In
governance it means a rule people vote on. Usually clear from context, occasionally not.

**"Model" means three things.** The file you wrote. The thing in the world you are describing.
And, in statistics, a fitted equation. People switch between them mid-sentence without noticing.

### The underlying idea has a name

What you have learned is a **generalized dynamical system**. A state space, a rule for moving
through it, and a set of legal inputs at each point.

If you meet that phrase, or the papers behind it, you will find definitions written in symbols
rather than words. They are saying what chapters 3, 14 and 16 said. The state space is your
`state`. The state update map is your `mechanism`. The admissible input map is your `allowed`.
You already know what they mean; you just have not seen the handwriting.

---

## 36. Where to go next

### Pick the guide for your field

Each one covers the same ground as this book, in a quarter of the length, using models that
somebody in that line of work would recognise immediately. Same concepts, same order, same
words — different examples.

| | For |
|---|---|
| [Game design and development](guides/games.md) | economy designers, gameplay programmers |
| [Quant finance and tokenomics](guides/tokenomics-quant.md) | quants, protocol and mechanism designers |
| [Life sciences](guides/life-sciences.md) | epidemiologists, pharmacologists, ecologists |
| [Control and engineering](guides/control-engineering.md) | control, mechanical and electrical engineers |
| [Collective decision-making and commons](guides/governance-commons.md) | councils, co-ops, commons, committees, DAOs |

Read one. Then read a second one from a field you know nothing about — it will take twenty
minutes and it is the fastest way to see that the structure is the same underneath, which is
the point this book opened with.

### Then build something small and wrong

Not a model of the thing you actually care about. Something you understand completely and that
does not matter: a savings jar, a card game you know, the queue at your coffee shop.

Get it running. Be surprised by it. Break an invariant on purpose and watch what happens. Put a
decision in the mechanism and then pull it out again, and feel the difference in your own file
rather than in somebody else's example.

The five questions in chapter 31 are the whole method, and they only become yours by being used
on something you chose.

### If you want to know why

The documentation beside this one answers different questions. `docs/core-semantics.md` says
precisely what everything means. `docs/registry.md` records every decision in the language,
with the alternatives that were considered and why they lost — including every open question
this book marked as undecided.

They are not written for beginners. After this book you are not one.

### A word for you

Somebody who can do this — take a system they care about, find its state, separate the
decisions from the consequences, write down what is legal and what must never happen, and then
ask a machine whether they are right — is a **roxstar**.

That is the whole job. You now know how.
