# UFI-Helper

Rust binary tool for Android-side helper tasks.

## Implemented Feature

- Daily traffic monitor
- Data source: `com.minikano.f50_sms/files/ufi_req -e /api/baseDeviceInfo`
- Parsing tool: `com.minikano.f50_sms/files/jq '.daily_data'`
- Alert channels: DingTalk robot webhook via `com.minikano.f50_sms/files/curl`
- Thresholds:
  - 15G alert once per day
  - 40G alert once per day
- Daily reset at local midnight (by date change)

## Runtime Config

All runtime variables are read from `.env` in the working directory.

If `.env` does not exist, the program creates it automatically and exits once.

Required/used variables:

- `DINGTALK_WEBHOOK`: DingTalk robot webhook URL (required when alert is triggered)
- `UFI_HELPER_STATE_DIR`: monitor state directory (default template: `./data`)
- `MONITOR_INTERVAL_SECONDS`: monitor loop interval seconds (default template: `300`)
- `UFI_HELPER_LOG_FILE`: log file path (default template: `./ufi-helper.log`)

## Build

```bash
cargo build --release
```

The binary will be at `target/release/ufi-helper`.

## Run

Single command only:

```bash
./ufi-helper
```

Behavior:

- Starts and immediately returns control to shell.
- Child process runs as long-term background daemon.
- Daemon loops forever and executes monitor checks by `MONITOR_INTERVAL_SECONDS`.
- Daily alert dedup is persisted in `daily_monitor_state.json` and resets at date change.
