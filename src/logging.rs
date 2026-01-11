use std::{env, path::PathBuf};

use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{EnvFilter, fmt};

fn resolve_log_dir() -> PathBuf {
    if let Ok(home) = env::var("HOME") {
        return PathBuf::from(home).join(".local/state/kestrel/log");
    }

    PathBuf::from("./logs")
}

pub fn init() -> WorkerGuard {
    let log_dir = resolve_log_dir();
    
    std::fs::create_dir_all(&log_dir).expect("cannot create log directory");

    let file_appender = tracing_appender::rolling::daily(
        log_dir,
        "kestrel.log",
    );
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("trace"))
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_target(false)
        .with_level(true)
        .with_timer(fmt::time::ChronoLocal::new("%Y-%m-%d %H:%M:%S".to_string()))
        .init();

    guard
}
