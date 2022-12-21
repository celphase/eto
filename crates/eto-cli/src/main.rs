mod commands;
mod logging;

use clap::{Parser, Subcommand};
use tracing::{event, Level};

use crate::commands::{list::ListCommand, package::PackageCommand, apply::ApplyCommand};

fn main() {
    let args = Cli::parse();

    logging::init();
    event!(Level::INFO, "running eto command line tool");

    // Run the specific given command
    let result = match args.command {
        Command::List(command) => commands::list::command(command),
        Command::Package(command) => commands::package::command(command),
        Command::Apply(command) => commands::apply::command(command),
    };

    // Log result
    if let Err(error) = result {
        event!(Level::ERROR, "failed:\n{:?}", error);
        std::process::exit(1);
    }

    event!(Level::INFO, "successfully completed");
}

#[derive(Parser, Debug)]
#[command(name = "cli", author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    List(ListCommand),
    Package(PackageCommand),
    Apply(ApplyCommand),
}
