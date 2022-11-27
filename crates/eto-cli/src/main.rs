mod logging;

use std::path::Path;

use anyhow::{anyhow, Error};
use clap::{Args, Parser, Subcommand};
use eto::Metadata;
use sysinfo::{System, SystemExt};
use tracing::{event, Level};

#[derive(Parser, Debug)]
#[command(name = "cli", author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Create a package from A and B state directories.
    Package(PackageCommand),
    /// Automatically update the current working directory by finding and applying a package.
    AutoUpdate,
}

#[derive(Args, Debug)]
struct PackageCommand {
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

fn main() {
    let args = Cli::parse();

    logging::init();
    event!(Level::INFO, "running eto command line tool");

    // Run the specific given command
    let result = match args.command {
        Command::Package(command) => command_package(command),
        Command::AutoUpdate => command_auto_update(),
    };

    // Log result
    if let Err(error) = result {
        event!(Level::ERROR, "failed:\n{:?}", error);
    } else {
        event!(Level::INFO, "successfully completed");
    }
}

fn command_package(command: PackageCommand) -> Result<(), Error> {
    let a = Path::new(&command.a);
    let b = Path::new(&command.b);
    let package = Path::new(&command.output);

    eto::package_diff(a, b, package)
}

fn command_auto_update() -> Result<(), Error> {
    // Safety check TODO: cleanup
    let metadata = Metadata::from_dir("./")?;

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
