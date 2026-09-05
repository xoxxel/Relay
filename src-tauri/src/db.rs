use crate::state::ClipItem;
use rusqlite::{params, Connection, Result};
use std::path::Path;

pub fn init_db(db_path: &Path) -> Result<Connection> {
    if let Some(parent) = db_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    let conn = Connection::open(db_path)?;

    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS clip_items (
            id TEXT PRIMARY KEY,
            content TEXT NOT NULL,
            device_label TEXT,
            created_at TEXT NOT NULL
        );
        CREATE INDEX IF NOT EXISTS idx_clip_created ON clip_items (created_at DESC);",
    )?;

    Ok(conn)
}

pub fn get_clips(conn: &Connection, limit: usize) -> Result<Vec<ClipItem>> {
    let mut stmt = conn.prepare(
        "SELECT id, content, device_label, created_at
         FROM clip_items
         ORDER BY created_at DESC
         LIMIT ?",
    )?;

    let rows = stmt.query_map(params![limit as i64], |row| {
        Ok(ClipItem {
            id: row.get(0)?,
            content: row.get(1)?,
            device_label: row.get(2)?,
            created_at: row.get(3)?,
        })
    })?;

    let mut clips = Vec::new();
    for row in rows {
        clips.push(row?);
    }

    Ok(clips)
}

pub fn add_clip(conn: &Connection, clip: &ClipItem) -> Result<()> {
    conn.execute(
        "INSERT INTO clip_items (id, content, device_label, created_at)
         VALUES (?, ?, ?, ?)",
        params![clip.id, clip.content, clip.device_label, clip.created_at],
    )?;
    Ok(())
}

pub fn delete_clip(conn: &Connection, id: &str) -> Result<bool> {
    let rows_affected = conn.execute("DELETE FROM clip_items WHERE id = ?", params![id])?;
    Ok(rows_affected > 0)
}

