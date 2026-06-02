//! 关联边管理
//! Edge Store Management
//!
//! 管理库存点之间的关联边（时序、话题、因果、情感、用户跳转）。
//! Manages edges between memory nodes (temporal, topic, causal, emotion, user jump).

use crate::utils::{current_timestamp, generate_edge_id};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 关联边类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EdgeType {
    Temporal,
    Topic,
    Causal,
    Emotion,
    UserJump,
}

/// 记忆关联边
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEdge {
    pub edge_id: String,
    pub source_node: String,
    pub target_node: String,
    pub edge_type: EdgeType,
    pub weight: f32,
    pub last_update: u64,
}

impl MemoryEdge {
    pub fn new(source: String, target: String, edge_type: EdgeType, weight: f32) -> Self {
        Self {
            edge_id: generate_edge_id(),
            source_node: source,
            target_node: target,
            edge_type,
            weight: weight.clamp(0.0, 1.0),
            last_update: current_timestamp(),
        }
    }
}

/// 边存储
#[derive(Debug, Default)]
pub struct EdgeStore {
    edges: HashMap<String, MemoryEdge>,
    adjacency: HashMap<String, Vec<String>>,
}

impl EdgeStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_edge(&mut self, edge: MemoryEdge) {
        let id = edge.edge_id.clone();
        let source = edge.source_node.clone();
        self.adjacency.entry(source).or_insert_with(Vec::new).push(id.clone());
        self.edges.insert(id, edge);
    }

    pub fn get_edge(&self, edge_id: &str) -> Option<&MemoryEdge> {
        self.edges.get(edge_id)
    }

    pub fn get_outgoing(&self, node_id: &str) -> Vec<&MemoryEdge> {
        self.adjacency
            .get(node_id)
            .map(|ids| ids.iter().filter_map(|id| self.edges.get(id)).collect())
            .unwrap_or_default()
    }

    pub fn update_weight(&mut self, edge_id: &str, delta: f32) {
        if let Some(edge) = self.edges.get_mut(edge_id) {
            edge.weight = (edge.weight + delta).clamp(0.0, 1.0);
            edge.last_update = current_timestamp();
        }
    }

    pub fn decay_all(&mut self, decay_rate: f32) {
        for edge in self.edges.values_mut() {
            edge.weight *= 1.0 - decay_rate;
            if edge.weight < 0.01 {
                edge.weight = 0.0;
            }
        }
    }

    pub fn prune_zero_weight(&mut self) -> usize {
        let before = self.edges.len();
        self.edges.retain(|_, e| e.weight > 0.0);
        // 清理邻接表
        for (_, ids) in self.adjacency.iter_mut() {
            ids.retain(|id| self.edges.contains_key(id));
        }
        before - self.edges.len()
    }
}