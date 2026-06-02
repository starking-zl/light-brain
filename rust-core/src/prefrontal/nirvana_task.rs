//! 涅槃任务
//! Nirvana Task
//!
//! 在腐败信号累积时触发，识别冗余、提取养分（道）、留下舍利。
//! Triggered when corruption signals accumulate. Identifies redundancy,
//! extracts nutrients (Tao), and leaves behind relics.

use crate::utils::current_timestamp;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 涅槃记录（舍利）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NirvanaRecord {
    pub id: String,
    pub timestamp: u64,
    pub extracted_patterns: Vec<String>,
    pub removed_items: Vec<String>,
    pub retained_core: Vec<String>,
}

/// 腐败指标
#[derive(Debug, Clone, Default)]
pub struct CorruptionMetrics {
    pub sweat_corruption: f32,   // 汗水地图淤积
    pub shadow_corruption: f32,  // 影子累积
    pub aging_corruption: f32,   // 衰老冗余
}

/// 涅槃任务管理器
#[derive(Debug)]
pub struct NirvanaTask {
    records: Vec<NirvanaRecord>,
    corruption_threshold: f32,
    last_check: u64,
}

impl NirvanaTask {
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
            corruption_threshold: 0.6,
            last_check: current_timestamp(),
        }
    }

    /// 检查是否应触发涅槃
    pub fn should_trigger(&self, metrics: &CorruptionMetrics) -> bool {
        let composite = (metrics.sweat_corruption + metrics.shadow_corruption + metrics.aging_corruption) / 3.0;
        composite >= self.corruption_threshold
    }

    /// 执行涅槃
    pub fn nirvana(&mut self, tao_miner: &mut super::TaoMiner, redundant_items: Vec<String>) -> NirvanaRecord {
        // 提取道（稳定模式）
        let patterns = tao_miner.mine_patterns();
        
        let record = NirvanaRecord {
            id: crate::utils::generate_uuid(),
            timestamp: current_timestamp(),
            extracted_patterns: patterns,
            removed_items: redundant_items,
            retained_core: Vec::new(), // 从爱约束层获取不可丢弃清单
        };
        
        self.records.push(record.clone());
        record
    }

    /// 获取所有涅槃记录
    pub fn records(&self) -> &[NirvanaRecord] {
        &self.records
    }
}

impl Default for NirvanaTask {
    fn default() -> Self {
        Self::new()
    }
}