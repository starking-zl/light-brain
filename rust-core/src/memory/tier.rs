//! 三层状态管理器
//! Three-Tier State Manager
//!
//! 管理记忆条目在活跃库、沉寂库、垃圾库之间的流转。
//! Manages the transition of memory entries among Active, Dormant, and Garbage tiers.

use super::{Decayable, MemoryTier};
use crate::utils::{current_timestamp, days_between, DecayParams};

/// 三层流转配置
/// Three-tier transition configuration
#[derive(Debug, Clone)]
pub struct TierConfig {
    /// 降级至沉寂库的权重阈值
    /// Weight threshold for downgrading to Dormant
    pub dormant_threshold: f32,
    /// 降级至垃圾库的权重阈值
    /// Weight threshold for downgrading to Garbage
    pub garbage_threshold: f32,
    /// 衰减参数
    /// Decay parameters
    pub decay_params: DecayParams,
}

impl Default for TierConfig {
    fn default() -> Self {
        Self {
            dormant_threshold: 0.3,
            garbage_threshold: 0.1,
            decay_params: DecayParams::default(),
        }
    }
}

/// 三层状态管理器
/// Three-Tier State Manager
#[derive(Debug, Clone)]
pub struct TierManager {
    config: TierConfig,
}

impl TierManager {
    /// 创建新的状态管理器
    /// Create a new state manager
    pub fn new(config: TierConfig) -> Self {
        Self { config }
    }

    /// 使用默认配置创建
    /// Create with default configuration
    pub fn default() -> Self {
        Self::new(TierConfig::default())
    }

    /// 更新单条记忆条目的层级状态
    /// Update the tier state of a single memory entry
    ///
    /// 基于当前权重决定新层级。核心知识永远不会降级。
    /// Determines new tier based on current weight. Core knowledge never downgrades.
    pub fn update_tier<E: Decayable>(&self, entry: &mut E) {
        if entry.is_core() {
            return; // 核心知识永不降级 / Core knowledge never downgrades
        }

        let weight = entry.weight();
        let new_tier = if weight < self.config.garbage_threshold {
            MemoryTier::Garbage
        } else if weight < self.config.dormant_threshold {
            MemoryTier::Dormant
        } else {
            MemoryTier::Active
        };

        entry.set_tier(new_tier);
    }

    /// 计算并应用衰减，然后更新层级
    /// Calculate and apply decay, then update tier
    ///
    /// 先根据时间衰减计算新权重，再决定层级。
    /// First calculates new weight based on time decay, then determines tier.
    pub fn decay_and_update<E: Decayable>(&self, entry: &mut E) {
        if entry.is_core() {
            return;
        }

        let now = current_timestamp();
        let days_elapsed = days_between(entry.last_accessed(), now) as f32;
        let new_weight = entry.calculate_decayed_weight(&self.config.decay_params, days_elapsed);
        entry.set_weight(new_weight);
        self.update_tier(entry);
    }

    /// 批量更新条目层级
    /// Batch update entries' tiers
    pub fn update_tiers_batch<E: Decayable>(&self, entries: &mut [E]) {
        for entry in entries {
            self.decay_and_update(entry);
        }
    }

    /// 判断条目是否应被唤醒（从沉寂库回到活跃库）
    /// Determine if an entry should be awakened (from Dormant to Active)
    ///
    /// 当条目被访问时调用，若当前处于沉寂库且权重仍高于沉寂阈值，
    /// 则将其提升回活跃库。
    /// Called when an entry is accessed. If currently Dormant and weight still
    /// above dormant threshold, promotes it back to Active.
    pub fn try_awaken<E: Decayable>(&self, entry: &mut E) -> bool {
        if entry.is_core() {
            return false;
        }
        if entry.tier() == MemoryTier::Dormant && entry.weight() >= self.config.dormant_threshold {
            entry.set_tier(MemoryTier::Active);
            return true;
        }
        false
    }

    /// 标记条目被访问（更新最后访问时间）
    /// Mark entry as accessed (update last accessed time)
    pub fn mark_accessed<E: Decayable>(&self, entry: &mut E) {
        entry.set_last_accessed(current_timestamp());
        // 访问时尝试唤醒
        let _ = self.try_awaken(entry);
    }

    /// 获取配置引用
    /// Get configuration reference
    pub fn config(&self) -> &TierConfig {
        &self.config
    }
}

impl Default for TierManager {
    fn default() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::Decayable;

    struct TestEntry {
        weight: f32,
        last_accessed: u64,
        access_frequency: f32,
        certainty: f32,
        tier: MemoryTier,
        core: bool,
    }

    impl Decayable for TestEntry {
        fn weight(&self) -> f32 { self.weight }
        fn set_weight(&mut self, w: f32) { self.weight = w; }
        fn last_accessed(&self) -> u64 { self.last_accessed }
        fn set_last_accessed(&mut self, ts: u64) { self.last_accessed = ts; }
        fn access_frequency(&self) -> f32 { self.access_frequency }
        fn certainty(&self) -> f32 { self.certainty }
        fn tier(&self) -> MemoryTier { self.tier }
        fn set_tier(&mut self, t: MemoryTier) { self.tier = t; }
        fn is_core(&self) -> bool { self.core }
    }

    #[test]
    fn test_tier_transition() {
        let manager = TierManager::default();
        let mut entry = TestEntry {
            weight: 0.5,
            last_accessed: 0,
            access_frequency: 1.0,
            certainty: 1.0,
            tier: MemoryTier::Active,
            core: false,
        };
        // 0.5 > 0.3，应保持活跃
        manager.update_tier(&mut entry);
        assert_eq!(entry.tier(), MemoryTier::Active);

        entry.weight = 0.2;
        manager.update_tier(&mut entry);
        assert_eq!(entry.tier(), MemoryTier::Dormant);

        entry.weight = 0.05;
        manager.update_tier(&mut entry);
        assert_eq!(entry.tier(), MemoryTier::Garbage);
    }

    #[test]
    fn test_core_never_downgrades() {
        let manager = TierManager::default();
        let mut entry = TestEntry {
            weight: 0.05,
            last_accessed: 0,
            access_frequency: 1.0,
            certainty: 1.0,
            tier: MemoryTier::Active,
            core: true,
        };
        manager.update_tier(&mut entry);
        assert_eq!(entry.tier(), MemoryTier::Active);
    }

    #[test]
    fn test_awaken() {
        let manager = TierManager::default();
        let mut entry = TestEntry {
            weight: 0.5,
            last_accessed: 0,
            access_frequency: 1.0,
            certainty: 1.0,
            tier: MemoryTier::Dormant,
            core: false,
        };
        let awakened = manager.try_awaken(&mut entry);
        assert!(awakened);
        assert_eq!(entry.tier(), MemoryTier::Active);
    }
}