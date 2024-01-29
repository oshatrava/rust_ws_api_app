use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt, Layer};

pub fn setup_tracing() {
    let mut layers = Vec::new();

    if true {
        let stdout_log = tracing_subscriber::fmt::layer()
            .pretty()
            .with_filter(filter::LevelFilter::DEBUG)
            .boxed();
        layers.push(stdout_log);
    }

    tracing_subscriber::registry()
        .with(layers)
        .init();
}