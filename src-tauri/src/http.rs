use axum::{
    routing::{get, post},
    Json, Router,
    extract::State,
};
use std::sync::Arc;
use chrono::Utc;

use crate::config::load_mqtt_config;
use crate::state::{AppState, VoiceEntry, SpeakRequest, SpeakResponse};

/// HTTP server port
pub const VOICE_SERVER_PORT: u16 = 37779;

/// Start HTTP server for receiving voice requests
pub async fn start_http_server(state: Arc<AppState>) {
    let app = Router::new()
        .route("/", get(|| async {
            axum::response::Html(r#"<!DOCTYPE html>
<html><head><title>Voice Tray API</title>
<style>body{font-family:system-ui;max-width:600px;margin:40px auto;padding:20px;background:#1a1a2e;color:#eee}
h1{color:#0f9}code{background:#333;padding:2px 6px;border-radius:4px}
pre{background:#222;padding:15px;border-radius:8px;overflow-x:auto}</style></head>
<body><h1>🎙️ Voice Tray API</h1>
<p>Endpoints:</p>
<ul>
<li><code>POST /speak</code> - Queue text for speech</li>
<li><code>GET /timeline</code> - Get speech queue</li>
<li><code>GET /status</code> - Get server status</li>
</ul>
<h3>Example:</h3>
<pre>curl -X POST http://127.0.0.1:37779/speak \
  -H "Content-Type: application/json" \
  -d '{"text":"Hello!","voice":"Samantha"}'</pre>
</body></html>"#)
        }))
        .route("/speak", post(|State(state): State<Arc<AppState>>, Json(req): Json<SpeakRequest>| async move {
            let id = state.next_id.lock()
                .map(|mut next_id| {
                    let id = *next_id;
                    *next_id += 1;
                    id
                })
                .unwrap_or(0);

            let voice = req.voice.unwrap_or_else(|| "Samantha".to_string());
            let rate = req.rate.unwrap_or(220);

            let entry = VoiceEntry {
                id,
                timestamp: Utc::now(),
                text: req.text,
                voice: voice.clone(),
                rate,
                agent: req.agent,
                status: "queued".to_string(),
            };

            if let Ok(mut timeline) = state.timeline.lock() {
                timeline.push_back(entry);
                while timeline.len() > 100 {
                    timeline.pop_front();
                }
            }

            Json(SpeakResponse { id, status: "queued".to_string() })
        }))
        .route("/timeline", get(|State(state): State<Arc<AppState>>| async move {
            let entries = state.timeline.lock()
                .map(|t| t.iter().cloned().collect::<Vec<_>>())
                .unwrap_or_default();
            Json(entries)
        }))
        .route("/status", get(|State(state): State<Arc<AppState>>| async move {
            let (total, queued) = state.timeline.lock()
                .map(|t| (t.len(), t.iter().filter(|e| e.status == "queued").count()))
                .unwrap_or((0, 0));
            let is_speaking = state.is_speaking.lock().map(|g| *g).unwrap_or(false);
            let mqtt_status = state.mqtt_status.lock()
                .map(|g| g.clone())
                .unwrap_or_else(|_| "unknown".to_string());
            let config = load_mqtt_config();
            Json(serde_json::json!({
                "total": total,
                "queued": queued,
                "is_speaking": is_speaking,
                "mqtt_status": mqtt_status,
                "mqtt_broker": format!("{}:{}", config.broker, config.port)
            }))
        }))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", VOICE_SERVER_PORT))
        .await
        .expect("Failed to bind HTTP server");

    println!("Voice HTTP server listening on http://127.0.0.1:{}", VOICE_SERVER_PORT);
    axum::serve(listener, app).await.unwrap();
}
