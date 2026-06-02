//! 梦任务（碎片随机连接）
//! Dream Task (Fragment Random Connection)
//!
//! 在夜间模式下，随机连接沉寂库、否决日志和创意库的碎片，存入梦境沉积层。
//! In night mode, randomly connects fragments from dormant library, veto logs, and creative incubator,
//! storing them in the dream sediment layer.

use crate::utils::current_timestamp;
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// 梦境记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DreamRecord {
    pub id: String,
    pub timestamp: u64,
    pub fragments: Vec<String>,
    pub description: String,
}

/// 梦任务管理器
#[derive(Debug)]
pub struct DreamTask {
    dreams: VecDeque<DreamRecord>,
    max_dreams: usize,
    retention_days: u64,
}

impl DreamTask {
    pub fn new() -> Self {
        Self {
            dreams: VecDeque::new(),
            max_dreams: 100,
            retention_days: 7,
        }
    }

    /// 执行一次梦任务（从多个来源抽取碎片并随机连接）
    pub fn dream(&mut self, dormant_fragments: &[String], veto_fragments: &[String], creative_fragments: &[String]) {
        let mut rng = thread_rng();
        let mut fragments = Vec::new();
        
        // 随机抽取碎片
        if let Some(f) = dormant_fragments.choose(&mut rng) {
            fragments.push(f.clone());
        }
        if let Some(f) = veto_fragments.choose(&mut rng) {
            fragments.push(f.clone());
        }
        if let Some(f) = creative_fragments.choose(&mut rng) {
            fragments.push(f.clone());
        }

        if fragments.len() >= 2 {
            let record = DreamRecord {
                id: crate::utils::generate_uuid(),
                timestamp: current_timestamp(),
                fragments: fragments.clone(),
                description: format!("梦: {} 个碎片连接", fragments.len()),
            };
            self.dreams.push_back(record);
        }

        // 清理过期梦境
        let cutoff = current_timestamp() - self.retention_days * 86400;
        while self.dreams.front().map(|d| d.timestamp < cutoff).unwrap_or(false) {
            self.dreams.pop_front();
        }
        if self.dreams.len() > self.max_dreams {
            self.dreams.pop_front();
        }
    }

    /// 获取最近的梦境
    pub fn recent_dreams(&self, limit: usize) -> Vec<DreamRecord> {
        self.dreams.iter().rev().take(limit).cloned().collect()
    }
}

impl Default for DreamTask {
    fn default() -> Self {
        Self::new()
    }
}