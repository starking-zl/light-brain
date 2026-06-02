//! 时间提供者接口与实现
//! Time provider trait and implementations
//!
//! 提供可插拔的时间获取机制，便于在测试中注入模拟时间。
//! Provides a pluggable time acquisition mechanism for easier testing with mock time.

use std::sync::Arc;
use chrono::{DateTime, Utc};

/// 时间提供者 trait
/// Time provider trait
pub trait TimeProvider: Send + Sync {
    /// 获取当前 UTC 时间戳（秒）
    /// Get current UTC timestamp in seconds
    fn now_secs(&self) -> u64;

    /// 获取当前 UTC 日期时间
    /// Get current UTC datetime
    fn now_datetime(&self) -> DateTime<Utc>;

    /// 计算从某个时间戳至今的天数
    /// Calculate days elapsed since a given timestamp
    fn days_elapsed(&self, since_secs: u64) -> f64 {
        let now = self.now_secs();
        if now >= since_secs {
            (now - since_secs) as f64 / 86400.0
        } else {
            0.0
        }
    }
}

/// 系统时间提供者（生产环境使用）
/// System time provider (for production use)
#[derive(Debug, Clone, Default)]
pub struct SystemTimeProvider;

impl TimeProvider for SystemTimeProvider {
    fn now_secs(&self) -> u64 {
        Utc::now().timestamp() as u64
    }

    fn now_datetime(&self) -> DateTime<Utc> {
        Utc::now()
    }
}

/// 模拟时间提供者（用于测试）
/// Mock time provider (for testing)
#[derive(Debug, Clone)]
pub struct MockTimeProvider {
    current_time: Arc<std::sync::Mutex<u64>>,
}

impl MockTimeProvider {
    /// 创建新的模拟时间提供者，设置初始时间
    /// Create a new mock time provider with initial time
    pub fn new(initial_time_secs: u64) -> Self {
        Self {
            current_time: Arc::new(std::sync::Mutex::new(initial_time_secs)),
        }
    }

    /// 推进时间（秒）
    /// Advance time by seconds
    pub fn advance(&self, seconds: u64) {
        let mut time = self.current_time.lock().unwrap();
        *time += seconds;
    }

    /// 设置时间
    /// Set time to a specific value
    pub fn set(&self, time_secs: u64) {
        let mut time = self.current_time.lock().unwrap();
        *time = time_secs;
    }
}

impl TimeProvider for MockTimeProvider {
    fn now_secs(&self) -> u64 {
        *self.current_time.lock().unwrap()
    }

    fn now_datetime(&self) -> DateTime<Utc> {
        DateTime::from_timestamp(self.now_secs() as i64, 0)
            .unwrap_or_else(|| Utc::now())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_time_provider() {
        let provider = SystemTimeProvider;
        let now = provider.now_secs();
        assert!(now > 0);
    }

    #[test]
    fn test_mock_time_provider() {
        let mock = MockTimeProvider::new(1000);
        assert_eq!(mock.now_secs(), 1000);
        mock.advance(3600);
        assert_eq!(mock.now_secs(), 4600);
        mock.set(5000);
        assert_eq!(mock.now_secs(), 5000);
    }

    #[test]
    fn test_days_elapsed() {
        let mock = MockTimeProvider::new(100000);
        let since = 100000 - 86400; // 1 天前 / 1 day ago
        assert!((mock.days_elapsed(since) - 1.0).abs() < 0.001);
    }
}