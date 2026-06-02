//! 接地置信度计算
//! Grounding Confidence Calculation
//!
//! 提供置信度计算的辅助函数。
//! Provides helper functions for confidence calculation.

/// 计算 softmax 概率
/// Compute softmax probabilities
pub fn softmax(scores: &[f32]) -> Vec<f32> {
    let max_score = scores.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    let exp_scores: Vec<f32> = scores.iter().map(|s| ((s - max_score).exp())).collect();
    let sum: f32 = exp_scores.iter().sum();
    exp_scores.iter().map(|e| e / sum).collect()
}

/// 自适应权重调整（根据知识库完整度）
/// Adaptive weight adjustment based on knowledge base completeness
pub fn adaptive_weights(kb_completeness: f32) -> (f32, f32, f32) {
    let w_prob = 0.4;
    let w_recon = 0.4;
    let w_semantic = 0.2 * kb_completeness.min(1.0);

    let total = w_prob + w_recon + w_semantic;
    (w_prob / total, w_recon / total, w_semantic / total)
}