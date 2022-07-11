use tracing::{error, info, warn};
use tracing_subscriber::{Layer, prelude::__tracing_subscriber_SubscriberExt};
use std::io;
fn main() {
    let stdout_log = tracing_subscriber::fmt::layer().pretty().with_writer(io::stdout);
    let stderr_log = tracing_subscriber::fmt::layer().pretty().with_writer(io::stderr).with_filter(tracing_subscriber::filter::LevelFilter::ERROR);

    let logger = tracing_subscriber::Registry::default()
        .with(stdout_log)
        .with(stderr_log);

    tracing::subscriber::set_global_default(logger).unwrap();

    info!("This is a test");
    warn!("This is a warning");
    error!("This is an error");
}
