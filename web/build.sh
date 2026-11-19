#!/usr/bin/env bash
# Build the corpus explorer as static files in web/crates/hcapp/dist (see README.md).
#   web/build.sh                     production build (size-optimised, LTO, wasm-opt if present)
#   PUBLIC_URL=/repo/ web/build.sh   when the site is served from a sub-path (GitHub Pages)
set -euo pipefail
cd "$(dirname "$0")"

python3 export.py                              # the corpus files + preset questions + manifest, from ../surql, ../queries
cargo run --release -q -p hccheck              # load it as the browser will; every question must answer, writes must not
# A build id for cache-busting the files Trunk doesn't hash (the worker, its wasm, the dataset).
export HC_BUILD="$(python3 -c 'import json; print(json.load(open("crates/hcapp/public/data/manifest.json"))["version"][:8])')"
(cd crates/hcapp && trunk build --release --cargo-profile dist ${PUBLIC_URL:+--public-url "$PUBLIC_URL"})
# The worker's wasm is loaded by hcworker_loader.js (no integrity hash), so it can be optimised after the build.
if command -v wasm-opt >/dev/null; then
  wasm-opt -Oz --strip-debug --strip-producers --enable-bulk-memory --enable-nontrapping-float-to-int --enable-sign-ext \
    --enable-mutable-globals --enable-reference-types crates/hcapp/dist/hcworker_bg.wasm -o crates/hcapp/dist/hcworker_bg.opt.wasm \
    && mv crates/hcapp/dist/hcworker_bg.opt.wasm crates/hcapp/dist/hcworker_bg.wasm \
    || echo "wasm-opt failed; keeping the unoptimised worker"
fi
du -sh crates/hcapp/dist
