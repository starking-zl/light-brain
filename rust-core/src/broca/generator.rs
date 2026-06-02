//! 语言生成器
//! Language Generator
//!
//! 调用 Python 侧轻量语言模型（SLM）生成候选文本。
//! Calls Python-side lightweight language model (SLM) to generate candidate texts.

use super::{CreativeMode, SemanticPlan};

/// 语言生成器
/// Language generator
#[derive(Debug)]
pub struct LanguageGenerator {
    // 实际模型调用通过 Python 绑定，Rust 侧仅定义接口
    // Actual model invocation via Python binding; Rust side only defines interface
}

impl LanguageGenerator {
    pub fn new() -> Self {
        Self
    }

    /// 生成多个候选文本
    /// Generate multiple candidate texts
    ///
    /// 实际实现将通过 PyO3 调用 Python 侧的 SLM。
    /// Actual implementation will call Python-side SLM via PyO3.
    pub fn generate_candidates(&self, plan: &SemanticPlan, mode: &CreativeMode) -> Vec<String> {
        // 占位实现：返回基于模板的简单候选
        // Placeholder: return simple template-based candidates
        let prompt = self.build_prompt(plan, mode);
        vec![self.mock_generate(&prompt)]
    }

    fn build_prompt(&self, plan: &SemanticPlan, mode: &CreativeMode) -> String {
        let facts_str = plan.facts_to_express
            .iter()
            .map(|f| format!("{}: {}", f.attribute, f.value))
            .collect::<Vec<_>>()
            .join("；");
        
        format!(
            "意图：{}\n事实：{}\n语气：{}\n请生成回复：",
            plan.intent, facts_str, plan.tone_description
        )
    }

    fn mock_generate(&self, prompt: &str) -> String {
        // 极简 mock，实际应由模型生成
        if prompt.contains("事实：") {
            "根据已知信息，这是相关回答。".to_string()
        } else {
            "我不太确定，请提供更多信息。".to_string()
        }
    }
}

impl Default for LanguageGenerator {
    fn default() -> Self {
        Self::new()
    }
}