//! 事件存储
//! Event Store
//!
//! 基于 SQLite 实现情景事件的持久化存储。
//! SQLite-based persistent storage for episodic events.

use super::{EpisodicEvent, PerceptionLabels};
use rusqlite::{Connection, params, Result as RusqliteResult};

/// 事件存储 trait
/// Event Store trait
pub trait EventStore: Send + Sync {
    fn insert(&mut self, event: EpisodicEvent) -> Result<String, String>;
    fn get(&self, id: &str) -> Option<EpisodicEvent>;
    fn get_all(&self) -> Vec<EpisodicEvent>;
    fn get_by_node(&self, node_id: &str) -> Vec<EpisodicEvent>;
    fn get_by_time_range(&self, start: u64, end: u64) -> Vec<EpisodicEvent>;
    fn update_node_id(&mut self, event_id: &str, node_id: &str) -> Result<(), String>;
}

/// SQLite 事件存储
/// SQLite Event Store
pub struct SqliteEventStore {
    conn: Connection,
}

impl SqliteEventStore {
    pub fn new(db_path: &str) -> Result<Self, String> {
        let conn = Connection::open(db_path).map_err(|e| e.to_string())?;
        
        // 创建事件表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS episodic_events (
                id TEXT PRIMARY KEY,
                timestamp INTEGER NOT NULL,
                user_input TEXT NOT NULL,
                perception_labels TEXT NOT NULL,
                decision_package TEXT,
                response TEXT NOT NULL,
                node_id TEXT,
                was_corrected INTEGER NOT NULL DEFAULT 0,
                emotion TEXT NOT NULL DEFAULT 'neutral',
                importance REAL NOT NULL DEFAULT 0.5,
                modality TEXT NOT NULL DEFAULT 'text',
                asset_uri TEXT,
                feature_vector TEXT
            )",
            [],
        ).map_err(|e| e.to_string())?;

        // 创建索引
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_events_timestamp ON episodic_events(timestamp)",
            [],
        ).map_err(|e| e.to_string())?;
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_events_node_id ON episodic_events(node_id)",
            [],
        ).map_err(|e| e.to_string())?;

        Ok(Self { conn })
    }
}

impl EventStore for SqliteEventStore {
    fn insert(&mut self, event: EpisodicEvent) -> Result<String, String> {
        let labels_json = serde_json::to_string(&event.perception_labels).map_err(|e| e.to_string())?;
        let feature_vector_json = event.feature_vector.as_ref()
            .map(|v| serde_json::to_string(v).unwrap_or_default());

        self.conn.execute(
            "INSERT INTO episodic_events (
                id, timestamp, user_input, perception_labels, decision_package,
                response, node_id, was_corrected, emotion, importance, modality,
                asset_uri, feature_vector
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            params![
                event.id,
                event.timestamp,
                event.user_input,
                labels_json,
                event.decision_package,
                event.response,
                event.node_id,
                event.was_corrected as i32,
                event.emotion,
                event.importance,
                event.modality,
                event.asset_uri,
                feature_vector_json,
            ],
        ).map_err(|e| e.to_string())?;

        Ok(event.id)
    }

    fn get(&self, id: &str) -> Option<EpisodicEvent> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM episodic_events WHERE id = ?"
        ).ok()?;
        let row = stmt.query_row(params![id], |row| {
            Ok(row_to_event(row)?)
        }).ok()?;
        Some(row)
    }

    fn get_all(&self) -> Vec<EpisodicEvent> {
        let mut stmt = self.conn.prepare("SELECT * FROM episodic_events ORDER BY timestamp DESC").ok()?;
        let rows = stmt.query_map([], |row| row_to_event(row)).ok()?;
        rows.filter_map(|r| r.ok()).collect()
    }

    fn get_by_node(&self, node_id: &str) -> Vec<EpisodicEvent> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM episodic_events WHERE node_id = ? ORDER BY timestamp DESC"
        ).ok()?;
        let rows = stmt.query_map(params![node_id], |row| row_to_event(row)).ok()?;
        rows.filter_map(|r| r.ok()).collect()
    }

    fn get_by_time_range(&self, start: u64, end: u64) -> Vec<EpisodicEvent> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM episodic_events WHERE timestamp BETWEEN ? AND ? ORDER BY timestamp DESC"
        ).ok()?;
        let rows = stmt.query_map(params![start, end], |row| row_to_event(row)).ok()?;
        rows.filter_map(|r| r.ok()).collect()
    }

    fn update_node_id(&mut self, event_id: &str, node_id: &str) -> Result<(), String> {
        self.conn.execute(
            "UPDATE episodic_events SET node_id = ? WHERE id = ?",
            params![node_id, event_id],
        ).map_err(|e| e.to_string())?;
        Ok(())
    }
}

/// 将数据库行转换为 EpisodicEvent
fn row_to_event(row: &rusqlite::Row) -> RusqliteResult<EpisodicEvent> {
    let labels_json: String = row.get("perception_labels")?;
    let perception_labels: PerceptionLabels = serde_json::from_str(&labels_json)
        .unwrap_or_default();

    let feature_vector_json: Option<String> = row.get("feature_vector")?;
    let feature_vector = feature_vector_json
        .and_then(|s| serde_json::from_str(&s).ok());

    Ok(EpisodicEvent {
        id: row.get("id")?,
        timestamp: row.get("timestamp")?,
        user_input: row.get("user_input")?,
        perception_labels,
        decision_package: row.get("decision_package")?,
        response: row.get("response")?,
        node_id: row.get("node_id")?,
        was_corrected: row.get::<_, i32>("was_corrected")? != 0,
        emotion: row.get("emotion")?,
        importance: row.get("importance")?,
        modality: row.get("modality")?,
        asset_uri: row.get("asset_uri")?,
        feature_vector,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_event_store() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db").to_str().unwrap().to_string();
        let mut store = SqliteEventStore::new(&db_path).unwrap();

        let event = EpisodicEvent::default();
        let id = event.id.clone();
        store.insert(event).unwrap();

        let retrieved = store.get(&id).unwrap();
        assert_eq!(retrieved.id, id);
    }
}