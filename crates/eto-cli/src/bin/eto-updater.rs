use std::path::Path;

use eto::{patch_directory, Metadata};
use sysinfo::{System, SystemExt};
use tracing::{event, Level};

fn main() {
    eto_cli::init();
    event!(Level::INFO, "running eto-updater");

    // Safety check TODO: cleanup
    let metadata = Metadata::from_dir("./").expect("eto.json is missing");
    let mut system = System::new();
    system.refresh_processes();
    for process in metadata.not_running {
        if system.processes_by_name(&process).count() != 0 {
            event!(
                Level::ERROR,
                "{} is currently running, close this before applying update",
                process
            );
        }
    }

    // Scan for a package.zip
    let result = glob::glob("./*.zip");
    let package = if let Some(result) = result
        .ok()
        .and_then(|mut paths| paths.next())
        .and_then(|result| result.ok())
    {
        result
    } else {
        event!(Level::ERROR, "couldn't find package zip");
        return;
    };

    let directory = Path::new("./");
    let result = patch_directory(&package, directory);

    if let Err(error) = result {
        event!(Level::ERROR, "failed:\n{:?}", error);
    } else {
        event!(Level::INFO, "successfully completed");

        // Delete the package file since we're done successfully
        let _ = std::fs::remove_file(package);
    }
}
