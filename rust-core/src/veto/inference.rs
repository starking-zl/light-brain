//! 推理污染否决
//! Inference Contamination Veto
//!
//! 防止低质量感知或知识不完整时强行推理，污染当前对话。
//! Prevents forced inference when perception quality is low or knowledge is incomplete,
//! avoiding contamination of the current conversation.

use super::{VetoAction, VetoCategory, VetoContext, VetoOperation, VetoPriority, VetoSignal};

/// 推理污染否决检查器
/// Inference Contamination Veto Checker
#[derive(Debug, Default)]
pub struct InferenceContaminationVeto;

impl InferenceContaminationVeto {
    /// 创建新的推理污染否决检查器
    /// Create a new inference contamination veto checker
    pub fn new() -> Self {
        Self
    }

    /// 执行推理污染检查
    /// Perform inference contamination check
    pub fn check(&self, context: &VetoContext) -> Option<VetoSignal> {
        match context.operation {
            VetoOperation::Inference => {
                self.check_inference_quality(context)
            }
            _ => None,
        }
    }

    /// 检查推理质量
    /// Check inference quality
    fn check_inference_quality(&self, context: &VetoContext) -> Option<VetoSignal> {
        // 检查接地置信度
        // Check grounding confidence
        if let Some(confidence) = context.grounding_confidence {
            if confidence < 0.3 {
                // 创意模式下静默（不阻断），严谨模式下触发澄清
                // Silent in creative mode, trigger clarification in rigorous mode
                let action = if context.is_creative_mode {
                    VetoAction::Skip
                } else {
                    VetoAction::TriggerClarification
                };
                return Some(VetoSignal::new(
                    VetoPriority::Inference,
                    VetoCategory::LowConfidence,
                    format!("接地置信度过低 ({:.2})", confidence),
                    action,
                    0.8,
                ));
            }
        }

        // 检查知识完整度
        // Check knowledge completeness
        if let Some(completeness) = context.knowledge_completeness {
            if completeness < 0.2 {
                // 知识不完整时，如果前额叶计划以检索结果为主要依据，则降级
                // If knowledge is incomplete and prefrontal plans to rely on retrieval, degrade
                return Some(VetoSignal::new(
                    VetoPriority::Inference,
                    VetoCategory::IncompleteKnowledge,
                    format!("知识完整度过低 ({:.2})", completeness),
                    VetoAction::DegradeToUnknown,
                    0.75,
                ));
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::veto::VetoContext;

    #[test]
    fn test_low_confidence_triggers_clarification_in_normal_mode() {
        let checker = InferenceContaminationVeto::new();
        let context = VetoContext {
            operation: VetoOperation::Inference,
            user_input: None,
            knowledge_entry: None,
            event: None,
            grounding_confidence: Some(0.2),
            knowledge_completeness: None,
            is_creative_mode: false,
        };
        let signal = checker.check(&context).unwrap();
        assert_eq!(signal.suggested_action, VetoAction::TriggerClarification);
    }

    #[test]
    fn test_low_confidence_skipped_in_creative_mode() {
        let checker = InferenceContaminationVeto::new();
        let context = VetoContext {
            operation: VetoOperation::Inference,
            user_input: None,
            knowledge_entry: None,
            event: None,
            grounding_confidence: Some(0.2),
            knowledge_completeness: None,
            is_creative_mode: true,
        };
        let signal = checker.check(&context).unwrap();
        assert_eq!(signal.suggested_action, VetoAction::Skip);
    }

    #[test]
    fn test_incomplete_knowledge_degrade() {
        let checker = InferenceContaminationVeto::new();
        let context = VetoContext {
            operation: VetoOperation::Inference,
            user_input: None,
            knowledge_entry: None,
            event: None,
            grounding_confidence: Some(0.8),
            knowledge_completeness: Some(0.1),
            is_creative_mode: false,
        };
        let signal = checker.check(&context).unwrap();
        assert_eq!(signal.suggested_action, VetoAction::DegradeToUnknown);
    }
}