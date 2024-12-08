/// Initializes a tracing subscriber for debugging failed test cases.
#[cfg(test)]
pub fn initialize() {
    let _ = tracing_subscriber::fmt()
        .with_test_writer()
        .with_env_filter(
            tracing_subscriber::EnvFilter::builder()
                .with_default_directive(tracing::level_filters::LevelFilter::DEBUG.into())
                .from_env_lossy(),
        )
        .try_init();
}
