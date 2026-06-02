//! 规则引擎
//! Rule Engine
//!
//! 根据意图和情感极性匹配预设的风格规则。规则可配置，支持动态加载。
//! Matches preset style rules based on intent and emotional polarity.
//! Rules are configurable and support dynamic loading.

use super::StyleModifier;
use std::collections::HashMap;

/// 规则条目
/// Rule entry
#[derive(Debug, Clone)]
pub struct StyleRule {
    /// 意图模式（支持通配符 "*" 表示任意）
    /// Intent pattern (supports wildcard "*" for any)
    pub intent_pattern: String,
    /// 情感极性阈值范围 (min, max)
    /// Polarity threshold range (min, max)
    pub polarity_range: (f32, f32),
    /// 匹配后应用的风格修饰符
    /// Style modifier to apply when matched
    pub style: StyleModifier,
}

impl StyleRule {
    /// 检查给定意图和极性是否匹配此规则
    /// Check if the given intent and polarity match this rule
    pub fn matches(&self, intent: &str, polarity: f32) -> bool {
        // 意图匹配（支持通配符）
        // Intent matching (supports wildcard)
        let intent_match = self.intent_pattern == "*" || self.intent_pattern == intent;
        // 极性范围匹配
        // Polarity range matching
        let polarity_match = polarity >= self.polarity_range.0 && polarity <= self.polarity_range.1;
        intent_match && polarity_match
    }
}

/// 规则引擎
/// Rule Engine
#[derive(Debug)]
pub struct RuleEngine {
    rules: Vec<StyleRule>,
    default_style: StyleModifier,
    // 情感阻止词（若输入包含这些词，则可能触发否决）
    // Emotionally blocked keywords (inputs containing these may trigger veto)
    blocked_keywords: Vec<&'static str>,
}

impl RuleEngine {
    /// 创建新的规则引擎，包含内置默认规则
    /// Create a new rule engine with built-in default rules
    pub fn new() -> Self {
        Self {
            rules: Self::default_rules(),
            default_style: StyleModifier::default(),
            blocked_keywords: vec!["恨", "讨厌", "攻击", "hate", "attack"],
        }
    }

    /// 使用自定义规则创建
    /// Create with custom rules
    pub fn with_rules(rules: Vec<StyleRule>) -> Self {
        Self {
            rules,
            default_style: StyleModifier::default(),
            blocked_keywords: vec!["恨", "讨厌", "攻击", "hate", "attack"],
        }
    }

    /// 内置默认规则集
    /// Built-in default rule set
    fn default_rules() -> Vec<StyleRule> {
        vec![
            // 询问事实：正常风格，略正式
            StyleRule {
                intent_pattern: "询问事实".to_string(),
                polarity_range: (-1.0, 1.0),
                style: StyleModifier {
                    warmth: 0.4,
                    formality: 0.7,
                    defensiveness: 0.3,
                    humor: 0.1,
                    label: "专业".to_string(),
                },
            },
            // 表达积极情感：热情风格
            StyleRule {
                intent_pattern: "*".to_string(),
                polarity_range: (0.5, 1.0),
                style: StyleModifier {
                    warmth: 0.9,
                    formality: 0.4,
                    defensiveness: 0.2,
                    humor: 0.4,
                    label: "热情".to_string(),
                },
            },
            // 表达消极情感：谨慎防御风格
            StyleRule {
                intent_pattern: "*".to_string(),
                polarity_range: (-1.0, -0.3),
                style: StyleModifier {
                    warmth: 0.3,
                    formality: 0.6,
                    defensiveness: 0.8,
                    humor: 0.0,
                    label: "谨慎".to_string(),
                },
            },
            // 指令请求：简洁高效
            StyleRule {
                intent_pattern: "指令请求".to_string(),
                polarity_range: (-1.0, 1.0),
                style: StyleModifier {
                    warmth: 0.3,
                    formality: 0.8,
                    defensiveness: 0.2,
                    humor: 0.0,
                    label: "简洁".to_string(),
                },
            },
            // 闲聊：轻松友好
            StyleRule {
                intent_pattern: "闲聊".to_string(),
                polarity_range: (-1.0, 1.0),
                style: StyleModifier {
                    warmth: 0.7,
                    formality: 0.3,
                    defensiveness: 0.2,
                    humor: 0.5,
                    label: "友好".to_string(),
                },
            },
        ]
    }

    /// 根据意图和极性匹配风格
    /// Match style based on intent and polarity
    pub fn match_style(&self, intent: &str, polarity: f32) -> StyleModifier {
        // 按顺序匹配，返回第一个匹配的规则
        // Match in order, return the first matching rule
        for rule in &self.rules {
            if rule.matches(intent, polarity) {
                return rule.style.clone();
            }
        }
        self.default_style.clone()
    }

    /// 添加新规则
    /// Add a new rule
    pub fn add_rule(&mut self, rule: StyleRule) {
        self.rules.push(rule);
    }

    /// 检查文本是否包含情感阻止词
    /// Check if text contains emotionally blocked keywords
    pub fn is_emotionally_blocked(&self, text: &str) -> bool {
        let lower_text = text.to_lowercase();
        self.blocked_keywords
            .iter()
            .any(|&kw| lower_text.contains(&kw.to_lowercase()))
    }

    /// 从 JSON 配置加载规则
    /// Load rules from JSON configuration
    ///
    /// 未来实现，从 config/amygdala_rules.json 加载。
    /// Future implementation: load from config/amygdala_rules.json.
    pub fn load_from_config(_config_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // TODO: 实现 JSON 反序列化
        Ok(Self::new())
    }
}

impl Default for RuleEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_matching() {
        let engine = RuleEngine::new();
        // 积极情感应匹配热情风格
        let style = engine.match_style("任意意图", 0.8);
        assert_eq!(style.label, "热情");
        // 询问事实应匹配专业风格
        let style = engine.match_style("询问事实", 0.0);
        assert_eq!(style.label, "专业");
        // 消极情感应匹配谨慎风格
        let style = engine.match_style("任意意图", -0.5);
        assert_eq!(style.label, "谨慎");
    }

    #[test]
    fn test_blocked_keywords() {
        let engine = RuleEngine::new();
        assert!(engine.is_emotionally_blocked("我恨你"));
        assert!(!engine.is_emotionally_blocked("我喜欢你"));
    }
}