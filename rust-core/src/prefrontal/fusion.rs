//! 融合引擎
//! Fusion Engine
//!
//! 将各模块输出组合为符号决策包。
//! Combines outputs from various modules into a symbolic decision package.

use super::ScheduleAction;
use crate::amygdala::StyleModifier;
use crate::broca::{DecisionPackage, FactItem, FallbackAction};
use crate::thalamus::{LabelGroundingInfo, ThalamusOutput};

/// 融合引擎
/// Fusion engine
#[derive(Debug, Default)]
pub struct FusionEngine;

impl FusionEngine {
    pub fn new() -> Self {
        Self
    }

    /// 融合生成决策包
    pub fn fuse(&self, perception: &ThalamusOutput, schedule: ScheduleAction) -> DecisionPackage {
        let best = perception.candidates.first();
        
        let intent = best.map(|c| c.label.clone()).unwrap_or_default();
        let grounding_confidence = best.map(|c| c.grounding_confidence).unwrap_or(0.0);
        
        // 事实内容需从小脑获取，此处为占位
        let facts = Vec::new();
        
        // 风格需从杏仁核获取，此处为默认
        let style = StyleModifier::default();
        
        let needs_clarification = perception.need_clarification || schedule.trigger_clarification;
        
        let fallback_action = if facts.is_empty() && schedule.call_cerebellum {
            FallbackAction::Unknown
        } else {
            FallbackAction::Normal
        };

        DecisionPackage {
            intent,
            facts,
            style,
            needs_clarification,
            clarification_options: best.map(|c| c.entities.clone()).unwrap_or_default(),
            fallback_action,
            grounding_confidence,
            knowledge_completeness: if facts.is_empty() { 0.0 } else { 0.5 },
        }
    }
}