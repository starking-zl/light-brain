//! 语义规划
//! Semantic Planning
//!
//! 将符号决策包转换为抽象语义骨架，明确信息结构和关键实体。
//! Converts symbolic decision package into abstract semantic skeleton,
//! clarifying information structure and key entities.

use super::{CreativeMode, DecisionPackage, FactItem, FallbackAction};
use serde::{Deserialize, Serialize};

/// 语义规划结果
/// Semantic plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticPlan {
    /// 意图类别
    pub intent: String,
    /// 需要表达的事实列表
    pub facts_to_express: Vec<FactItem>,
    /// 语气描述
    pub tone_description: String,
    /// 结构模板（如 "陈述事实", "问答", "澄清"）
    pub structure: String,
    /// 关键实体（必须准确呈现）
    pub key_entities: Vec<String>,
    /// 禁止表达的内容（来自护栏）
    pub forbidden_expressions: Vec<String>,
}

/// 语义规划器
/// Semantic planner
#[derive(Debug, Default)]
pub struct SemanticPlanner;

impl SemanticPlanner {
    pub fn new() -> Self {
        Self
    }

    /// 根据决策包和模式生成语义规划
    pub fn plan(&self, package: &DecisionPackage, mode: &CreativeMode) -> SemanticPlan {
        let structure = self.determine_structure(package);
        let tone = self.determine_tone(&package.style, mode);
        let facts = package.facts.clone();
        let key_entities = self.extract_key_entities(&facts);
        
        SemanticPlan {
            intent: package.intent.clone(),
            facts_to_express: facts,
            tone_description: tone,
            structure,
            key_entities,
            forbidden_expressions: Vec::new(), // 由护栏层填充
        }
    }

    fn determine_structure(&self, package: &DecisionPackage) -> String {
        match package.fallback_action {
            FallbackAction::Unknown => "unknown".to_string(),
            FallbackAction::Clarify => "clarify".to_string(),
            _ if package.facts.is_empty() => "simple_response".to_string(),
            _ => "informative".to_string(),
        }
    }

    fn determine_tone(&self, style: &super::StyleModifier, mode: &CreativeMode) -> String {
        if style.warmth > 0.7 {
            "热情友好".to_string()
        } else if style.formality > 0.7 {
            "正式专业".to_string()
        } else if style.defensiveness > 0.7 {
            "谨慎保守".to_string()
        } else {
            match mode {
                CreativeMode::Rigorous => "严谨客观",
                CreativeMode::Daily => "自然流畅",
                CreativeMode::Creative => "开放联想",
                CreativeMode::Counterfactual => "反事实推演",
            }.to_string()
        }
    }

    fn extract_key_entities(&self, facts: &[FactItem]) -> Vec<String> {
        facts.iter().map(|f| f.subject.clone()).collect()
    }
}