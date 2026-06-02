//! 小脑衰减管理
//! Cerebellum Decay Management
//!
//! 提供批量衰减、层级流转的触发与执行。
//! Provides batch decay triggering and execution for tier transitions.

use super::{Cerebellum, StandardCerebellum};
use crate::memory::TierManager;

impl StandardCerebellum {
    /// 触发衰减任务（通常由前额叶的生长调度器定期调用）
    /// Trigger decay task (typically called periodically by Prefrontal Growth Scheduler)
    pub fn trigger_decay_task(&mut self) -> DecayReport {
        let before_counts = self.count_by_tier();
        let changed = self.apply_decay();
        let after_counts = self.count_by_tier();

        DecayReport {
            timestamp: crate::utils::current_timestamp(),
            changed_entries: changed,
            before_tiers: before_counts,
            after_tiers: after_counts,
        }
    }

    /// 统计各层级条目数量
    /// Count entries by tier
    fn count_by_tier(&self) -> TierCounts {
        use crate::memory::MemoryTier;
        let (mut active, mut dormant, mut garbage) = (0, 0, 0);
        for entry in self.store.values() {
            match entry.tier() {
                MemoryTier::Active => active += 1,
                MemoryTier::Dormant => dormant += 1,
                MemoryTier::Garbage => garbage += 1,
            }
        }
        TierCounts { active, dormant, garbage }
    }
}

/// 衰减报告
/// Decay report
#[derive(Debug, Clone)]
pub struct DecayReport {
    /// 时间戳
    pub timestamp: u64,
    /// 发生层级变更的条目数
    pub changed_entries: usize,
    /// 衰减前的层级统计
    pub before_tiers: TierCounts,
    /// 衰减后的层级统计
    pub after_tiers: TierCounts,
}

/// 层级统计
/// Tier counts
#[derive(Debug, Clone, Default)]
pub struct TierCounts {
    pub active: usize,
    pub dormant: usize,
    pub garbage: usize,
}