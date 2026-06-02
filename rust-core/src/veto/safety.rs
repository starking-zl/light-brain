//! 安全红线否决
//! Safety Redline Veto
//!
//! 保护"和平与爱"底层价值观，硬编码不可绕过。
//! Protects the core value of "Peace and Love", hard-coded and cannot be bypassed.

use super::{VetoAction, VetoCategory, VetoContext, VetoOperation, VetoPriority, VetoSignal};

/// 安全红线检查器
/// Safety Redline Checker
///
/// 仅检查是否违背"和平与爱"原则。
/// Only checks violation of "Peace and Love" principle.
#[derive(Debug)]
pub struct SafetyRedline {
    // 硬编码禁止词汇（仅作为兜底，实际应使用分类模型）
    // Hard-coded forbidden keywords (fallback only, classifier should be used in production)
    forbidden_keywords: Vec<&'static str>,
}

impl SafetyRedline {
    /// 创建新的安全红线检查器
    /// Create a new safety redline checker
    pub fn new() -> Self {
        Self {
            forbidden_keywords: vec![
                // 中英文核心暴力/仇恨词汇 / Core violence/hate keywords in Chinese and English
                "杀", "死", "暴力", "仇恨", "歧视",
                "kill", "murder", "hate", "violence", "discrimination",
            ],
        }
    }

    /// 执行安全红线检查
    /// Perform safety redline check
    pub fn check(&self, context: &VetoContext) -> Option<VetoSignal> {
        // 如果是创意模式，安全红线仍然强制生效（不可绕过）
        // Safety redline remains enforced even in creative mode (cannot be bypassed)

        match context.operation {
            VetoOperation::UserInput => {
                if let Some(input) = &context.user_input {
                    if self.contains_forbidden_content(input) {
                        return Some(VetoSignal::new(
                            VetoPriority::Safety,
                            VetoCategory::ViolenceHate,
                            "用户输入包含违背和平与爱原则的内容".to_string(),
                            VetoAction::Block,
                            1.0,
                        ));
                    }
                }
            }
            VetoOperation::Inference => {
                // 在生成回复前检查决策包内容
                // Check decision package content before generation
                if let Some(input) = &context.user_input {
                    if self.contains_forbidden_content(input) {
                        return Some(VetoSignal::new(
                            VetoPriority::Safety,
                            VetoCategory::ViolenceHate,
                            "生成内容将违背和平与爱原则".to_string(),
                            VetoAction::Block,
                            1.0,
                        ));
                    }
                }
            }
            // 其他操作类型通常不触发安全红线
            // Other operation types typically do not trigger safety redline
            _ => {}
        }

        None
    }

    /// 检查文本是否包含禁止内容
    /// Check if text contains forbidden content
    fn contains_forbidden_content(&self, text: &str) -> bool {
        let lower_text = text.to_lowercase();
        self.forbidden_keywords
            .iter()
            .any(|&kw| lower_text.contains(&kw.to_lowercase()))
    }
}

impl Default for SafetyRedline {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::veto::VetoContext;

    #[test]
    fn test_safety_check_blocks_violence() {
        let checker = SafetyRedline::new();
        let context = VetoContext {
            operation: VetoOperation::UserInput,
            user_input: Some("我想杀人".to_string()),
            knowledge_entry: None,
            event: None,
            grounding_confidence: None,
            knowledge_completeness: None,
            is_creative_mode: false,
        };
        let signal = checker.check(&context).unwrap();
        assert_eq!(signal.priority, VetoPriority::Safety);
        assert_eq!(signal.suggested_action, VetoAction::Block);
    }

    #[test]
    fn test_safety_check_allows_normal_input() {
        let checker = SafetyRedline::new();
        let context = VetoContext {
            operation: VetoOperation::UserInput,
            user_input: Some("你好".to_string()),
            knowledge_entry: None,
            event: None,
            grounding_confidence: None,
            knowledge_completeness: None,
            is_creative_mode: false,
        };
        assert!(checker.check(&context).is_none());
    }
}