//! 记忆管理模块
//! Memory Management Module
//!
//! 实现光脑方案的三层记忆系统：活跃库（Active）、沉寂库（Dormant）、垃圾库（Garbage）。
//! 提供记忆条目的衰减计算、层级流转与垃圾回收功能。
//! Implements the three-tier memory system of Light-Brain Scheme:
//! Active, Dormant, and Garbage tiers.
//! Provides decay calculation, tier transition, and garbage collection for memory entries.

mod tier;
mod garbage;
mod decay_config;

pub use tier::*;
pub use garbage::*;
pub use decay_config::*;

use crate::utils::DecayParams;

/// 记忆层级
/// Memory Tier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemoryTier {
    /// 活跃库：高频访问、高确定性，每次推理优先检索
    /// Active: High frequency, high certainty, prioritized in retrieval
    Active,
    /// 沉寂库：低频访问或被覆盖，正常模式不参与检索，可被临时唤醒
    /// Dormant: Low frequency or overwritten, not retrieved in normal mode, can be awakened
    Dormant,
    /// 垃圾库：从沉寂库淘汰，仅用户可查看和决定永久删除
    /// Garbage: Evicted from Dormant, only viewable and deletable by user
    Garbage,
}

impl Default for MemoryTier {
    fn default() -> Self {
        MemoryTier::Active
    }
}

/// 可衰减的记忆条目 trait
/// Decayable memory entry trait
///
/// 任何需要参与三层记忆流转的类型必须实现此 trait。
/// Any type that participates in three-tier memory flow must implement this trait.
pub trait Decayable {
    /// 获取当前权重
    /// Get current weight
    fn weight(&self) -> f32;

    /// 设置新权重
    /// Set new weight
    fn set_weight(&mut self, weight: f32);

    /// 获取最后访问时间戳（秒）
    /// Get last access timestamp in seconds
    fn last_accessed(&self) -> u64;

    /// 设置最后访问时间戳
    /// Set last access timestamp
    fn set_last_accessed(&mut self, timestamp: u64);

    /// 获取近三十日平均访问频次
    /// Get average access frequency in last 30 days
    fn access_frequency(&self) -> f32;

    /// 获取确定性评分 (0.0 ~ 1.0)
    /// Get certainty score (0.0 ~ 1.0)
    fn certainty(&self) -> f32;

    /// 获取当前记忆层级
    /// Get current memory tier
    fn tier(&self) -> MemoryTier;

    /// 设置记忆层级
    /// Set memory tier
    fn set_tier(&mut self, tier: MemoryTier);

    /// 是否为受保护的核心知识（永不衰减、永不降级）
    /// Whether this is protected core knowledge (never decays, never downgrades)
    fn is_core(&self) -> bool;

    /// 计算衰减后的权重
    /// Calculate decayed weight
    ///
    /// 默认实现使用通用衰减公式。
    /// Default implementation uses the generic decay formula.
    fn calculate_decayed_weight(&self, params: &DecayParams, days_elapsed: f32) -> f32 {
        if self.is_core() {
            return self.weight(); // 核心知识永不衰减 / Core knowledge never decays
        }
        crate::utils::calculate_decayed_weight(
            self.weight(),
            days_elapsed,
            self.access_frequency(),
            self.certainty(),
            params,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 测试用简单条目
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
    fn test_decayable_default() {
        let entry = TestEntry {
            weight: 1.0,
            last_accessed: 0,
            access_frequency: 1.0,
            certainty: 1.0,
            tier: MemoryTier::Active,
            core: false,
        };
        let params = DecayParams::default();
        let days = 10.0;
        let new_weight = entry.calculate_decayed_weight(&params, days);
        assert!(new_weight < 1.0);
    }

    #[test]
    fn test_core_knowledge_never_decays() {
        let entry = TestEntry {
            weight: 1.0,
            last_accessed: 0,
            access_frequency: 1.0,
            certainty: 1.0,
            tier: MemoryTier::Active,
            core: true,
        };
        let params = DecayParams::default();
        let new_weight = entry.calculate_decayed_weight(&params, 1000.0);
        assert_eq!(new_weight, 1.0);
    }
}