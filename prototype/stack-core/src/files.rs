//! Files for scripts: the real file system on desktop, an in-memory folder on the web (which the player
//! mirrors to the browser's private file system). Text files only, UTF-8. Paths use `/`.
use std::collections::BTreeMap;

pub trait Fs {
    fn read(&self, path: &str) -> Result<String, String>;
    fn read_bytes(&self, path: &str) -> Result<Vec<u8>, String>;
    fn write(&mut self, path: &str, text: &str) -> Result<(), String>;
    fn delete(&mut self, path: &str) -> Result<(), String>;
    fn rename(&mut self, from: &str, to: &str) -> Result<(), String>;
    fn create_folder(&mut self, path: &str) -> Result<(), String>;
    fn is_file(&self, path: &str) -> bool;
    fn is_folder(&self, path: &str) -> bool;
    /// (files, folders) directly inside `path`, names only, sorted.
    fn list(&self, path: &str) -> Result<(Vec<String>, Vec<String>), String>;
    /// Changes since the last call, for mirroring (web): ("write", path, text), ("delete", path, ""),
    /// ("folder", path, ""). The real file system returns nothing.
    fn take_changes(&mut self) -> Vec<(String, String, String)> { vec![] }
    /// Restore a file (Some(text)) or folder (None) from browser storage, without recording a change.
    fn preload(&mut self, _path: &str, _text: Option<&str>) {}
}

/// Join a relative path onto a base folder; absolute paths (`/…`, `C:\…`, `~/…`) are kept.
pub fn resolve_path(base: &str, path: &str) -> String {
    let p = path.trim();
    let absolute = p.starts_with('/') || p.starts_with('~') || p.chars().nth(1) == Some(':');
    let joined = if absolute || base.is_empty() { p.to_string() } else { format!("{}/{}", base.trim_end_matches('/'), p) };
    // Normalize `.` and `..` segments.
    let mut parts: Vec<&str> = vec![];
    let lead = if joined.starts_with('/') { "/" } else { "" };
    for seg in joined.split('/') {
        match seg { "" | "." => {}, ".." => { if parts.last().is_some_and(|l| *l != "..") { parts.pop(); } else if lead.is_empty() { parts.push(".."); } } s => parts.push(s) }
    }
    format!("{lead}{}", parts.join("/"))
}

/// The browser-side disk: everything in memory, changes queued for the player to persist.
#[derive(Default)]
pub struct MemFs {
    files: BTreeMap<String, String>,
    folders: std::collections::BTreeSet<String>,
    changes: Vec<(String, String, String)>,
}

impl MemFs {
    pub fn new() -> Self { MemFs::default() }
    /// Load a file without recording a change (used when restoring from browser storage).
    pub fn preload(&mut self, path: &str, text: &str) {
        let p = resolve_path("/", path);
        self.add_parents(&p);
        self.files.insert(p, text.to_string());
    }
    fn add_parents(&mut self, p: &str) {
        let mut cur = String::new();
        let segs: Vec<&str> = p.trim_start_matches('/').split('/').collect();
        for s in &segs[..segs.len().saturating_sub(1)] { cur = format!("{cur}/{s}"); self.folders.insert(cur.clone()); }
    }
    fn key(path: &str) -> String { resolve_path("/", path) }
}

impl Fs for MemFs {
    fn read(&self, path: &str) -> Result<String, String> { self.files.get(&Self::key(path)).cloned().ok_or_else(|| format!("no file \"{path}\"")) }
    fn read_bytes(&self, path: &str) -> Result<Vec<u8>, String> { self.read(path).map(|s| s.into_bytes()) }
    fn write(&mut self, path: &str, text: &str) -> Result<(), String> {
        let k = Self::key(path);
        if self.folders.contains(&k) { return Err(format!("\"{path}\" is a folder")); }
        self.add_parents(&k);
        self.files.insert(k.clone(), text.to_string());
        self.changes.push(("write".into(), k, text.to_string()));
        Ok(())
    }
    fn delete(&mut self, path: &str) -> Result<(), String> {
        let k = Self::key(path);
        if self.files.remove(&k).is_some() { self.changes.push(("delete".into(), k, String::new())); return Ok(()); }
        if self.folders.contains(&k) {
            if self.files.keys().any(|f| f.starts_with(&format!("{k}/"))) { return Err(format!("folder \"{path}\" is not empty")); }
            self.folders.remove(&k);
            self.changes.push(("delete".into(), k, String::new()));
            return Ok(());
        }
        Err(format!("no file \"{path}\""))
    }
    fn rename(&mut self, from: &str, to: &str) -> Result<(), String> {
        let text = self.read(from)?;
        if self.is_file(to) { return Err(format!("\"{to}\" already exists")); }
        self.write(to, &text)?;
        self.delete(from)
    }
    fn create_folder(&mut self, path: &str) -> Result<(), String> {
        let k = Self::key(path);
        self.add_parents(&format!("{k}/x"));
        self.changes.push(("folder".into(), k, String::new()));
        Ok(())
    }
    fn is_file(&self, path: &str) -> bool { self.files.contains_key(&Self::key(path)) }
    fn is_folder(&self, path: &str) -> bool { let k = Self::key(path); k == "/" || self.folders.contains(&k) }
    fn list(&self, path: &str) -> Result<(Vec<String>, Vec<String>), String> {
        let k = Self::key(path);
        if !self.is_folder(&k) { return Err(format!("no folder \"{path}\"")); }
        let prefix = if k == "/" { "/".to_string() } else { format!("{k}/") };
        let direct = |p: &String| p.starts_with(&prefix) && !p[prefix.len()..].contains('/');
        let files = self.files.keys().filter(|p| direct(p)).map(|p| p[prefix.len()..].to_string()).collect();
        let folders = self.folders.iter().filter(|p| direct(p) && p.len() > prefix.len()).map(|p| p[prefix.len()..].to_string()).collect();
        Ok((files, folders))
    }
    fn take_changes(&mut self) -> Vec<(String, String, String)> { std::mem::take(&mut self.changes) }
    fn preload(&mut self, path: &str, text: Option<&str>) {
        match text {
            Some(t) => MemFs::preload(self, path, t),
            None => { let k = Self::key(path); self.add_parents(&format!("{k}/x")); }
        }
    }
}

/// The real file system (desktop). `~/` means the home folder.
#[cfg(not(target_arch = "wasm32"))]
pub struct NativeFs;

#[cfg(not(target_arch = "wasm32"))]
impl NativeFs {
    fn p(path: &str) -> std::path::PathBuf {
        match path.strip_prefix("~/") {
            Some(rest) => std::env::var_os("HOME").map(std::path::PathBuf::from).unwrap_or_default().join(rest),
            None => std::path::PathBuf::from(path),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Fs for NativeFs {
    fn read(&self, path: &str) -> Result<String, String> { std::fs::read_to_string(Self::p(path)).map_err(|e| format!("{path}: {e}")) }
    fn read_bytes(&self, path: &str) -> Result<Vec<u8>, String> { std::fs::read(Self::p(path)).map_err(|e| format!("{path}: {e}")) }
    fn write(&mut self, path: &str, text: &str) -> Result<(), String> {
        let p = Self::p(path);
        if let Some(d) = p.parent() { if !d.as_os_str().is_empty() { std::fs::create_dir_all(d).map_err(|e| format!("{path}: {e}"))?; } }
        std::fs::write(p, text).map_err(|e| format!("{path}: {e}"))
    }
    fn delete(&mut self, path: &str) -> Result<(), String> {
        let p = Self::p(path);
        if p.is_dir() { std::fs::remove_dir(p) } else { std::fs::remove_file(p) }.map_err(|e| format!("{path}: {e}"))
    }
    fn rename(&mut self, from: &str, to: &str) -> Result<(), String> {
        if Self::p(to).exists() { return Err(format!("\"{to}\" already exists")); }
        std::fs::rename(Self::p(from), Self::p(to)).map_err(|e| format!("{from}: {e}"))
    }
    fn create_folder(&mut self, path: &str) -> Result<(), String> { std::fs::create_dir_all(Self::p(path)).map_err(|e| format!("{path}: {e}")) }
    fn is_file(&self, path: &str) -> bool { Self::p(path).is_file() }
    fn is_folder(&self, path: &str) -> bool { Self::p(path).is_dir() }
    fn list(&self, path: &str) -> Result<(Vec<String>, Vec<String>), String> {
        let mut files = vec![];
        let mut folders = vec![];
        for e in std::fs::read_dir(Self::p(path)).map_err(|e| format!("{path}: {e}"))?.flatten() {
            let name = e.file_name().to_string_lossy().into_owned();
            if e.path().is_dir() { folders.push(name) } else { files.push(name) }
        }
        files.sort();
        folders.sort();
        Ok((files, folders))
    }
}

/// The file system a new runtime starts with: the real one on desktop, an empty in-memory one on the web.
pub fn default_fs() -> Box<dyn Fs> {
    #[cfg(not(target_arch = "wasm32"))]
    { Box::new(NativeFs) }
    #[cfg(target_arch = "wasm32")]
    { Box::new(MemFs::new()) }
}
