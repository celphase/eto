mod diff;
mod state;

use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use anyhow::{Context, Error};
use serde::{Deserialize, Serialize};
use tracing::{event, Level};
use zip::{write::FileOptions, CompressionMethod, ZipWriter};

pub use crate::{diff::Diff, state::State};

pub fn package_diff(old_path: &Path, new_path: &Path, package_path: &Path) -> Result<(), Error> {
    let old = State::read_dir(old_path).context("failed to read old state")?;
    let new = State::read_dir(new_path).context("failed to read new state")?;

    let diff = Diff::from_states(&old, &new);

    generate_package(&PathBuf::from(new_path), diff, package_path);

    Ok(())
}

fn generate_package(new_path: &Path, diff: Diff, package_path: &Path) {
    event!(
        Level::INFO,
        path = package_path.display().to_string(),
        "generating package"
    );

    // Generate the metadata manifest used when applying and for information
    let manifest = Manifest { diff };
    let manifest_json = serde_json::to_string(&manifest).unwrap();

    // Create the target package
    let file = File::create(&package_path).unwrap();
    let mut zip = ZipWriter::new(file);
    let options = FileOptions::default().compression_method(CompressionMethod::Deflated);

    // Write the manifest to the package
    zip.start_file("eto-manifest.json", options).unwrap();
    zip.write_all(manifest_json.as_bytes()).unwrap();

    // Write all files that are either new or changed, as we need their content
    let mut buffer = Vec::new();
    for path in manifest.diff.new {
        write_file(&mut zip, &path, new_path, &mut buffer);
    }
    for path in manifest.diff.change {
        write_file(&mut zip, &path, new_path, &mut buffer);
    }

    zip.finish().unwrap();
}

fn write_file(zip: &mut ZipWriter<File>, path: &PathBuf, new_path: &Path, buffer: &mut Vec<u8>) {
    let options = FileOptions::default().compression_method(CompressionMethod::Deflated);
    zip.start_file(path.to_string_lossy(), options).unwrap();

    let mut source_path = new_path.to_path_buf();
    source_path.push(path);

    // Read the entire file
    let mut file = File::open(source_path).unwrap();
    file.read_to_end(buffer).unwrap();

    // Write the contents to the zip
    zip.write_all(buffer).unwrap();

    // Clean the buffer for future use
    buffer.clear();
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Manifest {
    pub diff: Diff,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metadata {
    pub version: String,
}

impl Metadata {
    pub fn from_dir(directory: &Path) -> Result<Self, Error> {
        let mut path = directory.to_path_buf();
        path.push("eto.json");

        let file = File::open(path).context("unable to open eto.json")?;
        let data = serde_json::from_reader(file)?;

        Ok(data)
    }
}
