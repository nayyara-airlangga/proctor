use tracing::level_filters::LevelFilter;
use tracing_subscriber::{filter::Targets, layer::SubscriberExt, EnvFilter, Registry};

pub fn init_tracer() {
    let env_filter = EnvFilter::from_default_env();
    let level_filter = if cfg!(debug_assertions) {
        LevelFilter::DEBUG
    } else {
        LevelFilter::INFO
    };

    let targets = Targets::new().with_default(level_filter);
    let subscriber = Registry::default().with(env_filter).with(targets);

    let pretty_log = if cfg!(debug_assertions) {
        Some(tracing_subscriber::fmt::layer().pretty())
    } else {
        None
    };
    let subscriber = subscriber.with(pretty_log);

    let json_log = if !cfg!(debug_assertions) {
        Some(tracing_subscriber::fmt::layer().json())
    } else {
        None
    };
    let subscriber = subscriber.with(json_log);

    tracing::subscriber::set_global_default(subscriber).expect("Failed to enable tracing");
}
