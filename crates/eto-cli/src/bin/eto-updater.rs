use std::path::Path;

use eto::patch_directory;
use tracing::{event, Level};

fn main() {
    eto_cli::init();
    event!(Level::INFO, "running eto-updater");

    // TODO: Scan for a package.zip
    let package = Path::new("./package.zip");
    let directory = Path::new("./");
    let result = patch_directory(package, directory);

    if let Err(error) = result {
        event!(Level::ERROR, "failed:\n{:?}", error);
    } else {
        event!(Level::INFO, "successfully completed");
    }
}
