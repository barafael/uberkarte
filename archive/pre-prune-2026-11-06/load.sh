#!/usr/bin/env bash
# Start SurrealDB on a fresh store and load the corpus.
#   ./load.sh            -> wipe ./data, reload everything
#   SURREAL=/path ./load.sh
#
# NOTE: the surreal launcher execs a binary named `surreal-db`, so both
# process names must be killed. A stale server silently absorbs loads and
# duplicates every RELATE edge (UPSERT nodes dedupe, edges do not).
set -euo pipefail
cd "$(dirname "$0")"

SURREAL="${SURREAL:-$HOME/.surrealdb/surreal}"
ADDR="127.0.0.1:8123"
NS=hypercard
DB=corpus

pkill -x surreal 2>/dev/null || true
pkill -x surreal-db 2>/dev/null || true
for _ in $(seq 1 20); do
  curl -s --max-time 1 "http://$ADDR/health" >/dev/null 2>&1 || break
  sleep 0.5
done
if curl -s --max-time 1 "http://$ADDR/health" >/dev/null 2>&1; then
  echo "error: something still serves $ADDR — refusing to load into a stale server" >&2
  exit 1
fi

rm -rf ./data
nohup "$SURREAL" start --user root --pass root --bind "$ADDR" "surrealkv://$PWD/data" >/tmp/surreal.log 2>&1 &
for _ in $(seq 1 40); do
  curl -s --max-time 1 "http://$ADDR/health" >/dev/null 2>&1 && break
  sleep 0.5
done

sql() {
  curl -s -u root:root -H "Surreal-NS: $NS" -H "Surreal-DB: $DB" \
    -H "Content-Type: application/json" --data-binary "$1" "http://$ADDR/sql"
}

# guard: a fresh store must have no tables; if it does, we are talking to a
# server that kept an old store alive
if sql "INFO FOR DB;" | grep -q '"concept"'; then
  echo "error: store already contains tables — stale server, refusing to load" >&2
  exit 1
fi

fail=0
for f in surql/*.surql; do
  resp=$(sql "$(cat "$f")") || true
  if echo "$resp" | grep -qiE '"status":"ERR"|"details"|Parse error'; then
    echo "---- errors in $f ----" >&2
    echo "$resp" >&2
    fail=1
  fi
done
[ "$fail" -eq 0 ] && echo "loaded OK" || { echo "load had errors" >&2; exit 1; }
