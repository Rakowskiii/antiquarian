use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub use tracing::{debug, error, info, trace, warn};

pub fn init_tracing() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::filter::LevelFilter::INFO)
        .with(tracing_subscriber::fmt::layer().pretty())
        .init();
}
