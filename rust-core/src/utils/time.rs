//! 时间相关工具函数
//! Time-related utility functions
//!
//! 提供时间戳转换、天数计算等辅助功能。
//! Provides helper functions for timestamp conversion, day calculation, etc.

use chrono::{DateTime, Utc};

/// 将 Unix 时间戳（秒）转换为 DateTime<Utc>
/// Convert Unix timestamp (seconds) to DateTime<Utc>
pub fn timestamp_to_datetime(secs: u64) -> Option<DateTime<Utc>> {
    DateTime::from_timestamp(secs as i64, 0)
}

/// 获取当前 Unix 时间戳（秒）
/// Get current Unix timestamp in seconds
pub fn current_timestamp() -> u64 {
    Utc::now().timestamp() as u64
}

/// 计算两个时间戳之间的天数差
/// Calculate days difference between two timestamps
pub fn days_between(earlier_secs: u64, later_secs: u64) -> f64 {
    if later_secs >= earlier_secs {
        (later_secs - earlier_secs) as f64 / 86400.0
    } else {
        0.0
    }
}

/// 判断时间戳是否在最近 N 天内
/// Check if a timestamp is within the last N days
pub fn is_within_days(timestamp_secs: u64, days: u64) -> bool {
    let now = current_timestamp();
    if now >= timestamp_secs {
        (now - timestamp_secs) <= days * 86400
    } else {
        false
    }
}

/// 获取 N 天前的时间戳
/// Get timestamp of N days ago
pub fn days_ago_timestamp(days: u64) -> u64 {
    current_timestamp().saturating_sub(days * 86400)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp_conversion() {
        let now_secs = current_timestamp();
        let dt = timestamp_to_datetime(now_secs).unwrap();
        assert_eq!(dt.timestamp() as u64, now_secs);
    }

    #[test]
    fn test_days_between() {
        let t1 = 100000;
        let t2 = t1 + 86400;
        assert!((days_between(t1, t2) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_is_within_days() {
        let now = current_timestamp();
        let yesterday = now - 86400;
        assert!(is_within_days(yesterday, 2));
        assert!(!is_within_days(yesterday, 0));
    }
}