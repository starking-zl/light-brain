//! 工作记忆
//! Working Memory
//!
//! 维护最近对话的上下文状态，支持组块压缩和工作记忆库分页。
//! Maintains recent conversation context, supports chunk compression and working memory store paging.

use crate::hippocampus::EpisodicEvent;
use crate::thalamus::PerceptionLabels;
use std::collections::VecDeque;

/// 工作记忆槽位
/// Working memory slot
#[derive(Debug, Clone)]
pub struct WorkingMemorySlot {
    pub intent: String,
    pub entities: Vec<String>,
    pub topic_vector: Vec<f32>,
    pub timestamp: u64,
    pub importance: f32,
}

/// 工作记忆
/// Working memory
#[derive(Debug)]
pub struct WorkingMemory {
    slots: VecDeque<WorkingMemorySlot>,
    capacity: usize,
    context_vector: Vec<f32>,
    context_gate: f32,
}

impl WorkingMemory {
    pub fn new() -> Self {
        Self {
            slots: VecDeque::new(),
            capacity: 7,
            context_vector: Vec::new(),
            context_gate: 0.7,
        }
    }

    /// 更新工作记忆（每轮对话结束时调用）
    pub fn update(&mut self, event: &EpisodicEvent) {
        let slot = WorkingMemorySlot {
            intent: event.perception_labels.intent.clone(),
            entities: event.perception_labels.entities.clone(),
            topic_vector: event.feature_vector.clone().unwrap_or_default(),
            timestamp: event.timestamp,
            importance: event.importance,
        };

        self.slots.push_back(slot);
        if self.slots.len() > self.capacity {
            self.slots.pop_front();
        }

        self.update_context_vector();
    }

    /// 更新上下文向量
    fn update_context_vector(&mut self) {
        if let Some(feature) = self.slots.back().and_then(|s| {
            if s.topic_vector.is_empty() { None } else { Some(&s.topic_vector) }
        }) {
            if self.context_vector.is_empty() {
                self.context_vector = feature.clone();
            } else {
                // 门控更新
                for i in 0..self.context_vector.len().min(feature.len()) {
                    self.context_vector[i] = self.context_gate * self.context_vector[i]
                        + (1.0 - self.context_gate) * feature[i];
                }
            }
        }
    }

    /// 获取当前上下文向量
    pub fn context_vector(&self) -> Vec<f32> {
        self.context_vector.clone()
    }

    /// 获取所有槽位
    pub fn slots(&self) -> &VecDeque<WorkingMemorySlot> {
        &self.slots
    }
}

impl Default for WorkingMemory {
    fn default() -> Self {
        Self::new()
    }
}