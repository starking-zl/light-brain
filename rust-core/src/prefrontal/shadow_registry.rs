//! 影子注册表（被否认的自我）
//! Shadow Registry (Denied Selves)
//!
//! 索引所有被否决、被压制的知识或输出，构成自我的深层轮廓。
//! Indexes all vetoed or suppressed knowledge/outputs, forming the deep contour of the self.

use crate::veto::VetoSignal;
use crate::utils::current_timestamp;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 影子条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShadowEntry {
    pub id: String,
    pub content: String,
    pub source: ShadowSource,
    pub timestamp: u64,
    pub integration_status: IntegrationStatus,
    pub recurrence_count: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShadowSource {
    Veto,
    RejectedCreative,
    DormantDiscard,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntegrationStatus {
    Pending,
    Integrated,
    Reconciled,
}

/// 影子注册表
#[derive(Debug, Default)]
pub struct ShadowRegistry {
    shadows: HashMap<String, ShadowEntry>,
}

impl ShadowRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// 记录一条被否决的影子
    pub fn record_veto(&mut self, signal: &VetoSignal) {
        let content = format!("{:?}: {}", signal.category, signal.reason);
        self.add_shadow(content, ShadowSource::Veto);
    }

    /// 记录被丢弃的创意
    pub fn record_rejected_creative(&mut self, content: &str) {
        self.add_shadow(content.to_string(), ShadowSource::RejectedCreative);
    }

    fn add_shadow(&mut self, content: String, source: ShadowSource) {
        // 检查是否已存在相似内容
        if let Some(existing) = self.shadows.values_mut().find(|e| e.content == content) {
            existing.recurrence_count += 1;
            existing.timestamp = current_timestamp();
            return;
        }

        let entry = ShadowEntry {
            id: crate::utils::generate_uuid(),
            content,
            source,
            timestamp: current_timestamp(),
            integration_status: IntegrationStatus::Pending,
            recurrence_count: 1,
        };
        self.shadows.insert(entry.id.clone(), entry);
    }

    /// 获取所有待整合的影子
    pub fn pending_shadows(&self) -> Vec<&ShadowEntry> {
        self.shadows.values().filter(|e| e.integration_status == IntegrationStatus::Pending).collect()
    }

    /// 标记影子为已整合
    pub fn mark_integrated(&mut self, id: &str) {
        if let Some(entry) = self.shadows.get_mut(id) {
            entry.integration_status = IntegrationStatus::Integrated;
        }
    }
}