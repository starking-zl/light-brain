//! 道挖掘器
//! Tao Miner
//!
//! 从长期生长日志和涅槃领悟中挖掘稳定行为模式（道）。
//! Mines stable behavioral patterns (Tao) from long-term growth logs and nirvana insights.

use std::collections::HashMap;

/// 稳定模式
#[derive(Debug, Clone)]
pub struct StablePattern {
    pub pattern_type: String,
    pub description: String,
    pub confidence: f32,
    pub occurrence_count: u32,
}

/// 道挖掘器
#[derive(Debug, Default)]
pub struct TaoMiner {
    patterns: HashMap<String, StablePattern>,
}

impl TaoMiner {
    pub fn new() -> Self {
        Self::default()
    }

    /// 记录一次观察（用于模式挖掘）
    pub fn observe(&mut self, pattern_type: &str, description: &str) {
        let entry = self.patterns.entry(pattern_type.to_string()).or_insert(StablePattern {
            pattern_type: pattern_type.to_string(),
            description: description.to_string(),
            confidence: 0.5,
            occurrence_count: 0,
        });
        entry.occurrence_count += 1;
        entry.confidence = (entry.confidence * 0.9 + 0.1).min(1.0);
    }

    /// 挖掘稳定模式
    pub fn mine_patterns(&self) -> Vec<String> {
        self.patterns
            .values()
            .filter(|p| p.confidence >= 0.7 && p.occurrence_count >= 5)
            .map(|p| p.description.clone())
            .collect()
    }

    /// 获取所有模式
    pub fn all_patterns(&self) -> Vec<StablePattern> {
        self.patterns.values().cloned().collect()
    }
}