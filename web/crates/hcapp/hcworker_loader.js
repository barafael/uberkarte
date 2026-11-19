// Starts the corpus worker (crates/hcworker), which Trunk builds as a no-modules script. The query string
// (?v=<build>) is passed on so a new deploy never mixes with cached files of an older one.
importScripts("./hcworker.js" + self.location.search);
wasm_bindgen("./hcworker_bg.wasm" + self.location.search);
