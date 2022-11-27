mod logging;

use std::path::Path;

use clap::{Args, Parser, Subcommand};
use eto::{patch_directory, Metadata};
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

    match args.command {
        Command::Package(command) => command_package(command),
        Command::Update => command_update(),
    }
}

fn command_package(command: PackageCommand) {
    let old = Path::new(&command.old);
    let new = Path::new(&command.new);
    let package = Path::new(&command.package);

    let result = eto::package_diff(old, new, package);

    if let Err(error) = result {
        event!(Level::ERROR, "failed:\n{:?}", error);
    } else {
        event!(Level::INFO, "successfully completed");
    }
}

fn command_update() {
    // Safety check TODO: cleanup
    let metadata = Metadata::from_dir("./");
    let metadata = match metadata {
        Ok(v) => v,
        Err(error) => {
            event!(Level::ERROR, "failed:\n{:?}", error);
            return;
        }
    };

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
