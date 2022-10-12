use tracing::{event, Level};
use tracing_subscriber::{
    prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter,
};

pub fn init() {
    // Log if something goes wrong so we actually see it in the log file
    std::panic::set_hook(Box::new(panic_hook));

    // Create the file logger
    let file_appender = tracing_appender::rolling::never("./", "eto.log");
    let file_subscriber = tracing_subscriber::fmt::layer()
        .with_writer(file_appender)
        .with_ansi(false);

    tracing_subscriber::registry()
        .with(EnvFilter::new("info"))
        .with(file_subscriber)
        .with(tracing_subscriber::fmt::layer())
        .init()
}

fn panic_hook(panic_info: &std::panic::PanicInfo) {
    event!(Level::ERROR, "eto... bleh!");

    if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
        event!(Level::ERROR, "panic occurred: {s:?}");
    } else {
        event!(Level::ERROR, "panic occurred");
    }

    if let Some(location) = panic_info.location() {
        event!(
            Level::ERROR,
            "panic occurred in file '{}' at line {}",
            location.file(),
            location.line(),
        );
    } else {
        event!(
            Level::ERROR,
            "panic occurred but can't get location information..."
        );
    }
}
