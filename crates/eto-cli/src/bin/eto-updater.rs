use tracing::{event, Level};

fn main() {
    eto_cli::init();

    event!(Level::INFO, "running eto-updater");

    let testing: Option<String> = None;
    testing.unwrap();

    event!(Level::INFO, "successfully completed");
}
