use crate::db;
use crate::state::{AppState, ClipItem};
use axum::{
    extract::{Path as AxumPath, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateClipRequest {
    pub content: String,
    pub device_label: Option<String>,
}

pub async fn list_clips(
    State(app_state): State<AppState>,
) -> Result<Json<Vec<ClipItem>>, StatusCode> {
    let conn = app_state.db.lock().await;
    let clips = db::get_clips(&conn, 50).map_err(|e| {
        eprintln!("Database error fetching clips: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(clips))
}

pub async fn create_clip(
    State(app_state): State<AppState>,
    Json(payload): Json<CreateClipRequest>,
) -> Result<Json<ClipItem>, StatusCode> {
    let content = payload.content.trim().to_string();
    if content.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let id = Uuid::new_v4().to_string()[..8].to_string();
    let created_at = Utc::now().to_rfc3339();
    let device_label = payload
        .device_label
        .filter(|s| !s.trim().is_empty())
        .or_else(|| Some("Web Device".to_string()));

    let clip = ClipItem {
        id,
        content,
        device_label,
        created_at,
    };

    // Save to SQLite
    {
        let conn = app_state.db.lock().await;
        db::add_clip(&conn, &clip).map_err(|e| {
            eprintln!("Database error adding clip: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    }

    // Broadcast live event to all connected clients
    let event = json!({
        "type": "clip_added",
        "data": clip.clone()
    })
    .to_string();
    let _ = app_state.broadcast_tx.send(event);

    Ok(Json(clip))
}

pub async fn delete_clip(
    State(app_state): State<AppState>,
    AxumPath(id): AxumPath<String>,
) -> Result<StatusCode, StatusCode> {
    {
        let conn = app_state.db.lock().await;
        let deleted = db::delete_clip(&conn, &id).map_err(|e| {
            eprintln!("Database error deleting clip: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        if !deleted {
            return Err(StatusCode::NOT_FOUND);
        }
    }

    // Broadcast deletion event
    let event = json!({
        "type": "clip_removed",
        "data": { "id": id }
    })
    .to_string();
    let _ = app_state.broadcast_tx.send(event);

    Ok(StatusCode::OK)
}

