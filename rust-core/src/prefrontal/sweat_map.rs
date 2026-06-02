//! 汗水地图（能力地形）
//! Sweat Map (Capability Terrain)
//!
//! 记录各推理路径的延迟、配额消耗和冲突频率，绘制系统能力地形图。
//! Records latency, quota consumption, and conflict frequency for each inference path,
//! drawing a capability terrain map of the system.

use crate::thalamus::PerceptionLabels;
use std::collections::HashMap;

/// 汗水条目
#[derive(Debug, Clone, Default)]
pub struct SweatEntry {
    /// 平均延迟（毫秒）
    pub avg_latency_ms: f32,
    /// 累计配额消耗
    pub total_quota_consumed: f32,
    /// 冲突次数
    pub conflict_count: u32,
    /// 调用次数
    pub call_count: u32,
}

/// 汗水地图
#[derive(Debug, Default)]
pub struct SweatMap {
    paths: HashMap<String, SweatEntry>,
    max_entries: usize,
}

impl SweatMap {
    pub fn new() -> Self {
        Self {
            paths: HashMap::new(),
            max_entries: 1000,
        }
    }

    /// 记录一次推理路径的消耗
    pub fn record(&mut self, labels: &PerceptionLabels) {
        let key = format!("{}:{}", labels.intent, labels.domain);
        let entry = self.paths.entry(key).or_insert(SweatEntry::default());
        entry.call_count += 1;
        // 延迟、配额消耗需外部传入，此处简化
    }

    /// 记录详细指标
    pub fn record_detailed(&mut self, intent: &str, domain: &str, latency_ms: f32, quota: f32, had_conflict: bool) {
        let key = format!("{}:{}", intent, domain);
        let entry = self.paths.entry(key).or_insert(SweatEntry::default());
        entry.call_count += 1;
        entry.avg_latency_ms = (entry.avg_latency_ms * (entry.call_count - 1) as f32 + latency_ms) / entry.call_count as f32;
        entry.total_quota_consumed += quota;
        if had_conflict {
            entry.conflict_count += 1;
        }

        // LRU 淘汰
        if self.paths.len() > self.max_entries {
            // 简化：移除调用次数最少的
            if let Some(oldest) = self.paths.iter().min_by_key(|(_, v)| v.call_count).map(|(k, _)| k.clone()) {
                self.paths.remove(&oldest);
            }
        }
    }

    /// 获取能力地形概要
    pub fn terrain_summary(&self) -> Vec<(String, SweatEntry)> {
        let mut entries: Vec<_> = self.paths.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
        entries.sort_by(|a, b| b.1.call_count.cmp(&a.1.call_count));
        entries
    }
}