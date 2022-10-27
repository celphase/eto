use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use anyhow::{Context, Error};
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
        let ignores: Result<Vec<_>, _> = metadata
            .ignore
            .into_iter()
            .map(|ignore| Pattern::new(&ignore))
            .collect();
        let mut ignores = ignores?;

        // Always ignore eto.log
        ignores.push(Pattern::new("eto.log").unwrap());

        // Read all state files (this includes the metadata file intentionally)
        'outer: for entry in WalkDir::new(directory) {
            let entry = entry?;
            let path = entry.path();

            // We don't handle directories directly, just files
            if entry.path().is_dir() {
                continue;
            }

            let path_str = path.display().to_string();
            event!(Level::DEBUG, path = path_str, "checking path");

            // Check if we need to ignore this
            let diff_path = diff_paths(path, directory).unwrap();
            for ignore in &ignores {
                if ignore.matches_path(&diff_path) {
                    event!(Level::DEBUG, path = path_str, "ignoring");
                    continue 'outer;
                }
            }

            // We've found a file, hash it so we can track changes
            let bytes = std::fs::read(path).context("failed to read file to track")?;
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
