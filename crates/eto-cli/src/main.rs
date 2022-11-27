mod commands;
mod logging;

use clap::{Parser, Subcommand};
use tracing::{event, Level};

use crate::commands::{auto_patch::AutoPatchCommand, package::PackageCommand};

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
    AutoPatch(AutoPatchCommand),
}

fn main() {
    let args = Cli::parse();

    logging::init();
    event!(Level::INFO, "running eto command line tool");

    // Run the specific given command
    let result = match args.command {
        Command::Package(command) => commands::package::command(command),
        Command::AutoPatch(_) => commands::auto_patch::command(),
    };

    // Log result
    if let Err(error) = result {
        event!(Level::ERROR, "failed:\n{:?}", error);
    } else {
        event!(Level::INFO, "successfully completed");
    }
}
