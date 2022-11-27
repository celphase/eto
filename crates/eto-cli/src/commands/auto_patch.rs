use std::path::Path;

use anyhow::{anyhow, Error};
use clap::Args;

pub fn command() -> Result<(), Error> {
    // Scan for a package.etopack
    let result = glob::glob("./*.etopack");
    let package = if let Some(result) = result
        .ok()
        .and_then(|mut paths| paths.next())
        .and_then(|result| result.ok())
    {
        result
    } else {
        return Err(anyhow!("couldn't find package"));
    };

    let directory = Path::new("./");
    eto::patch_directory(&package, directory)?;

    // Delete the package file since we're done successfully
    let _ = std::fs::remove_file(package);

    Ok(())
}

/// Automatically patch a working directory by finding and applying a package.
///
/// This command is intended to be used either from a script, or called by a program to update
/// itself.
/// If you are using this for self-updating, make sure you copy the eto binary to a temporary
/// location so it can update itself too.
#[derive(Args, Debug)]
pub struct AutoPatchCommand {}
