pub struct Log;

impl Log {
    pub fn new() -> Self {
        tracing_subscriber::fmt()
            .with_timer(tracing_subscriber::fmt::time::uptime())
            .with_max_level(tracing::Level::INFO)
            .with_target(true)
            .with_span_events(
                tracing_subscriber::fmt::format::FmtSpan::ENTER
                    | tracing_subscriber::fmt::format::FmtSpan::CLOSE,
            )
            .compact()
            .init();

        Self
    }
}

impl Default for Log {
    fn default() -> Self {
        Self::new()
    }
}
