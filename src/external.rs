use crate::config;
use anyhow::{anyhow, Context, Result};
use std::io::Read;
use std::process::{Command, Stdio};

pub fn fetch_daily_usage_bytes() -> Result<u64> {
    let mut ufi_child = Command::new(config::UFI_REQ_BIN)
        .args(["-e", config::DAILY_USAGE_API])
        .stdout(Stdio::piped())
        .spawn()
        .with_context(|| format!("failed to start {}", config::UFI_REQ_BIN))?;

    let ufi_stdout = ufi_child
        .stdout
        .take()
        .ok_or_else(|| anyhow!("failed to capture ufi_req stdout"))?;

    let mut jq_child = Command::new(config::JQ_BIN)
        .arg(".daily_data")
        .stdin(Stdio::from(ufi_stdout))
        .stdout(Stdio::piped())
        .spawn()
        .with_context(|| format!("failed to start {}", config::JQ_BIN))?;

    let mut jq_output = String::new();
    jq_child
        .stdout
        .as_mut()
        .ok_or_else(|| anyhow!("failed to capture jq stdout"))?
        .read_to_string(&mut jq_output)
        .context("failed to read jq output")?;

    let jq_status = jq_child.wait().context("failed to wait jq process")?;
    let ufi_status = ufi_child.wait().context("failed to wait ufi_req process")?;

    if !ufi_status.success() {
        return Err(anyhow!("ufi_req exited with status: {}", ufi_status));
    }
    if !jq_status.success() {
        return Err(anyhow!("jq exited with status: {}", jq_status));
    }

    let trimmed = jq_output.trim();
    if trimmed.is_empty() || trimmed == "null" {
        return Err(anyhow!("daily_data is empty or null"));
    }

    trimmed
        .parse::<u64>()
        .with_context(|| format!("failed to parse daily_data as u64: {}", trimmed))
}

pub fn send_dingtalk_text(webhook: &str, content: &str) -> Result<()> {
    let payload = format!(
        r#"{{"msgtype":"text","text":{{"content":{}}}}}"#,
        serde_json::to_string(content)?
    );

    let status = Command::new(config::CURL_BIN)
        .args([
            "-sS",
            "-X",
            "POST",
            "-H",
            "Content-Type: application/json",
            "-d",
            &payload,
            webhook,
        ])
        .status()
        .with_context(|| format!("failed to start {}", config::CURL_BIN))?;

    if !status.success() {
        return Err(anyhow!("curl exited with status: {}", status));
    }

    Ok(())
}
