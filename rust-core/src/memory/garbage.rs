//! 垃圾回收器
//! Garbage Collector
//!
//! 管理垃圾库的清理，用户可查看并决定永久删除。
//! Manages cleanup of the Garbage tier. User can view and decide permanent deletion.

use super::{Decayable, MemoryTier};
use std::collections::HashSet;

/// 垃圾回收器
/// Garbage Collector
#[derive(Debug, Clone, Default)]
pub struct GarbageCollector {
    /// 待删除条目的 ID 集合
    /// Set of entry IDs pending deletion
    pending_deletion: HashSet<String>,
}

impl GarbageCollector {
    /// 创建新的垃圾回收器
    /// Create a new garbage collector
    pub fn new() -> Self {
        Self::default()
    }

    /// 标记条目为待删除
    /// Mark an entry as pending deletion
    pub fn mark_for_deletion(&mut self, entry_id: String) {
        self.pending_deletion.insert(entry_id);
    }

    /// 取消标记
    /// Unmark an entry
    pub fn unmark(&mut self, entry_id: &str) {
        self.pending_deletion.remove(entry_id);
    }

    /// 检查条目是否被标记为待删除
    /// Check if an entry is marked for deletion
    pub fn is_marked(&self, entry_id: &str) -> bool {
        self.pending_deletion.contains(entry_id)
    }

    /// 获取所有待删除条目 ID
    /// Get all pending deletion entry IDs
    pub fn pending_ids(&self) -> Vec<String> {
        self.pending_deletion.iter().cloned().collect()
    }

    /// 执行删除：从给定的条目集合中移除所有标记的条目
    /// Execute deletion: remove all marked entries from the given collection
    ///
    /// 返回被删除的条目数量。
    /// Returns the number of deleted entries.
    pub fn collect<E>(&mut self, entries: &mut Vec<E>) -> usize
    where
        E: Decayable + HasId,
    {
        let before = entries.len();
        entries.retain(|e| !self.pending_deletion.contains(e.id()));
        let deleted = before - entries.len();
        self.pending_deletion.clear();
        deleted
    }

    /// 自动收集垃圾库中的条目（标记所有垃圾库条目）
    /// Auto-collect entries in Garbage tier (mark all garbage tier entries)
    pub fn auto_mark_garbage<E>(&mut self, entries: &[E])
    where
        E: Decayable + HasId,
    {
        for entry in entries {
            if entry.tier() == MemoryTier::Garbage && !entry.is_core() {
                self.mark_for_deletion(entry.id().to_string());
            }
        }
    }
}

/// 具有 ID 的条目 trait
/// Trait for entries that have an ID
pub trait HasId {
    fn id(&self) -> &str;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::Decayable;

    struct TestEntry {
        id: String,
        weight: f32,
        last_accessed: u64,
        access_frequency: f32,
        certainty: f32,
        tier: MemoryTier,
        core: bool,
    }

    impl HasId for TestEntry {
        fn id(&self) -> &str { &self.id }
    }

    impl Decayable for TestEntry {
        fn weight(&self) -> f32 { self.weight }
        fn set_weight(&mut self, w: f32) { self.weight = w; }
        fn last_accessed(&self) -> u64 { self.last_accessed }
        fn set_last_accessed(&mut self, ts: u64) { self.last_accessed = ts; }
        fn access_frequency(&self) -> f32 { self.access_frequency }
        fn certainty(&self) -> f32 { self.certainty }
        fn tier(&self) -> MemoryTier { self.tier }
        fn set_tier(&mut self, t: MemoryTier) { self.tier = t; }
        fn is_core(&self) -> bool { self.core }
    }

    #[test]
    fn test_garbage_collector() {
        let mut gc = GarbageCollector::new();
        let mut entries = vec![
            TestEntry {
                id: "1".to_string(),
                weight: 0.5,
                last_accessed: 0,
                access_frequency: 1.0,
                certainty: 1.0,
                tier: MemoryTier::Garbage,
                core: false,
            },
            TestEntry {
                id: "2".to_string(),
                weight: 0.5,
                last_accessed: 0,
                access_frequency: 1.0,
                certainty: 1.0,
                tier: MemoryTier::Active,
                core: false,
            },
        ];

        gc.auto_mark_garbage(&entries);
        assert_eq!(gc.pending_ids().len(), 1);
        assert!(gc.is_marked("1"));

        let deleted = gc.collect(&mut entries);
        assert_eq!(deleted, 1);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].id(), "2");
    }
}