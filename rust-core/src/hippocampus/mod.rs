//! 海马体模块
//! Hippocampus Module
//!
//! 海马体是光脑方案的情景记忆中枢，负责记录对话事件、构建网状库存点、
//! 支持图扩散检索和记忆巩固。
//! The Hippocampus is the episodic memory center of the Light-Brain Scheme,
//! responsible for recording conversation events, building networked memory nodes,
//! supporting graph diffusion retrieval and memory consolidation.

mod event_store;
mod node_store;
mod edge_store;
mod graph_walker;
mod retrieval;
mod consolidation;
mod creative_incubator;

pub use event_store::*;
pub use node_store::*;
pub use edge_store::*;
pub use graph_walker::*;
pub use retrieval::*;
pub use consolidation::*;
pub use creative_incubator::*;

use crate::memory::{Decayable, MemoryTier, TierManager};
use crate::utils::{current_timestamp, generate_event_id, generate_node_id, generate_edge_id};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 情景事件
/// Episodic Event
///
/// 记录单次对话交互的完整信息。
/// Records complete information of a single conversational interaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodicEvent {
    /// 事件唯一标识
    /// Event unique identifier
    pub id: String,
    /// 时间戳（秒）
    /// Timestamp in seconds
    pub timestamp: u64,
    /// 用户输入
    /// User input
    pub user_input: String,
    /// 丘脑感知标签（意图、情感极性、话题等）
    /// Thalamus perception labels (intent, emotional polarity, topic, etc.)
    pub perception_labels: PerceptionLabels,
    /// 前额叶决策包（可选序列化）
    /// Prefrontal decision package (optional serialized)
    pub decision_package: Option<String>,
    /// 布罗卡区生成的回复
    /// Broca's Area generated response
    pub response: String,
    /// 所属库存点 ID（若已分配）
    /// Associated memory node ID (if assigned)
    pub node_id: Option<String>,
    /// 事件是否曾被用户纠正
    /// Whether this event was corrected by the user
    pub was_corrected: bool,
    /// 情感标记
    /// Emotion tag
    pub emotion: String,
    /// 重要性权重 (0.0 ~ 1.0)
    /// Importance weight (0.0 ~ 1.0)
    pub importance: f32,
    /// 模态类型（text / image / audio）
    /// Modality type (text / image / audio)
    pub modality: String,
    /// 资产 URI（多模态时指向文件）
    /// Asset URI (points to file for multimodal)
    pub asset_uri: Option<String>,
    /// 特征向量（用于语义检索）
    /// Feature vector (for semantic retrieval)
    pub feature_vector: Option<Vec<f32>>,
}

/// 丘脑感知标签
/// Thalamus perception labels
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerceptionLabels {
    /// 意图类别
    pub intent: String,
    /// 情感极性 (-1.0 ~ 1.0)
    pub polarity: f32,
    /// 话题领域
    pub domain: String,
    /// 关键词列表
    pub keywords: Vec<String>,
    /// 实体列表
    pub entities: Vec<String>,
    /// 接地置信度
    pub grounding_confidence: f32,
}

impl Default for EpisodicEvent {
    fn default() -> Self {
        Self {
            id: generate_event_id(),
            timestamp: current_timestamp(),
            user_input: String::new(),
            perception_labels: PerceptionLabels::default(),
            decision_package: None,
            response: String::new(),
            node_id: None,
            was_corrected: false,
            emotion: "neutral".to_string(),
            importance: 0.5,
            modality: "text".to_string(),
            asset_uri: None,
            feature_vector: None,
        }
    }
}

impl EpisodicEvent {
    /// 创建新的事件
    /// Create a new event
    pub fn new(user_input: String, response: String, labels: PerceptionLabels) -> Self {
        Self {
            user_input,
            response,
            perception_labels: labels,
            ..Default::default()
        }
    }

    /// 设置重要性
    /// Set importance
    pub fn with_importance(mut self, importance: f32) -> Self {
        self.importance = importance.clamp(0.0, 1.0);
        self
    }

    /// 设置情感
    /// Set emotion
    pub fn with_emotion(mut self, emotion: &str) -> Self {
        self.emotion = emotion.to_string();
        self
    }

    /// 标记为被纠正
    /// Mark as corrected
    pub fn mark_corrected(&mut self) {
        self.was_corrected = true;
    }
}

/// 为 EpisodicEvent 实现 HasId trait
impl crate::memory::HasId for EpisodicEvent {
    fn id(&self) -> &str {
        &self.id
    }
}

/// 海马体 trait
/// Hippocampus trait
pub trait Hippocampus: Send + Sync {
    /// 存储事件
    /// Store an event
    fn store_event(&mut self, event: EpisodicEvent) -> Result<String, String>;

    /// 检索印象包（根据查询意图）
    /// Retrieve impression pack based on query intent
    fn retrieve_impression(&mut self, query: &QueryIntent) -> ImpressionPack;

    /// 执行记忆巩固（将高频事件转化为小脑知识候选）
    /// Perform memory consolidation (convert high-frequency events to knowledge candidates)
    fn consolidate(&mut self) -> Vec<KnowledgeCandidate>;

    /// 获取事件存储引用
    /// Get reference to event store
    fn event_store(&self) -> &dyn EventStore;

    /// 获取库存点存储引用
    /// Get reference to node store
    fn node_store(&self) -> &dyn NodeStore;
}

/// 查询意图
/// Query Intent
#[derive(Debug, Clone)]
pub struct QueryIntent {
    /// 查询模式
    pub query_mode: QueryMode,
    /// 当前实体列表
    pub current_entities: Vec<String>,
    /// 话题向量（来自丘脑）
    pub topic_vector: Vec<f32>,
    /// 时间提示（如 "最近三天"）
    pub time_hint: Option<String>,
    /// 最大结果数
    pub max_results: usize,
    /// 模态提示
    pub modality_hint: Option<String>,
}

/// 查询模式
/// Query Mode
#[derive(Debug, Clone)]
pub enum QueryMode {
    /// 话题网捞取（宽扩散）
    TopicTrawl { depth: usize, decay: f32 },
    /// 多线索试探（多入口并行）
    MultiProbe { depth: usize },
    /// 时间线浏览
    Timeline,
    /// 预取缓存
    Prefetch,
    /// 慢思考检索（深游走）
    DeepLink { depth: usize, decay: f32 },
}

impl Default for QueryIntent {
    fn default() -> Self {
        Self {
            query_mode: QueryMode::TopicTrawl { depth: 2, decay: 0.5 },
            current_entities: Vec::new(),
            topic_vector: Vec::new(),
            time_hint: None,
            max_results: 5,
            modality_hint: None,
        }
    }
}

/// 印象包
/// Impression Pack
///
/// 包含检索到的记忆碎片及其扩散路径。
/// Contains retrieved memory fragments and their diffusion paths.
#[derive(Debug, Clone)]
pub struct ImpressionPack {
    /// 记忆碎片列表
    pub fragments: Vec<MemoryFragment>,
    /// 扩散追踪路径（入口 → ... → 库存点）
    pub trace: Vec<String>,
}

/// 记忆碎片
/// Memory Fragment
///
/// 从情景事件中提取的记忆片段。
/// Memory fragment extracted from episodic events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryFragment {
    /// 碎片 ID（可复用事件 ID）
    pub fragment_id: String,
    /// 所属库存点 ID
    pub node_id: String,
    /// 模态类型
    pub modality: String,
    /// 内容摘要
    pub content: String,
    /// 特征向量
    pub feature_vector: Option<Vec<f32>>,
    /// 资产 URI
    pub asset_uri: Option<String>,
    /// 时间戳
    pub timestamp: u64,
    /// 情感标记
    pub emotion: String,
    /// 重要性权重
    pub importance: f32,
    /// 匹配得分
    pub relevance_score: f32,
}

/// 知识候选（用于巩固至小脑）
/// Knowledge Candidate (for consolidation to Cerebellum)
#[derive(Debug, Clone)]
pub struct KnowledgeCandidate {
    /// 主体
    pub subject: String,
    /// 属性
    pub attribute: String,
    /// 值
    pub value: serde_json::Value,
    /// 确定性（基于事件重要性、频次等计算）
    pub certainty: f32,
    /// 来源事件 ID 列表
    pub source_event_ids: Vec<String>,
}

/// 海马体标准实现
/// Standard Hippocampus implementation
#[derive(Debug)]
pub struct StandardHippocampus {
    event_store: SqliteEventStore,
    node_store: NetworkedNodeStore,
    edge_store: EdgeStore,
    tier_manager: TierManager,
    consolidation_engine: ConsolidationEngine,
    retrieval_engine: RetrievalEngine,
}

impl StandardHippocampus {
    /// 创建新的海马体实例
    /// Create a new Hippocampus instance
    pub fn new(db_path: &str) -> Result<Self, String> {
        let event_store = SqliteEventStore::new(db_path)?;
        let node_store = NetworkedNodeStore::new();
        let edge_store = EdgeStore::new();
        let tier_manager = TierManager::default();
        let consolidation_engine = ConsolidationEngine::new();
        let retrieval_engine = RetrievalEngine::new();

        Ok(Self {
            event_store,
            node_store,
            edge_store,
            tier_manager,
            consolidation_engine,
            retrieval_engine,
        })
    }
}

impl Hippocampus for StandardHippocampus {
    fn store_event(&mut self, event: EpisodicEvent) -> Result<String, String> {
        // 存储事件
        let event_id = self.event_store.insert(event)?;

        // 异步触发库存点分配（简化：立即分配）
        // TODO: 实际应根据语义相似度分配到现有节点或创建新节点

        Ok(event_id)
    }

    fn retrieve_impression(&mut self, query: &QueryIntent) -> ImpressionPack {
        self.retrieval_engine.retrieve(
            &mut self.event_store,
            &mut self.node_store,
            &mut self.edge_store,
            query,
            &mut self.tier_manager,
        )
    }

    fn consolidate(&mut self) -> Vec<KnowledgeCandidate> {
        self.consolidation_engine.consolidate(
            &mut self.event_store,
            &mut self.node_store,
            &mut self.tier_manager,
        )
    }

    fn event_store(&self) -> &dyn EventStore {
        &self.event_store
    }

    fn node_store(&self) -> &dyn NodeStore {
        &self.node_store
    }
}