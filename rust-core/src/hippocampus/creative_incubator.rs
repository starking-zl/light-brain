//! 创意孵化器
//! Creative Incubator
//!
//! 存储创意模式下被否决的知识候选，提供7天孵化期供用户筛选。
//! Stores knowledge candidates vetoed in creative mode, providing a 7-day incubation period for user screening.

use crate::utils::current_timestamp;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 创意孵化条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreativeIncubatorEntry {
    pub id: String,
    pub knowledge_candidate: super::KnowledgeCandidate,
    pub created_at: u64,
    pub expires_at: u64,
    pub exempted: bool,
}

/// 创意孵化器
#[derive(Debug, Default)]
pub struct CreativeIncubator {
    entries: HashMap<String, CreativeIncubatorEntry>,
    retention_days: u64,
    capacity: usize,
}

impl CreativeIncubator {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            retention_days: 7,
            capacity: 50,
        }
    }

    pub fn add(&mut self, candidate: super::KnowledgeCandidate) -> String {
        let id = crate::utils::generate_uuid();
        let now = current_timestamp();
        let entry = CreativeIncubatorEntry {
            id: id.clone(),
            knowledge_candidate: candidate,
            created_at: now,
            expires_at: now + self.retention_days * 86400,
            exempted: false,
        };
        
        // FIFO 淘汰
        if self.entries.len() >= self.capacity {
            let oldest = self.entries.values()
                .min_by_key(|e| e.created_at)
                .map(|e| e.id.clone());
            if let Some(id) = oldest {
                self.entries.remove(&id);
            }
        }
        
        self.entries.insert(id.clone(), entry);
        id
    }

    pub fn get_all(&self) -> Vec<&CreativeIncubatorEntry> {
        let now = current_timestamp();
        self.entries.values()
            .filter(|e| e.expires_at > now)
            .collect()
    }

    pub fn mark_exempted(&mut self, id: &str) {
        if let Some(entry) = self.entries.get_mut(id) {
            entry.exempted = true;
        }
    }

    pub fn cleanup_expired(&mut self) -> usize {
        let now = current_timestamp();
        let before = self.entries.len();
        self.entries.retain(|_, e| e.expires_at > now);
        before - self.entries.len()
    }

    pub fn remove(&mut self, id: &str) -> Option<CreativeIncubatorEntry> {
        self.entries.remove(id)
    }
}