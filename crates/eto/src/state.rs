use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use anyhow::Error;
use glob::Pattern;
use pathdiff::diff_paths;
use tracing::{event, Level};
use walkdir::WalkDir;

use crate::Metadata;

#[derive(Default, Debug, Clone)]
pub struct State {
    pub version: String,
    pub files: HashMap<PathBuf, String>,
}

impl State {
    pub fn read_dir(directory: &Path) -> Result<Self, Error> {
        event!(
            Level::INFO,
            directory = directory.display().to_string(),
            "reading state"
        );

        let mut files = HashMap::new();

        // Read state metadata
        let metadata = Metadata::from_dir(directory)?;
        event!(Level::INFO, version = metadata.version, "metadata");

        // Compile ignore patterns
        let mut ignores: Vec<_> = metadata
            .ignore
            .into_iter()
            .map(|ignore| Pattern::new(&ignore).unwrap())
            .collect();

        // Always ignore eto.log
        ignores.push(Pattern::new("eto.log").unwrap());

        // Read all state files (this includes the metadata file intentionally)
        'outer: for entry in WalkDir::new(directory) {
            let entry = entry.unwrap();
            let path = entry.path();

            // We don't handle directories directly, just files
            if entry.path().is_dir() {
                continue;
            }

            // Check if we need to ignore this
            let diff_path = diff_paths(path, directory).unwrap();
            for ignore in &ignores {
                if ignore.matches_path(&diff_path) {
                    event!(Level::DEBUG, path = path.display().to_string(), "ignoring");
                    continue 'outer;
                }
            }

            // We've found a file, hash it so we can track changes
            let bytes = std::fs::read(path).unwrap();
            let hash = sha256::digest_bytes(&bytes);

            event!(
                Level::DEBUG,
                path = diff_path.display().to_string(),
                hash,
                "adding"
            );
            files.insert(diff_path, hash);
        }

        Ok(Self {
            version: metadata.version,
            files,
        })
    }
}
