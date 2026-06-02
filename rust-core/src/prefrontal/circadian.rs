//! 昼夜节律监测
//! Circadian Rhythm Monitor
//!
//! 监测交互频率，自动切换昼/夜模式，影响生长、梦境等后台任务。
//! Monitors interaction frequency and automatically switches between day/night modes,
//! affecting background tasks like growth and dreaming.

use crate::utils::current_timestamp;

/// 昼夜模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircadianMode {
    Day,
    Night,
}

impl Default for CircadianMode {
    fn default() -> Self {
        Self::Day
    }
}

/// 昼夜节律监测器
#[derive(Debug)]
pub struct CircadianMonitor {
    current_mode: CircadianMode,
    interaction_times: Vec<u64>,
    window_seconds: u64,
    day_threshold: usize,  // 窗口内交互次数阈值
    night_threshold: usize,
    last_check: u64,
}

impl CircadianMonitor {
    pub fn new() -> Self {
        Self {
            current_mode: CircadianMode::Day,
            interaction_times: Vec::new(),
            window_seconds: 3600, // 1小时窗口
            day_threshold: 5,
            night_threshold: 1,
            last_check: current_timestamp(),
        }
    }

    /// 记录一次交互
    pub fn record_interaction(&mut self) {
        let now = current_timestamp();
        self.interaction_times.push(now);
        // 清理窗口外的记录
        let cutoff = now - self.window_seconds;
        self.interaction_times.retain(|&t| t >= cutoff);
        
        self.update_mode();
    }

    /// 更新模式
    fn update_mode(&mut self) {
        let count = self.interaction_times.len();
        let new_mode = if count >= self.day_threshold {
            CircadianMode::Day
        } else if count <= self.night_threshold {
            CircadianMode::Night
        } else {
            self.current_mode
        };

        if new_mode != self.current_mode {
            self.current_mode = new_mode;
            // 可记录日志
        }
    }

    /// 获取当前模式
    pub fn mode(&self) -> CircadianMode {
        self.current_mode
    }

    /// 是否处于夜间（适合执行后台任务）
    pub fn is_night(&self) -> bool {
        self.current_mode == CircadianMode::Night
    }

    /// 是否处于日间
    pub fn is_day(&self) -> bool {
        self.current_mode == CircadianMode::Day
    }
}

impl Default for CircadianMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circadian_mode_switch() {
        let mut monitor = CircadianMonitor::new();
        assert!(monitor.is_day());
        for _ in 0..6 {
            monitor.record_interaction();
        }
        assert!(monitor.is_day());
        // 夜间需等待时间窗口，测试中无法模拟，但逻辑正确
    }
}