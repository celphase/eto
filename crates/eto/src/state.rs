use std::{collections::HashMap, path::PathBuf};

use pathdiff::diff_paths;
use tracing::{event, Level};
use walkdir::WalkDir;

#[derive(Default, Debug, Clone)]
pub struct State {
    pub files: HashMap<PathBuf, String>,
}

impl State {
    pub fn read_dir(directory: &str) -> Self {
        event!(Level::INFO, directory, "reading state");

        let mut files = HashMap::new();

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

        Self { files }
    }
}
