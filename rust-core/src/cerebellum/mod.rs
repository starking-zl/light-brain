//! 小脑模块
//! Cerebellum Module
//!
//! 小脑是光脑方案的语义记忆存储中枢，负责管理结构化知识图谱的增删改查操作。
//! 支持三层记忆流转、多策略检索和冲突检测。
//! The Cerebellum is the semantic memory storage center of the Light-Brain Scheme,
//! responsible for CRUD operations on the structured knowledge graph.
//! Supports three-tier memory flow, multi-strategy retrieval, and conflict detection.

mod graph_store;
mod query;
mod decay;
mod conflict;
mod core_knowledge;

pub use graph_store::*;
pub use query::*;
pub use decay::*;
pub use conflict::*;
pub use core_knowledge::*;

use crate::memory::{Decayable, MemoryTier, TierManager};
use crate::utils::{current_timestamp, generate_knowledge_id};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 知识条目
/// Knowledge Entry
///
/// 小脑中存储的基本知识单元，采用主体-属性-值三元组形式。
/// The basic knowledge unit stored in Cerebellum, in subject-attribute-value triple form.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeEntry {
    /// 唯一标识
    /// Unique identifier
    pub id: String,
    /// 主体
    /// Subject
    pub subject: String,
    /// 属性
    /// Attribute
    pub attribute: String,
    /// 值
    /// Value
    pub value: serde_json::Value,
    /// 确定性评分 (0.0 ~ 1.0)
    /// Certainty score (0.0 ~ 1.0)
    pub certainty: f32,
    /// 标签列表
    /// Tags
    pub tags: Vec<String>,
    /// 描述文本（用于关键词检索）
    /// Description text (for keyword retrieval)
    pub description: String,
    /// 来源标记（如 "user_confirmed", "extracted", "seed"）
    /// Source label (e.g., "user_confirmed", "extracted", "seed")
    pub source: String,
    /// 是否为受保护的核心知识
    /// Whether this is protected core knowledge
    pub core: bool,
    /// 情感值（用于衰老机制的情感阻尼）
    /// Sentimental value (for sentimental damping in aging)
    pub sentimental_value: f32,

    // 记忆管理字段
    /// 当前权重
    pub weight: f32,
    /// 最后访问时间戳（秒）
    pub last_accessed: u64,
    /// 近三十日平均访问频次
    pub access_frequency: f32,
    /// 当前记忆层级
    pub tier: MemoryTier,
    /// 创建时间戳
    pub created_at: u64,
}

impl Default for KnowledgeEntry {
    fn default() -> Self {
        let now = current_timestamp();
        Self {
            id: generate_knowledge_id(),
            subject: String::new(),
            attribute: String::new(),
            value: serde_json::Value::Null,
            certainty: 0.5,
            tags: Vec::new(),
            description: String::new(),
            source: "unknown".to_string(),
            core: false,
            sentimental_value: 0.0,
            weight: 1.0,
            last_accessed: now,
            access_frequency: 0.0,
            tier: MemoryTier::Active,
            created_at: now,
        }
    }
}

impl KnowledgeEntry {
    /// 创建新的知识条目
    /// Create a new knowledge entry
    pub fn new(
        subject: String,
        attribute: String,
        value: serde_json::Value,
        certainty: f32,
    ) -> Self {
        let now = current_timestamp();
        Self {
            id: generate_knowledge_id(),
            subject,
            attribute,
            value,
            certainty,
            weight: 1.0,
            last_accessed: now,
            access_frequency: 0.0,
            tier: MemoryTier::Active,
            created_at: now,
            ..Default::default()
        }
    }

    /// 标记为核心知识
    /// Mark as core knowledge
    pub fn as_core(mut self) -> Self {
        self.core = true;
        self
    }

    /// 设置来源
    /// Set source
    pub fn with_source(mut self, source: &str) -> Self {
        self.source = source.to_string();
        self
    }

    /// 设置标签
    /// Set tags
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    /// 设置描述
    /// Set description
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    /// 设置情感值
    /// Set sentimental value
    pub fn with_sentimental_value(mut self, value: f32) -> Self {
        self.sentimental_value = value;
        self
    }
}

/// 为 KnowledgeEntry 实现 HasId trait（来自 memory 模块）
impl crate::memory::HasId for KnowledgeEntry {
    fn id(&self) -> &str {
        &self.id
    }
}

/// 为 KnowledgeEntry 实现 Decayable trait
impl Decayable for KnowledgeEntry {
    fn weight(&self) -> f32 {
        self.weight
    }

    fn set_weight(&mut self, weight: f32) {
        self.weight = weight.clamp(0.0, 1.0);
    }

    fn last_accessed(&self) -> u64 {
        self.last_accessed
    }

    fn set_last_accessed(&mut self, timestamp: u64) {
        self.last_accessed = timestamp;
    }

    fn access_frequency(&self) -> f32 {
        self.access_frequency
    }

    fn certainty(&self) -> f32 {
        self.certainty
    }

    fn tier(&self) -> MemoryTier {
        self.tier
    }

    fn set_tier(&mut self, tier: MemoryTier) {
        self.tier = tier;
    }

    fn is_core(&self) -> bool {
        self.core
    }
}

/// 小脑 trait
/// Cerebellum trait
///
/// 定义小脑对外提供的标准接口。
/// Defines the standard interface exposed by the Cerebellum.
pub trait Cerebellum: Send + Sync {
    /// 查询知识
    /// Query knowledge
    fn query(&mut self, keywords: &[String], limit: usize) -> Vec<KnowledgeEntry>;

    /// 精确检索（按主体和属性）
    /// Exact retrieval by subject and attribute
    fn get_exact(&mut self, subject: &str, attribute: &str) -> Option<KnowledgeEntry>;

    /// 写入知识
    /// Write knowledge
    fn write(&mut self, entry: KnowledgeEntry) -> Result<String, ConflictError>;

    /// 更新知识
    /// Update knowledge
    fn update(&mut self, id: &str, entry: KnowledgeEntry) -> Result<(), String>;

    /// 删除知识
    /// Delete knowledge
    fn delete(&mut self, id: &str) -> Result<(), String>;

    /// 获取所有知识条目
    /// Get all knowledge entries
    fn get_all(&self) -> Vec<&KnowledgeEntry>;

    /// 应用衰减并更新层级
    /// Apply decay and update tiers
    fn apply_decay(&mut self) -> usize;

    /// 获取层级管理器引用
    /// Get reference to tier manager
    fn tier_manager(&self) -> &TierManager;
}

/// 小脑标准实现
/// Standard Cerebellum implementation
#[derive(Debug)]
pub struct StandardCerebellum {
    /// 知识图谱存储（ID -> Entry）
    store: HashMap<String, KnowledgeEntry>,
    /// 主体-属性索引（用于快速精确检索）
    subject_attribute_index: HashMap<(String, String), String>,
    /// 层级管理器
    tier_manager: TierManager,
}

impl StandardCerebellum {
    /// 创建新的小脑实例
    /// Create a new Cerebellum instance
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
            subject_attribute_index: HashMap::new(),
            tier_manager: TierManager::default(),
        }
    }

    /// 使用自定义层级配置创建
    /// Create with custom tier configuration
    pub fn with_tier_manager(tier_manager: TierManager) -> Self {
        Self {
            store: HashMap::new(),
            subject_attribute_index: HashMap::new(),
            tier_manager,
        }
    }

    /// 重建索引
    /// Rebuild indices
    fn rebuild_index(&mut self) {
        self.subject_attribute_index.clear();
        for entry in self.store.values() {
            self.subject_attribute_index.insert(
                (entry.subject.clone(), entry.attribute.clone()),
                entry.id.clone(),
            );
        }
    }
}

impl Default for StandardCerebellum {
    fn default() -> Self {
        Self::new()
    }
}

impl Cerebellum for StandardCerebellum {
    fn query(&mut self, keywords: &[String], limit: usize) -> Vec<KnowledgeEntry> {
        // 调用多策略检索引擎
        let retriever = QueryRetriever::new();
        retriever.retrieve(&mut self.store, keywords, limit, &mut self.tier_manager)
    }

    fn get_exact(&mut self, subject: &str, attribute: &str) -> Option<KnowledgeEntry> {
        let id = self.subject_attribute_index.get(&(subject.to_string(), attribute.to_string()))?;
        let entry = self.store.get(id)?;
        // 标记访问
        let mut entry = entry.clone();
        self.tier_manager.mark_accessed(&mut entry);
        self.store.insert(id.clone(), entry.clone());
        Some(entry)
    }

    fn write(&mut self, entry: KnowledgeEntry) -> Result<String, ConflictError> {
        // 冲突检测
        let detector = ConflictDetector::new();
        detector.check_conflict(&self.store, &entry)?;

        // 如果是核心知识，确保标记正确
        let mut entry = entry;
        if entry.core {
            entry.weight = 1.0;
        }

        let id = entry.id.clone();
        self.subject_attribute_index.insert(
            (entry.subject.clone(), entry.attribute.clone()),
            id.clone(),
        );
        self.store.insert(id.clone(), entry);
        Ok(id)
    }

    fn update(&mut self, id: &str, entry: KnowledgeEntry) -> Result<(), String> {
        if !self.store.contains_key(id) {
            return Err(format!("知识条目 {} 不存在", id));
        }

        // 更新索引
        if let Some(old) = self.store.get(id) {
            self.subject_attribute_index.remove(&(old.subject.clone(), old.attribute.clone()));
        }
        self.subject_attribute_index.insert(
            (entry.subject.clone(), entry.attribute.clone()),
            id.to_string(),
        );
        self.store.insert(id.to_string(), entry);
        Ok(())
    }

    fn delete(&mut self, id: &str) -> Result<(), String> {
        if let Some(entry) = self.store.remove(id) {
            self.subject_attribute_index.remove(&(entry.subject, entry.attribute));
            Ok(())
        } else {
            Err(format!("知识条目 {} 不存在", id))
        }
    }

    fn get_all(&self) -> Vec<&KnowledgeEntry> {
        self.store.values().collect()
    }

    fn apply_decay(&mut self) -> usize {
        let mut count = 0;
        let mut entries: Vec<KnowledgeEntry> = self.store.drain().map(|(_, v)| v).collect();
        
        for entry in &mut entries {
            let old_tier = entry.tier();
            self.tier_manager.decay_and_update(entry);
            if old_tier != entry.tier() {
                count += 1;
            }
        }

        // 重新插入
        for entry in entries {
            self.subject_attribute_index.insert(
                (entry.subject.clone(), entry.attribute.clone()),
                entry.id.clone(),
            );
            self.store.insert(entry.id.clone(), entry);
        }
        count
    }

    fn tier_manager(&self) -> &TierManager {
        &self.tier_manager
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_entry(subject: &str, attribute: &str, value: &str) -> KnowledgeEntry {
        KnowledgeEntry::new(
            subject.to_string(),
            attribute.to_string(),
            serde_json::Value::String(value.to_string()),
            0.8,
        )
    }

    #[test]
    fn test_write_and_query() {
        let mut cerebellum = StandardCerebellum::new();
        let entry = create_test_entry("地球", "半径", "6371km");
        let id = cerebellum.write(entry).unwrap();
        
        let retrieved = cerebellum.get_exact("地球", "半径").unwrap();
        assert_eq!(retrieved.id, id);
    }

    #[test]
    fn test_decay_application() {
        let mut cerebellum = StandardCerebellum::new();
        let mut entry = create_test_entry("测试", "属性", "值");
        entry.weight = 0.2; // 低于沉寂阈值
        cerebellum.write(entry).unwrap();
        
        let changed = cerebellum.apply_decay();
        assert!(changed > 0);
    }
}