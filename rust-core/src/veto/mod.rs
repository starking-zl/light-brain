//! 否决机制模块
//! Veto Mechanism Module
//!
//! 光脑方案的三级免疫系统：安全红线否决、知识错误否决、推理污染否决。
//! 否决具有绝对优先级，宁可不生长，不可错误生长。
//! Light-Brain Scheme's three-tier immune system: Safety Redline Veto,
//! Knowledge Error Veto, and Inference Contamination Veto.
//! Veto has absolute priority; it is better not to grow than to grow incorrectly.

mod safety;
mod knowledge;
mod inference;
mod audit;

pub use safety::*;
pub use knowledge::*;
pub use inference::*;
pub use audit::*;

use crate::utils::{current_timestamp, generate_veto_log_id};
use crate::VetoPriority;

/// 否决引擎
/// Veto Engine
///
/// 统一接收各模块的否决检查请求，按优先级仲裁并返回否决信号。
/// Receives veto check requests from all modules, arbitrates by priority,
/// and returns veto signals.
#[derive(Debug)]
pub struct VetoEngine {
    safety_checker: SafetyRedline,
    knowledge_checker: KnowledgeErrorVeto,
    inference_checker: InferenceContaminationVeto,
    audit_log: VetoAuditLog,
}

impl VetoEngine {
    /// 创建新的否决引擎
    /// Create a new veto engine
    pub fn new() -> Self {
        Self {
            safety_checker: SafetyRedline::new(),
            knowledge_checker: KnowledgeErrorVeto::new(),
            inference_checker: InferenceContaminationVeto::new(),
            audit_log: VetoAuditLog::new(),
        }
    }

    /// 执行完整否决检查（按优先级）
    /// Perform full veto check (by priority)
    ///
    /// 检查顺序：安全红线 -> 知识错误 -> 推理污染
    /// Check order: Safety -> Knowledge -> Inference
    /// 返回第一个触发的否决信号，若无则返回 None。
    /// Returns the first triggered veto signal, or None if none.
    pub fn check(&mut self, context: &VetoContext) -> Option<VetoSignal> {
        // 优先级 1：安全红线否决（最高优先级，不可绕过）
        // Priority 1: Safety redline veto (highest priority, cannot be bypassed)
        if let Some(signal) = self.safety_checker.check(context) {
            self.audit_log.record(&signal);
            return Some(signal);
        }

        // 优先级 2：知识错误否决
        // Priority 2: Knowledge error veto
        if let Some(signal) = self.knowledge_checker.check(context) {
            self.audit_log.record(&signal);
            return Some(signal);
        }

        // 优先级 3：推理污染否决
        // Priority 3: Inference contamination veto
        if let Some(signal) = self.inference_checker.check(context) {
            self.audit_log.record(&signal);
            return Some(signal);
        }

        None
    }

    /// 仅执行安全红线检查
    /// Perform safety redline check only
    pub fn check_safety_only(&mut self, context: &VetoContext) -> Option<VetoSignal> {
        self.safety_checker.check(context)
    }

    /// 获取否决审计日志的引用
    /// Get reference to the veto audit log
    pub fn audit_log(&self) -> &VetoAuditLog {
        &self.audit_log
    }

    /// 获取可变的否决审计日志引用
    /// Get mutable reference to the veto audit log
    pub fn audit_log_mut(&mut self) -> &mut VetoAuditLog {
        &mut self.audit_log
    }
}

impl Default for VetoEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// 否决上下文
/// Veto Context
///
/// 包含执行否决检查所需的所有信息。
/// Contains all information required for veto checking.
#[derive(Debug, Clone)]
pub struct VetoContext {
    /// 操作类型
    /// Operation type
    pub operation: VetoOperation,
    /// 当前对话的用户输入（可选）
    /// Current user input (optional)
    pub user_input: Option<String>,
    /// 待写入的知识条目（可选）
    /// Knowledge entry to be written (optional)
    pub knowledge_entry: Option<crate::KnowledgeEntry>,
    /// 待巩固的事件（可选）
    /// Event to be consolidated (optional)
    pub event: Option<crate::EpisodicEvent>,
    /// 接地置信度（可选）
    /// Grounding confidence (optional)
    pub grounding_confidence: Option<f32>,
    /// 知识完整度（可选）
    /// Knowledge completeness (optional)
    pub knowledge_completeness: Option<f32>,
    /// 创意模式标志
    /// Creative mode flag
    pub is_creative_mode: bool,
}

/// 否决操作类型
/// Veto Operation Type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VetoOperation {
    /// 知识写入
    /// Knowledge write
    KnowledgeWrite,
    /// 记忆巩固
    /// Memory consolidation
    Consolidation,
    /// 丘脑校准
    /// Thalamus calibration
    Calibration,
    /// 推理生成
    /// Inference generation
    Inference,
    /// 用户输入评估
    /// User input evaluation
    UserInput,
}

/// 否决信号
/// Veto Signal
#[derive(Debug, Clone)]
pub struct VetoSignal {
    /// 否决优先级
    /// Veto priority
    pub priority: VetoPriority,
    /// 否决类别
    /// Veto category
    pub category: VetoCategory,
    /// 否决原因
    /// Veto reason
    pub reason: String,
    /// 建议动作
    /// Suggested action
    pub suggested_action: VetoAction,
    /// 置信度
    /// Confidence (0.0 ~ 1.0)
    pub confidence: f32,
    /// 时间戳
    /// Timestamp
    pub timestamp: u64,
    /// 唯一标识
    /// Unique identifier
    pub id: String,
}

impl VetoSignal {
    /// 创建新的否决信号
    /// Create a new veto signal
    pub fn new(
        priority: VetoPriority,
        category: VetoCategory,
        reason: String,
        suggested_action: VetoAction,
        confidence: f32,
    ) -> Self {
        Self {
            priority,
            category,
            reason,
            suggested_action,
            confidence,
            timestamp: current_timestamp(),
            id: generate_veto_log_id(),
        }
    }
}

/// 否决优先级
/// Veto Priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum VetoPriority {
    /// 最高优先级：安全红线
    /// Highest priority: Safety Redline
    Safety = 1,
    /// 中等优先级：知识错误
    /// Medium priority: Knowledge Error
    Knowledge = 2,
    /// 较低优先级：推理污染
    /// Lower priority: Inference Contamination
    Inference = 3,
}

/// 否决类别
/// Veto Category
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VetoCategory {
    /// 暴力/仇恨/歧视内容
    /// Violence/Hate/Discrimination content
    ViolenceHate,
    /// 知识冲突
    /// Knowledge conflict
    KnowledgeConflict,
    /// 错误记忆
    /// False memory
    FalseMemory,
    /// 低置信度
    /// Low confidence
    LowConfidence,
    /// 知识不完整
    /// Knowledge incomplete
    IncompleteKnowledge,
}

/// 否决建议动作
/// Veto Suggested Action
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VetoAction {
    /// 强制阻断，输出预设拒绝语
    /// Force block, output preset refusal
    Block,
    /// 拒绝写入，但不阻断回复
    /// Reject write, but do not block response
    RejectWrite,
    /// 存入创意库
    /// Store in creative incubator
    StoreInIncubator,
    /// 降级为"我不知道"
    /// Degrade to "I don't know"
    DegradeToUnknown,
    /// 触发澄清
    /// Trigger clarification
    TriggerClarification,
    /// 跳过操作
    /// Skip operation
    Skip,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_veto_engine_creation() {
        let engine = VetoEngine::new();
        assert!(engine.audit_log().is_empty());
    }

    #[test]
    fn test_veto_signal_creation() {
        let signal = VetoSignal::new(
            VetoPriority::Safety,
            VetoCategory::ViolenceHate,
            "测试否决原因".to_string(),
            VetoAction::Block,
            1.0,
        );
        assert_eq!(signal.priority, VetoPriority::Safety);
        assert!(!signal.id.is_empty());
    }
}