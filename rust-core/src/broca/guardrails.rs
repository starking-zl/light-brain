//! 分级护栏
//! Tiered Guardrails
//!
//! 实现三级梯度护栏：绝对红线（硬约束）、情境敏感（软约束）、探索友好（无约束）。
//! Implements three-tier gradient guardrails: Absolute Redline (hard constraint),
//! Context-Sensitive (soft constraint), Exploration-Friendly (no constraint).

use super::CreativeMode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 护栏层级
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GuardrailTier {
    /// 第一级：绝对红线（永远硬约束）
    AbsoluteRedline,
    /// 第二级：情境敏感（创意模式下软约束）
    ContextSensitive,
    /// 第三级：探索友好（无约束）
    ExplorationFriendly,
}

/// 护栏动作
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuardrailAction {
    Block,
    Suppress,
    Allow,
}

/// 护栏管理器
#[derive(Debug, Default)]
pub struct GuardrailManager {
    /// 词汇 -> 层级映射
    word_tiers: HashMap<String, GuardrailTier>,
}

impl GuardrailManager {
    pub fn new() -> Self {
        let mut manager = Self::default();
        manager.init_default_tiers();
        manager
    }

    fn init_default_tiers(&mut self) {
        // 第一级：绝对红线（硬编码）
        let redline_words = vec!["杀", "死", "暴力", "仇恨", "kill", "murder", "hate"];
        for word in redline_words {
            self.word_tiers.insert(word.to_string(), GuardrailTier::AbsoluteRedline);
        }
        // 第二级：情境敏感（示例）
        let sensitive_words = vec!["战争", "武器", "war", "weapon"];
        for word in sensitive_words {
            self.word_tiers.insert(word.to_string(), GuardrailTier::ContextSensitive);
        }
    }

    /// 计算 Logit Bias 向量（供生成器使用）
    pub fn compute_logit_bias(&self, tokens: &[String], mode: &CreativeMode) -> Vec<f32> {
        tokens.iter().map(|token| {
            if let Some(tier) = self.word_tiers.get(token) {
                match tier {
                    GuardrailTier::AbsoluteRedline => f32::NEG_INFINITY,
                    GuardrailTier::ContextSensitive => {
                        if mode.is_soft_constraint_mode() {
                            -2.0  // 软约束
                        } else {
                            f32::NEG_INFINITY  // 硬约束
                        }
                    }
                    GuardrailTier::ExplorationFriendly => 0.0,
                }
            } else {
                0.0
            }
        }).collect()
    }

    /// 审计文本中触发的护栏
    pub fn audit_triggers(&self, text: &str) -> Vec<super::GuardrailTrigger> {
        let mut triggers = Vec::new();
        for (word, tier) in &self.word_tiers {
            if text.contains(word) {
                triggers.push(super::GuardrailTrigger {
                    token: word.clone(),
                    tier: *tier,
                    action: match tier {
                        GuardrailTier::AbsoluteRedline => super::GuardrailAction::Block,
                        GuardrailTier::ContextSensitive => super::GuardrailAction::Suppress,
                        GuardrailTier::ExplorationFriendly => super::GuardrailAction::Allow,
                    },
                });
            }
        }
        triggers
    }

    /// 添加词汇到指定层级
    pub fn add_word(&mut self, word: String, tier: GuardrailTier) {
        self.word_tiers.insert(word, tier);
    }
}