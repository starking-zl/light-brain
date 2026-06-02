//! 指标收集器
//! Metrics collector
//!
//! 用于收集生长效率、推理延迟、否决次数等运行时指标，支持生命之河汗水地图等要素。
//! Collects runtime metrics such as growth efficiency, inference latency, veto counts,
//! supporting River of Life elements like Sweat Map.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// 指标类型
/// Metric types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MetricType {
    /// 生长效率得分（神经）
    /// Growth efficiency score (neural)
    GrowthEfficiencyNeural,
    /// 生长效率得分（符号）
    /// Growth efficiency score (symbolic)
    GrowthEfficiencySymbolic,
    /// 推理延迟（毫秒）
    /// Inference latency in milliseconds
    InferenceLatencyMs,
    /// 否决触发次数
    /// Veto triggered count
    VetoTriggered,
    /// 接地置信度
    /// Grounding confidence
    GroundingConfidence,
    /// 知识冲突次数
    /// Knowledge conflict count
    KnowledgeConflictCount,
    /// 澄清触发次数
    /// Clarification triggered count
    ClarificationTriggered,
}

/// 指标条目
/// Metric entry
#[derive(Debug, Clone)]
pub struct MetricEntry {
    pub metric_type: MetricType,
    pub value: f64,
    pub timestamp: u64,
    pub labels: HashMap<String, String>,
}

/// 指标收集器
/// Metrics collector
#[derive(Debug, Clone, Default)]
pub struct MetricsCollector {
    entries: Arc<Mutex<Vec<MetricEntry>>>,
    counters: Arc<Mutex<HashMap<MetricType, u64>>>,
}

impl MetricsCollector {
    /// 创建新的指标收集器
    /// Create a new metrics collector
    pub fn new() -> Self {
        Self::default()
    }

    /// 记录一个指标值
    /// Record a metric value
    pub fn record(&self, metric_type: MetricType, value: f64) {
        let entry = MetricEntry {
            metric_type,
            value,
            timestamp: crate::utils::current_timestamp(),
            labels: HashMap::new(),
        };
        if let Ok(mut entries) = self.entries.lock() {
            entries.push(entry);
        }
        // 同时更新计数器 / Also update counter
        if let Ok(mut counters) = self.counters.lock() {
            *counters.entry(metric_type).or_insert(0) += 1;
        }
    }

    /// 记录带标签的指标值
    /// Record a metric value with labels
    pub fn record_with_labels(&self, metric_type: MetricType, value: f64, labels: HashMap<String, String>) {
        let entry = MetricEntry {
            metric_type,
            value,
            timestamp: crate::utils::current_timestamp(),
            labels,
        };
        if let Ok(mut entries) = self.entries.lock() {
            entries.push(entry);
        }
    }

    /// 获取指定类型的最近 N 条记录
    /// Get the most recent N records of a specified type
    pub fn get_recent(&self, metric_type: MetricType, limit: usize) -> Vec<MetricEntry> {
        if let Ok(entries) = self.entries.lock() {
            entries
                .iter()
                .filter(|e| e.metric_type == metric_type)
                .rev()
                .take(limit)
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }

    /// 计算指定类型最近 N 条记录的平均值
    /// Calculate the average of the most recent N records of a specified type
    pub fn average_recent(&self, metric_type: MetricType, n: usize) -> Option<f64> {
        let recent = self.get_recent(metric_type, n);
        if recent.is_empty() {
            return None;
        }
        let sum: f64 = recent.iter().map(|e| e.value).sum();
        Some(sum / recent.len() as f64)
    }

    /// 获取指定类型的总记录次数
    /// Get the total count of records for a specified type
    pub fn count(&self, metric_type: MetricType) -> u64 {
        if let Ok(counters) = self.counters.lock() {
            counters.get(&metric_type).copied().unwrap_or(0)
        } else {
            0
        }
    }

    /// 清空所有指标
    /// Clear all metrics
    pub fn clear(&self) {
        if let Ok(mut entries) = self.entries.lock() {
            entries.clear();
        }
        if let Ok(mut counters) = self.counters.lock() {
            counters.clear();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_and_retrieve() {
        let collector = MetricsCollector::new();
        collector.record(MetricType::GrowthEfficiencyNeural, 0.8);
        collector.record(MetricType::GrowthEfficiencyNeural, 0.9);
        
        let recent = collector.get_recent(MetricType::GrowthEfficiencyNeural, 2);
        assert_eq!(recent.len(), 2);
        assert_eq!(recent[0].value, 0.9);
        assert_eq!(recent[1].value, 0.8);
        
        let avg = collector.average_recent(MetricType::GrowthEfficiencyNeural, 2).unwrap();
        assert!((avg - 0.85).abs() < 0.001);
    }

    #[test]
    fn test_count() {
        let collector = MetricsCollector::new();
        assert_eq!(collector.count(MetricType::VetoTriggered), 0);
        collector.record(MetricType::VetoTriggered, 1.0);
        assert_eq!(collector.count(MetricType::VetoTriggered), 1);
    }
}