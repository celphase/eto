use std::{
    borrow::Cow,
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use anyhow::{bail, Context, Error};
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use serde::{Deserialize, Serialize};
use tar::Archive;
use tracing::{event, Level};

use crate::{diff::Diff, state::State, Metadata};

// TODO: The package API needs an overhaul, adding a low-level encoder/decoder abstraction.

pub fn create_package(a_path: &Path, b_path: &Path, package_path: &Path) -> Result<(), Error> {
    let a = State::read_dir(a_path).context("failed to read a state")?;
    let b = State::read_dir(b_path).context("failed to read b state")?;

    let diff = Diff::from_states(&a, &b);

    create_write_package(&PathBuf::from(b_path), diff, package_path)
        .context("failed to create package")?;

    Ok(())
}

fn create_write_package(b_path: &Path, diff: Diff, package_path: &Path) -> Result<(), Error> {
    event!(
        Level::INFO,
        path = package_path.display().to_string(),
        "generating package"
    );

    // Create the target package file to write to with header
    let mut file = File::create(package_path).context("failed to create package file")?;
    file.write_all(MAGIC.as_bytes())?;

    // Write content
    write_manifest(&mut file, &diff).context("failed to write manifest")?;
    write_files(b_path, &mut file, &diff).context("failed to write files")?;

    Ok(())
}

fn write_manifest(file: &mut File, diff: &Diff) -> Result<(), Error> {
    // Create the metadata manifest used when applying and for information
    let manifest = Manifest {
        version: "0.1.0".to_string(),
        diff: Cow::Borrowed(diff),
    };
    let manifest_json = serde_json::to_string(&manifest).unwrap();

    // Write the manifest to the package
    write_u32(file, manifest_json.as_bytes().len() as u32)?;
    file.write_all(manifest_json.as_bytes())?;

    Ok(())
}

fn write_files(b_path: &Path, file: &mut File, diff: &Diff) -> Result<(), Error> {
    // TODO: Write in-place rather than to memory.
    // To do this, we need to use the `seek` functions available on File.

    // Create the archive in-memory
    let mut archive = Vec::new();
    let encoder = GzEncoder::new(&mut archive, Compression::default());
    let mut tar = tar::Builder::new(encoder);

    // Write all files that are either new or changed, as we need their content
    for path in &diff.add {
        write_file(&mut tar, path, b_path)?;
    }
    for path in &diff.change {
        write_file(&mut tar, path, b_path)?;
    }

    // Write the generated archive to the file
    tar.finish()?;
    drop(tar);
    write_u32(file, archive.len() as u32)?;
    file.write_all(&archive)?;

    Ok(())
}

fn write_u32(file: &mut File, value: u32) -> Result<(), Error> {
    let length_bytes = value.to_le_bytes();
    file.write_all(&length_bytes)?;
    Ok(())
}

fn write_file<W: Write>(
    tar: &mut tar::Builder<W>,
    path: &PathBuf,
    b_path: &Path,
) -> Result<(), Error> {
    let mut source_path = b_path.to_path_buf();
    source_path.push(path);

    let mut file = File::open(source_path).context("unable to open diff file")?;
    tar.append_file(path, &mut file)?;

    Ok(())
}

pub fn apply_to_directory(package_path: &Path, directory_path: &Path) -> Result<(), Error> {
    // Check the directory is an eto directory and load its metadata
    let metadata =
        Metadata::from_dir(directory_path).context("directory is not an eto tracked directory")?;

    // Open the package
    let mut file = open_package_file(package_path)?;

    // Load the manifest from the package
    let manifest = read_manifest(&mut file)?;
    // Verify current version matches the manifest old version
    if manifest.diff.old_version != metadata.version {
        bail!(
            "package was made for version \"{}\", but current version is \"{}\"",
            manifest.diff.old_version,
            metadata.version
        );
    }

    // Apply changes from the manifest
    read_unpack_files(&mut file, directory_path).context("failed to decode gzip data block")?;
    for delete in &manifest.diff.delete {
        event!(Level::INFO, path = delete.display().to_string(), "delete");

        let mut target = package_path.to_owned();
        target.push(delete);
        let result = std::fs::remove_file(target);

        // We don't particularly care if this doesn't succeed, but if it doesn't at least log it
        if result.is_err() {
            event!(Level::WARN, "attempted to remove file that doesn't exist");
        }
    }

    Ok(())
}

pub fn read_package_manifest(package_path: &Path) -> Result<Manifest, Error> {
    let mut file = open_package_file(package_path)?;
    read_manifest(&mut file)
}

fn open_package_file(path: &Path) -> Result<File, Error> {
    event!(
        Level::INFO,
        path = path.display().to_string(),
        "loading package"
    );
    let mut file = File::open(path).context("unable to find package")?;

    // Verify the magic
    let mut magic_bytes = [0, 0, 0, 0, 0, 0, 0, 0];
    file.read_exact(&mut magic_bytes)?;
    let magic = std::str::from_utf8(&magic_bytes);
    if magic != Ok(MAGIC) {
        bail!("package magic number does not match, file is likely not a package or has an unsupported format version");
    }

    Ok(file)
}

fn read_manifest(file: &mut File) -> Result<Manifest<'static>, Error> {
    let bytes = read_u32(file)?;
    let reader = file.take(bytes as u64);
    let manifest: Manifest = serde_json::from_reader(reader).context("malformed manifest json")?;

    Ok(manifest)
}

fn read_unpack_files(file: &mut File, directory_path: &Path) -> Result<(), Error> {
    let bytes = read_u32(file)?;
    event!(Level::INFO, bytes, "reading archive");
    let reader = file.take(bytes as u64);
    let mut archive = Archive::new(GzDecoder::new(reader));

    for change in archive.entries()? {
        let mut change = change?;

        // Create the final path
        let path = change.path()?;
        let mut target_path = directory_path.to_path_buf();
        target_path.push(&path);

        // Create the parent directory if necessary
        if let Some(prefix) = target_path.parent() {
            std::fs::create_dir_all(prefix)?;
        }

        event!(Level::INFO, path = path.display().to_string(), "writing");
        change.unpack(&target_path)?;
    }

    Ok(())
}

fn read_u32(file: &mut File) -> Result<u32, Error> {
    let mut value_bytes = [0, 0, 0, 0];
    file.read_exact(&mut value_bytes)?;
    Ok(u32::from_le_bytes(value_bytes))
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Manifest<'a> {
    pub version: String,
    pub diff: Cow<'a, Diff>,
}

const MAGIC: &str = "EtoPack1";
