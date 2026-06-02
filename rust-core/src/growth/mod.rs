//! 生长机制模块
//! Growth Mechanism Module
//!
//! 光脑方案的核心生长机制，包括 Hebbian 突触新生、神经元新生（斐波那契递推）、
//! 知识抽取、满足感记忆和生长执行调度。
//! Core growth mechanisms of Light-Brain Scheme, including Hebbian synaptic growth,
//! neurogenesis (Fibonacci progression), knowledge extraction, satiation memory,
//! and growth execution scheduling.

mod hebbian;
mod neurogenesis;
mod extractor;
mod satiation;
mod fibonacci;
mod scheduler;

// 公开导出 / Public exports
pub use hebbian::*;
pub use neurogenesis::*;
pub use extractor::KnowledgeExtractor;      // 知识抽取器公开
// ExtractedCandidate 为 pub(crate)，不对外导出 / ExtractedCandidate is pub(crate), not exported publicly
pub use satiation::*;
pub use fibonacci::*;
pub use scheduler::*;

use crate::utils::MetricsCollector;
use serde::{Deserialize, Serialize};

/// 生长事件类型
/// Growth event type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GrowthType {
    /// Hebbian 突触新生 / Hebbian synaptic growth
    Hebbian,
    /// 神经元新生 / Neurogenesis
    Neurogenesis,
    /// 知识图谱扩展 / Knowledge graph expansion
    KnowledgeExpansion,
}

/// 生长事件记录
/// Growth event record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrowthRecord {
    /// 事件 ID / Event ID
    pub id: String,
    /// 生长类型 / Growth type
    pub growth_type: GrowthType,
    /// 发生时间戳 / Timestamp
    pub timestamp: u64,
    /// 消耗的配额 / Quota consumed
    pub quota_consumed: f32,
    /// 效率得分（生长后的性能增益） / Efficiency score (performance gain after growth)
    pub efficiency_score: Option<f32>,
    /// 满足感评分（来自用户反馈） / Satiation score (from user feedback)
    pub satiation_score: Option<f32>,
    /// 详细信息 / Details
    pub details: String,
}

impl GrowthRecord {
    /// 创建新的生长记录
    /// Create a new growth record
    pub fn new(growth_type: GrowthType, quota_consumed: f32, details: String) -> Self {
        Self {
            id: crate::utils::generate_growth_log_id(),
            growth_type,
            timestamp: crate::utils::current_timestamp(),
            quota_consumed,
            efficiency_score: None,
            satiation_score: None,
            details,
        }
    }

    /// 设置效率得分
    /// Set efficiency score
    pub fn with_efficiency(mut self, score: f32) -> Self {
        self.efficiency_score = Some(score);
        self
    }

    /// 设置满足感评分
    /// Set satiation score
    pub fn with_satiation(mut self, score: f32) -> Self {
        self.satiation_score = Some(score);
        self
    }
}

/// 生长配额配置
/// Growth quota configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrowthQuotaConfig {
    /// 每日基础配额（神经连接数） / Daily base quota (neural connections)
    pub base_neural_connections: u32,
    /// 每日基础配额（符号知识条目数） / Daily base quota (symbolic entries)
    pub base_symbolic_entries: u32,
    /// 成熟度衰减系数 λ / Maturity decay lambda
    pub maturity_decay_lambda: f32,
    /// 配额结转比例 / Quota carryover ratio
    pub carryover_ratio: f32,
}

impl Default for GrowthQuotaConfig {
    fn default() -> Self {
        Self {
            base_neural_connections: 100,
            base_symbolic_entries: 50,
            maturity_decay_lambda: 0.5,
            carryover_ratio: 0.3,
        }
    }
}

/// 生长效率追踪器
/// Growth efficiency tracker
#[derive(Debug, Clone, Default)]
pub struct EfficiencyTracker {
    /// 神经生长历史效率得分 / Historical efficiency scores for neural growth
    neural_scores: Vec<f32>,
    /// 符号生长历史效率得分 / Historical efficiency scores for symbolic growth
    symbolic_scores: Vec<f32>,
}

impl EfficiencyTracker {
    /// 创建新的效率追踪器
    /// Create a new efficiency tracker
    pub fn new() -> Self {
        Self::default()
    }

    /// 记录神经生长效率
    /// Record neural growth efficiency
    pub fn record_neural(&mut self, score: f32) {
        self.neural_scores.push(score);
        if self.neural_scores.len() > 10 {
            self.neural_scores.remove(0);
        }
    }

    /// 记录符号生长效率
    /// Record symbolic growth efficiency
    pub fn record_symbolic(&mut self, score: f32) {
        self.symbolic_scores.push(score);
        if self.symbolic_scores.len() > 10 {
            self.symbolic_scores.remove(0);
        }
    }

    /// 获取神经生长平均效率
    /// Get average neural growth efficiency
    pub fn neural_efficiency(&self) -> f32 {
        if self.neural_scores.is_empty() {
            return 0.5;
        }
        self.neural_scores.iter().sum::<f32>() / self.neural_scores.len() as f32
    }

    /// 获取符号生长平均效率
    /// Get average symbolic growth efficiency
    pub fn symbolic_efficiency(&self) -> f32 {
        if self.symbolic_scores.is_empty() {
            return 0.5;
        }
        self.symbolic_scores.iter().sum::<f32>() / self.symbolic_scores.len() as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_growth_record_creation() {
        let record = GrowthRecord::new(
            GrowthType::KnowledgeExpansion,
            1.0,
            "测试知识扩展".to_string(),
        );
        assert_eq!(record.growth_type, GrowthType::KnowledgeExpansion);
        assert!(!record.id.is_empty());
    }

    #[test]
    fn test_efficiency_tracker() {
        let mut tracker = EfficiencyTracker::new();
        assert_eq!(tracker.neural_efficiency(), 0.5);
        tracker.record_neural(0.8);
        tracker.record_neural(0.9);
        assert_eq!(tracker.neural_efficiency(), 0.85);
    }
}