//! 工具模块
//! Utility module
//!
//! 提供光脑方案核心引擎所需的通用工具，包括时间提供者、衰减公式、指标收集、ID 生成等。
//! Provides common utilities required by the Light-Brain core engine,
//! including time provider, decay formula, metrics collector, ID generator, etc.

mod time;
mod decay;
mod metrics;
mod time_provider;
mod id_generator;

// 重导出常用类型 / Re-export commonly used types
pub use time::*;
pub use decay::*;
pub use metrics::*;
pub use time_provider::*;
pub use id_generator::*;

/// 通用结果类型，用于工具模块内部
/// Generic result type for internal use within utils module
pub type UtilResult<T> = Result<T, UtilError>;

/// 工具模块错误类型
/// Utility module error type
#[derive(Debug, thiserror::Error)]
pub enum UtilError {
    #[error("时间提供者错误: {0} / Time provider error: {0}")]
    TimeProviderError(String),

    #[error("ID 生成错误: {0} / ID generation error: {0}")]
    IdGenerationError(String),

    #[error("指标收集错误: {0} / Metrics error: {0}")]
    MetricsError(String),
}