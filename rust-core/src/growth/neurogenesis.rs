//! 神经元新生
//! Neurogenesis
//!
//! 在预测误差持续高的区域插入新神经元，数量遵循斐波那契递推。
//! Inserts new neurons in regions with persistently high prediction error,
//! with quantity following Fibonacci progression.

use super::FibonacciSequence;
use crate::utils::current_timestamp;

/// 神经元新生配置
/// Neurogenesis configuration
#[derive(Debug, Clone)]
pub struct NeurogenesisConfig {
    /// 误差阈值（超过此值触发新生）
    pub error_threshold: f32,
    /// 连续误差步数阈值
    pub consecutive_steps_threshold: u32,
    /// 斐波那契数列生成器
    pub fibonacci: FibonacciSequence,
    /// 最大神经元数量（防止无限生长）
    pub max_neurons: u32,
}

impl Default for NeurogenesisConfig {
    fn default() -> Self {
        Self {
            error_threshold: 1.5,
            consecutive_steps_threshold: 500,
            fibonacci: FibonacciSequence::new(),
            max_neurons: 10000,
        }
    }
}

/// 区域误差追踪
/// Region error tracking
#[derive(Debug, Clone)]
struct RegionErrorTracker {
    region_id: String,
    consecutive_high_error_steps: u32,
    last_error: f32,
    baseline_error: f32,
}

/// 神经元新生管理器
/// Neurogenesis manager
#[derive(Debug)]
pub struct NeurogenesisManager {
    config: NeurogenesisConfig,
    regions: HashMap<String, RegionErrorTracker>,
}

impl NeurogenesisManager {
    pub fn new(config: NeurogenesisConfig) -> Self {
        Self {
            config,
            regions: HashMap::new(),
        }
    }

    /// 记录区域预测误差
    /// Record prediction error for a region
    pub fn record_error(&mut self, region_id: &str, error: f32) {
        let tracker = self.regions.entry(region_id.to_string()).or_insert(RegionErrorTracker {
            region_id: region_id.to_string(),
            consecutive_high_error_steps: 0,
            last_error: error,
            baseline_error: error,
        });

        tracker.last_error = error;
        
        // 动态更新基线误差（平滑）
        tracker.baseline_error = tracker.baseline_error * 0.99 + error * 0.01;

        // 检查是否高于阈值
        if error > tracker.baseline_error * self.config.error_threshold {
            tracker.consecutive_high_error_steps += 1;
        } else {
            tracker.consecutive_high_error_steps = 0;
        }
    }

    /// 检查区域是否应该进行神经元新生
    /// Check if a region should undergo neurogenesis
    pub fn should_grow(&self, region_id: &str) -> bool {
        if let Some(tracker) = self.regions.get(region_id) {
            tracker.consecutive_high_error_steps >= self.config.consecutive_steps_threshold
        } else {
            false
        }
    }

    /// 获取应进行新生的区域列表及新生数量
    /// Get regions that should undergo neurogenesis and the number of new neurons
    pub fn get_growth_candidates(&mut self) -> Vec<(String, u32)> {
        let mut candidates = Vec::new();
        let mut fib = self.config.fibonacci.clone();
        
        for (region_id, tracker) in &mut self.regions {
            if tracker.consecutive_high_error_steps >= self.config.consecutive_steps_threshold {
                let count = fib.next();
                candidates.push((region_id.clone(), count));
                // 重置计数器
                tracker.consecutive_high_error_steps = 0;
            }
        }
        
        // 更新配置中的斐波那契数列状态
        self.config.fibonacci = fib;
        candidates
    }

    /// 重置指定区域的追踪状态
    pub fn reset_region(&mut self, region_id: &str) {
        if let Some(tracker) = self.regions.get_mut(region_id) {
            tracker.consecutive_high_error_steps = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_neurogenesis_trigger() {
        let config = NeurogenesisConfig {
            error_threshold: 1.2,
            consecutive_steps_threshold: 5,
            ..Default::default()
        };
        let mut manager = NeurogenesisManager::new(config);
        
        // 记录多次高误差
        for _ in 0..6 {
            manager.record_error("region1", 0.5); // 假设基线 0.1，0.5 > 0.1*1.2
        }
        
        assert!(manager.should_grow("region1"));
        let candidates = manager.get_growth_candidates();
        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].1, 2); // 斐波那契第一项为 2 (1+1)
    }
}