//! 唯一 ID 生成器
//! Unique ID generator
//!
//! 为知识条目、事件、库存点等生成全局唯一标识。
//! Generates globally unique identifiers for knowledge entries, events, memory nodes, etc.

use uuid::Uuid;

/// 生成 UUID v4 字符串
/// Generate UUID v4 string
pub fn generate_uuid() -> String {
    Uuid::new_v4().to_string()
}

/// 生成带前缀的 ID（便于区分类型）
/// Generate prefixed ID for type distinction
pub fn generate_prefixed_id(prefix: &str) -> String {
    format!("{}_{}", prefix, generate_uuid())
}

/// 知识条目 ID 前缀
/// Knowledge entry ID prefix
pub const KNOWLEDGE_ENTRY_PREFIX: &str = "k";
/// 情景事件 ID 前缀
/// Episodic event ID prefix
pub const EPISODIC_EVENT_PREFIX: &str = "evt";
/// 记忆库存点 ID 前缀
/// Memory node ID prefix
pub const MEMORY_NODE_PREFIX: &str = "node";
/// 记忆关联边 ID 前缀
/// Memory edge ID prefix
pub const MEMORY_EDGE_PREFIX: &str = "edge";
/// 生长日志 ID 前缀
/// Growth log ID prefix
pub const GROWTH_LOG_PREFIX: &str = "grow";
/// 否决日志 ID 前缀
/// Veto log ID prefix
pub const VETO_LOG_PREFIX: &str = "veto";

/// 生成知识条目 ID
/// Generate knowledge entry ID
pub fn generate_knowledge_id() -> String {
    generate_prefixed_id(KNOWLEDGE_ENTRY_PREFIX)
}

/// 生成情景事件 ID
/// Generate episodic event ID
pub fn generate_event_id() -> String {
    generate_prefixed_id(EPISODIC_EVENT_PREFIX)
}

/// 生成记忆库存点 ID
/// Generate memory node ID
pub fn generate_node_id() -> String {
    generate_prefixed_id(MEMORY_NODE_PREFIX)
}

/// 生成记忆关联边 ID
/// Generate memory edge ID
pub fn generate_edge_id() -> String {
    generate_prefixed_id(MEMORY_EDGE_PREFIX)
}

/// 生成生长日志 ID
/// Generate growth log ID
pub fn generate_growth_log_id() -> String {
    generate_prefixed_id(GROWTH_LOG_PREFIX)
}

/// 生成否决日志 ID
/// Generate veto log ID
pub fn generate_veto_log_id() -> String {
    generate_prefixed_id(VETO_LOG_PREFIX)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uuid_uniqueness() {
        let id1 = generate_uuid();
        let id2 = generate_uuid();
        assert_ne!(id1, id2);
        assert_eq!(id1.len(), 36); // 标准 UUID 格式长度 / Standard UUID format length
    }

    #[test]
    fn test_prefixed_id() {
        let id = generate_prefixed_id("test");
        assert!(id.starts_with("test_"));
    }

    #[test]
    fn test_knowledge_id() {
        let id = generate_knowledge_id();
        assert!(id.starts_with("k_"));
    }
}