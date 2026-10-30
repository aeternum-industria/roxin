#!/usr/bin/env python3
"""Independent Python reference for the Roxin fixtures.

Every model here is written from its mathematical definition, NOT from the Roxin source, so it
checks the interpreter instead of echoing it.

    python3 tools/reference.py check .          # compare reference vs .expected (today/ files)
    python3 tools/reference.py write-pending .  # (re)write pending/*.expected from the reference

Output format matches the interpreter's `print_job`: a header `job N  name = value, ...`, one line
per row, and an optional `stopped at step N: ...` line. Floats print like Rust's `{}`: shortest
round-trip digits, never an exponent, and no trailing `.0`.
"""
import math, os, re, sys
from decimal import Decimal

# ---------------------------------------------------------------- formatting
def rust_f64(x):
    if x != x: return "NaN"
    if x in (math.inf, -math.inf): return "inf" if x > 0 else "-inf"
    s = format(Decimal(repr(x)), "f")
    if "." in s:
        s = s.rstrip("0").rstrip(".")
    return "0" if s in ("-0", "") else s if s != "-0" else "0"

def fmt(v):
    if isinstance(v, bool): return "true" if v else "false"
    if isinstance(v, int): return str(v)
    return rust_f64(v)

def row_text(row):
    return "{ " + ", ".join(f"{k}: {fmt(v)}" for k, v in row.items()) + " }"

class Job:
    def __init__(self, index, overrides, rows, stop=None):
        self.index, self.overrides, self.rows, self.stop = index, overrides, rows, stop

def render(jobs):
    out = []
    for j in jobs:
        out.append(f"job {j.index}  " + ", ".join(f"{k} = {fmt(v)}" for k, v in j.overrides))
        for i, r in enumerate(j.rows):
            out.append(f"  {i:>3}  {row_text(r)}")
        if j.stop:
            out.append(f"  stopped at step {j.stop[0]}: {j.stop[1]}")
    return "\n".join(out) + "\n"

class Stop(Exception):
    pass

def simulate(init, step, n, until=None, invariant=None, **params):
    """Rows 0..n; step(s, k, **params) -> new row dict. Mirrors core-semantics section 6."""
    rows, s = [dict(init)], dict(init)
    if invariant and not invariant(s, **params):
        return rows, (0, "invariant violated")
    for k in range(1, n + 1):
        if until and until(s):
            break
        try:
            s = step(s, k, **params)
        except Stop as e:
            return rows, (k, str(e))
        rows.append(dict(s))
        if invariant and not invariant(s, **params):
            return rows, (k, "invariant violated")
    return rows, None

def sweep(index_start, grid, run):
    jobs = []
    for i, ov in enumerate(grid):
        rows, stop = run(**dict(ov))
        jobs.append(Job(index_start + i, ov, rows, stop))
    return jobs

def one(run, **kw):
    rows, stop = run(**kw)
    return [Job(0, [], rows, stop)]

I128 = 2**127 - 1
def fin(x):
    if x != x or x in (math.inf, -math.inf): raise Stop("arithmetic error: the result is not a finite number")
    return x
def chk(x):
    if abs(x) > I128: raise Stop("arithmetic error: integer overflow")
    return x

# ------------------------------------------------------------ today's models
def doubling():  return one(lambda: simulate({"x": 1}, lambda s, k: {"x": s["x"] * 2}, 100))
def fibonacci(): return one(lambda: simulate({"a": 0, "b": 1}, lambda s, k: {"a": s["b"], "b": s["a"] + s["b"]}, 100))
def factorial(): return one(lambda: simulate({"n": 0, "f": 1}, lambda s, k: {"n": s["n"] + 1, "f": s["f"] * (s["n"] + 1)}, 30))
def factorial_overflow():
    return one(lambda: simulate({"n": 0, "f": 1}, lambda s, k: {"n": s["n"] + 1, "f": chk(s["f"] * (s["n"] + 1))}, 40))
def bitcoin_halving():
    def step(s, k):
        subsidy = 5_000_000_000 // 2 ** s["era"]
        return {"era": s["era"] + 1, "supply": s["supply"] + subsidy * 210000}
    return one(lambda: simulate({"era": 0, "supply": 0}, step, 40))
def compound_interest():
    return sweep(0, [[("rate", r)] for r in (0.01, 0.05, 0.1)],
        lambda rate: simulate({"balance": 1000}, lambda s, k: {"balance": s["balance"] * (1 + rate)}, 10))
def logistic_map():
    return sweep(0, [[("r", r)] for r in (2.5, 3.2, 3.9)],
        lambda r: simulate({"x": 0.2}, lambda s, k: {"x": r * s["x"] * (1 - s["x"])}, 40))
def radioactive_decay():
    return sweep(0, [[("decay", d)] for d in (0.05, 0.1)],
        lambda decay: simulate({"atoms": 1000}, lambda s, k: {"atoms": s["atoms"] * (1 - decay)}, 20))
def newton_cooling():
    return one(lambda: simulate({"temp": 90}, lambda s, k: {"temp": s["temp"] + 0.1 * (20 - s["temp"])}, 30))
def water_tank():
    return one(lambda: simulate({"level": 0}, lambda s, k: {"level": s["level"] + 2 - 0.1 * s["level"]}, 100))
def sir():
    def run(beta):
        def step(s, k):
            inf, rec = beta * s["s"] * s["i"], 0.1 * s["i"]
            return {"s": s["s"] - inf, "i": s["i"] + inf - rec, "r": s["r"] + rec}
        return simulate({"s": 0.999, "i": 0.001, "r": 0}, step, 300)
    return sweep(0, [[("beta", b)] for b in (0.1, 0.2, 0.4)], run)
def oral_dose():
    def run(ka):
        def step(s, k):
            ab, cl = ka * s["gut"], 0.1 * s["plasma"]
            return {"gut": s["gut"] - ab, "plasma": s["plasma"] + ab - cl, "eliminated": s["eliminated"] + cl}
        return simulate({"gut": 100, "plasma": 0, "eliminated": 0}, step, 40)
    return sweep(0, [[("ka", v)] for v in (0.1, 0.3, 0.6)], run)
def michaelis_menten():
    def run(km):
        def step(s, k):
            rate = 1 * s["substrate"] / (km + s["substrate"])
            return {"substrate": s["substrate"] - rate * 0.1, "product": s["product"] + rate * 0.1}
        return simulate({"substrate": 10, "product": 0}, step, 100)
    return sweep(0, [[("km", v)] for v in (0.5, 2, 8)], run)
def thermostat():
    def run(k):
        return simulate({"temp": 10}, lambda s, i: {"temp": s["temp"] + k * (20 - s["temp"])}, 8)
    return sweep(0, [[("k", v)] for v in (0.25, 0.5, 1.5)], run)
def predator_prey():
    def step(s, k):
        kills = 0.002 * s["prey"] * s["pred"]
        return {"prey": s["prey"] + 0.1 * s["prey"] - kills, "pred": s["pred"] + 0.5 * kills - 0.1 * s["pred"]}
    return one(lambda: simulate({"prey": 40, "pred": 9}, step, 2))
def pid():
    def run(kp):
        def step(s, k):
            err = 10 - s["pos"]
            force = kp * err + 0.05 * s["integral"] + 0.1 * (err - s["prev_err"])
            return {"pos": s["pos"] + force, "integral": s["integral"] + err, "prev_err": err}
        return simulate({"pos": 0, "integral": 0, "prev_err": 0}, step, 60)
    return sweep(0, [[("kp", v)] for v in (0.2, 0.5, 1.2)], run)
def unstable_pendulum():
    def step(s, k):
        return {"theta": fin(s["theta"] + 1 * s["omega"]), "omega": fin(s["omega"] + 1 * 1_000_000 * s["theta"])}
    return one(lambda: simulate({"theta": 0.01, "omega": 0}, step, 400))
def float_overflow():
    return one(lambda: simulate({"x": 1}, lambda s, k: {"x": fin(s["x"] * 1e150)}, 12))
def float_div_zero():
    def step(s, k):
        if s["x"] == 0: raise Stop("division by zero")
        return {"x": s["x"] - 1}
    return one(lambda: simulate({"x": 2}, step, 5))
def precedence():
    return one(lambda: ([{k: 0 for k in "abcdefghi"},
                         {"a": 14, "b": 512, "c": -4, "d": 3, "e": 3, "f": -4, "g": 2, "h": 20, "i": 7}], None))
def syntax_torture():
    def run(small):
        return simulate({"n": 1_000_000, "x": 0}, lambda s, k: {"n": s["n"] + 1, "x": s["x"] + small * 1e3}, 3)
    return sweep(0, [[("small", v)] for v in (2.5e-3, 5e-3)], run)

# -------------------------------------------------------- pending models
def marbles():
    def step(s, k):
        d = (s["box_b"] > s["box_a"]) - (s["box_b"] < s["box_a"])
        a1 = d if k % 2 == 0 else 0
        a2 = d if k % 3 == 0 else 0
        return {"box_a": s["box_a"] + a1 + a2, "box_b": s["box_b"] - a1 - a2}
    return one(lambda: simulate({"box_a": 10, "box_b": 0}, step, 10))
def collatz():
    return one(lambda: simulate({"n": 27}, lambda s, k: {"n": s["n"] // 2 if s["n"] % 2 == 0 else 3 * s["n"] + 1},
                                200, until=lambda s: s["n"] == 1))
def repeated_dosing():
    return one(lambda: simulate({"conc": 0}, lambda s, k: {"conc": s["conc"] * (1 - 0.1) + (100 if k % 6 == 1 else 0)}, 120))
def bang_bang():
    def step(s, k):
        on = True if s["temp"] < 18 else False if s["temp"] > 22 else s["on"]
        return {"temp": s["temp"] + (2 if s["on"] else 0.0) - 0.1 * (s["temp"] - 5), "on": on}
    # the Roxin mechanism lists `on` first, then `temp`; the state's field order is (temp, on)
    return one(lambda: simulate({"temp": 15, "on": False}, step, 100))
def cooldowns():
    return one(lambda: simulate({"fireballs": 0, "heals": 0},
        lambda s, k: {"fireballs": s["fireballs"] + (1 if k % 4 == 0 else 0), "heals": s["heals"] + (1 if k % 3 == 0 else 0)}, 24))
def idle_clicker():
    def step(s, k):
        cost = 10 + 5 * s["clickers"]
        buy = 1 if s["coins"] >= cost else 0
        return {"coins": s["coins"] + 1 + s["clickers"] - buy * cost, "clickers": s["clickers"] + buy}
    return one(lambda: simulate({"coins": 0, "clickers": 0}, step, 60))
def amm(fee, trade=10.0, n=50):
    def step(s, k):
        a = trade
        if not (0 <= a <= s["x"]): raise Stop("admissibility")
        return {"x": s["x"] + a, "y": s["x"] * s["y"] / (s["x"] + a * (1 - fee))}
    return simulate({"x": 1000, "y": 1000}, step, n, invariant=lambda s: s["x"] * s["y"] >= 1_000_000)
def amm_constant_product():
    return sweep(0, [[("fee", f)] for f in (0.003, 0.01)], lambda fee: amm(fee))
def amm_fee_zero():
    return one(lambda: amm(0.0, trade=7.0))
def token_vesting():
    T, M = 1_000_003, 12
    def step(s, k):
        u = T * k // M - T * (k - 1) // M
        return {"locked": s["locked"] - u, "released": s["released"] + u}
    return one(lambda: simulate({"locked": T, "released": 0}, step, 12, invariant=lambda s: s["locked"] + s["released"] == T))
def sir_invariant():
    def run(beta):
        def step(s, k):
            inf, rec = beta * s["s"] * s["i"], 0.1 * s["i"]
            return {"s": s["s"] - inf, "i": s["i"] + inf - rec, "r": s["r"] + rec}
        return simulate({"s": 0.999, "i": 0.001, "r": 0}, step, 300,
                        invariant=lambda s: abs(s["s"] + s["i"] + s["r"] - 1) <= 1e-9)
    return sweep(0, [[("beta", b)] for b in (0.1, 0.2, 0.4)], run)
def combat_allowed():
    def step(s, k):
        if not (0 <= 3 <= s["mana"]): raise Stop("admissibility violation: `spell` = 3 is not in 0..2, state { mana: %d, enemy_hp: %d }" % (s["mana"], s["enemy_hp"]))
        return {"mana": s["mana"] - 3 + 1, "enemy_hp": s["enemy_hp"] - 6}
    return one(lambda: simulate({"mana": 10, "enemy_hp": 100}, step, 10))
def bonding_curve():
    def step(s, k):
        return {"supply": s["supply"] + 10.0, "reserve": s["reserve"] + 10.0 * (1 + 0.01 * (s["supply"] + 10.0 / 2))}
    return one(lambda: simulate({"supply": 0, "reserve": 0}, step, 50,
        invariant=lambda s: abs(s["reserve"] - (1 * s["supply"] + 0.01 * s["supply"] ** 2 / 2)) <= 1e-6))
def thermostat_allowed():
    def run(k):
        def step(s, i):
            heat = k * (20 - s["temp"])
            if not (0 <= heat <= 5): raise Stop("admissibility violation: `heat` = %s is not in 0..5, state { temp: %s }" % (fmt(heat), fmt(s["temp"])))
            return {"temp": s["temp"] + heat}
        return simulate({"temp": 10}, step, 8)
    return sweep(0, [[("k", v)] for v in (0.5, 1.5)], run)
def invariant_violation():
    return one(lambda: simulate({"n": 0}, lambda s, k: {"n": s["n"] + 1}, 10, invariant=lambda s: s["n"] < 5))
def until_already_true():
    return one(lambda: simulate({"n": 3}, lambda s, k: {"n": s["n"] + 1}, 10, until=lambda s: s["n"] >= 3))
def invariant_beats_until():
    return one(lambda: simulate({"n": 0}, lambda s, k: {"n": s["n"] + 1}, 10,
                                invariant=lambda s: s["n"] < 3, until=lambda s: s["n"] >= 3))

# ------------------------------------------------- D7 random numbers (reference)
M64 = (1 << 64) - 1
def splitmix64_stream(state):
    while True:
        state = (state + 0x9E3779B97F4A7C15) & M64
        z = state
        z = ((z ^ (z >> 30)) * 0xBF58476D1CE4E5B9) & M64
        z = ((z ^ (z >> 27)) * 0x94D049BB133111EB) & M64
        yield z ^ (z >> 31)
def rotl(x, k): return ((x << k) | (x >> (64 - k))) & M64
class Xoshiro256pp:
    def __init__(self, job_seed):
        sm = splitmix64_stream(job_seed)
        self.s = [next(sm) for _ in range(4)]
    def next_u64(self):
        s = self.s
        result = (rotl((s[0] + s[3]) & M64, 23) + s[0]) & M64
        t = (s[1] << 17) & M64
        s[2] ^= s[0]; s[3] ^= s[1]; s[1] ^= s[2]; s[0] ^= s[3]
        s[2] ^= t; s[3] = rotl(s[3], 45)
        return result
    def uniform01(self): return (self.next_u64() >> 11) * 2.0 ** -53
def job_seed(base, j): return next(splitmix64_stream(base ^ j))
def random_walk():
    jobs = []
    for j in range(3):
        rng = Xoshiro256pp(job_seed(42, j))
        def step(s, k):
            return {"pos": s["pos"] + (1 if rng.uniform01() < 0.5 else -1)}
        rows, stop = simulate({"pos": 0}, step, 20)
        jobs.append(Job(j, [], rows, stop))
    return jobs

# --------------------------------------------- statistical references (printed)
def stats():
    import random
    r = random.Random(1)
    ruin = dur = 0
    N = 4000
    for _ in range(N):
        b, t = 10, 0
        while 0 < b < 20 and t < 5000:
            b += 1 if r.random() < 0.5 else -1; t += 1
        ruin += (b == 0); dur += t
    print(f"gamblers_ruin: P(ruin) ~ {ruin/N:.3f} (theory 0.5), mean duration ~ {dur/N:.1f} (theory 100)")
    q = 0; cnt = 0
    for _ in range(200):
        L = 0
        for t in range(5000):
            L = max(L + (r.random() < 0.3) - (r.random() < 0.5), 0)
            if t >= 4000: q += L; cnt += 1
    print(f"mm1_queue: mean length ~ {q/cnt:.3f} (theory 0.75)")
    pulls = 0; R = 5000
    for _ in range(R):
        since, n = 0, 0
        while True:
            n += 1
            if r.random() < 0.006 or since + 1 >= 90: break
            since += 1
        pulls += n
    print(f"gacha_pity: mean pulls ~ {pulls/R:.2f} (theory {(1-0.994**90)/0.006:.2f})")

# ------------------------------------------------------------------ driver
TODAY = ["doubling", "fibonacci", "factorial", "factorial_overflow", "bitcoin_halving", "compound_interest",
         "logistic_map", "radioactive_decay", "newton_cooling", "water_tank", "sir", "oral_dose",
         "michaelis_menten", "thermostat", "predator_prey", "pid", "unstable_pendulum", "float_overflow",
         "float_div_zero", "precedence", "syntax_torture"]
PENDING = ["marbles", "collatz", "repeated_dosing", "bang_bang", "cooldowns", "idle_clicker",
           "amm_constant_product", "amm_fee_zero", "token_vesting", "sir_invariant", "combat_allowed",
           "bonding_curve", "thermostat_allowed", "invariant_violation", "until_already_true",
           "invariant_beats_until", "random_walk"]

def parse_expected(text):
    jobs, cur = [], None
    for line in text.splitlines():
        m = re.match(r"job (\d+)  ?(.*)", line)
        if m:
            cur = {"index": int(m.group(1)), "ov": m.group(2), "rows": [], "stop": None}; jobs.append(cur); continue
        m = re.match(r"\s+(\d+)\s+\{ (.*) \}$", line)
        if m:
            row = {}
            for part in m.group(2).split(", "):
                k, v = part.split(": ")
                row[k] = (v == "true") if v in ("true", "false") else float(v) if re.search(r"[.eE]|inf|NaN", v) else int(v)
            cur["rows"].append(row); continue
        m = re.match(r"\s+stopped at step (\d+): (.*)", line)
        if m: cur["stop"] = (int(m.group(1)), m.group(2))
    return jobs

def close(a, b):
    if isinstance(a, bool) or isinstance(b, bool): return a == b
    if isinstance(a, int) and isinstance(b, int): return a == b
    return math.isclose(float(a), float(b), rel_tol=1e-9, abs_tol=1e-12)

def check(root):
    bad = 0
    for name in TODAY:
        path = os.path.join(root, "today", name.replace("_", "-") if name == "predator_prey" else name)
        path += ".expected"
        if not os.path.exists(path):
            print(f"MISSING {path}"); bad += 1; continue
        got = parse_expected(open(path).read())
        ref = globals()[name]()
        problems = []
        if len(got) != len(ref): problems.append(f"job count {len(got)} vs {len(ref)}")
        for g, r in zip(got, ref):
            if len(g["rows"]) != len(r.rows): problems.append(f"job {r.index}: {len(g['rows'])} rows vs {len(r.rows)}")
            for i, (gr, rr) in enumerate(zip(g["rows"], r.rows)):
                for key in rr:
                    if key not in gr or not close(gr[key], rr[key]):
                        problems.append(f"job {r.index} row {i} {key}: {gr.get(key)} vs {rr[key]}"); break
                else: continue
                break
            if (g["stop"] is None) != (r.stop is None) or (g["stop"] and g["stop"][0] != r.stop[0]):
                problems.append(f"job {r.index}: stop {g['stop']} vs {r.stop}")
        print(("OK      " if not problems else "DIFFER  ") + name + ("" if not problems else "  " + "; ".join(problems[:3])))
        bad += bool(problems)
    stats()
    return bad

def write_pending(root):
    for name in PENDING:
        jobs = globals()[name]()
        path = os.path.join(root, "pending", name + ".expected")
        open(path, "w").write(render(jobs))
        print("wrote", path)

if __name__ == "__main__":
    mode, root = sys.argv[1], sys.argv[2]
    if mode == "check": sys.exit(1 if check(root) else 0)
    elif mode == "write-pending": write_pending(root)
