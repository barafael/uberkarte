//! Client for the corpus worker (crates/hcworker): starts it, tracks loading, and turns its replies into
//! futures. (After the omdurman rulebook archive.)

use std::cell::RefCell;
use std::collections::HashMap;

use futures::channel::oneshot;
use hccore::{Progress, QueryResult, Reply, Request, Tables};
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{MessageEvent, Worker};

#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Loading(Progress),
    Ready { rows: u64, ms: f64 },
    Failed(String),
}

#[derive(Clone, Copy)]
pub struct Kb {
    pub status: RwSignal<Status>,
    /// Record counts per table once loaded; also tells which strings are record ids.
    pub tables: RwSignal<Tables>,
}

struct Inner {
    worker: Worker,
    next_id: u64,
    pending: HashMap<u64, oneshot::Sender<QueryResult>>,
}

/// Set by build.sh (the dataset version); "dev" for local builds.
pub const BUILD: &str = match option_env!("HC_BUILD") { Some(b) => b, None => "dev" };

thread_local! {
    static INNER: RefCell<Option<Inner>> = const { RefCell::new(None) };
}

impl Kb {
    pub fn start() -> Kb {
        let kb = Kb {
            status: RwSignal::new(Status::Loading(Progress { file: "manifest.json".into(), done_bytes: 0, total_bytes: 1, ms: 0.0 })),
            tables: RwSignal::new(Tables::new()),
        };
        let worker = Worker::new(&format!("./hcworker_loader.js?v={BUILD}")).expect("start the database worker");
        let onmessage = Closure::<dyn FnMut(MessageEvent)>::new(move |ev: MessageEvent| {
            let Some(text) = ev.data().as_string() else { return };
            match serde_json::from_str::<Reply>(&text) {
                Ok(Reply::Progress(p)) => kb.status.set(Status::Loading(p)),
                Ok(Reply::Ready { tables, ms, .. }) => {
                    let rows = tables.values().sum();
                    kb.tables.set(tables);
                    kb.status.set(Status::Ready { rows, ms });
                }
                Ok(Reply::Failed { error }) => kb.status.set(Status::Failed(error)),
                Ok(Reply::Result { id, result }) => {
                    if let Some(tx) = INNER.with(|i| i.borrow_mut().as_mut().and_then(|i| i.pending.remove(&id))) {
                        let _ = tx.send(result);
                    }
                }
                Err(e) => kb.status.set(Status::Failed(format!("bad reply from the worker: {e}"))),
            }
        });
        worker.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
        onmessage.forget();
        INNER.with(|i| *i.borrow_mut() = Some(Inner { worker, next_id: 1, pending: HashMap::new() }));
        kb
    }

    pub async fn query(&self, q: String) -> QueryResult {
        let rx = INNER.with(|i| {
            let mut i = i.borrow_mut();
            let inner = i.as_mut()?;
            let id = inner.next_id;
            inner.next_id += 1;
            let (tx, rx) = oneshot::channel();
            inner.pending.insert(id, tx);
            let msg = serde_json::to_string(&Request { id, query: q }).ok()?;
            inner.worker.post_message(&JsValue::from_str(&msg)).ok()?;
            Some(rx)
        });
        match rx {
            Some(rx) => rx.await.unwrap_or_else(|_| QueryResult::failed("the worker stopped")),
            None => QueryResult::failed("the worker is not running"),
        }
    }

    /// Whether a string is a record id of a loaded table ("concept:stack").
    pub fn is_record(&self, s: &str) -> bool {
        let Some((table, key)) = s.split_once(':') else { return false };
        !key.is_empty() && !key.contains(' ') && self.tables.with_untracked(|t| t.contains_key(table))
    }
}
