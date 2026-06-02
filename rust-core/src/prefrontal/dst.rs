//! 对话状态跟踪器
//! Dialog State Tracker
//!
//! 维护话题栈、实体指代表和待决问题队列。
//! Maintains topic stack, entity reference table, and pending question queue.

use crate::hippocampus::EpisodicEvent;
use std::collections::{HashMap, VecDeque};

/// 话题
#[derive(Debug, Clone)]
pub struct Topic {
    pub name: String,
    pub start_turn: usize,
    pub last_turn: usize,
}

/// 待决问题
#[derive(Debug, Clone)]
pub struct PendingQuestion {
    pub id: String,
    pub question: String,
    pub raised_at: usize,
    pub resolved: bool,
}

/// 对话状态跟踪器
#[derive(Debug, Default)]
pub struct DialogStateTracker {
    topic_stack: Vec<Topic>,
    entity_reference: HashMap<String, f32>, // 实体 -> 显著性
    pending_questions: VecDeque<PendingQuestion>,
    turn_count: usize,
}

impl DialogStateTracker {
    pub fn new() -> Self {
        Self::default()
    }

    /// 更新对话状态
    pub fn update(&mut self, event: &EpisodicEvent) {
        self.turn_count += 1;

        // 更新话题栈
        let topic_name = event.perception_labels.domain.clone();
        if let Some(topic) = self.topic_stack.last_mut() {
            if topic.name == topic_name {
                topic.last_turn = self.turn_count;
            } else {
                self.topic_stack.push(Topic {
                    name: topic_name,
                    start_turn: self.turn_count,
                    last_turn: self.turn_count,
                });
            }
        } else {
            self.topic_stack.push(Topic {
                name: topic_name,
                start_turn: self.turn_count,
                last_turn: self.turn_count,
            });
        }

        // 更新实体显著性
        for entity in &event.perception_labels.entities {
            *self.entity_reference.entry(entity.clone()).or_insert(0.0) += 1.0;
        }
        // 衰减
        for salience in self.entity_reference.values_mut() {
            *salience *= 0.9;
        }
    }

    /// 获取当前话题
    pub fn current_topic(&self) -> Option<&Topic> {
        self.topic_stack.last()
    }

    /// 消解指代（简化）
    pub fn resolve_pronoun(&self, pronoun: &str) -> Option<String> {
        self.entity_reference
            .iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(e, _)| e.clone())
    }
}