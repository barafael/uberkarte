//! stack-core: the object model, the change journal, the one-file format and
//! the script engine with its message path. Nothing here knows about a UI.
pub mod files;
pub mod model;
pub mod script;
pub mod demo;

pub use model::{canon_prop, migrate_joins, migrate_logic, Asset, AssetKind, Change, ChangeKind, Kind, Obj, PlayerRequest, Shape, SoundCmd, StackFile, Stroke, Tool, Value, World, FORMAT, HOME, LIB0, VISIT0, GoStack, Menu, MenuItem};
pub use script::{hypertalk_blocks, hypertalk_words, Rt, Waiting};
