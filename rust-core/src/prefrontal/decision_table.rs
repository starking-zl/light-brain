//! 决策表调度
//! Decision Table Scheduling
//!
//! 根据意图和领域匹配调度规则，支持动态优先级（道）。
//! Matches scheduling rules based on intent and domain, supports dynamic priority (Tao).

use crate::thalamus::{LabelGroundingInfo, ThalamusOutput};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 调度动作
/// Scheduling action
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScheduleAction {
    /// 是否调用小脑检索知识
    pub call_cerebellum: bool,
    /// 是否调用杏仁核评估风格
    pub call_amygdala: bool,
    /// 是否调用海马体获取历史上下文
    pub call_hippocampus: bool,
    /// 是否触发澄清
    pub trigger_clarification: bool,
}

impl Default for ScheduleAction {
    fn default() -> Self {
        Self {
            call_cerebellum: true,
            call_amygdala: true,
            call_hippocampus: false,
            trigger_clarification: false,
        }
    }
}

/// 意图模式：可以是单个字符串或字符串数组
/// Intent pattern: can be a single string or an array of strings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IntentPattern {
    Single(String),
    Multiple(Vec<String>),
    Wildcard,
}

impl IntentPattern {
    /// 检查意图是否匹配此模式
    pub fn matches(&self, intent: &str) -> bool {
        match self {
            IntentPattern::Single(pattern) => pattern == "*" || pattern == intent,
            IntentPattern::Multiple(patterns) => {
                patterns.iter().any(|p| p == "*" || p == intent)
            }
            IntentPattern::Wildcard => true,
        }
    }
}

impl Default for IntentPattern {
    fn default() -> Self {
        IntentPattern::Wildcard
    }
}

/// 决策规则
/// Decision rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionRule {
    /// 规则ID
    pub id: String,
    /// 意图模式（支持通配符"*"）
    pub intent_pattern: IntentPattern,
    /// 领域模式（支持通配符"*"）
    pub domain_pattern: String,
    /// 调度动作
    pub schedule: ScheduleAction,
    /// 使用次数（动态优先级）
    pub usage_count: u32,
    /// 成功率（动态优先级）
    pub success_rate: f32,
}

impl DecisionRule {
    /// 检查是否匹配给定的意图和领域
    pub fn matches(&self, intent: &str, domain: &str) -> bool {
        let intent_match = self.intent_pattern.matches(intent);
        let domain_match = self.domain_pattern == "*" || self.domain_pattern == domain;
        intent_match && domain_match
    }

    /// 计算动态优先级得分
    pub fn priority_score(&self) -> f32 {
        (self.usage_count as f32).ln_1p() * self.success_rate
    }
}

/// 决策表
/// Decision table
#[derive(Debug)]
pub struct DecisionTable {
    rules: Vec<DecisionRule>,
    default_schedule: ScheduleAction,
}

impl DecisionTable {
    pub fn new() -> Self {
        let mut table = Self {
            rules: Vec::new(),
            default_schedule: ScheduleAction::default(),
        };
        table.init_default_rules();
        table
    }

    fn init_default_rules(&mut self) {
        // 询问事实（中文和英文）
        self.rules.push(DecisionRule {
            id: "rule_fact".to_string(),
            intent_pattern: IntentPattern::Multiple(vec![
                "询问事实".to_string(),
                "ask_fact".to_string(),
            ]),
            domain_pattern: "*".to_string(),
            schedule: ScheduleAction {
                call_cerebellum: true,
                call_amygdala: true,
                call_hippocampus: false,
                trigger_clarification: false,
            },
            usage_count: 0,
            success_rate: 1.0,
        });
        // 闲聊（中文和英文）
        self.rules.push(DecisionRule {
            id: "rule_chitchat".to_string(),
            intent_pattern: IntentPattern::Multiple(vec![
                "闲聊".to_string(),
                "chitchat".to_string(),
            ]),
            domain_pattern: "*".to_string(),
            schedule: ScheduleAction {
                call_cerebellum: false,
                call_amygdala: true,
                call_hippocampus: true,
                trigger_clarification: false,
            },
            usage_count: 0,
            success_rate: 1.0,
        });
        // 指令请求（中文和英文）
        self.rules.push(DecisionRule {
            id: "rule_instruction".to_string(),
            intent_pattern: IntentPattern::Multiple(vec![
                "指令请求".to_string(),
                "instruction".to_string(),
            ]),
            domain_pattern: "*".to_string(),
            schedule: ScheduleAction {
                call_cerebellum: true,
                call_amygdala: false,
                call_hippocampus: false,
                trigger_clarification: false,
            },
            usage_count: 0,
            success_rate: 1.0,
        });
        // 默认规则（通配符）
        self.rules.push(DecisionRule {
            id: "rule_default".to_string(),
            intent_pattern: IntentPattern::Wildcard,
            domain_pattern: "*".to_string(),
            schedule: ScheduleAction::default(),
            usage_count: 0,
            success_rate: 1.0,
        });
    }

    /// 匹配调度动作
    pub fn match_schedule(&mut self, perception: &ThalamusOutput) -> ScheduleAction {
        if perception.candidates.is_empty() {
            return self.default_schedule;
        }

        let best = &perception.candidates[0];
        
        // 按优先级得分排序规则
        self.rules.sort_by(|a, b| {
            b.priority_score()
                .partial_cmp(&a.priority_score())
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        for rule in &mut self.rules {
            if rule.matches(&best.label, &best.domain) {
                rule.usage_count += 1;
                return rule.schedule;
            }
        }

        self.default_schedule
    }

    /// 更新规则成功率（基于用户反馈）
    pub fn update_success_rate(&mut self, rule_id: &str, success: bool) {
        if let Some(rule) = self.rules.iter_mut().find(|r| r.id == rule_id) {
            let alpha = 0.1;
            rule.success_rate = rule.success_rate * (1.0 - alpha) + (if success { 1.0 } else { 0.0 }) * alpha;
        }
    }

    /// 添加自定义规则
    pub fn add_rule(&mut self, rule: DecisionRule) {
        self.rules.push(rule);
    }

    /// 从 JSON 配置文件加载规则
    pub fn load_from_file(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let loaded_rules: Vec<DecisionRule> = serde_json::from_str(&content)?;
        // 合并或替换现有规则
        self.rules.extend(loaded_rules);
        Ok(())
    }
}

impl Default for DecisionTable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intent_pattern_single() {
        let pattern = IntentPattern::Single("询问事实".to_string());
        assert!(pattern.matches("询问事实"));
        assert!(!pattern.matches("闲聊"));
    }

    #[test]
    fn test_intent_pattern_multiple() {
        let pattern = IntentPattern::Multiple(vec!["询问事实".to_string(), "ask_fact".to_string()]);
        assert!(pattern.matches("询问事实"));
        assert!(pattern.matches("ask_fact"));
        assert!(!pattern.matches("闲聊"));
    }

    #[test]
    fn test_intent_pattern_wildcard() {
        let pattern = IntentPattern::Wildcard;
        assert!(pattern.matches("任何意图"));
    }

    #[test]
    fn test_rule_matching() {
        let mut table = DecisionTable::new();
        let perception = ThalamusOutput {
            candidates: vec![LabelGroundingInfo {
                label: "询问事实".to_string(),
                domain: "天文".to_string(),
                ..Default::default()
            }],
            need_clarification: false,
            recommended_k: 1,
        };
        let schedule = table.match_schedule(&perception);
        assert!(schedule.call_cerebellum);
    }

    #[test]
    fn test_english_intent_matching() {
        let mut table = DecisionTable::new();
        let perception = ThalamusOutput {
            candidates: vec![LabelGroundingInfo {
                label: "ask_fact".to_string(),
                domain: "astronomy".to_string(),
                ..Default::default()
            }],
            need_clarification: false,
            recommended_k: 1,
        };
        let schedule = table.match_schedule(&perception);
        assert!(schedule.call_cerebellum);
    }
}