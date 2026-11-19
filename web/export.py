#!/usr/bin/env python3
"""Export the corpus for the browser: crates/hcapp/public/data/
  - the surql/*.surql files, in load order, with `PERMISSIONS FOR select FULL` added to every table
    (so the sealed guest session can read and nothing else),
  - examples.json: the preset questions, from queries/examples.surql (`-- N. title` headings),
  - manifest.json: the files, their sizes and hashes, and a version.
The corpus itself is not changed."""
import hashlib, json, pathlib, re, sys

root = pathlib.Path(__file__).resolve().parent.parent
out = pathlib.Path(__file__).resolve().parent / "crates/hcapp/public/data"
out.mkdir(parents=True, exist_ok=True)
for old in out.glob("*.surql"):
    old.unlink()

table_line = re.compile(r"^(\s*DEFINE TABLE\b[^;]*?)(\s*;)\s*$", re.I)
files, load = [], []
for src in sorted((root / "surql").glob("*.surql")):
    text = src.read_text()
    lines, tables = [], 0
    for line in text.split("\n"):
        m = table_line.match(line)
        if m and "PERMISSIONS" not in line.upper():
            line = f"{m.group(1)} PERMISSIONS FOR select FULL{m.group(2)}"
            tables += 1
        lines.append(line)
    # Every table definition must have been a single line we could extend.
    defined = len(re.findall(r"\bDEFINE TABLE\b", text, re.I))
    if defined != tables:
        sys.exit(f"{src.name}: {defined} DEFINE TABLE statements but {tables} rewritten; keep each on one line")
    body = "\n".join(lines)
    (out / src.name).write_text(body)
    b = body.encode()
    files.append({"path": src.name, "bytes": len(b), "sha256": hashlib.sha256(b).hexdigest()})
    load.append(src.name)

# Preset questions: each `-- N. title` heading and the statements up to the next comment heading.
examples, cur = [], None
for line in (root / "queries/examples.surql").read_text().split("\n"):
    m = re.match(r"^--\s*(\d+)\.\s*(.+)$", line)
    if m:
        cur = {"title": m.group(2).strip(), "query": ""}
        examples.append(cur)
    elif line.startswith("--"):
        cur = None
    elif cur is not None:
        cur["query"] += line + "\n"
for e in examples:
    e["query"] = e["query"].strip()
examples = [e for e in examples if e["query"]]
(out / "examples.json").write_text(json.dumps(examples, indent=1, ensure_ascii=False))

version = hashlib.sha256("".join(f["sha256"] for f in files).encode()).hexdigest()
(out / "manifest.json").write_text(json.dumps({"format": 1, "version": version, "load": load, "files": files}, indent=1))
print(f"exported {len(files)} files ({sum(f['bytes'] for f in files)} bytes) and {len(examples)} questions; version {version[:8]}")
