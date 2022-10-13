use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use anyhow::Error;
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

        // TODO: Ignore certain files

        // Read state metadata
        let metadata = Metadata::from_dir(directory)?;
        event!(Level::INFO, version = metadata.version, "metadata");

        // Read all state files (this includes the metadata file intentionally)
        for entry in WalkDir::new(directory) {
            let entry = entry.unwrap();
            let path = entry.path();

            if entry.path().is_dir() {
                continue;
            }

            // We've found a file, hash it so we can track changes
            let bytes = std::fs::read(path).unwrap();
            let hash = sha256::digest_bytes(&bytes);

            let path = diff_paths(path, directory).unwrap();
            event!(Level::DEBUG, path = path.display().to_string(), hash);
            files.insert(path, hash);
        }

        Ok(Self {
            version: metadata.version,
            files,
        })
    }
}
