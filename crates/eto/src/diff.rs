use std::path::PathBuf;

use tracing::{event, Level};

use crate::state::State;

#[derive(Default, Debug)]
pub struct Diff {
    new: Vec<PathBuf>,
    change: Vec<PathBuf>,
    delete: Vec<PathBuf>,
}

impl Diff {
    pub fn from_states(old: &State, new: &State) -> Self {
        event!(Level::INFO, "diffing states");

        let mut diff = Self::default();

        // Go through all files in new to check them against the old state
        for (path, hash) in &new.files {
            // Check if they exist in the old state
            if let Some(old_hash) = old.files.get(path) {
                // It does exist, check if it changed
                if old_hash != hash {
                    event!(
                        Level::INFO,
                        path = path.display().to_string(),
                        "change"
                    );
                    diff.change.push(path.clone());
                }
            } else {
                // It doesn't exist, so it's new
                event!(Level::INFO, path = path.display().to_string(), "new");
                diff.new.push(path.clone());
            }
        }

        // Go through all old files, and check if any were deleted in the new state
        for path in old.files.keys() {
            if !new.files.contains_key(path) {
                event!(
                    Level::INFO,
                    path = path.display().to_string(),
                    "delete"
                );
                diff.delete.push(path.clone());
            }
        }

        diff
    }
}
