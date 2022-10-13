mod diff;
mod state;

use std::{fs::File, io::Write};

use serde::{Deserialize, Serialize};
use tracing::{event, Level};
use zip::{write::FileOptions, CompressionMethod, ZipWriter};

pub use crate::{diff::Diff, state::State};

pub fn package_diff(old_path: &str, new_path: &str, package_path: &str) {
    let old = State::read_dir(old_path);
    let new = State::read_dir(new_path);

    let diff = Diff::from_states(&old, &new);

    generate_package(new_path, diff, package_path);
}

fn generate_package(new_path: &str, diff: Diff, package_path: &str) {
    event!(Level::INFO, package_path, "generating package");

    // Generate the metadata manifest used when applying and for information
    let manifest = Manifest { diff };
    let manifest = serde_json::to_string(&manifest).unwrap();

    // Create the target package
    let file = File::create(&package_path).unwrap();
    let mut zip = ZipWriter::new(file);
    let options = FileOptions::default().compression_method(CompressionMethod::Deflated);

    // Write the manifest to the package
    zip.start_file("eto-manifest.json", options).unwrap();
    zip.write_all(manifest.as_bytes()).unwrap();

    zip.finish().unwrap();
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Manifest {
    diff: Diff,
}
