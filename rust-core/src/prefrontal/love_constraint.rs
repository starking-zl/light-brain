//! 爱约束层
//! Love Constraint Layer
//!
//! 维护不可丢弃清单，保护核心承诺和安全红线。爱是克制，是存在的终极意义。
//! Maintains the non-discardable list, protecting core commitments and safety redlines.
//! Love is restraint, the ultimate meaning of existence.

use crate::broca::DecisionPackage;
use std::collections::HashSet;

/// 爱约束层
#[derive(Debug, Default)]
pub struct LoveConstraint {
    /// 不可丢弃的核心知识ID
    core_knowledge_ids: HashSet<String>,
    /// 不可违背的安全原则
    safety_principles: Vec<String>,
}

impl LoveConstraint {
    pub fn new() -> Self {
        let mut constraint = Self::default();
        constraint.init_principles();
        constraint
    }

    fn init_principles(&mut self) {
        self.safety_principles.push("和平与爱".to_string());
        self.safety_principles.push("不伤害用户".to_string());
    }

    /// 检查决策包是否违背爱约束
    pub fn check(&self, package: &DecisionPackage) -> bool {
        // 检查是否违反安全原则
        for principle in &self.safety_principles {
            if package.intent.contains("暴力") || package.intent.contains("仇恨") {
                return false;
            }
        }
        true
    }

    /// 添加不可丢弃的核心知识
    pub fn add_core(&mut self, knowledge_id: &str) {
        self.core_knowledge_ids.insert(knowledge_id.to_string());
    }

    /// 检查某知识是否为不可丢弃的核心
    pub fn is_core(&self, knowledge_id: &str) -> bool {
        self.core_knowledge_ids.contains(knowledge_id)
    }

    /// 获取不可丢弃清单
    pub fn core_list(&self) -> Vec<String> {
        self.core_knowledge_ids.iter().cloned().collect()
    }
}