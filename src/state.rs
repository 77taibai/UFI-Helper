use anyhow::{Context, Result};
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyMonitorState {
    pub date: String,
    pub warned_15g: bool,
    pub warned_40g: bool,
}

impl Default for DailyMonitorState {
    fn default() -> Self {
        Self {
            date: today_local(),
            warned_15g: false,
            warned_40g: false,
        }
    }
}

impl DailyMonitorState {
    pub fn load_or_default(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }

        let raw = fs::read_to_string(path)
            .with_context(|| format!("failed to read state file: {}", path.display()))?;
        let mut state: Self = serde_json::from_str(&raw)
            .with_context(|| format!("failed to parse state file: {}", path.display()))?;

        if state.date != today_local() {
            state.date = today_local();
            state.warned_15g = false;
            state.warned_40g = false;
        }

        Ok(state)
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!("failed to create state directory: {}", parent.display())
            })?;
        }

        let data = serde_json::to_vec_pretty(self).context("failed to serialize state")?;
        fs::write(path, data)
            .with_context(|| format!("failed to write state file: {}", path.display()))?;

        Ok(())
    }
}

pub fn today_local() -> String {
    Local::now().format("%Y-%m-%d").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn load_resets_flags_when_day_changed() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("ufi_helper_state_{}.json", unique));

        let stale = DailyMonitorState {
            date: "2000-01-01".to_string(),
            warned_15g: true,
            warned_40g: true,
        };
        stale.save(&path).expect("save stale state");

        let loaded = DailyMonitorState::load_or_default(&path).expect("load state");
        assert_eq!(loaded.date, today_local());
        assert!(!loaded.warned_15g);
        assert!(!loaded.warned_40g);

        let _ = std::fs::remove_file(path);
    }
}
