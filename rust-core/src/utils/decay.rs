//! 通用衰减公式
//! Generic decay formulas
//!
//! 提供符合光脑方案三层记忆衰减所需的数学公式。
//! Provides mathematical formulas required for the three-tier memory decay in Light-Brain Scheme.

/// 衰减公式参数
/// Decay formula parameters
#[derive(Debug, Clone)]
pub struct DecayParams {
    /// 全局衰减系数 λ
    pub lambda: f32,
    /// 防止除零的小量 ε
    pub epsilon: f32,
    /// 基础确定性系数（默认 1.0）
    pub base_certainty: f32,
}

impl Default for DecayParams {
    fn default() -> Self {
        Self {
            lambda: 0.1,
            epsilon: 1e-6,
            base_certainty: 1.0,
        }
    }
}

/// 计算衰减后的权重
/// Calculate decayed weight
///
/// 公式：w_new = w_original × exp(-λ × Δt / (f + ε)) × c
/// Formula: w_new = w_original × exp(-λ × Δt / (f + ε)) × c
///
/// # 参数 / Arguments
/// * `original_weight` - 原始权重 / original weight
/// * `days_elapsed` - 距离上次访问的天数 / days since last access
/// * `access_frequency` - 近三十日平均访问频次 / average access frequency in last 30 days
/// * `certainty` - 知识自身的确定性评分 / knowledge certainty score (0.0 ~ 1.0)
/// * `params` - 衰减参数 / decay parameters
pub fn calculate_decayed_weight(
    original_weight: f32,
    days_elapsed: f32,
    access_frequency: f32,
    certainty: f32,
    params: &DecayParams,
) -> f32 {
    // 确保访问频次不为零 / Ensure access frequency is not zero
    let f = access_frequency.max(params.epsilon);
    let exponent = -params.lambda * days_elapsed / f;
    let decay_factor = exponent.exp();
    original_weight * decay_factor * certainty
}

/// 简化版衰减计算，使用默认参数
/// Simplified decay calculation with default parameters
pub fn decay_weight_simple(
    original_weight: f32,
    days_elapsed: f32,
    access_frequency: f32,
    certainty: f32,
) -> f32 {
    calculate_decayed_weight(
        original_weight,
        days_elapsed,
        access_frequency,
        certainty,
        &DecayParams::default(),
    )
}

/// 计算时间衰减因子（不考虑访问频次）
/// Calculate time decay factor (without access frequency)
pub fn time_decay_factor(days_elapsed: f32, lambda: f32) -> f32 {
    (-lambda * days_elapsed).exp()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decay_basic() {
        let params = DecayParams::default();
        // 原始权重 1.0，经过 10 天，访问频次 1.0 次/天，确定性 1.0
        let new_weight = calculate_decayed_weight(1.0, 10.0, 1.0, 1.0, &params);
        // λ=0.1, exponent = -0.1*10/1 = -1.0, exp(-1)≈0.3679
        assert!((new_weight - 0.3679).abs() < 0.01);
    }

    #[test]
    fn test_high_frequency_slows_decay() {
        let params = DecayParams::default();
        // 高频访问 f=10，衰减更慢 / High frequency f=10, slower decay
        let weight_low_f = calculate_decayed_weight(1.0, 10.0, 1.0, 1.0, &params);
        let weight_high_f = calculate_decayed_weight(1.0, 10.0, 10.0, 1.0, &params);
        assert!(weight_high_f > weight_low_f);
    }

    #[test]
    fn test_certainty_scaling() {
        let params = DecayParams::default();
        let weight_full = calculate_decayed_weight(1.0, 10.0, 1.0, 1.0, &params);
        let weight_half = calculate_decayed_weight(1.0, 10.0, 1.0, 0.5, &params);
        assert!((weight_half - weight_full * 0.5).abs() < 0.001);
    }
}