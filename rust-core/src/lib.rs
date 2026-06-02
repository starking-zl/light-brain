//! 光脑方案 Rust 核心引擎
//! Light-Brain Scheme Rust Core Engine
//!
//! 本库实现了光脑方案的六个核心脑区模块（丘脑、前额叶、杏仁核、小脑、海马体、布罗卡区）
//! 以及贯穿性机制：生长机制、三层记忆管理、否决机制。
//! This library implements the six core brain-region modules (Thalamus, Prefrontal Cortex,
//! Amygdala, Cerebellum, Hippocampus, Broca's Area) and cross-cutting mechanisms:
//! Growth Mechanism, Three-Tier Memory Management, and Veto Mechanism.

// ========== 模块声明 / Module Declarations ==========
pub mod utils;
pub mod thalamus;
pub mod prefrontal;
pub mod amygdala;
pub mod cerebellum;
pub mod hippocampus;
pub mod broca;
pub mod growth;
pub mod memory;
pub mod veto;

// ========== 核心类型导出 / Core Type Exports ==========

// 通用类型 / Common types
pub use utils::{
    TimeProvider, SystemTimeProvider, MockTimeProvider,
    DecayFormula, MetricsCollector,
};

// 丘脑 / Thalamus
pub use thalamus::{
    Thalamus, ThalamusOutput, LabelGroundingInfo,
};

// 前额叶 / Prefrontal Cortex
pub use prefrontal::{
    Prefrontal, DecisionPackage, WorkingMemory, DialogStateTracker,
};

// 杏仁核 / Amygdala
pub use amygdala::{
    Amygdala, StyleModifier, VetoSignal, VetoPriority,
};

// 小脑 / Cerebellum
pub use cerebellum::{
    Cerebellum, KnowledgeEntry, QueryResult,
};

// 海马体 / Hippocampus
pub use hippocampus::{
    Hippocampus, EpisodicEvent, ImpressionPack, MemoryNode, MemoryEdge,
};

// 布罗卡区 / Broca's Area
pub use broca::{
    Broca, GeneratedText, GuardrailTier,
};

// 生长机制 / Growth mechanism
pub use growth::{
    GrowthExecutor,      // 执行层调度器 / Execution layer scheduler
    HebbianTracker,
    NeurogenesisConfig,
    KnowledgeExtractor,
    SatiationMemory,
    FibonacciSequence,
};

// 前额叶的生长策略调度器（重导出以明确区分）
// Prefrontal growth strategy scheduler (re-exported for clear distinction)
pub use prefrontal::GrowthScheduler;

// 记忆管理 / Memory management
pub use memory::{
    TierManager, GarbageCollector, MemoryTier, DecayConfig,
};

// 否决机制 / Veto mechanism
pub use veto::{
    VetoEngine, SafetyRedline, KnowledgeErrorVeto, InferenceContaminationVeto,
};

// ========== 全局常量 / Global Constants ==========

/// 光脑方案版本号
/// Light-Brain Scheme version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// 工作记忆默认容量（7±2 个组块）
/// Default working memory capacity (7±2 chunks)
pub const DEFAULT_WORKING_MEMORY_CAPACITY: usize = 7;

/// 图扩散默认深度
/// Default graph diffusion depth
pub const DEFAULT_GRAPH_DIFFUSION_DEPTH: usize = 2;

/// 衰减公式全局 lambda 系数（可配置）
/// Global decay lambda coefficient (configurable)
pub const DEFAULT_DECAY_LAMBDA: f32 = 0.1;

/// 种子知识库文件路径
/// Seed knowledge base file path
pub const SEED_KNOWLEDGE_PATH: &str = "data/knowledge/seed.json";

/// 配置文件目录
/// Configuration directory
pub const CONFIG_DIR: &str = "config";

// ========== 错误类型 / Error Types ==========

/// 光脑方案核心错误类型
/// Light-Brain Scheme core error type
#[derive(Debug, thiserror::Error)]
pub enum LightBrainError {
    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON 序列化错误: {0}")]
    Json(#[from] serde_json::Error),

    #[error("知识冲突: {0}")]
    KnowledgeConflict(String),

    #[error("否决触发: {priority:?} - {reason}")]
    VetoTriggered { priority: VetoPriority, reason: String },

    #[error("模块未初始化: {module_name}")]
    ModuleNotInitialized { module_name: String },

    #[error("配置错误: {0}")]
    ConfigError(String),

    #[error("数据库错误: {0}")]
    DatabaseError(String),

    #[error("未知错误: {0}")]
    Unknown(String),
}

/// 光脑方案结果类型别名
/// Light-Brain Scheme result type alias
pub type Result<T> = std::result::Result<T, LightBrainError>;

// ========== 初始化函数 / Initialization Function ==========

/// 初始化光脑方案核心系统
/// Initialize the Light-Brain Scheme core system
///
/// 此函数应在前端启动时调用一次，完成各模块的初始化与连接。
/// This function should be called once at startup to initialize and connect all modules.
pub fn init(config_dir: Option<&str>) -> Result<()> {
    // 初始化日志（若需要）
    // Initialize logging (if needed)
    
    // 加载全局配置
    // Load global configuration
    let _config_dir = config_dir.unwrap_or(CONFIG_DIR);
    
    // 各模块将在各自的 init 函数中完成初始化
    // Each module will complete initialization in its own init function
    
    Ok(())
}

// ========== 单元测试 / Unit Tests ==========
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_constant() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let lb_err: LightBrainError = io_err.into();
        assert!(matches!(lb_err, LightBrainError::Io(_)));
    }
}