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

    eto::package_diff(&args.old, &args.new, &args.package);

    event!(Level::INFO, "successfully completed");
}
