use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, Runtime};

const MAX_EVENTS: usize = 50;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum EventKind {
    Scan,
    Cleanup,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum EventStatus {
    Pending,
    Success,
    Error,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ActivityEvent {
    pub id: String,
    pub kind: EventKind,
    pub status: EventStatus,
    pub at: String,
    pub bytes: u64,
}

pub fn load<R: Runtime>(app: &AppHandle<R>) -> Vec<ActivityEvent> {
    path(app).map(|p| load_from(&p)).unwrap_or_default()
}

pub fn save<R: Runtime>(app: &AppHandle<R>, events: &[ActivityEvent]) -> Result<(), String> {
    let Some(path) = path(app) else {
        return Ok(());
    };
    save_to(&path, events)
}

fn load_from(path: &Path) -> Vec<ActivityEvent> {
    fs::read_to_string(path)
        .ok()
        .and_then(|raw| serde_json::from_str(&raw).ok())
        .unwrap_or_default()
}

fn save_to(path: &Path, events: &[ActivityEvent]) -> Result<(), String> {
    let mut events = events.to_vec();
    if events.len() > MAX_EVENTS {
        events.drain(..events.len() - MAX_EVENTS);
    }

    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir).map_err(|err| err.to_string())?;
    }
    let json = serde_json::to_string_pretty(&events).map_err(|err| err.to_string())?;
    fs::write(path, json).map_err(|err| err.to_string())
}

fn path<R: Runtime>(app: &AppHandle<R>) -> Option<PathBuf> {
    app.path().app_data_dir().ok().map(|dir| dir.join("history.json"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn history_path(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("cachebin-history-{name}-{}.json", std::process::id()))
    }

    fn sample_event(kind: EventKind, status: EventStatus, bytes: u64) -> ActivityEvent {
        ActivityEvent {
            id: format!("{kind:?}-{status:?}-{bytes}"),
            kind,
            status,
            at: crate::time::now_rfc3339(),
            bytes,
        }
    }

    #[test]
    fn save_then_load_round_trips_through_disk() {
        let path = history_path("roundtrip");
        let events = vec![
            sample_event(EventKind::Scan, EventStatus::Success, 1024),
            sample_event(EventKind::Cleanup, EventStatus::Success, 512),
        ];

        save_to(&path, &events).expect("save");
        let back = load_from(&path);

        assert_eq!(back.len(), 2);
        assert_eq!(back[0].kind, EventKind::Scan);
        assert_eq!(back[0].status, EventStatus::Success);
        assert_eq!(back[0].bytes, 1024);
        assert_eq!(back[1].kind, EventKind::Cleanup);
        assert_eq!(back[1].bytes, 512);

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn event_status_serializes_lowercase() {
        assert_eq!(serde_json::to_string(&EventStatus::Pending).unwrap(), r#""pending""#);
        assert_eq!(serde_json::to_string(&EventStatus::Success).unwrap(), r#""success""#);
        assert_eq!(serde_json::to_string(&EventStatus::Error).unwrap(), r#""error""#);
        assert_eq!(serde_json::from_str::<EventStatus>(r#""pending""#).unwrap(), EventStatus::Pending);
        assert_eq!(serde_json::from_str::<EventStatus>(r#""error""#).unwrap(), EventStatus::Error);
    }

    #[test]
    fn load_from_missing_file_returns_empty() {
        let path = history_path("missing");
        let _ = std::fs::remove_file(&path);
        assert!(load_from(&path).is_empty());
    }

    #[test]
    fn save_caps_events_at_max() {
        let path = history_path("cap");
        let events = (0..(MAX_EVENTS + 10))
            .map(|index| sample_event(EventKind::Scan, EventStatus::Pending, index as u64))
            .collect::<Vec<_>>();

        save_to(&path, &events).expect("save");

        let capped = load_from(&path);
        assert_eq!(capped.len(), MAX_EVENTS);
        // Oldest events are dropped first.
        assert_eq!(capped[0].bytes, 10);

        let _ = std::fs::remove_file(&path);
    }
}
