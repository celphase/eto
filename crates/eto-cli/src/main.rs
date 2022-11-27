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
    Package(PackageCommand),
    Update,
}

#[derive(Args, Debug)]
struct PackageCommand {
    #[arg(long)]
    old: String,

    #[arg(long)]
    new: String,

    #[arg(short, long)]
    package: String,
}

fn main() {
    let args = Cli::parse();

    logging::init();
    event!(Level::INFO, "running eto-packager");

    let result = match args.command {
        Command::Package(command) => command_package(command),
        Command::Update => command_update(),
    };

    if let Err(error) = result {
        event!(Level::ERROR, "failed:\n{:?}", error);
    } else {
        event!(Level::INFO, "successfully completed");
    }
}

fn command_package(command: PackageCommand) -> Result<(), Error> {
    let old = Path::new(&command.old);
    let new = Path::new(&command.new);
    let package = Path::new(&command.package);

    eto::package_diff(old, new, package)
}

fn command_update() -> Result<(), Error> {
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

    // Scan for a package.zip
    let result = glob::glob("./*.zip");
    let package = if let Some(result) = result
        .ok()
        .and_then(|mut paths| paths.next())
        .and_then(|result| result.ok())
    {
        result
    } else {
        return Err(anyhow!("couldn't find package zip"));
    };

    let directory = Path::new("./");
    eto::patch_directory(&package, directory)?;

    // Delete the package file since we're done successfully
    let _ = std::fs::remove_file(package);

    Ok(())
}
