use std::path::Path;

use anyhow::Error;
use clap::Args;

pub fn command(command: PackageCommand) -> Result<(), Error> {
    let a = Path::new(&command.a);
    let b = Path::new(&command.b);
    let package = Path::new(&command.package);

    eto::package::create_package(a, b, package)
}

/// Create a patch package from A to B state directories.
#[derive(Args, Debug)]
pub struct PackageCommand {
    /// Path to a directory containing state A.
    #[arg(short, value_name = "PATH")]
    a: String,

    /// Path to a directory containing state B.
    #[arg(short, value_name = "PATH")]
    b: String,

    /// Output path for the package, including the extension.
    /// Recommended to use a .etopack extension.
    #[arg(short, long, value_name = "PATH")]
    package: String,
}
