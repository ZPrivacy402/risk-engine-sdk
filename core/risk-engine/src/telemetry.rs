use tracing::info;

pub fn init_telemetry() {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_thread_ids(true)
        .with_level(true)
        .init();

    info!("Telemetry initialized");
}
