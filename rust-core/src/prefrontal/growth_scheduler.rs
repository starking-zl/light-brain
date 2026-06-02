//! 生长调度器（策略层）
//! Growth Scheduler (Strategy Layer)
//!
//! 负责全局生长配额的策略分配、效率追踪与自平衡。
//! 配额分配基于效率得分，在长期运行中自然趋向黄金分割比。
//! Responsible for global growth quota strategy allocation, efficiency tracking, and self-balancing.
//! Quota allocation is based on efficiency scores, naturally converging to the golden ratio over long-term operation.

use crate::growth::{EfficiencyTracker, GrowthExecutor, GrowthQuotaConfig, GrowthRecord, GrowthType};
use crate::utils::current_timestamp;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// 生长调度器
/// Growth Scheduler
#[derive(Debug)]
pub struct GrowthScheduler {
    /// 配额配置
    config: GrowthQuotaConfig,
    /// 效率追踪器
    efficiency_tracker: EfficiencyTracker,
    /// 当前周期剩余配额（神经）
    remaining_neural_quota: f32,
    /// 当前周期剩余配额（符号）
    remaining_symbolic_quota: f32,
    /// 上次分配时间戳
    last_allocation: u64,
    /// 生长执行器（实际执行生长操作）
    executor: GrowthExecutor,
    /// 系统成熟度（0.0 ~ 1.0，值越大越成熟）
    maturity: f32,
    /// 历史配额分配记录
    allocation_history: VecDeque<QuotaAllocation>,
}

/// 配额分配记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotaAllocation {
    pub timestamp: u64,
    pub neural_quota: f32,
    pub symbolic_quota: f32,
    pub neural_efficiency: f32,
    pub symbolic_efficiency: f32,
    pub actual_ratio: f32,
}

impl GrowthScheduler {
    pub fn new() -> Self {
        Self {
            config: GrowthQuotaConfig::default(),
            efficiency_tracker: EfficiencyTracker::new(),
            remaining_neural_quota: 0.0,
            remaining_symbolic_quota: 0.0,
            last_allocation: current_timestamp(),
            executor: GrowthExecutor::new(Default::default()),
            maturity: 0.0,
            allocation_history: VecDeque::with_capacity(100),
        }
    }

    /// 分配并执行生长（由前额叶定期调用）
    pub fn allocate_and_execute(&mut self) -> Vec<GrowthRecord> {
        self.allocate_quota();
        self.executor.execute_hebbian_growth();
        self.executor.execute_neurogenesis()
    }

    /// 分配配额（基于效率得分自平衡）
    fn allocate_quota(&mut self) {
        let now = current_timestamp();
        let days_since_last = (now - self.last_allocation) as f32 / 86400.0;
        if days_since_last < 1.0 / 24.0 { // 至少间隔1小时
            return;
        }

        // 计算总配额
        let maturity_factor = (-self.config.maturity_decay_lambda * self.maturity).exp();
        let total_base = (self.config.base_neural_connections + self.config.base_symbolic_entries) as f32;
        let total_quota = total_base * maturity_factor;

        // 获取效率得分
        let neural_eff = self.efficiency_tracker.neural_efficiency();
        let symbolic_eff = self.efficiency_tracker.symbolic_efficiency();

        // 自平衡分配：配额比 = 效率比
        let sum_eff = neural_eff + symbolic_eff;
        let neural_ratio = if sum_eff > 0.0 { neural_eff / sum_eff } else { 0.5 };
        let symbolic_ratio = 1.0 - neural_ratio;

        let neural_quota = total_quota * neural_ratio;
        let symbolic_quota = total_quota * symbolic_ratio;

        // 加上结转
        self.remaining_neural_quota = self.remaining_neural_quota * self.config.carryover_ratio + neural_quota;
        self.remaining_symbolic_quota = self.remaining_symbolic_quota * self.config.carryover_ratio + symbolic_quota;

        // 记录历史
        let allocation = QuotaAllocation {
            timestamp: now,
            neural_quota: self.remaining_neural_quota,
            symbolic_quota: self.remaining_symbolic_quota,
            neural_efficiency: neural_eff,
            symbolic_efficiency: symbolic_eff,
            actual_ratio: self.remaining_neural_quota / (self.remaining_symbolic_quota + 1e-6),
        };
        self.allocation_history.push_back(allocation);
        if self.allocation_history.len() > 100 {
            self.allocation_history.pop_front();
        }

        self.last_allocation = now;
    }

    /// 消费神经生长配额
    pub fn consume_neural_quota(&mut self, amount: f32) -> bool {
        if self.remaining_neural_quota >= amount {
            self.remaining_neural_quota -= amount;
            true
        } else {
            false
        }
    }

    /// 消费符号生长配额
    pub fn consume_symbolic_quota(&mut self, amount: f32) -> bool {
        if self.remaining_symbolic_quota >= amount {
            self.remaining_symbolic_quota -= amount;
            true
        } else {
            false
        }
    }

    /// 记录生长效率（由各模块在生长后调用）
    pub fn record_efficiency(&mut self, growth_type: GrowthType, score: f32) {
        match growth_type {
            GrowthType::Hebbian | GrowthType::Neurogenesis => {
                self.efficiency_tracker.record_neural(score);
            }
            GrowthType::KnowledgeExpansion => {
                self.efficiency_tracker.record_symbolic(score);
            }
        }
    }

    /// 更新系统成熟度
    pub fn update_maturity(&mut self, new_maturity: f32) {
        self.maturity = new_maturity.clamp(0.0, 1.0);
    }

    /// 获取当前配额比例（用于日志/监控）
    pub fn current_quota_ratio(&self) -> f32 {
        self.remaining_neural_quota / (self.remaining_symbolic_quota + 1e-6)
    }

    /// 获取生长执行器的可变引用（供外部触发具体生长）
    pub fn executor_mut(&mut self) -> &mut GrowthExecutor {
        &mut self.executor
    }
}

impl Default for GrowthScheduler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quota_allocation() {
        let mut scheduler = GrowthScheduler::new();
        scheduler.allocate_quota();
        assert!(scheduler.remaining_neural_quota > 0.0);
        assert!(scheduler.remaining_symbolic_quota > 0.0);
    }

    #[test]
    fn test_quota_consumption() {
        let mut scheduler = GrowthScheduler::new();
        scheduler.remaining_neural_quota = 10.0;
        assert!(scheduler.consume_neural_quota(5.0));
        assert_eq!(scheduler.remaining_neural_quota, 5.0);
        assert!(!scheduler.consume_neural_quota(10.0));
    }
}