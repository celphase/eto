use std::path::Path;

use anyhow::Error;
use clap::Args;

pub fn command(command: ListCommand) -> Result<(), Error> {
    let package = Path::new(&command.package);
    let manifest = eto::package::read_package_manifest(package)?;

    println!("Add:");
    for path in &manifest.diff.add {
        println!("- {}", path.display());
    }
    println!();

    println!("Change:");
    for path in &manifest.diff.change {
        println!("- {}", path.display());
    }
    println!();

    println!("Delete:");
    for path in &manifest.diff.delete {
        println!("- {}", path.display());
    }
    println!();

    Ok(())
}

/// List the metadata and contents of a package.
#[derive(Args, Debug)]
pub struct ListCommand {
    /// Location of the package to list.
    #[arg(short, long)]
    package: String,
}
