//! The explorer's database core, shared by the browser worker, the native check (`hccheck`) and the app.
//!
//! Without features this is only the protocol between the app and the worker. The `engine` feature adds
//! [`Kb`], the corpus in in-memory SurrealDB (engine.rs); `web` adds fetching text in the browser (web.rs).

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[cfg(feature = "engine")]
mod engine;
#[cfg(feature = "engine")]
pub use engine::{Error, Kb, Source};
#[cfg(feature = "web")]
pub mod web;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileEntry {
    pub path: String,
    pub bytes: u64,
    pub sha256: String,
}

/// `data/manifest.json`, written by export.py: the corpus files in load order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Manifest {
    pub format: u32,
    pub version: String,
    pub load: Vec<String>,
    pub files: Vec<FileEntry>,
}

impl Manifest {
    pub fn bytes(&self, path: &str) -> u64 {
        self.files.iter().find(|f| f.path == path).map_or(0, |f| f.bytes)
    }
}

/// A preset question (`data/examples.json`, from queries/examples.surql).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Example {
    pub title: String,
    pub query: String,
}

/// Record counts per table, once loaded.
pub type Tables = BTreeMap<String, u64>;

/// Loading progress, reported after each file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Progress {
    pub file: String,
    pub done_bytes: u64,
    pub total_bytes: u64,
    pub ms: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatementResult {
    pub result: Value,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryResult {
    pub ms: f64,
    pub statements: Vec<StatementResult>,
    /// Set when the query as a whole failed (e.g. a parse error).
    pub error: Option<String>,
}

impl QueryResult {
    pub fn failed(error: impl Into<String>) -> Self {
        QueryResult { ms: 0.0, statements: vec![], error: Some(error.into()) }
    }

    /// The answer to a query sent before the corpus is loaded.
    pub fn loading() -> Self {
        Self::failed("the corpus is still loading")
    }

    pub fn first_error(&self) -> Option<&str> {
        self.error.as_deref().or_else(|| self.statements.iter().find_map(|s| s.error.as_deref()))
    }
}

/// App → worker: run a query.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub id: u64,
    pub query: String,
}

/// Worker → app.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Reply {
    Progress(Progress),
    Ready { manifest: Manifest, tables: Tables, ms: f64 },
    Failed { error: String },
    Result { id: u64, result: QueryResult },
}
