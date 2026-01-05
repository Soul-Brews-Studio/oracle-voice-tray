use std::sync::Arc;
use std::time::Duration;
use std::process::Command;

use crate::state::AppState;

/// Update tray icon based on speaking state and MQTT connection
/// Uses a specific lock order to prevent deadlocks: mqtt_status -> icons -> tray_icon
pub fn update_tray_icon(state: &Arc<AppState>, speaking: bool) {
    let mqtt_status = match state.mqtt_status.lock() {
        Ok(guard) => guard.clone(),
        Err(_) => return,
    };

    let icon = if mqtt_status != "connected" {
        match state.disconnected_icon.lock() {
            Ok(guard) => guard.clone(),
            Err(_) => return,
        }
    } else if speaking {
        match state.speaking_icon.lock() {
            Ok(guard) => guard.clone(),
            Err(_) => return,
        }
    } else {
        match state.idle_icon.lock() {
            Ok(guard) => guard.clone(),
            Err(_) => return,
        }
    };

    if let Ok(tray_guard) = state.tray_icon.lock() {
        if let Some(ref tray) = *tray_guard {
            if let Some(img) = icon {
                let _ = tray.set_icon(Some(img));
            }
        }
    }
}

/// Speak text using macOS say command with rate
pub fn speak_text(text: &str, voice: &str, rate: u32) {
    let _ = Command::new("say")
        .args(["-v", voice, "-r", &rate.to_string(), text])
        .spawn()
        .and_then(|mut child| child.wait());
}

/// Process voice queue in a background thread
pub fn process_queue(state: Arc<AppState>) {
    std::thread::spawn(move || {
        loop {
            let entry_opt = {
                let Ok(mut timeline) = state.timeline.lock() else {
                    std::thread::sleep(Duration::from_millis(100));
                    continue;
                };
                if let Some(e) = timeline.iter_mut().find(|e| e.status == "queued") {
                    e.status = "speaking".to_string();
                    Some(e.clone())
                } else {
                    None
                }
            };

            if let Some(entry) = entry_opt {
                if let Ok(mut is_speaking) = state.is_speaking.lock() {
                    *is_speaking = true;
                }
                update_tray_icon(&state, true);

                speak_text(&entry.text, &entry.voice, entry.rate);

                if let Ok(mut timeline) = state.timeline.lock() {
                    if let Some(e) = timeline.iter_mut().find(|e| e.id == entry.id) {
                        e.status = "done".to_string();
                    }
                }
                if let Ok(mut is_speaking) = state.is_speaking.lock() {
                    *is_speaking = false;
                }
                update_tray_icon(&state, false);
            }

            std::thread::sleep(Duration::from_millis(100));
        }
    });
}
