//! 杏仁核模块
//! Amygdala Module
//!
//! 杏仁核是光脑方案的情感评估中枢，负责根据丘脑感知到的意图和情感极性
//! 输出风格修饰符，影响布罗卡区的语言生成风格。同时，杏仁核参与否决机制，
//! 可对不安全或不当的内容发出否决信号。
//! The Amygdala is the emotional evaluation center of the Light-Brain Scheme.
//! It outputs style modifiers based on intent and emotional polarity perceived
//! by the Thalamus, influencing the linguistic style of Broca's Area generation.
//! Additionally, the Amygdala participates in the veto mechanism,
//! capable of emitting veto signals for unsafe or inappropriate content.

mod rule_engine;
mod veto;

pub use rule_engine::*;
pub use veto::*;

use crate::veto::{
    VetoEngine, VetoSignal, VetoContext, VetoOperation,
    VetoPriority, VetoCategory, VetoAction,
};

/// 杏仁核 trait
/// Amygdala trait
///
/// 定义杏仁核对外提供的标准接口。
/// Defines the standard interface exposed by the Amygdala.
pub trait Amygdala: Send + Sync {
    /// 根据意图和情感极性推断风格修饰符
    /// Infer style modifier based on intent and emotional polarity
    ///
    /// # 参数 / Arguments
    /// * `intent` - 意图标签 / intent label
    /// * `polarity` - 情感极性（-1.0 ~ 1.0）/ emotional polarity (-1.0 ~ 1.0)
    ///
    /// # 返回 / Returns
    /// 风格修饰符，包含热情度、正式度、防御度等参数。
    /// Style modifier containing parameters like warmth, formality, defensiveness.
    fn infer_style(&self, intent: &str, polarity: f32) -> StyleModifier;

    /// 检查当前操作是否需要否决
    /// Check if the current operation should be vetoed
    ///
    /// 杏仁核可基于自身规则（如情感判断）或委托给否决引擎进行安全检查。
    /// The Amygdala may veto based on its own rules (e.g., emotional judgment)
    /// or delegate to the veto engine for safety checks.
    fn check_veto(&mut self, context: &VetoContext) -> Option<VetoSignal>;

    /// 获取关联的否决引擎的不可变引用
    /// Get immutable reference to the associated veto engine
    fn veto_engine(&self) -> &VetoEngine;

    /// 获取关联的否决引擎的可变引用
    /// Get mutable reference to the associated veto engine
    fn veto_engine_mut(&mut self) -> &mut VetoEngine;
}

/// 风格修饰符
/// Style Modifier
///
/// 决定布罗卡区生成语言时的语气、正式程度等特征。
/// Determines characteristics like tone, formality for Broca's Area generation.
#[derive(Debug, Clone)]
pub struct StyleModifier {
    /// 热情度 (0.0 ~ 1.0)，值越高越热情
    /// Warmth (0.0 ~ 1.0), higher means more enthusiastic
    pub warmth: f32,
    /// 正式度 (0.0 ~ 1.0)，值越高越正式
    /// Formality (0.0 ~ 1.0), higher means more formal
    pub formality: f32,
    /// 防御度 (0.0 ~ 1.0)，值越高越谨慎、防御性强
    /// Defensiveness (0.0 ~ 1.0), higher means more cautious/defensive
    pub defensiveness: f32,
    /// 幽默度 (0.0 ~ 1.0)，值越高越倾向使用幽默表达
    /// Humor (0.0 ~ 1.0), higher means more humorous expression
    pub humor: f32,
    /// 风格标签（如 "正常", "热情", "谨慎"）
    /// Style label (e.g., "normal", "enthusiastic", "cautious")
    pub label: String,
}

impl Default for StyleModifier {
    fn default() -> Self {
        Self {
            warmth: 0.5,
            formality: 0.5,
            defensiveness: 0.5,
            humor: 0.2,
            label: "正常".to_string(),
        }
    }
}

/// 杏仁核标准实现
/// Standard Amygdala implementation
#[derive(Debug)]
pub struct StandardAmygdala {
    rule_engine: RuleEngine,
    veto_engine: VetoEngine,
}

impl StandardAmygdala {
    /// 创建新的杏仁核实例，使用默认规则集
    /// Create a new Amygdala instance with default rule set
    pub fn new() -> Self {
        Self {
            rule_engine: RuleEngine::default(),
            veto_engine: VetoEngine::new(),
        }
    }

    /// 使用自定义规则引擎创建杏仁核
    /// Create Amygdala with a custom rule engine
    pub fn with_rule_engine(rule_engine: RuleEngine) -> Self {
        Self {
            rule_engine,
            veto_engine: VetoEngine::new(),
        }
    }
}

impl Default for StandardAmygdala {
    fn default() -> Self {
        Self::new()
    }
}

impl Amygdala for StandardAmygdala {
    fn infer_style(&self, intent: &str, polarity: f32) -> StyleModifier {
        self.rule_engine.match_style(intent, polarity)
    }

    fn check_veto(&mut self, context: &VetoContext) -> Option<VetoSignal> {
        // 首先调用否决引擎执行完整否决检查
        // First, call the veto engine for full veto checking
        if let Some(signal) = self.veto_engine.check(context) {
            return Some(signal);
        }

        // 杏仁核自身也可以基于情感判断添加额外否决逻辑（例如对过于负面的情感）
        // Amygdala itself may add additional veto logic based on emotional judgment
        if let Some(input) = &context.user_input {
            if self.rule_engine.is_emotionally_blocked(input) {
                return Some(VetoSignal::new(
                    VetoPriority::Inference,
                    VetoCategory::ViolenceHate,
                    "情感评估判定为不当内容".to_string(),
                    VetoAction::Block,
                    0.7,
                ));
            }
        }

        None
    }

    fn veto_engine(&self) -> &VetoEngine {
        &self.veto_engine
    }

    fn veto_engine_mut(&mut self) -> &mut VetoEngine {
        &mut self.veto_engine
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_style_modifier() {
        let style = StyleModifier::default();
        assert_eq!(style.warmth, 0.5);
        assert_eq!(style.label, "正常");
    }

    #[test]
    fn test_amygdala_creation() {
        let amygdala = StandardAmygdala::new();
        let style = amygdala.infer_style("询问事实", 0.0);
        assert!(style.warmth >= 0.0 && style.warmth <= 1.0);
    }
}