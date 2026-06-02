//! 知识冲突检测
//! Knowledge Conflict Detection
//!
//! 在写入新知识时检测与现有知识的冲突，支持普通冲突和核心知识冲突。
//! Detects conflicts with existing knowledge when writing new entries,
//! supporting regular conflicts and core knowledge conflicts.

use super::KnowledgeEntry;
use std::collections::HashMap;

/// 冲突错误类型
/// Conflict error type
#[derive(Debug, Clone, PartialEq)]
pub struct ConflictError {
    pub conflict_type: ConflictType,
    pub message: String,
    pub existing_id: Option<String>,
}

impl std::fmt::Display for ConflictError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ConflictError {}

/// 冲突类型
/// Conflict type
#[derive(Debug, Clone, PartialEq)]
pub enum ConflictType {
    /// 与核心知识冲突（不可覆盖）
    CoreConflict,
    /// 普通冲突（确定性较高的覆盖确定性较低的）
    RegularConflict,
    /// 完全相同（幂等操作）
    Identical,
}

/// 冲突检测器
/// Conflict detector
#[derive(Debug, Default)]
pub struct ConflictDetector {
    /// 确定性阈值：现有确定性高于此值时拒绝被低确定性覆盖
    certainty_threshold: f32,
}

impl ConflictDetector {
    /// 创建新的冲突检测器
    /// Create a new conflict detector
    pub fn new() -> Self {
        Self {
            certainty_threshold: 0.8,
        }
    }

    /// 设置确定性阈值
    /// Set certainty threshold
    pub fn with_threshold(mut self, threshold: f32) -> Self {
        self.certainty_threshold = threshold;
        self
    }

    /// 检查知识冲突
    /// Check knowledge conflict
    pub fn check_conflict(
        &self,
        store: &HashMap<String, KnowledgeEntry>,
        new_entry: &KnowledgeEntry,
    ) -> Result<(), ConflictError> {
        // 查找是否存在相同主体和属性的条目
        for existing in store.values() {
            if existing.subject == new_entry.subject && existing.attribute == new_entry.attribute {
                // 完全相同（幂等）
                if existing.value == new_entry.value && existing.certainty == new_entry.certainty {
                    return Ok(());
                }

                // 与核心知识冲突
                if existing.core {
                    return Err(ConflictError {
                        conflict_type: ConflictType::CoreConflict,
                        message: format!(
                            "与核心知识冲突: {} {} 已有核心知识",
                            new_entry.subject, new_entry.attribute
                        ),
                        existing_id: Some(existing.id.clone()),
                    });
                }

                // 普通冲突：现有确定性高于阈值且大于新确定性
                if existing.certainty >= self.certainty_threshold && existing.certainty > new_entry.certainty {
                    return Err(ConflictError {
                        conflict_type: ConflictType::RegularConflict,
                        message: format!(
                            "知识冲突: 已有确定性 {:.2} > 新确定性 {:.2}",
                            existing.certainty, new_entry.certainty
                        ),
                        existing_id: Some(existing.id.clone()),
                    });
                }
            }
        }
        Ok(())
    }

    /// 检测是否存在冲突并返回冲突的现有条目
    /// Check for conflict and return the conflicting existing entry
    pub fn find_conflict<'a>(
        &self,
        store: &'a HashMap<String, KnowledgeEntry>,
        new_entry: &KnowledgeEntry,
    ) -> Option<&'a KnowledgeEntry> {
        store.values().find(|existing| {
            existing.subject == new_entry.subject
                && existing.attribute == new_entry.attribute
                && existing.value != new_entry.value
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cerebellum::KnowledgeEntry;

    #[test]
    fn test_conflict_detection() {
        let detector = ConflictDetector::new();
        let mut store = HashMap::new();

        let existing = KnowledgeEntry::new(
            "地球".to_string(),
            "半径".to_string(),
            serde_json::Value::String("6371km".to_string()),
            0.9,
        );
        store.insert(existing.id.clone(), existing);

        let new_low_certainty = KnowledgeEntry::new(
            "地球".to_string(),
            "半径".to_string(),
            serde_json::Value::String("6000km".to_string()),
            0.5,
        );

        let err = detector.check_conflict(&store, &new_low_certainty).unwrap_err();
        assert_eq!(err.conflict_type, ConflictType::RegularConflict);
    }

    #[test]
    fn test_core_conflict() {
        let detector = ConflictDetector::new();
        let mut store = HashMap::new();

        let core_entry = KnowledgeEntry::new(
            "光脑".to_string(),
            "原则".to_string(),
            serde_json::Value::String("和平与爱".to_string()),
            1.0,
        )
        .as_core();
        store.insert(core_entry.id.clone(), core_entry);

        let new_entry = KnowledgeEntry::new(
            "光脑".to_string(),
            "原则".to_string(),
            serde_json::Value::String("其他".to_string()),
            0.9,
        );

        let err = detector.check_conflict(&store, &new_entry).unwrap_err();
        assert_eq!(err.conflict_type, ConflictType::CoreConflict);
    }
}