//! The only state waygate writes: done marks, pane widths and cached summaries.
//! Claude Code's own files are never modified.

use std::{collections::HashMap, fs, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Store {
    /// Session id to the `updated` time it was marked done at. New activity
    /// after that time puts it back in "Waiting on me".
    #[serde(default)]
    pub done: HashMap<String, i64>,
    #[serde(default)]
    pub left: Option<u16>,
    #[serde(default)]
    pub right: Option<u16>,
    #[serde(default)]
    pub summaries: HashMap<String, Summary>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Summary {
    pub updated: i64,
    pub made: i64,
    pub text: String,
}

fn path() -> Option<PathBuf> {
    dirs::data_dir().map(|d| d.join("waygate").join("state.json"))
}

/// Where state lived before the rename from cs. Read only, as a fallback
/// until the first save writes the new path.
fn old_path() -> Option<PathBuf> {
    dirs::data_dir().map(|d| d.join("cs").join("state.json"))
}

impl Store {
    pub fn load() -> Self {
        path()
            .and_then(|p| fs::read(p).ok())
            .or_else(|| fs::read(old_path()?).ok())
            .and_then(|b| serde_json::from_slice(&b).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) {
        let Some(p) = path() else { return };
        if let Some(dir) = p.parent() {
            let _ = fs::create_dir_all(dir);
        }
        if let Ok(json) = serde_json::to_vec_pretty(self) {
            let tmp = p.with_extension("tmp");
            if fs::write(&tmp, json).is_ok() {
                let _ = fs::rename(tmp, p);
            }
        }
    }
}
