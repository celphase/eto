use std::path::Path;

use anyhow::Error;
use clap::Args;

pub fn command(command: PackageCommand) -> Result<(), Error> {
    let a = Path::new(&command.a);
    let b = Path::new(&command.b);
    let package = Path::new(&command.output);

    eto::package_diff(a, b, package)
}

#[derive(Args, Debug)]
pub struct PackageCommand {
    /// Path to a directory containing state A.
    #[arg(short)]
    a: String,

    /// Path to a directory containing state B.
    #[arg(short)]
    b: String,

    /// Output path for the package, including the executable.
    /// Recommended to use a .etopack extension.
    #[arg(short, long)]
    output: String,
}
