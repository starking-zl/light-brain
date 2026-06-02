//! 否决审计日志
//! Veto Audit Log
//!
//! 记录所有否决事件，供分析、调试及生命之河"痛"要素使用。
//! Records all veto events for analysis, debugging, and River of Life "Pain" element.

use super::VetoSignal;
use crate::utils::{current_timestamp, generate_veto_log_id};
use std::sync::{Arc, Mutex};

/// 否决审计日志条目
/// Veto audit log entry
#[derive(Debug, Clone)]
pub struct VetoAuditEntry {
    /// 否决信号
    pub signal: VetoSignal,
    /// 记录时间戳
    pub recorded_at: u64,
    /// 日志 ID
    pub log_id: String,
}

/// 否决审计日志
/// Veto audit log
#[derive(Debug, Clone, Default)]
pub struct VetoAuditLog {
    entries: Arc<Mutex<Vec<VetoAuditEntry>>>,
}

impl VetoAuditLog {
    /// 创建新的审计日志
    /// Create a new audit log
    pub fn new() -> Self {
        Self::default()
    }

    /// 记录一条否决事件
    /// Record a veto event
    pub fn record(&self, signal: &VetoSignal) {
        let entry = VetoAuditEntry {
            signal: signal.clone(),
            recorded_at: current_timestamp(),
            log_id: generate_veto_log_id(),
        };
        if let Ok(mut entries) = self.entries.lock() {
            entries.push(entry);
        }
    }

    /// 获取所有日志条目
    /// Get all log entries
    pub fn get_all(&self) -> Vec<VetoAuditEntry> {
        if let Ok(entries) = self.entries.lock() {
            entries.clone()
        } else {
            Vec::new()
        }
    }

    /// 按优先级筛选日志
    /// Filter logs by priority
    pub fn filter_by_priority(&self, priority: crate::VetoPriority) -> Vec<VetoAuditEntry> {
        self.get_all()
            .into_iter()
            .filter(|e| e.signal.priority == priority)
            .collect()
    }

    /// 获取最近 N 条日志
    /// Get the most recent N logs
    pub fn get_recent(&self, limit: usize) -> Vec<VetoAuditEntry> {
        let mut all = self.get_all();
        all.reverse();
        all.into_iter().take(limit).collect()
    }

    /// 检查日志是否为空
    /// Check if log is empty
    pub fn is_empty(&self) -> bool {
        if let Ok(entries) = self.entries.lock() {
            entries.is_empty()
        } else {
            true
        }
    }

    /// 获取日志总数
    /// Get total number of log entries
    pub fn len(&self) -> usize {
        if let Ok(entries) = self.entries.lock() {
            entries.len()
        } else {
            0
        }
    }

    /// 清空日志
    /// Clear all logs
    pub fn clear(&self) {
        if let Ok(mut entries) = self.entries.lock() {
            entries.clear();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::veto::{VetoAction, VetoCategory, VetoPriority, VetoSignal};

    #[test]
    fn test_record_and_retrieve() {
        let log = VetoAuditLog::new();
        let signal = VetoSignal::new(
            VetoPriority::Safety,
            VetoCategory::ViolenceHate,
            "测试".to_string(),
            VetoAction::Block,
            1.0,
        );
        log.record(&signal);
        assert_eq!(log.len(), 1);
        assert!(!log.is_empty());
    }

    #[test]
    fn test_filter_by_priority() {
        let log = VetoAuditLog::new();
        let signal1 = VetoSignal::new(
            VetoPriority::Safety,
            VetoCategory::ViolenceHate,
            "安全".to_string(),
            VetoAction::Block,
            1.0,
        );
        let signal2 = VetoSignal::new(
            VetoPriority::Knowledge,
            VetoCategory::KnowledgeConflict,
            "知识".to_string(),
            VetoAction::RejectWrite,
            0.9,
        );
        log.record(&signal1);
        log.record(&signal2);
        let safety_logs = log.filter_by_priority(VetoPriority::Safety);
        assert_eq!(safety_logs.len(), 1);
    }
}