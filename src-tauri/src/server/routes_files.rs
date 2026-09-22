use crate::state::{AppState, FileItem};
use axum::{
    body::Body,
    extract::{Multipart, Query, State},
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::json;
use std::fs;
use std::path::{Path, PathBuf};
use tokio_util::io::ReaderStream;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct ListFilesQuery {
    pub path: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FileActionQuery {
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct MkdirRequest {
    pub path: String,
}

pub fn safe_join(base: &Path, rel: &str) -> Result<PathBuf, StatusCode> {
    let clean_rel = rel.trim_start_matches('/').trim_start_matches('\\');
    let mut target = base.to_path_buf();
    for component in Path::new(clean_rel).components() {
        match component {
            std::path::Component::Normal(c) => target.push(c),
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                return Err(StatusCode::FORBIDDEN);
            }
            std::path::Component::RootDir | std::path::Component::Prefix(_) => {
                return Err(StatusCode::FORBIDDEN);
            }
        }
    }

    if !base.exists() {
        let _ = fs::create_dir_all(base);
    }

    if let (Ok(canon_base), Ok(canon_target)) = (base.canonicalize(), target.canonicalize()) {
        if !canon_target.starts_with(&canon_base) {
            return Err(StatusCode::FORBIDDEN);
        }
    }

    Ok(target)
}

pub async fn list_files(
    State(app_state): State<AppState>,
    Query(params): Query<ListFilesQuery>,
) -> Result<Json<Vec<FileItem>>, StatusCode> {
    let inner = app_state.inner.lock().await;
    let base_folder = inner.shared_folder.clone();
    drop(inner);

    let subpath = params.path.unwrap_or_default();
    let target_dir = safe_join(&base_folder, &subpath)?;

    if !target_dir.exists() {
        let _ = fs::create_dir_all(&target_dir);
    }

    let entries = fs::read_dir(&target_dir).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut items = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        let metadata = match entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };

        let is_dir = metadata.is_dir();
        let size_bytes = if is_dir { 0 } else { metadata.len() };
        let name = entry.file_name().to_string_lossy().to_string();

        let rel_path = path
            .strip_prefix(&base_folder)
            .unwrap_or(&path)
            .to_string_lossy()
            .to_string();

        let modified_at = metadata.modified().ok().map(|st| {
            let dt: DateTime<Utc> = st.into();
            dt.to_rfc3339()
        });

        let mime_type = if is_dir {
            None
        } else {
            mime_guess::from_path(&path)
                .first()
                .map(|m| m.to_string())
        };

        items.push(FileItem {
            name,
            path: rel_path,
            is_dir,
            size_bytes,
            modified_at,
            mime_type,
        });
    }

    items.sort_by(|a, b| {
        match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        }
    });

    Ok(Json(items))
}

pub async fn upload_files(
    State(app_state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<Vec<FileItem>>, StatusCode> {
    let inner = app_state.inner.lock().await;
    let base_folder = inner.shared_folder.clone();
    drop(inner);

    let mut dest_subpath = String::new();
    let mut uploaded_items = Vec::new();

    while let Ok(Some(mut field)) = multipart.next_field().await {
        let field_name = field.name().unwrap_or_default().to_string();

        if field_name == "path" {
            if let Ok(text) = field.text().await {
                dest_subpath = text;
            }
            continue;
        }

        let file_name = field
            .file_name()
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("upload_{}", Uuid::new_v4().to_string()[..8].to_string()));

        let target_dir = safe_join(&base_folder, &dest_subpath)?;
        let target_file_path = safe_join(&target_dir, &file_name)?;

        if let Some(parent) = target_file_path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        let mut file = tokio::fs::File::create(&target_file_path)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        use tokio::io::AsyncWriteExt;
        while let Ok(Some(chunk)) = field.chunk().await {
            file.write_all(&chunk)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        }
        file.flush().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let metadata = fs::metadata(&target_file_path).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let size_bytes = metadata.len();
        let rel_path = target_file_path
            .strip_prefix(&base_folder)
            .unwrap_or(&target_file_path)
            .to_string_lossy()
            .to_string();

        let modified_at = metadata.modified().ok().map(|st| {
            let dt: DateTime<Utc> = st.into();
            dt.to_rfc3339()
        });

        let mime_type = mime_guess::from_path(&target_file_path)
            .first()
            .map(|m| m.to_string());

        let item = FileItem {
            name: file_name,
            path: rel_path,
            is_dir: false,
            size_bytes,
            modified_at,
            mime_type,
        };

        // Broadcast file_added event
        let event = json!({
            "type": "file_added",
            "data": item.clone()
        })
        .to_string();
        let _ = app_state.broadcast_tx.send(event);

        uploaded_items.push(item);
    }

    Ok(Json(uploaded_items))
}

pub async fn download_file(
    State(app_state): State<AppState>,
    Query(params): Query<FileActionQuery>,
) -> Result<Response, StatusCode> {
    let inner = app_state.inner.lock().await;
    let base_folder = inner.shared_folder.clone();
    drop(inner);

    let target_file = safe_join(&base_folder, &params.path)?;
    if !target_file.exists() || target_file.is_dir() {
        return Err(StatusCode::NOT_FOUND);
    }

    let file_name = target_file
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "download".to_string());

    let metadata = fs::metadata(&target_file).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let size_bytes = metadata.len();
    let mime = mime_guess::from_path(&target_file)
        .first_or_octet_stream()
        .to_string();

    let file = tokio::fs::File::open(&target_file)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    let encoded_filename = urlencoding::encode(&file_name);
    let content_disposition = format!(
        "attachment; filename=\"{}\"; filename*=UTF-8''{}",
        file_name, encoded_filename
    );

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        mime.parse().unwrap_or(header::HeaderValue::from_static("application/octet-stream")),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        content_disposition.parse().unwrap_or(header::HeaderValue::from_static("attachment")),
    );
    headers.insert(header::CONTENT_LENGTH, size_bytes.to_string().parse().unwrap());

    Ok((headers, body).into_response())
}

pub async fn make_directory(
    State(app_state): State<AppState>,
    Json(payload): Json<MkdirRequest>,
) -> Result<StatusCode, StatusCode> {
    let inner = app_state.inner.lock().await;
    let base_folder = inner.shared_folder.clone();
    drop(inner);

    let target_dir = safe_join(&base_folder, &payload.path)?;
    if target_dir.exists() {
        return Ok(StatusCode::OK);
    }

    fs::create_dir_all(&target_dir).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let folder_name = target_dir
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();

    let rel_path = target_dir
        .strip_prefix(&base_folder)
        .unwrap_or(&target_dir)
        .to_string_lossy()
        .to_string();

    // Broadcast folder_created event
    let event = json!({
        "type": "file_added",
        "data": {
            "name": folder_name,
            "path": rel_path,
            "is_dir": true,
            "size_bytes": 0,
            "modified_at": Utc::now().to_rfc3339(),
            "mime_type": None::<String>
        }
    })
    .to_string();
    let _ = app_state.broadcast_tx.send(event);

    Ok(StatusCode::CREATED)
}

pub async fn delete_file(
    State(app_state): State<AppState>,
    Query(params): Query<FileActionQuery>,
) -> Result<StatusCode, StatusCode> {
    let inner = app_state.inner.lock().await;
    let base_folder = inner.shared_folder.clone();
    drop(inner);

    let target = safe_join(&base_folder, &params.path)?;
    if !target.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    if target.is_dir() {
        fs::remove_dir_all(&target).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    } else {
        fs::remove_file(&target).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    // Broadcast file_removed event
    let event = json!({
        "type": "file_removed",
        "data": { "path": params.path }
    })
    .to_string();
    let _ = app_state.broadcast_tx.send(event);

    Ok(StatusCode::OK)
}

const MAX_TEXT_BYTES: u64 = 1024 * 1024;

#[derive(Deserialize)]
pub struct EditTextRequest {
    pub path: String,
    pub content: String,
    pub original: String,
}

fn read_editable_text(path: &Path) -> Result<String, StatusCode> {
    let metadata = fs::metadata(path).map_err(|_| StatusCode::NOT_FOUND)?;
    if !metadata.is_file() { return Err(StatusCode::BAD_REQUEST); }
    if metadata.len() > MAX_TEXT_BYTES { return Err(StatusCode::PAYLOAD_TOO_LARGE); }
    use std::io::Read;
    let mut bytes = Vec::new();
    fs::File::open(path).map_err(|_| StatusCode::FORBIDDEN)?
        .take(MAX_TEXT_BYTES + 1).read_to_end(&mut bytes)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if bytes.len() as u64 > MAX_TEXT_BYTES { return Err(StatusCode::PAYLOAD_TOO_LARGE); }
    let text = String::from_utf8(bytes).map_err(|_| StatusCode::UNSUPPORTED_MEDIA_TYPE)?;
    if text.contains('\0') { return Err(StatusCode::UNSUPPORTED_MEDIA_TYPE); }
    Ok(text)
}

pub async fn read_text(
    State(state): State<AppState>, Query(params): Query<FileActionQuery>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let inner = state.inner.lock().await;
    let path = safe_join(&inner.shared_folder, &params.path)?;
    Ok(Json(json!({ "content": read_editable_text(&path)? })))
}

fn replace_text(base: &Path, payload: &EditTextRequest) -> Result<(), StatusCode> {
    if payload.content.len() as u64 > MAX_TEXT_BYTES { return Err(StatusCode::PAYLOAD_TOO_LARGE); }
    if payload.content.contains('\0') { return Err(StatusCode::UNSUPPORTED_MEDIA_TYPE); }
    let path = safe_join(base, &payload.path)?.canonicalize().map_err(|_| StatusCode::NOT_FOUND)?;
    if !path.starts_with(base.canonicalize().map_err(|_| StatusCode::NOT_FOUND)?) {
        return Err(StatusCode::FORBIDDEN);
    }
    if read_editable_text(&path)? != payload.original { return Err(StatusCode::CONFLICT); }
    let temp = path.with_file_name(format!(".relay-edit-{}", Uuid::new_v4()));
    let result = (|| -> std::io::Result<()> {
        use std::io::Write;
        let mut file = fs::OpenOptions::new().write(true).create_new(true).open(&temp)?;
        file.set_permissions(fs::metadata(&path)?.permissions())?;
        file.write_all(payload.content.as_bytes())?;
        file.sync_all()?;
        fs::rename(&temp, &path)
    })();
    if result.is_err() { let _ = fs::remove_file(&temp); }
    result.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn save_text(
    State(state): State<AppState>, Json(payload): Json<EditTextRequest>,
) -> Result<StatusCode, StatusCode> {
    // Serialize editor saves so two devices cannot both pass the original-content check.
    let inner = state.inner.lock().await;
    replace_text(&inner.shared_folder, &payload)?;
    let _ = state.broadcast_tx.send(json!({ "type": "file_updated", "data": { "path": payload.path } }).to_string());
    Ok(StatusCode::NO_CONTENT)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn text_edit_is_atomic_and_rejects_conflicts_and_binary() {
        let base = std::env::temp_dir().join(format!("relay-edit-test-{}", Uuid::new_v4()));
        fs::create_dir_all(&base).unwrap();
        let path = base.join("note.txt");
        fs::write(&path, "hello فارسی").unwrap();
        let edit = EditTextRequest { path: "note.txt".into(), original: "hello فارسی".into(), content: "updated متن".into() };
        replace_text(&base, &edit).unwrap();
        assert_eq!(read_editable_text(&path).unwrap(), "updated متن");
        assert_eq!(replace_text(&base, &edit), Err(StatusCode::CONFLICT));
        assert_eq!(fs::read_to_string(&path).unwrap(), "updated متن");
        fs::write(base.join("binary"), b"abc\0def").unwrap();
        assert_eq!(read_editable_text(&base.join("binary")), Err(StatusCode::UNSUPPORTED_MEDIA_TYPE));
        fs::write(base.join("large.txt"), vec![b'a'; MAX_TEXT_BYTES as usize + 1]).unwrap();
        assert_eq!(read_editable_text(&base.join("large.txt")), Err(StatusCode::PAYLOAD_TOO_LARGE));
        assert_eq!(replace_text(&base, &EditTextRequest { path: "../escape".into(), original: "".into(), content: "no".into() }), Err(StatusCode::FORBIDDEN));
        #[cfg(unix)] {
            std::os::unix::fs::symlink("/etc/hosts", base.join("outside.txt")).unwrap();
            assert_eq!(replace_text(&base, &EditTextRequest { path: "outside.txt".into(), original: "".into(), content: "no".into() }), Err(StatusCode::FORBIDDEN));
        }
        fs::remove_dir_all(base).unwrap();
    }

    #[test]
    fn test_safe_join_normal() {
        let temp_dir = std::env::temp_dir().join("relay_test_normal");
        let _ = fs::create_dir_all(&temp_dir);
        let joined = safe_join(&temp_dir, "photos/vacation.jpg");
        assert!(joined.is_ok());
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_safe_join_traversal_rejected() {
        let temp_dir = std::env::temp_dir().join("relay_test_traversal");
        let _ = fs::create_dir_all(&temp_dir);

        let joined_dotdot = safe_join(&temp_dir, "../../../etc/passwd");
        assert_eq!(joined_dotdot.unwrap_err(), StatusCode::FORBIDDEN);

        let _joined_root = safe_join(&temp_dir, "/etc/shadow");
        let _ = fs::remove_dir_all(&temp_dir);
    }
}
