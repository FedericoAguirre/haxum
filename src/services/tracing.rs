use tracing_subscriber::fmt;

pub async fn init() {
    // Configure a custom event formatter
    let format = fmt::format()
        .with_level(true) // don't include levels in formatted output
        .with_target(true) // don't include targets
        .with_thread_ids(true) // include the thread ID of the current thread
        .with_thread_names(true) // include the name of the current thread
        .with_timer(fmt::time::ChronoLocal::rfc_3339()); // use RFC 3339 format for timestamps

    // Create a `fmt` subscriber that uses our custom event format, and set it
    // as the default.
    tracing_subscriber::fmt().event_format(format).init();
}
