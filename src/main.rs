mod logging;

fn main() {
    let _log_guard = logging::init();
    tracing::info!(version = env!("CARGO_PKG_VERSION"), "KestrelQueue running,");
}
