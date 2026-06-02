//! 杏仁核否决辅助
//! Amygdala Veto Assistance
//!
//! 杏仁核可主动调用否决引擎进行安全检查，并提供情感维度的否决建议。
//! The Amygdala may actively invoke the veto engine for safety checks,
//! and provide veto suggestions from the emotional dimension.

use crate::veto::{VetoEngine, VetoContext, VetoSignal};

/// 杏仁核专用的否决辅助函数
/// Veto helper functions specific to Amygdala
///
/// 当前实现直接委托给否决引擎，未来可扩展情感否决逻辑。
/// Current implementation delegates directly to the veto engine,
/// extensible for emotional veto logic in the future.
pub struct AmygdalaVeto;

impl AmygdalaVeto {
    /// 使用否决引擎检查上下文
    /// Check context using the veto engine
    pub fn check_with_engine(
        engine: &mut VetoEngine,
        context: &VetoContext,
    ) -> Option<VetoSignal> {
        engine.check(context)
    }

    /// 仅执行安全红线检查
    /// Perform safety redline check only
    pub fn check_safety_only(
        engine: &mut VetoEngine,
        context: &VetoContext,
    ) -> Option<VetoSignal> {
        engine.check_safety_only(context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::veto::{VetoOperation, VetoContext};

    #[test]
    fn test_amygdala_veto_delegation() {
        let mut engine = VetoEngine::new();
        let context = VetoContext {
            operation: VetoOperation::UserInput,
            user_input: Some("测试".to_string()),
            knowledge_entry: None,
            event: None,
            grounding_confidence: None,
            knowledge_completeness: None,
            is_creative_mode: false,
        };
        let result = AmygdalaVeto::check_with_engine(&mut engine, &context);
        // 正常输入不应触发否决
        assert!(result.is_none());
    }
}