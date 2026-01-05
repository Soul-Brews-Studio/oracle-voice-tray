use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::Mutex;
use chrono::{DateTime, Utc};
use tauri::{tray::TrayIcon, image::Image};

/// Voice entry for timeline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceEntry {
    pub id: u64,
    pub timestamp: DateTime<Utc>,
    pub text: String,
    pub voice: String,
    pub rate: u32,
    pub agent: Option<String>,
    pub status: String, // "queued", "speaking", "done"
}

/// Request to speak
#[derive(Debug, Deserialize)]
pub struct SpeakRequest {
    pub text: String,
    pub voice: Option<String>,
    pub agent: Option<String>,
    pub rate: Option<u32>,
}

/// Response from speak endpoint
#[derive(Debug, Serialize)]
pub struct SpeakResponse {
    pub id: u64,
    pub status: String,
}

/// Shared application state
pub struct AppState {
    pub timeline: Mutex<VecDeque<VoiceEntry>>,
    pub next_id: Mutex<u64>,
    pub is_speaking: Mutex<bool>,
    pub mqtt_status: Mutex<String>,
    pub mqtt_reconnect: Mutex<bool>,
    pub tray_icon: Mutex<Option<TrayIcon>>,
    pub idle_icon: Mutex<Option<Image<'static>>>,
    pub speaking_icon: Mutex<Option<Image<'static>>>,
    pub disconnected_icon: Mutex<Option<Image<'static>>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            timeline: Mutex::new(VecDeque::with_capacity(100)),
            next_id: Mutex::new(1),
            is_speaking: Mutex::new(false),
            mqtt_status: Mutex::new("disconnected".to_string()),
            mqtt_reconnect: Mutex::new(false),
            tray_icon: Mutex::new(None),
            idle_icon: Mutex::new(None),
            speaking_icon: Mutex::new(None),
            disconnected_icon: Mutex::new(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_voice_entry_serialization() {
        let entry = VoiceEntry {
            id: 1,
            timestamp: Utc::now(),
            text: "Hello world".to_string(),
            voice: "Samantha".to_string(),
            rate: 200,
            agent: Some("test-agent".to_string()),
            status: "queued".to_string(),
        };

        let json = serde_json::to_string(&entry).expect("serialize");
        let parsed: VoiceEntry = serde_json::from_str(&json).expect("deserialize");

        assert_eq!(parsed.id, entry.id);
        assert_eq!(parsed.text, entry.text);
        assert_eq!(parsed.voice, entry.voice);
        assert_eq!(parsed.rate, entry.rate);
        assert_eq!(parsed.agent, entry.agent);
        assert_eq!(parsed.status, entry.status);
    }

    #[test]
    fn test_speak_request_deserialization() {
        let json = r#"{"text":"Hello"}"#;
        let req: SpeakRequest = serde_json::from_str(json).expect("deserialize");
        assert_eq!(req.text, "Hello");
        assert!(req.voice.is_none());
        assert!(req.agent.is_none());
        assert!(req.rate.is_none());

        let json = r#"{"text":"Test","voice":"Alex","agent":"my-agent","rate":150}"#;
        let req: SpeakRequest = serde_json::from_str(json).expect("deserialize");
        assert_eq!(req.text, "Test");
        assert_eq!(req.voice, Some("Alex".to_string()));
        assert_eq!(req.agent, Some("my-agent".to_string()));
        assert_eq!(req.rate, Some(150));
    }

    #[test]
    fn test_app_state_default() {
        let state = AppState::default();

        let timeline = state.timeline.lock().expect("lock");
        assert!(timeline.is_empty());
        drop(timeline);

        let next_id = state.next_id.lock().expect("lock");
        assert_eq!(*next_id, 1);
        drop(next_id);

        let is_speaking = state.is_speaking.lock().expect("lock");
        assert!(!*is_speaking);
        drop(is_speaking);

        let mqtt_status = state.mqtt_status.lock().expect("lock");
        assert_eq!(*mqtt_status, "disconnected");
    }

    #[test]
    fn test_timeline_capacity() {
        let state = AppState::default();

        {
            let mut timeline = state.timeline.lock().expect("lock");
            for i in 0..105 {
                timeline.push_back(VoiceEntry {
                    id: i,
                    timestamp: Utc::now(),
                    text: format!("Message {}", i),
                    voice: "Samantha".to_string(),
                    rate: 200,
                    agent: None,
                    status: "done".to_string(),
                });
                while timeline.len() > 100 {
                    timeline.pop_front();
                }
            }
            assert_eq!(timeline.len(), 100);
            assert_eq!(timeline.front().map(|e| e.id), Some(5));
        }
    }
}
