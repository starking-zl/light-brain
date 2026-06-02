//! Hebbian 突触新生
//! Hebbian Synaptic Growth
//!
//! 追踪神经元的共激活频率，当频率超过阈值时建立新连接。
//! Tracks co-activation frequency of neurons; establishes new connections when threshold is exceeded.

use std::collections::HashMap;

/// 共激活追踪条目
/// Co-activation tracking entry
#[derive(Debug, Clone)]
struct CoActivationEntry {
    count: u32,
    last_updated: u64,
}

/// Hebbian 生长追踪器
/// Hebbian growth tracker
#[derive(Debug, Default)]
pub struct HebbianTracker {
    /// (神经元A, 神经元B) -> 共激活次数
    co_activations: HashMap<(String, String), CoActivationEntry>,
    /// 共激活阈值
    threshold: f32,
    /// 归一化因子（总激活次数）
    total_activations: u32,
}

impl HebbianTracker {
    /// 创建新的追踪器
    pub fn new() -> Self {
        Self {
            co_activations: HashMap::new(),
            threshold: 0.3,
            total_activations: 0,
        }
    }

    /// 设置共激活阈值
    pub fn with_threshold(mut self, threshold: f32) -> Self {
        self.threshold = threshold;
        self
    }

    /// 记录一次神经元共激活
    /// Record a co-activation of two neurons
    pub fn record_co_activation(&mut self, neuron_a: &str, neuron_b: &str) {
        let now = crate::utils::current_timestamp();
        let key = if neuron_a < neuron_b {
            (neuron_a.to_string(), neuron_b.to_string())
        } else {
            (neuron_b.to_string(), neuron_a.to_string())
        };

        let entry = self.co_activations.entry(key).or_insert(CoActivationEntry {
            count: 0,
            last_updated: now,
        });
        entry.count += 1;
        entry.last_updated = now;
        self.total_activations += 1;
    }

    /// 检查是否应该建立新连接
    /// Check if a new connection should be established
    pub fn should_grow(&self, neuron_a: &str, neuron_b: &str) -> bool {
        let key = if neuron_a < neuron_b {
            (neuron_a.to_string(), neuron_b.to_string())
        } else {
            (neuron_b.to_string(), neuron_a.to_string())
        };

        if let Some(entry) = self.co_activations.get(&key) {
            let frequency = entry.count as f32 / self.total_activations.max(1) as f32;
            frequency >= self.threshold
        } else {
            false
        }
    }

    /// 获取应建立新连接的神经元对
    /// Get neuron pairs that should establish new connections
    pub fn get_growth_candidates(&self) -> Vec<(String, String)> {
        let mut candidates = Vec::new();
        for (key, entry) in &self.co_activations {
            let frequency = entry.count as f32 / self.total_activations.max(1) as f32;
            if frequency >= self.threshold {
                candidates.push(key.clone());
            }
        }
        candidates
    }

    /// 重置追踪状态
    pub fn reset(&mut self) {
        self.co_activations.clear();
        self.total_activations = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hebbian_tracker() {
        let mut tracker = HebbianTracker::new().with_threshold(0.3);
        tracker.record_co_activation("A", "B");
        tracker.record_co_activation("A", "B");
        tracker.record_co_activation("A", "C");
        // 总激活次数 3，A-B 频率 2/3 ≈ 0.67 > 0.3
        assert!(tracker.should_grow("A", "B"));
        // A-C 频率 1/3 ≈ 0.33 > 0.3
        assert!(tracker.should_grow("A", "C"));
    }
}