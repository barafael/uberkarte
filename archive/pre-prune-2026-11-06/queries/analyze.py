#!/usr/bin/env python3
"""Graph analysis of the hypercard corpus. Expects a running server (see load.sh)."""
import collections
import json
import os
import urllib.request

URL = os.environ.get("HC_URL", "http://127.0.0.1:8123/sql")
AUTH = "root:root"


def sql(q):
    req = urllib.request.Request(URL, data=q.encode(), method="POST")
    import base64
    req.add_header("Authorization", "Basic " + base64.b64encode(AUTH.encode()).decode())
    req.add_header("Surreal-NS", "hypercard")
    req.add_header("Surreal-DB", "corpus")
    req.add_header("Content-Type", "application/json")
    with urllib.request.urlopen(req) as r:
        return json.load(r)


def q(query):
    res = sql(query)
    if isinstance(res, dict) and res.get("status") != "OK":
        raise RuntimeError(res)
    out = []
    for item in res:
        if item.get("status") != "OK":
            raise RuntimeError(item)
        r = item.get("result")
        out.append(r if isinstance(r, list) else ([] if r is None else [r]))
    return out


EDGES = ["part_of", "implements", "exemplifies", "influenced", "derived_from",
         "succeeded", "created_by", "contributed_to", "owned_by", "applied_to",
         "about", "documented_in", "maps_to", "limits", "motivates", "addresses",
         "raises", "contrasts_with", "in_thread", "enables", "located_in",
         "uses", "demonstrates", "extracted_from", "defined_in", "relates_to",
         "specifies", "worked_with", "employed_by", "participated_in",
         "spun_off_from", "renamed_to", "published_by"]
NODES = ["concept", "person", "org", "artifact", "event", "source", "thread",
         "keyword", "code"]

# raw in/out are full record ids ("concept:hypercard"); meta::id() would strip
# the table prefix and break cross-table id comparison
edges = []
for t in EDGES:
    for row in q(f"SELECT `in`, `out` FROM {t};")[0]:
        edges.append((t, row["in"], row["out"]))

allnodes = set()
for t in NODES:
    for row in q(f"SELECT id FROM {t};")[0]:
        allnodes.add(row["id"])

deg = collections.Counter()
outdeg, indeg = collections.Counter(), collections.Counter()
adj_out = collections.defaultdict(list)
adj_und = collections.defaultdict(set)
for t, s, d in edges:
    deg[s] += 1; deg[d] += 1
    outdeg[s] += 1; indeg[d] += 1
    adj_out[s].append((t, d))
    adj_und[s].add(d); adj_und[d].add(s)

print(f"nodes={len(allnodes)} edges={len(edges)}")
print("\n== edge count by table ==")
cnt = collections.Counter(t for t, _, _ in edges)
for t, c in cnt.most_common(): print(f"  {t:16s} {c}")

print("\n== top 20 hubs (total degree) ==")
for n, d in deg.most_common(20):
    print(f"  {d:3d}  {n}   (out {outdeg[n]} / in {indeg[n]})")

iso = sorted(n for n in allnodes if deg[n] == 0)
leaf = sorted(n for n in allnodes if deg[n] == 1)
print(f"\n== isolated (degree 0): {len(iso)} ==")
for n in iso: print("  ", n)
print(f"\n== leaves (degree 1): {len(leaf)} ==")
by_tbl = collections.Counter(n.split(':')[0] for n in leaf)
print("  by table:", dict(by_tbl))

seen, stack = {"artifact:hypercard"}, ["artifact:hypercard"]
while stack:
    x = stack.pop()
    for y in adj_und.get(x, ()):
        if y not in seen: seen.add(y); stack.append(y)
print(f"\n== weakly connected component of hypercard: {len(seen)}/{len(allnodes)} ==")
print("== outside it ==")
for n in sorted(allnodes - seen): print("  ", n)


def bfs(start, goal, rel=None, direction="out"):
    if start not in allnodes or goal not in allnodes: return None
    prev, seen = {start: None}, {start}
    fr = [start]
    while fr:
        nxt = []
        for x in fr:
            if x == goal:
                path = []
                while x: path.append(x); x = prev[x]
                return list(reversed(path))
            for t, y in adj_out.get(x, ()) if direction == "out" else []:
                if rel and t != rel: continue
                if y not in seen: seen.add(y); prev[y] = x; nxt.append(y)
            if direction == "und":
                for y in adj_und.get(x, ()):
                    if y not in seen: seen.add(y); prev[y] = x; nxt.append(y)
        fr = nxt
    return None


for a, b, rel in [("concept:memex", "artifact:hypercard", "influenced"),
                  ("artifact:hypercard", "artifact:javascript", "influenced"),
                  ("artifact:hypercard", "artifact:decker", "influenced")]:
    p = bfs(a, b, rel=rel)
    print(f"\n== {rel} path {a} -> {b} ==")
    print("  " + " -> ".join(p) if p else "  (none)")

print("\n== undirected path memex -> wiki ==")
p = bfs("concept:memex", "concept:wiki", direction="und")
print("  " + " - ".join(p) if p else "  (none)")

# full-text check
print("\n== full-text: body @@ 'hypertext' ==")
for row in q("SELECT id, body FROM concept WHERE body @@ 'hypertext' LIMIT 12;")[0]:
    print("  ", row["id"])

res = q("SELECT id FROM concept WHERE kind = 'tension';")[0]
tensions = {r["id"] for r in res}
addressed = {d for t, s, d in edges if t == "addresses"}
print("\n== tensions with no addressing analog ==")
for n in sorted(tensions - addressed): print("  ", n)

res = q("SELECT id FROM concept WHERE kind = 'question';")[0]
questions = {r["id"] for r in res}
raised = {d for t, s, d in edges if t == "raises"}
print(f"== questions raised: {len(questions & raised)}/{len(questions)} ==")
print("== questions never raised ==")
for n in sorted(questions - raised): print("  ", n)

# == reference + code corpus coverage =========================================
keywords = {r["id"] for t in ["keyword"] for r in q(f"SELECT id FROM {t};")[0]}
codes = {r["id"] for r in q("SELECT id FROM code;")[0]}
used_kw = {d for t, s, d in edges if t == "uses"}
related_kw = {s for t, s, d in edges if t == "relates_to"}
demo_codes = {s for t, s, d in edges if t == "demonstrates"}
sourced_codes = {s for t, s, d in edges if t == "extracted_from"}

print(f"\n== keyword coverage: {len(used_kw & keywords)} used in code, "
      f"{len(related_kw & keywords)} concept-linked ==")
ref_only = sorted(keywords - used_kw - related_kw)
print(f"   reference-only (defined_in only): {len(ref_only)}")

print(f"== code coverage: {len(demo_codes & codes)}/{len(codes)} demonstrate a concept, "
      f"{len(sourced_codes & codes)}/{len(codes)} have a source ==")

print("\n== most-used keywords ==")
for kw, c in collections.Counter(d for t, s, d in edges if t == "uses").most_common(10):
    print(f"  {c:2d}  {kw}")

print("\n== code samples per source ==")
for src, c in collections.Counter(d for t, s, d in edges if t == "extracted_from").most_common():
    print(f"  {c:2d}  {src}")

print("\n== concepts demonstrated by code ==")
for con, c in collections.Counter(d for t, s, d in edges if t == "demonstrates").most_common(10):
    print(f"  {c:2d}  {con}")


# == critique checks (notes/coverage-critique.md) ===============================
# Each block measures one of the critique's findings so progress is queryable.
print("\n\n######## critique checks ########")

CORE = ["concept", "person", "org", "artifact", "event", "source", "thread"]
core_nodes = {r["id"] for t in CORE for r in q(f"SELECT id FROM {t};")[0]}

# 1. provenance: a core node is "sourced" if any source->about->it or it->documented_in->source
sourced = ({d for t, s, d in edges if t == "about" and s.startswith("source:")}
           | {s for t, s, d in edges if t == "documented_in"})
hi = {}
for t in ["concept", "artifact", "person", "event"]:
    for r in q(f"SELECT id, confidence FROM {t};")[0]:
        hi[r["id"]] = r.get("confidence")
high_unsourced = sorted(n for n, c in hi.items() if c == "high" and n not in sourced)
print(f"\n== provenance: {len(sourced & core_nodes)}/{len(core_nodes)} core nodes sourced; "
      f"{len(high_unsourced)} high-confidence nodes unsourced ==")

# 2. connectivity without the curation layer
def components(edge_filter):
    adj = collections.defaultdict(set)
    for t, s, d in edges:
        if edge_filter(t):
            adj[s].add(d); adj[d].add(s)
    left, comps = set(allnodes), 0
    while left:
        comps += 1
        stack = [left.pop()]
        while stack:
            x = stack.pop()
            for y in adj.get(x, ()):
                if y in left: left.remove(y); stack.append(y)
    return comps
print(f"== components: all edges={components(lambda t: True)}, "
      f"without in_thread={components(lambda t: t != 'in_thread')}, "
      f"without in_thread/about/documented_in="
      f"{components(lambda t: t not in ('in_thread','about','documented_in'))} ==")

# 3. social graph
people = {r["id"] for r in q("SELECT id FROM person;")[0]}
pp = [(s, d) for t, s, d in edges if t == "worked_with"]
person_leaves = sorted(n for n in people if deg[n] == 1)
print(f"== social: {len(pp)} person-person edges; {len(person_leaves)}/{len(people)} persons are leaves ==")

# 4. events with actors
events = {r["id"] for r in q("SELECT id FROM event;")[0]}
actors = {d for t, s, d in edges if t == "participated_in"}
print(f"== events with a participant: {len(events & actors)}/{len(events)} ==")
for n in sorted(events - actors): print("   no actor:", n)

# 5. provenance edges that carry confidence
for rel in ("about", "documented_in"):
    rows = q(f"SELECT confidence FROM {rel};")[0]
    c = sum(1 for r in rows if r.get("confidence"))
    print(f"== {rel}: {c}/{len(rows)} edges carry confidence ==")

# 6. chronology
years = collections.Counter()
for t in ("artifact", "concept", "event"):
    for r in q(f"SELECT id, year, `when` FROM {t};")[0] if t != "event" else []:
        if r.get("year"): years[r["year"]] += 1
for r in q("SELECT `when` FROM event;")[0]:
    w = (r.get("when") or "")[:4]
    if w.isdigit(): years[int(w)] += 1
gaps = [y for y in range(1985, 2027) if y not in years]
print(f"== years 1985-2026 with no dated node: {gaps} ==")

# 7. artifact kinds in use
kinds = collections.Counter(r["kind"] for r in q("SELECT kind FROM artifact;")[0])
unused = {"file_format", "archive"} - set(kinds)
print(f"== artifact kinds: {dict(kinds)}; schema kinds unused: {sorted(unused)} ==")
pk = collections.Counter(r["kind"] for r in q("SELECT kind FROM concept;")[0])
print(f"== concept kinds: {dict(pk)} ==")
