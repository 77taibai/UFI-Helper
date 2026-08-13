mod config;
mod external;
mod monitor;
mod state;

use anyhow::{Context, Result};
use chrono::Local;
use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

const DAEMON_FLAG: &str = "UFI_HELPER_DAEMONIZED";

fn main() -> Result<()> {
    config::ensure_and_load_env()?;

    if env::var(DAEMON_FLAG).ok().as_deref() != Some("1") {
        spawn_daemon_process()?;
        return Ok(());
    }

    run_daemon_loop();
    Ok(())
}

fn spawn_daemon_process() -> Result<()> {
    let exe = env::current_exe().context("failed to resolve current executable")?;

    Command::new(exe)
        .env(DAEMON_FLAG, "1")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .context("failed to spawn daemon process")?;

    Ok(())
}

fn run_daemon_loop() {
    let interval = Duration::from_secs(config::monitor_interval_seconds());

    loop {
        match monitor::run_daily_monitor() {
            Ok(()) => log_line("monitor cycle completed"),
            Err(err) => log_line(&format!("monitor cycle failed: {err}")),
        }
        thread::sleep(interval);
    }
}

fn log_line(message: &str) {
    let path = config::log_file_path();
    if let Some(parent) = Path::new(&path).parent() {
        let _ = fs::create_dir_all(parent);
    }

    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&path) {
        let _ = writeln!(
            file,
            "{} {}",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            message
        );
    }
}
