use std::{collections::HashMap, path::PathBuf};

use tracing::{event, Level};
use walkdir::WalkDir;

pub fn package_diff(old: &str, new: &str, output: &str) {
    let old_state = read_state(old);
    let new_state = read_state(new);
}

fn read_state(path: &str) -> State {
    event!(Level::INFO, path, "reading state");

    let mut files = HashMap::new();

    for entry in WalkDir::new(path) {
        let entry = entry.unwrap();
        let path = entry.path();

        if entry.path().is_dir() {
            continue;
        }

        // We've found a file, hash it so we can track changes
        let bytes = std::fs::read(path).unwrap();
        let hash = sha256::digest_bytes(&bytes);

        event!(Level::DEBUG, path = path.display().to_string(), hash);
        files.insert(path.to_owned(), hash);
    }

    State { files }
}

struct State {
    files: HashMap<PathBuf, String>,
}
