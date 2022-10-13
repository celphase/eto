mod diff;
mod package;
mod state;

use std::{fs::File, path::Path};

use anyhow::{Context, Error};
use serde::{Deserialize, Serialize};

pub use crate::{
    diff::Diff,
    package::{package_diff, patch_directory, Manifest},
    state::State,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metadata {
    pub version: String,
    pub ignore: Vec<String>,
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
