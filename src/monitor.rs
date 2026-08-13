use crate::config;
use crate::external;
use crate::state::DailyMonitorState;
use anyhow::{anyhow, Result};

pub fn run_daily_monitor() -> Result<()> {
    let state_path = config::state_file_path();
    let mut state = DailyMonitorState::load_or_default(&state_path)?;

    let daily_usage = external::fetch_daily_usage_bytes()?;

    let mut changed = false;

    if daily_usage >= config::THRESHOLD_40G_BYTES && !state.warned_40g {
        notify_40g(daily_usage)?;
        state.warned_40g = true;
        state.warned_15g = true;
        changed = true;
    } else if daily_usage >= config::THRESHOLD_15G_BYTES && !state.warned_15g {
        notify_15g(daily_usage)?;
        state.warned_15g = true;
        changed = true;
    }

    if changed {
        state.save(&state_path)?;
    }

    Ok(())
}

fn notify_15g(bytes: u64) -> Result<()> {
    let webhook = config::dingtalk_webhook()
        .ok_or_else(|| anyhow!("DINGTALK_WEBHOOK is not set, cannot send 15G alert"))?;
    let message = format!(
        "{} 当日流量已超过 15G，当前: {}",
        config::DING_KEYWORD,
        format_bytes(bytes)
    );
    external::send_dingtalk_text(&webhook, &message)?;
    Ok(())
}

fn notify_40g(bytes: u64) -> Result<()> {
    let webhook = config::dingtalk_webhook()
        .ok_or_else(|| anyhow!("DINGTALK_WEBHOOK is not set, cannot send 40G alert"))?;
    let message = format!(
        "{} 当日流量已超过 40G，当前: {}",
        config::DING_KEYWORD,
        format_bytes(bytes)
    );
    external::send_dingtalk_text(&webhook, &message)?;
    Ok(())
}

fn format_bytes(bytes: u64) -> String {
    const GB: f64 = 1024.0 * 1024.0 * 1024.0;
    format!("{:.2} GB ({} Bytes)", bytes as f64 / GB, bytes)
}

#[cfg(test)]
mod tests {
    use super::format_bytes;

    #[test]
    fn format_bytes_for_one_gib() {
        let text = format_bytes(1024 * 1024 * 1024);
        assert_eq!(text, "1.00 GB (1073741824 Bytes)");
    }
}
