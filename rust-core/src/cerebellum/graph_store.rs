//! 图存储层
//! Graph Store Layer
//!
//! 提供知识图谱的底层存储操作封装。
//! Provides low-level storage operation encapsulation for the knowledge graph.

use super::KnowledgeEntry;
use std::collections::HashMap;

/// 图存储
/// Graph Store
///
/// 管理知识条目的增删改查及索引维护。
/// Manages CRUD operations and index maintenance for knowledge entries.
#[derive(Debug, Default)]
pub struct GraphStore {
    entries: HashMap<String, KnowledgeEntry>,
    subject_index: HashMap<String, Vec<String>>,
    attribute_index: HashMap<String, Vec<String>>,
}

impl GraphStore {
    /// 创建新的图存储
    /// Create a new graph store
    pub fn new() -> Self {
        Self::default()
    }

    /// 插入条目
    /// Insert an entry
    pub fn insert(&mut self, entry: KnowledgeEntry) {
        let id = entry.id.clone();
        let subject = entry.subject.clone();
        let attribute = entry.attribute.clone();

        // 更新索引
        self.subject_index
            .entry(subject)
            .or_insert_with(Vec::new)
            .push(id.clone());
        self.attribute_index
            .entry(attribute)
            .or_insert_with(Vec::new)
            .push(id.clone());

        self.entries.insert(id, entry);
    }

    /// 获取条目
    /// Get an entry
    pub fn get(&self, id: &str) -> Option<&KnowledgeEntry> {
        self.entries.get(id)
    }

    /// 获取可变条目
    /// Get mutable entry
    pub fn get_mut(&mut self, id: &str) -> Option<&mut KnowledgeEntry> {
        self.entries.get_mut(id)
    }

    /// 删除条目
    /// Remove an entry
    pub fn remove(&mut self, id: &str) -> Option<KnowledgeEntry> {
        if let Some(entry) = self.entries.remove(id) {
            // 清理索引
            if let Some(list) = self.subject_index.get_mut(&entry.subject) {
                list.retain(|x| x != id);
            }
            if let Some(list) = self.attribute_index.get_mut(&entry.attribute) {
                list.retain(|x| x != id);
            }
            Some(entry)
        } else {
            None
        }
    }

    /// 按主体查找
    /// Find by subject
    pub fn find_by_subject(&self, subject: &str) -> Vec<&KnowledgeEntry> {
        self.subject_index
            .get(subject)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.entries.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// 按属性查找
    /// Find by attribute
    pub fn find_by_attribute(&self, attribute: &str) -> Vec<&KnowledgeEntry> {
        self.attribute_index
            .get(attribute)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.entries.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// 获取所有条目
    /// Get all entries
    pub fn all(&self) -> Vec<&KnowledgeEntry> {
        self.entries.values().collect()
    }

    /// 获取条目数量
    /// Get number of entries
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// 是否为空
    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// 批量插入
    /// Batch insert
    pub fn extend(&mut self, entries: Vec<KnowledgeEntry>) {
        for entry in entries {
            self.insert(entry);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cerebellum::KnowledgeEntry;

    #[test]
    fn test_graph_store_operations() {
        let mut store = GraphStore::new();
        let entry = KnowledgeEntry::new(
            "测试".to_string(),
            "属性".to_string(),
            serde_json::Value::String("值".to_string()),
            1.0,
        );
        let id = entry.id.clone();
        store.insert(entry);

        assert_eq!(store.len(), 1);
        assert!(store.get(&id).is_some());

        let found = store.find_by_subject("测试");
        assert_eq!(found.len(), 1);

        store.remove(&id);
        assert!(store.is_empty());
    }
}