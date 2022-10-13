use std::path::Path;

use clap::Parser;
use tracing::{event, Level};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    old: String,

    #[arg(long)]
    new: String,

    #[arg(short, long)]
    package: String,
}

fn main() {
    let args = Args::parse();
    eto_cli::init();
    event!(Level::INFO, "running eto-packager");

    let old = Path::new(&args.old);
    let new = Path::new(&args.new);
    let package = Path::new(&args.package);

    let result = eto::package_diff(old, new, package);

    if let Err(error) = result {
        event!(Level::ERROR, "failed:\n{:?}", error);
    } else {
        event!(Level::INFO, "successfully completed");
    }
}
