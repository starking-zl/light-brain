//! 原型向量存储
//! Prototype Vector Store
//!
//! 维护每个符号标签的原型向量，支持余弦相似度匹配和在线校准。
//! Maintains prototype vectors for each symbolic label,
//! supports cosine similarity matching and online calibration.

use std::collections::HashMap;

/// 原型条目
/// Prototype entry
#[derive(Debug, Clone)]
pub struct PrototypeEntry {
    /// 标签名称
    pub label: String,
    /// 原型向量
    pub vector: Vec<f32>,
    /// 情感极性
    pub polarity: f32,
    /// 话题领域
    pub domain: String,
    /// 默认关键词
    pub default_keywords: Vec<String>,
}

/// 原型向量存储
/// Prototype vector store
#[derive(Debug)]
pub struct PrototypeStore {
    prototypes: HashMap<String, PrototypeEntry>,
    vector_dim: usize,
}

impl PrototypeStore {
    /// 创建新的原型存储
    /// Create a new prototype store
    pub fn new(vector_dim: usize) -> Self {
        Self {
            prototypes: HashMap::new(),
            vector_dim,
        }
    }

    /// 添加或更新原型
    /// Add or update a prototype
    pub fn upsert(&mut self, entry: PrototypeEntry) {
        if entry.vector.len() != self.vector_dim {
            // 维度不匹配，进行截断或填充（简化：仅记录警告）
            log::warn!("原型向量维度不匹配: 期望 {}, 实际 {}", self.vector_dim, entry.vector.len());
        }
        self.prototypes.insert(entry.label.clone(), entry);
    }

    /// 获取原型
    /// Get a prototype
    pub fn get(&self, label: &str) -> Option<&PrototypeEntry> {
        self.prototypes.get(label)
    }

    /// 获取所有原型标签
    /// Get all prototype labels
    pub fn labels(&self) -> Vec<String> {
        self.prototypes.keys().cloned().collect()
    }

    /// 计算输入向量与所有原型的余弦相似度，返回 Top-K
    /// Compute cosine similarity with all prototypes, return Top-K
    pub fn match_top_k(&self, vector: &[f32], k: usize) -> Vec<(String, f32, Vec<f32>)> {
        let mut similarities: Vec<(String, f32, Vec<f32>)> = self.prototypes
            .iter()
            .map(|(label, entry)| {
                let sim = cosine_similarity(vector, &entry.vector);
                (label.clone(), sim, entry.vector.clone())
            })
            .collect();

        // 按相似度降序排序
        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        similarities.truncate(k);
        similarities
    }

    /// 在线校准：根据用户反馈更新原型向量
    /// Online calibration: update prototype vector based on user feedback
    pub fn calibrate(&mut self, label: &str, feature_vector: &[f32], success: bool) {
        if let Some(entry) = self.prototypes.get_mut(label) {
            let learning_rate = if success { 0.1 } else { -0.05 };
            for (i, v) in feature_vector.iter().enumerate() {
                if i < entry.vector.len() {
                    entry.vector[i] = (entry.vector[i] + learning_rate * v).clamp(-1.0, 1.0);
                }
            }
        }
    }

    /// 从 JSON 文件加载原型
    /// Load prototypes from JSON file
    pub fn load_from_file(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let entries: Vec<PrototypeEntry> = serde_json::from_str(&content)?;
        for entry in entries {
            self.upsert(entry);
        }
        Ok(())
    }
}

/// 计算余弦相似度
/// Compute cosine similarity
fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let len = a.len().min(b.len());
    if len == 0 {
        return 0.0;
    }

    let mut dot = 0.0;
    let mut norm_a = 0.0;
    let mut norm_b = 0.0;

    for i in 0..len {
        dot += a[i] * b[i];
        norm_a += a[i] * a[i];
        norm_b += b[i] * b[i];
    }

    let denom = (norm_a.sqrt() * norm_b.sqrt()).max(1e-8);
    dot / denom
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        assert!((cosine_similarity(&a, &b) - 1.0).abs() < 0.001);

        let c = vec![0.0, 1.0, 0.0];
        assert!((cosine_similarity(&a, &c) - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_prototype_store() {
        let mut store = PrototypeStore::new(3);
        store.upsert(PrototypeEntry {
            label: "询问事实".to_string(),
            vector: vec![1.0, 0.0, 0.0],
            polarity: 0.0,
            domain: "通用".to_string(),
            default_keywords: vec!["事实".to_string()],
        });

        let matches = store.match_top_k(&[1.0, 0.1, 0.0], 1);
        assert_eq!(matches[0].0, "询问事实");
    }
}