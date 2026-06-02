//! 记忆巩固
//! Memory Consolidation
//!
//! 将高频情景事件转化为小脑知识候选。
//! Converts high-frequency episodic events into Cerebellum knowledge candidates.

use super::{EpisodicEvent, EventStore, KnowledgeCandidate, MemoryFragment, NodeStore};
use crate::memory::TierManager;

/// 巩固引擎
pub struct ConsolidationEngine {
    access_threshold: u32,
}

impl ConsolidationEngine {
    pub fn new() -> Self {
        Self {
            access_threshold: 20,
        }
    }

    pub fn with_threshold(mut self, threshold: u32) -> Self {
        self.access_threshold = threshold;
        self
    }

    pub fn consolidate(
        &self,
        event_store: &mut dyn EventStore,
        node_store: &mut dyn NodeStore,
        tier_manager: &mut TierManager,
    ) -> Vec<KnowledgeCandidate> {
        let mut candidates = Vec::new();

        // 遍历所有库存点，统计访问次数
        for node in node_store.get_all_nodes() {
            if node.access_count >= self.access_threshold as u64 {
                // 收集该节点的所有事件
                let events = event_store.get_by_node(&node.node_id);
                if events.is_empty() {
                    continue;
                }

                // 尝试提取知识三元组
                if let Some(candidate) = self.extract_knowledge(&node.node_id, &events) {
                    candidates.push(candidate);
                }

                // 标记节点已被巩固（重置访问计数）
                if let Some(node_mut) = node_store.get_node_mut(&node.node_id) {
                    node_mut.access_count = 0;
                }
            }
        }

        candidates
    }

    fn extract_knowledge(&self, node_id: &str, events: &[EpisodicEvent]) -> Option<KnowledgeCandidate> {
        // 简化实现：从第一个事件中提取主体和属性
        // 实际应使用更复杂的 NLP 抽取
        let event = events.first()?;
        let entities = &event.perception_labels.entities;
        if entities.len() < 2 {
            return None;
        }

        Some(KnowledgeCandidate {
            subject: entities[0].clone(),
            attribute: entities.get(1).unwrap_or(&"描述".to_string()).clone(),
            value: serde_json::Value::String(event.response.clone()),
            certainty: 0.5,
            source_event_ids: vec![event.id.clone()],
        })
    }
}