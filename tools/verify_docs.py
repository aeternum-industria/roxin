#!/usr/bin/env python3
"""Keeps the documentation honest.

    python3 tools/verify_docs.py --roxin ./target/release/roxin

Checks, over README.md and every Markdown file under docs/:

  1. A fenced block that follows `<!-- example: NAME.rox -->` must equal examples/NAME.rox exactly.
  2. A fenced block that follows `<!-- output: NAME.rox -->` must be made of lines that the real run of
     examples/NAME.rox prints, in order (a block may skip lines; it may not invent them).
  3. Every relative link points at a file that exists, and every #anchor points at a heading.

The roxin binary is run as `roxin FILE` and must print jobs in the usual format. Exit status is 1 on any failure.
"""
import argparse, os, re, subprocess, sys

ap = argparse.ArgumentParser()
ap.add_argument("--roxin", required=True, help="path to the roxin binary")
ap.add_argument("--root", default=".", help="repository root")
args = ap.parse_args()
root = os.path.abspath(args.root)

docs = [os.path.join(root, "README.md")]
for d, _, files in os.walk(os.path.join(root, "docs")):
    docs += [os.path.join(d, f) for f in files if f.endswith(".md")]

run_cache = {}
def actual(name):
    if name not in run_cache:
        p = subprocess.run([args.roxin, os.path.join(root, "examples", name)], capture_output=True, text=True)
        run_cache[name] = [l.rstrip() for l in (p.stdout + p.stderr).splitlines()]
    return run_cache[name]

def slug(heading):
    h = re.sub(r"[`*]", "", heading.strip().lower())
    h = re.sub(r"[^\w\- ]", "", h)
    return h.replace(" ", "-")

bad = 0
def fail(path, msg):
    global bad
    bad += 1
    print(f"FAIL {os.path.relpath(path, root)}: {msg}")

checked = {"example": 0, "output": 0, "link": 0}
for path in docs:
    text = open(path).read()
    lines = text.split("\n")
    i = 0
    while i < len(lines):
        m = re.match(r"<!-- (example|output): (\S+) -->", lines[i])
        if m and i + 1 < len(lines) and lines[i + 1].startswith("```"):
            kind, name = m.groups()
            j = i + 2
            block = []
            while not lines[j].startswith("```"):
                block.append(lines[j]); j += 1
            if kind == "example":
                want = open(os.path.join(root, "examples", name)).read().rstrip("\n")
                if "\n".join(block) != want:
                    fail(path, f"example block for {name} differs from examples/{name}")
                checked["example"] += 1
            else:
                real = actual(name)
                pos = 0
                for line in block:
                    line = line.rstrip()
                    if not line:
                        continue
                    try:
                        pos = real.index(line, pos) + 1
                    except ValueError:
                        fail(path, f"output line not produced by {name} (in order): {line[:70]}")
                        break
                checked["output"] += 1
            i = j
        i += 1
    # links
    headings = {slug(h) for h in re.findall(r"^#+\s+(.*)$", text, re.M)}
    for target in re.findall(r"\]\(([^)\s]+)\)", text):
        if target.startswith(("http://", "https://", "mailto:")):
            continue
        checked["link"] += 1
        file_part, _, anchor = target.partition("#")
        dest = path if not file_part else os.path.normpath(os.path.join(os.path.dirname(path), file_part))
        if not os.path.exists(dest):
            fail(path, f"broken link: {target}")
            continue
        if anchor and dest.endswith(".md"):
            dest_headings = headings if dest == path else {slug(h) for h in re.findall(r"^#+\s+(.*)$", open(dest).read(), re.M)}
            if anchor not in dest_headings:
                fail(path, f"broken anchor: {target}")

print(f"checked {checked['example']} example blocks, {checked['output']} output blocks, {checked['link']} links: "
      + ("ALL OK" if not bad else f"{bad} problem(s)"))
sys.exit(1 if bad else 0)
