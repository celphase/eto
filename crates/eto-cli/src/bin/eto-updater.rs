use tracing::{event, Level};

fn main() {
    eto_cli::init();
    event!(Level::INFO, "running eto-updater");

    event!(Level::INFO, "successfully completed");
}
