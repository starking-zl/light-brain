//! 多策略检索引擎
//! Multi-Strategy Retrieval Engine
//!
//! 按优先级执行精确匹配、标签匹配、关键词部分匹配和向量语义检索。
//! Performs exact match, tag match, keyword partial match, and vector semantic retrieval in priority order.

use super::KnowledgeEntry;
use crate::memory::TierManager;
use std::collections::HashMap;

/// 检索结果
/// Retrieval result
#[derive(Debug, Clone)]
pub struct QueryResult {
    /// 匹配的知识条目
    pub entry: KnowledgeEntry,
    /// 匹配得分
    pub score: f32,
    /// 匹配方式
    pub match_type: MatchType,
}

/// 匹配方式
/// Match type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatchType {
    /// 精确匹配（主体或属性）
    Exact,
    /// 标签匹配
    Tag,
    /// 关键词部分匹配
    Keyword,
    /// 向量语义检索
    Vector,
}

/// 检索引擎
/// Retrieval engine
#[derive(Debug, Default)]
pub struct QueryRetriever;

impl QueryRetriever {
    /// 创建新的检索引擎
    /// Create a new retrieval engine
    pub fn new() -> Self {
        Self
    }

    /// 执行多策略检索
    /// Perform multi-strategy retrieval
    pub fn retrieve(
        &self,
        store: &mut HashMap<String, KnowledgeEntry>,
        keywords: &[String],
        limit: usize,
        tier_manager: &mut TierManager,
    ) -> Vec<KnowledgeEntry> {
        let mut results: Vec<QueryResult> = Vec::new();

        // 策略1：精确匹配（主体或属性）
        for kw in keywords {
            for entry in store.values_mut() {
                if entry.subject == *kw || entry.attribute == *kw {
                    // 标记访问
                    tier_manager.mark_accessed(entry);
                    results.push(QueryResult {
                        entry: entry.clone(),
                        score: 1.0,
                        match_type: MatchType::Exact,
                    });
                }
            }
        }

        // 如果精确匹配已足够，返回
        if results.len() >= limit {
            results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
            return results
                .into_iter()
                .take(limit)
                .map(|r| r.entry)
                .collect();
        }

        // 策略2：标签匹配
        for kw in keywords {
            for entry in store.values_mut() {
                if entry.tags.iter().any(|t| t.contains(kw)) {
                    tier_manager.mark_accessed(entry);
                    results.push(QueryResult {
                        entry: entry.clone(),
                        score: 0.8,
                        match_type: MatchType::Tag,
                    });
                }
            }
        }

        // 策略3：描述关键词部分匹配
        for kw in keywords {
            let kw_lower = kw.to_lowercase();
            for entry in store.values_mut() {
                if entry.description.to_lowercase().contains(&kw_lower) {
                    tier_manager.mark_accessed(entry);
                    results.push(QueryResult {
                        entry: entry.clone(),
                        score: 0.5,
                        match_type: MatchType::Keyword,
                    });
                }
            }
        }

        // 策略4：向量语义检索（预留，v1.0 可返回空）
        // 未来实现：调用向量数据库进行语义相似度检索

        // 去重并排序
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        let mut seen = std::collections::HashSet::new();
        let mut final_results = Vec::new();
        for r in results {
            if seen.insert(r.entry.id.clone()) {
                final_results.push(r.entry);
            }
        }

        final_results.into_iter().take(limit).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cerebellum::KnowledgeEntry;

    fn create_entry(subject: &str, attribute: &str, tags: Vec<&str>, desc: &str) -> KnowledgeEntry {
        KnowledgeEntry::new(
            subject.to_string(),
            attribute.to_string(),
            serde_json::Value::String("test".to_string()),
            0.8,
        )
        .with_tags(tags.iter().map(|s| s.to_string()).collect())
        .with_description(desc)
    }

    #[test]
    fn test_multi_strategy_retrieval() {
        let mut store = HashMap::new();
        let mut tier_manager = TierManager::default();
        let entry1 = create_entry("地球", "半径", vec!["天文"], "地球的半径");
        let entry2 = create_entry("太阳", "温度", vec!["天文", "热量"], "太阳表面温度");
        store.insert(entry1.id.clone(), entry1);
        store.insert(entry2.id.clone(), entry2);

        let retriever = QueryRetriever::new();
        let results = retriever.retrieve(
            &mut store,
            &["地球".to_string()],
            5,
            &mut tier_manager,
        );
        assert!(!results.is_empty());
    }
}