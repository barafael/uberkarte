#!/usr/bin/env python3
"""Build reader-test.html from ../reader-test.md, so the screen and the paper sheet cannot differ.
Run: python3 notes/reader-test/build.py"""
import json
import pathlib
import re

here = pathlib.Path(__file__).parent
md = (here.parent / "reader-test.md").read_text()
handlers = []
for sec in re.split(r"^### ", md, flags=re.M)[1:]:
    m = re.match(r"(\d+)\. (\w+)", sec)
    if not m:
        continue
    blocks = re.findall(r"```\n(.*?)\n```", sec, re.S)
    context = re.search(r'Context: \*"(.*?)"\*', sec, re.S).group(1)
    question = re.search(r'Question: \*\*"(.*?)"\*\*', sec, re.S).group(1)
    key = re.search(r"^Key: (.*?)(?:\n\n|\Z)", sec, re.S | re.M).group(1)
    handlers.append({
        "n": int(m.group(1)), "name": m.group(2),
        "context": " ".join(context.split()), "question": " ".join(question.split()),
        "braces": blocks[0], "end": blocks[1], "key": " ".join(key.split()),
    })
assert [h["n"] for h in handlers] == [1, 2, 3, 4], handlers
page = (here / "template.html").read_text().replace("/*HANDLERS*/[]", json.dumps(handlers, indent=1, ensure_ascii=False))
(here / "reader-test.html").write_text(page)
print("wrote", here / "reader-test.html", "with", len(handlers), "handlers")
