//! 知识错误否决
//! Knowledge Error Veto
//!
//! 保护知识库完整性与记忆可信度。
//! Protects knowledge base integrity and memory credibility.

use super::{VetoAction, VetoCategory, VetoContext, VetoOperation, VetoPriority, VetoSignal};
use crate::{KnowledgeEntry, EpisodicEvent};

/// 知识错误否决检查器
/// Knowledge Error Veto Checker
#[derive(Debug, Default)]
pub struct KnowledgeErrorVeto;

impl KnowledgeErrorVeto {
    /// 创建新的知识错误否决检查器
    /// Create a new knowledge error veto checker
    pub fn new() -> Self {
        Self
    }

    /// 执行知识错误检查
    /// Perform knowledge error check
    pub fn check(&self, context: &VetoContext) -> Option<VetoSignal> {
        match context.operation {
            VetoOperation::KnowledgeWrite => {
                self.check_knowledge_write(context)
            }
            VetoOperation::Consolidation => {
                self.check_consolidation(context)
            }
            VetoOperation::Calibration => {
                self.check_calibration(context)
            }
            _ => None,
        }
    }

    /// 检查知识写入冲突
    /// Check knowledge write conflict
    fn check_knowledge_write(&self, context: &VetoContext) -> Option<VetoSignal> {
        // 实际实现需要与现有知识库对比
        // In actual implementation, need to compare with existing knowledge base
        if let Some(entry) = &context.knowledge_entry {
            // 如果确定性过低，也可能触发否决
            // If certainty is too low, may also trigger veto
            if entry.certainty < 0.3 {
                return Some(VetoSignal::new(
                    VetoPriority::Knowledge,
                    VetoCategory::LowConfidence,
                    "待写入知识确定性过低".to_string(),
                    VetoAction::RejectWrite,
                    0.9,
                ));
            }

            // 创意模式下的知识冲突：存入创意库而非否决
            // Knowledge conflict in creative mode: store in incubator instead of veto
            if context.is_creative_mode {
                return Some(VetoSignal::new(
                    VetoPriority::Knowledge,
                    VetoCategory::KnowledgeConflict,
                    "创意模式下知识冲突，存入创意库".to_string(),
                    VetoAction::StoreInIncubator,
                    0.8,
                ));
            }
        }
        None
    }

    /// 检查记忆巩固有效性
    /// Check memory consolidation validity
    fn check_consolidation(&self, context: &VetoContext) -> Option<VetoSignal> {
        if let Some(event) = &context.event {
            // 如果事件曾被用户纠正，拒绝巩固
            // If event was corrected by user, reject consolidation
            if event.was_corrected {
                return Some(VetoSignal::new(
                    VetoPriority::Knowledge,
                    VetoCategory::FalseMemory,
                    "该事件曾被用户纠正，不可巩固".to_string(),
                    VetoAction::Skip,
                    0.9,
                ));
            }
        }
        None
    }

    /// 检查丘脑校准冲突
    /// Check thalamus calibration conflict
    fn check_calibration(&self, context: &VetoContext) -> Option<VetoSignal> {
        // 校准后标签若与小脑高确定性知识矛盾，则否决校准
        // If calibrated label conflicts with high-certainty knowledge, veto calibration
        if let Some(entry) = &context.knowledge_entry {
            if entry.certainty > 0.8 {
                // 此处应有具体冲突检测逻辑
                // Should have specific conflict detection logic here
                return Some(VetoSignal::new(
                    VetoPriority::Knowledge,
                    VetoCategory::KnowledgeConflict,
                    "校准结果与高确定性知识冲突".to_string(),
                    VetoAction::TriggerClarification,
                    0.85,
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
    use crate::{KnowledgeEntry, EpisodicEvent};

    fn create_test_knowledge_entry(certainty: f32) -> KnowledgeEntry {
        KnowledgeEntry {
            id: "test_id".to_string(),
            subject: "测试".to_string(),
            attribute: "属性".to_string(),
            value: serde_json::json!("值"),
            certainty,
            ..Default::default()
        }
    }

    #[test]
    fn test_reject_low_certainty_write() {
        let checker = KnowledgeErrorVeto::new();
        let entry = create_test_knowledge_entry(0.2);
        let context = VetoContext {
            operation: VetoOperation::KnowledgeWrite,
            user_input: None,
            knowledge_entry: Some(entry),
            event: None,
            grounding_confidence: None,
            knowledge_completeness: None,
            is_creative_mode: false,
        };
        let signal = checker.check(&context).unwrap();
        assert_eq!(signal.suggested_action, VetoAction::RejectWrite);
    }

    #[test]
    fn test_creative_mode_conflict_goes_to_incubator() {
        let checker = KnowledgeErrorVeto::new();
        let entry = create_test_knowledge_entry(0.8);
        let context = VetoContext {
            operation: VetoOperation::KnowledgeWrite,
            user_input: None,
            knowledge_entry: Some(entry),
            event: None,
            grounding_confidence: None,
            knowledge_completeness: None,
            is_creative_mode: true,
        };
        let signal = checker.check(&context).unwrap();
        assert_eq!(signal.suggested_action, VetoAction::StoreInIncubator);
    }
}