use anyhow::{anyhow, Context, Result};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub const UFI_REQ_BIN: &str = "/data/data/com.minikano.f50_sms/files/ufi_req";
pub const JQ_BIN: &str = "/data/data/com.minikano.f50_sms/files/jq";
pub const CURL_BIN: &str = "/data/data/com.minikano.f50_sms/files/curl";

pub const DAILY_USAGE_API: &str = "/api/baseDeviceInfo";
pub const DING_KEYWORD: &str = "[流量提示]";

pub const THRESHOLD_15G_BYTES: u64 = 15 * 1024 * 1024 * 1024;
pub const THRESHOLD_40G_BYTES: u64 = 40 * 1024 * 1024 * 1024;

const ENV_FILE: &str = ".env";
const DEFAULT_ENV_CONTENT: &str = "# 钉钉机器人 Webhook（必填）\nDINGTALK_WEBHOOK=\n\n# 状态文件目录\nUFI_HELPER_STATE_DIR=./data\n\n# 轮询间隔（秒）\nMONITOR_INTERVAL_SECONDS=300\n\n# 日志文件\nUFI_HELPER_LOG_FILE=./ufi-helper.log\n";

pub fn ensure_and_load_env() -> Result<()> {
    let env_path = Path::new(ENV_FILE);
    if !env_path.exists() {
        fs::write(env_path, DEFAULT_ENV_CONTENT)
            .with_context(|| format!("failed to create {}", ENV_FILE))?;
        return Err(anyhow!(
            "{} created. Please fill DINGTALK_WEBHOOK, then run again.",
            ENV_FILE
        ));
    }

    dotenvy::from_filename(env_path)
        .with_context(|| format!("failed to load {}", ENV_FILE))?;
    Ok(())
}

pub fn state_file_path() -> PathBuf {
    if let Ok(dir) = env::var("UFI_HELPER_STATE_DIR") {
        return PathBuf::from(dir).join("daily_monitor_state.json");
    }

    PathBuf::from("./data").join("daily_monitor_state.json")
}

pub fn dingtalk_webhook() -> Option<String> {
    env::var("DINGTALK_WEBHOOK").ok().filter(|v| !v.trim().is_empty())
}

pub fn monitor_interval_seconds() -> u64 {
    env::var("MONITOR_INTERVAL_SECONDS")
        .ok()
        .and_then(|v| v.trim().parse::<u64>().ok())
        .filter(|v| *v > 0)
        .unwrap_or(300)
}

pub fn log_file_path() -> PathBuf {
    env::var("UFI_HELPER_LOG_FILE")
        .ok()
        .filter(|v| !v.trim().is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("./ufi-helper.log"))
}
