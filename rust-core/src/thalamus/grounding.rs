//! 符号接地层
//! Symbol Grounding Layer
//!
//! 执行软接地，输出 Top-K 概率分布，并计算综合接地置信度。
//! Performs soft grounding, outputs Top-K probability distribution,
//! and computes composite grounding confidence.

use super::{LabelGroundingInfo, ThalamusConfig};

/// 接地引擎
/// Grounding engine
#[derive(Debug, Clone)]
pub struct GroundingEngine {
    config: ThalamusConfig,
}

impl GroundingEngine {
    pub fn new(config: ThalamusConfig) -> Self {
        Self { config }
    }

    /// 计算单个标签的完整接地信息
    /// Compute complete grounding information for a single label
    pub fn compute_grounding_info(
        &self,
        label: String,
        probability: f32,
        feature_vector: &[f32],
        reconstructed_vector: &[f32],
        prototype_vector: &[f32],
    ) -> LabelGroundingInfo {
        let recon_similarity = super::prototype::cosine_similarity(feature_vector, reconstructed_vector);
        let semantic_similarity = super::prototype::cosine_similarity(feature_vector, prototype_vector);

        // 三维融合计算接地置信度
        let grounding_confidence = self.fuse_confidence(
            probability,
            recon_similarity,
            semantic_similarity,
        );

        LabelGroundingInfo {
            label,
            probability,
            grounding_confidence,
            recon_similarity,
            semantic_similarity,
            polarity: 0.0,      // 从原型中获取，此处简化
            domain: String::new(),
            keywords: Vec::new(),
            entities: Vec::new(),
        }
    }

    /// 三维置信度融合
    /// Fuse three confidence dimensions
    fn fuse_confidence(&self, prob: f32, recon: f32, semantic: f32) -> f32 {
        // 权重：概率 0.4，重构 0.4，语义 0.2
        // 可根据知识库完整度动态调整（此处简化）
        let w_prob = 0.4;
        let w_recon = 0.4;
        let w_semantic = 0.2;

        w_prob * prob + w_recon * recon + w_semantic * semantic
    }

    /// 动态决定探索路径数 K
    /// Dynamically determine number of exploration paths K
    pub fn dynamic_k(&self, candidates: &[LabelGroundingInfo], config: &ThalamusConfig) -> usize {
        if candidates.is_empty() {
            return 0;
        }

        let mut k = config.default_k;

        // 高价值领域增加 K
        if let Some(first) = candidates.first() {
            if config.high_value_domains.contains(&first.domain) {
                k = k.max(3);
            }
        }

        // 冷启动增加 K
        // maturity 可通过外部传入，此处简化

        // 限制不超过候选数
        k.min(candidates.len())
    }
}