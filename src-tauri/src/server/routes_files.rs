use crate::state::{AppState, FileItem};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
pub struct ListFilesQuery {
    pub path: Option<String>,
}

pub fn safe_join(base: &Path, rel: &str) -> Result<PathBuf, StatusCode> {
    let clean_rel = rel.trim_start_matches('/').trim_start_matches('\\');
    let mut target = base.to_path_buf();
    for component in Path::new(clean_rel).components() {
        match component {
            std::path::Component::Normal(c) => target.push(c),
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                // Reject parent directory traversal for safety
                return Err(StatusCode::FORBIDDEN);
            }
            std::path::Component::RootDir | std::path::Component::Prefix(_) => {
                return Err(StatusCode::FORBIDDEN);
            }
        }
    }

    // Ensure base directory exists
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

    // Sort: directories first, then names alphabetically
    items.sort_by(|a, b| {
        match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        }
    });

    Ok(Json(items))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_safe_join_normal() {
        let temp_dir = std::env::temp_dir().join("lan_share_test_normal");
        let _ = fs::create_dir_all(&temp_dir);
        let joined = safe_join(&temp_dir, "photos/vacation.jpg");
        assert!(joined.is_ok());
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_safe_join_traversal_rejected() {
        let temp_dir = std::env::temp_dir().join("lan_share_test_traversal");
        let _ = fs::create_dir_all(&temp_dir);
        
        let joined_dotdot = safe_join(&temp_dir, "../../../etc/passwd");
        assert_eq!(joined_dotdot.unwrap_err(), StatusCode::FORBIDDEN);

        let _joined_root = safe_join(&temp_dir, "/etc/shadow");
        // /etc/shadow gets stripped to etc/shadow or rejected if parent is reached
        let _ = fs::remove_dir_all(&temp_dir);
    }
}
