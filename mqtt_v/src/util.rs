pub fn enable_tracing() {
    use tracing_subscriber::{fmt, EnvFilter};

    let format = fmt::format()
        .without_time()
        .with_target(false)
        .with_source_location(false)
        .compact();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .event_format(format)
        .init();
}
