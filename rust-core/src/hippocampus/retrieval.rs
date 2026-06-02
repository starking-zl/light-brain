//! 多模式检索引擎
//! Multi-Mode Retrieval Engine

use super::{
    EdgeStore, EpisodicEvent, EventStore, ImpressionPack, MemoryFragment,
    MemoryNode, NetworkedNodeStore, NodeStore, QueryIntent, QueryMode,
};
use crate::memory::TierManager;
use crate::utils::current_timestamp;
use std::collections::HashSet;

/// 检索引擎
/// Retrieval engine
pub struct RetrievalEngine;

impl RetrievalEngine {
    /// 创建新的检索引擎
    /// Create a new retrieval engine
    pub fn new() -> Self {
        Self
    }

    /// 根据查询意图执行检索
    /// Execute retrieval based on query intent
    pub fn retrieve(
        &self,
        event_store: &mut dyn EventStore,
        node_store: &mut dyn NodeStore,
        edge_store: &mut EdgeStore,
        query: &QueryIntent,
        tier_manager: &mut TierManager,
    ) -> ImpressionPack {
        // tier_manager 预留用于后续完善访问计数与衰减管理
        // tier_manager is reserved for future access count and decay management
        let _ = tier_manager;

        match query.query_mode {
            QueryMode::TopicTrawl { depth, decay } => {
                self.topic_trawl(event_store, node_store, edge_store, query, depth, decay)
            }
            QueryMode::Timeline => {
                self.timeline(event_store, query)
            }
            _ => self.default_retrieval(event_store, query),
        }
    }

    fn topic_trawl(
        &self,
        event_store: &mut dyn EventStore,
        node_store: &mut dyn NodeStore,
        edge_store: &mut EdgeStore,
        query: &QueryIntent,
        depth: usize,
        decay: f32,
    ) -> ImpressionPack {
        // 1. 定位入口节点
        let entry_nodes = self.locate_entry_nodes(node_store, &query.current_entities, &query.topic_vector);
        
        // 2. 图扩散
        let walker = super::graph_walker::GraphWalker::new(depth, decay);
        let diffusion = walker.walk(&entry_nodes, node_store, edge_store);
        
        // 3. 收集碎片
        let mut fragments = Vec::new();
        let mut seen = HashSet::new();
        
        for (node_id, activation) in diffusion.activated_nodes {
            if let Some(node) = node_store.get_node(&node_id) {
                // 标记节点被访问（未来可通过 tier_manager 实现）
                // Mark node as accessed (future implementation via tier_manager)
                let _ = node;
                
                let events = event_store.get_by_node(&node_id);
                for event in events {
                    if seen.insert(event.id.clone()) {
                        fragments.push(MemoryFragment {
                            fragment_id: event.id.clone(),
                            node_id: node_id.clone(),
                            modality: event.modality,
                            content: format!("Q: {}\nA: {}", event.user_input, event.response),
                            feature_vector: event.feature_vector,
                            asset_uri: event.asset_uri,
                            timestamp: event.timestamp,
                            emotion: event.emotion,
                            importance: event.importance,
                            relevance_score: activation * event.importance,
                        });
                    }
                }
            }
        }

        // 4. 排序并截取
        fragments.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap());
        fragments.truncate(query.max_results);

        ImpressionPack {
            fragments,
            trace: diffusion.path_trace,
        }
    }

    fn timeline(
        &self,
        event_store: &mut dyn EventStore,
        query: &QueryIntent,
    ) -> ImpressionPack {
        let now = current_timestamp();
        let start = query.time_hint.as_ref()
            .and_then(|h| parse_time_hint(h))
            .unwrap_or(now - 7 * 86400); // 默认最近7天 / default last 7 days
        
        let events = event_store.get_by_time_range(start, now);
        let fragments: Vec<MemoryFragment> = events.into_iter()
            .take(query.max_results)
            .map(|e| MemoryFragment {
                fragment_id: e.id,
                node_id: e.node_id.unwrap_or_default(),
                modality: e.modality,
                content: format!("Q: {}\nA: {}", e.user_input, e.response),
                feature_vector: e.feature_vector,
                asset_uri: e.asset_uri,
                timestamp: e.timestamp,
                emotion: e.emotion,
                importance: e.importance,
                relevance_score: 1.0,
            })
            .collect();

        ImpressionPack {
            fragments,
            trace: vec!["timeline".to_string()],
        }
    }

    fn default_retrieval(
        &self,
        event_store: &mut dyn EventStore,
        query: &QueryIntent,
    ) -> ImpressionPack {
        let events = event_store.get_all();
        let fragments: Vec<MemoryFragment> = events.into_iter()
            .take(query.max_results)
            .map(|e| MemoryFragment {
                fragment_id: e.id,
                node_id: e.node_id.unwrap_or_default(),
                modality: e.modality,
                content: format!("Q: {}\nA: {}", e.user_input, e.response),
                feature_vector: e.feature_vector,
                asset_uri: e.asset_uri,
                timestamp: e.timestamp,
                emotion: e.emotion,
                importance: e.importance,
                relevance_score: 1.0,
            })
            .collect();

        ImpressionPack {
            fragments,
            trace: vec!["default".to_string()],
        }
    }

    fn locate_entry_nodes(
        &self,
        node_store: &dyn NodeStore,
        entities: &[String],
        topic_vector: &[f32],
    ) -> Vec<String> {
        // 简化实现：通过实体匹配
        // Simplified implementation: match by entities
        let mut candidates = Vec::new();
        for node in node_store.get_all_nodes() {
            for entity in entities {
                if node.entities.contains(entity) {
                    candidates.push(node.node_id.clone());
                    break;
                }
            }
        }
        candidates.truncate(3);
        candidates
    }
}

fn parse_time_hint(hint: &str) -> Option<u64> {
    // 简化：处理"最近N天"格式
    // Simplified: handle "recent N days" format
    if hint.contains("天") {
        let days: u64 = hint.chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse().ok()?;
        Some(current_timestamp() - days * 86400)
    } else {
        None
    }
}