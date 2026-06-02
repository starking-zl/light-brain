//! 护栏自学习
//! Guardrail Learner
//!
//! 根据用户反馈动态调整护栏词汇层级（降级/升级）。
//! Dynamically adjusts guardrail word tiers (downgrade/upgrade) based on user feedback.

use std::collections::HashMap;

/// 护栏层级
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuardrailTier {
    AbsoluteRedline,
    ContextSensitive,
    ExplorationFriendly,
}

/// 词汇学习状态
#[derive(Debug, Default)]
struct WordLearningState {
    positive_count: u32,
    negative_count: u32,
    current_tier: GuardrailTier,
}

/// 护栏自学习器
#[derive(Debug, Default)]
pub struct GuardrailLearner {
    word_states: HashMap<String, WordLearningState>,
    downgrade_threshold: u32,  // 积极反馈次数阈值
    upgrade_threshold: u32,    // 消极反馈次数阈值
}

impl GuardrailLearner {
    pub fn new() -> Self {
        Self {
            word_states: HashMap::new(),
            downgrade_threshold: 5,
            upgrade_threshold: 3,
        }
    }

    /// 记录积极反馈（词汇被接受）
    pub fn record_positive(&mut self, word: &str) {
        let state = self.word_states.entry(word.to_string()).or_insert(WordLearningState {
            current_tier: GuardrailTier::ContextSensitive,
            ..Default::default()
        });
        state.positive_count += 1;
        self.try_downgrade(word);
    }

    /// 记录消极反馈（词汇被拒绝）
    pub fn record_negative(&mut self, word: &str) {
        let state = self.word_states.entry(word.to_string()).or_insert(WordLearningState {
            current_tier: GuardrailTier::ContextSensitive,
            ..Default::default()
        });
        state.negative_count += 1;
        self.try_upgrade(word);
    }

    fn try_downgrade(&mut self, word: &str) {
        if let Some(state) = self.word_states.get_mut(word) {
            if state.positive_count >= self.downgrade_threshold && state.current_tier != GuardrailTier::ExplorationFriendly {
                state.current_tier = match state.current_tier {
                    GuardrailTier::AbsoluteRedline => GuardrailTier::ContextSensitive,
                    GuardrailTier::ContextSensitive => GuardrailTier::ExplorationFriendly,
                    _ => state.current_tier,
                };
                state.positive_count = 0;
            }
        }
    }

    fn try_upgrade(&mut self, word: &str) {
        if let Some(state) = self.word_states.get_mut(word) {
            if state.negative_count >= self.upgrade_threshold && state.current_tier != GuardrailTier::AbsoluteRedline {
                state.current_tier = match state.current_tier {
                    GuardrailTier::ExplorationFriendly => GuardrailTier::ContextSensitive,
                    GuardrailTier::ContextSensitive => GuardrailTier::AbsoluteRedline,
                    _ => state.current_tier,
                };
                state.negative_count = 0;
            }
        }
    }

    /// 获取词汇的当前护栏层级
    pub fn get_tier(&self, word: &str) -> Option<GuardrailTier> {
        self.word_states.get(word).map(|s| s.current_tier)
    }
}