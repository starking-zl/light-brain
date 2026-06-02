//! 核心知识管理
//! Core Knowledge Management
//!
//! 提供核心知识的标记、保护和查询功能。
//! Provides marking, protection, and querying for core knowledge.

use super::KnowledgeEntry;
use std::collections::HashSet;

/// 核心知识管理器
/// Core knowledge manager
#[derive(Debug, Default)]
pub struct CoreKnowledgeManager {
    /// 核心知识 ID 集合
    core_ids: HashSet<String>,
}

impl CoreKnowledgeManager {
    /// 创建新的管理器
    /// Create a new manager
    pub fn new() -> Self {
        Self::default()
    }

    /// 标记为核心知识
    /// Mark as core knowledge
    pub fn mark_core(&mut self, entry: &mut KnowledgeEntry) {
        entry.core = true;
        entry.weight = 1.0;
        entry.certainty = 1.0;
        self.core_ids.insert(entry.id.clone());
    }

    /// 取消核心标记
    /// Unmark as core
    pub fn unmark_core(&mut self, entry: &mut KnowledgeEntry) {
        entry.core = false;
        self.core_ids.remove(&entry.id);
    }

    /// 检查条目是否为核心知识
    /// Check if an entry is core knowledge
    pub fn is_core(&self, id: &str) -> bool {
        self.core_ids.contains(id)
    }

    /// 获取所有核心知识的 ID
    /// Get all core knowledge IDs
    pub fn get_core_ids(&self) -> Vec<String> {
        self.core_ids.iter().cloned().collect()
    }

    /// 从条目集合中筛选核心知识
    /// Filter core knowledge from a collection
    pub fn filter_core<'a>(&self, entries: &'a [KnowledgeEntry]) -> Vec<&'a KnowledgeEntry> {
        entries.iter().filter(|e| e.core).collect()
    }

    /// 验证核心知识完整性（所有核心知识必须存在且确定性为1.0）
    /// Validate core knowledge integrity
    pub fn validate(&self, entries: &[KnowledgeEntry]) -> bool {
        for id in &self.core_ids {
            if let Some(entry) = entries.iter().find(|e| e.id == *id) {
                if !entry.core || entry.certainty < 1.0 {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cerebellum::KnowledgeEntry;

    #[test]
    fn test_core_knowledge_manager() {
        let mut manager = CoreKnowledgeManager::new();
        let mut entry = KnowledgeEntry::default();
        let id = entry.id.clone();

        manager.mark_core(&mut entry);
        assert!(entry.core);
        assert_eq!(entry.certainty, 1.0);
        assert!(manager.is_core(&id));

        manager.unmark_core(&mut entry);
        assert!(!entry.core);
        assert!(!manager.is_core(&id));
    }
}