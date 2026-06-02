//! 网状库存点管理
//! Networked Memory Node Management
//!
//! 管理记忆库存点的生命周期：创建、合并、分裂、衰减。
//! Manages the lifecycle of memory nodes: creation, merging, splitting, decay.

use super::{EpisodicEvent, MemoryFragment};
use crate::memory::{Decayable, MemoryTier, TierManager};
use crate::utils::{current_timestamp, generate_node_id};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 记忆库存点
/// Memory Node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryNode {
    /// 库存点唯一标识
    pub node_id: String,
    /// 语义中心向量
    pub centroid_vector: Vec<f32>,
    /// 核心实体列表
    pub entities: Vec<String>,
    /// 最早事件时间戳
    pub time_start: u64,
    /// 最晚事件时间戳
    pub time_end: u64,
    /// 主导情感
    pub dominant_emotion: String,
    /// 内部碎片数量
    pub fragment_count: usize,
    /// 访问次数
    pub access_count: u64,
    /// 最后访问时间戳
    pub last_access: u64,
    /// 状态
    pub status: NodeStatus,
    /// 情感值（用于衰老阻尼）
    pub sentimental_value: f32,

    // 记忆管理字段
    pub weight: f32,
    pub tier: MemoryTier,
}

/// 库存点状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NodeStatus {
    Active,
    Dormant,
    Pruned,
}

impl Default for MemoryNode {
    fn default() -> Self {
        Self {
            node_id: generate_node_id(),
            centroid_vector: Vec::new(),
            entities: Vec::new(),
            time_start: 0,
            time_end: 0,
            dominant_emotion: "neutral".to_string(),
            fragment_count: 0,
            access_count: 0,
            last_access: current_timestamp(),
            status: NodeStatus::Active,
            sentimental_value: 0.0,
            weight: 1.0,
            tier: MemoryTier::Active,
        }
    }
}

impl crate::memory::HasId for MemoryNode {
    fn id(&self) -> &str {
        &self.node_id
    }
}

impl Decayable for MemoryNode {
    fn weight(&self) -> f32 { self.weight }
    fn set_weight(&mut self, w: f32) { self.weight = w.clamp(0.0, 1.0); }
    fn last_accessed(&self) -> u64 { self.last_access }
    fn set_last_accessed(&mut self, ts: u64) { self.last_access = ts; }
    fn access_frequency(&self) -> f32 { 0.0 } // 暂简化
    fn certainty(&self) -> f32 { 1.0 }
    fn tier(&self) -> MemoryTier { self.tier }
    fn set_tier(&mut self, t: MemoryTier) { self.tier = t; }
    fn is_core(&self) -> bool { false }
}

/// 库存点存储 trait
pub trait NodeStore: Send + Sync {
    fn create_node(&mut self, event: &EpisodicEvent) -> MemoryNode;
    fn get_node(&self, node_id: &str) -> Option<&MemoryNode>;
    fn get_node_mut(&mut self, node_id: &str) -> Option<&mut MemoryNode>;
    fn get_all_nodes(&self) -> Vec<&MemoryNode>;
    fn update_centroid(&mut self, node_id: &str, fragments: &[MemoryFragment]);
    fn merge_nodes(&mut self, node_a: &str, node_b: &str) -> Result<String, String>;
    fn split_node(&mut self, node_id: &str) -> Vec<String>;
    fn apply_decay(&mut self, tier_manager: &mut TierManager);
}

/// 网状库存点存储实现
#[derive(Debug, Default)]
pub struct NetworkedNodeStore {
    nodes: HashMap<String, MemoryNode>,
    entity_index: HashMap<String, Vec<String>>,
}

impl NetworkedNodeStore {
    pub fn new() -> Self {
        Self::default()
    }
}

impl NodeStore for NetworkedNodeStore {
    fn create_node(&mut self, event: &EpisodicEvent) -> MemoryNode {
        let mut node = MemoryNode::default();
        node.time_start = event.timestamp;
        node.time_end = event.timestamp;
        node.entities = event.perception_labels.entities.clone();
        node.dominant_emotion = event.emotion.clone();
        node.fragment_count = 1;
        
        if let Some(vec) = &event.feature_vector {
            node.centroid_vector = vec.clone();
        }

        // 更新实体索引
        for entity in &node.entities {
            self.entity_index
                .entry(entity.clone())
                .or_insert_with(Vec::new)
                .push(node.node_id.clone());
        }

        self.nodes.insert(node.node_id.clone(), node.clone());
        node
    }

    fn get_node(&self, node_id: &str) -> Option<&MemoryNode> {
        self.nodes.get(node_id)
    }

    fn get_node_mut(&mut self, node_id: &str) -> Option<&mut MemoryNode> {
        self.nodes.get_mut(node_id)
    }

    fn get_all_nodes(&self) -> Vec<&MemoryNode> {
        self.nodes.values().collect()
    }

    fn update_centroid(&mut self, node_id: &str, fragments: &[MemoryFragment]) {
        if let Some(node) = self.nodes.get_mut(node_id) {
            if fragments.is_empty() { return; }
            
            let dim = fragments[0].feature_vector.as_ref().map(|v| v.len()).unwrap_or(0);
            if dim == 0 { return; }

            let mut sum = vec![0.0; dim];
            let mut count = 0;
            for frag in fragments {
                if let Some(vec) = &frag.feature_vector {
                    for (i, v) in vec.iter().enumerate() {
                        sum[i] += v;
                    }
                    count += 1;
                }
            }
            if count > 0 {
                for v in &mut sum { *v /= count as f32; }
                node.centroid_vector = sum;
            }
        }
    }

    fn merge_nodes(&mut self, node_a: &str, node_b: &str) -> Result<String, String> {
        let node_b = match self.nodes.remove(node_b) {
            Some(n) => n,
            None => return Err("节点不存在".to_string()),
        };
        let node_a = match self.nodes.get_mut(node_a) {
            Some(n) => n,
            None => return Err("节点不存在".to_string()),
        };

        // 合并属性
        node_a.time_end = node_a.time_end.max(node_b.time_end);
        node_a.fragment_count += node_b.fragment_count;
        node_a.entities.extend(node_b.entities);
        node_a.entities.sort();
        node_a.entities.dedup();

        // 合并向量（简单平均）
        if !node_a.centroid_vector.is_empty() && !node_b.centroid_vector.is_empty() {
            let len = node_a.centroid_vector.len();
            for i in 0..len {
                node_a.centroid_vector[i] = (node_a.centroid_vector[i] + node_b.centroid_vector[i]) / 2.0;
            }
        }

        Ok(node_a.node_id.clone())
    }

    fn split_node(&mut self, _node_id: &str) -> Vec<String> {
        // 暂不实现复杂的聚类分裂
        Vec::new()
    }

    fn apply_decay(&mut self, tier_manager: &mut TierManager) {
        for node in self.nodes.values_mut() {
            tier_manager.decay_and_update(node);
        }
    }
}