//! Application setup
use tracing_subscriber::EnvFilter;

use crate::core::settings::CONFIG;

pub fn init_tracing() {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().pretty())
        .with(EnvFilter::new(&CONFIG.log.level))
        .with(ErrorLayer::default())
        .init();
}
