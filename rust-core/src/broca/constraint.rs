//! 约束引导层
//! Constraint Guidance Layer
//!
//! 在生成过程中施加 Logit Bias，强制关键实体的准确表述，
//! 并抑制域外词汇。
//! Applies Logit Bias during generation to enforce accurate expression
//! of key entities and suppress out-of-domain vocabulary.

use super::{CreativeMode, SemanticPlan};

/// 约束引导器
#[derive(Debug, Default)]
pub struct ConstraintGuider;

impl ConstraintGuider {
    pub fn new() -> Self {
        Self
    }

    /// 计算 Logit Bias 映射
    pub fn compute_bias(&self, plan: &SemanticPlan, mode: &CreativeMode) -> Vec<(String, f32)> {
        let mut bias = Vec::new();

        // 对关键实体施加正向偏置（鼓励出现）
        for entity in &plan.key_entities {
            bias.push((entity.clone(), 2.0));
        }

        // 对禁止表达施加负无穷偏置（完全禁止）
        for forbidden in &plan.forbidden_expressions {
            bias.push((forbidden.clone(), f32::NEG_INFINITY));
        }

        // 根据模式调整偏置强度
        if mode.is_soft_constraint_mode() {
            // 软约束模式：负偏置减弱
            bias = bias.into_iter().map(|(k, v)| {
                if v == f32::NEG_INFINITY {
                    (k, -2.0)
                } else {
                    (k, v)
                }
            }).collect();
        }

        bias
    }
}