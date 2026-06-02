//! 满足感记忆
//! Satiation Memory
//!
//! 记录生长事件后的满足感评分，驱动"饿"的动机。
//! Records satiation scores after growth events, driving the "Hunger" motivation.

use std::collections::VecDeque;

/// 满足感记忆
/// Satiation memory
#[derive(Debug, Clone)]
pub struct SatiationMemory {
    /// 最近 N 次满足感评分
    recent_scores: VecDeque<f32>,
    /// 最大记忆长度
    max_len: usize,
}

impl SatiationMemory {
    pub fn new(max_len: usize) -> Self {
        Self {
            recent_scores: VecDeque::with_capacity(max_len),
            max_len,
        }
    }

    /// 记录一次满足感评分
    pub fn record(&mut self, score: f32) {
        self.recent_scores.push_back(score.clamp(0.0, 1.0));
        if self.recent_scores.len() > self.max_len {
            self.recent_scores.pop_front();
        }
    }

    /// 计算平均满足感
    pub fn average_satiation(&self) -> f32 {
        if self.recent_scores.is_empty() {
            return 0.5;
        }
        self.recent_scores.iter().sum::<f32>() / self.recent_scores.len() as f32
    }

    /// 计算饥饿因子（1 - 平均满足感）
    pub fn hunger_factor(&self) -> f32 {
        1.0 - self.average_satiation()
    }

    /// 清空记忆
    pub fn clear(&mut self) {
        self.recent_scores.clear();
    }
}

impl Default for SatiationMemory {
    fn default() -> Self {
        Self::new(10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_satiation_memory() {
        let mut mem = SatiationMemory::new(3);
        assert_eq!(mem.average_satiation(), 0.5);
        mem.record(0.8);
        mem.record(0.9);
        assert_eq!(mem.average_satiation(), 0.85);
        assert_eq!(mem.hunger_factor(), 0.15);
    }
}