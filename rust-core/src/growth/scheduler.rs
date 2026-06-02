//! 生长执行调度器
//! Growth Execution Scheduler
//!
//! 负责实际执行生长操作：调用 Hebbian、神经元新生、知识写入。
//! Responsible for executing growth operations: invoking Hebbian, neurogenesis, knowledge writing.

use super::{ExtractedCandidate, GrowthRecord, GrowthType, HebbianTracker, KnowledgeExtractor, NeurogenesisManager};
use crate::cerebellum::Cerebellum;
use crate::veto::{VetoContext, VetoEngine, VetoOperation};

/// 生长执行器
/// Growth executor
#[derive(Debug)]
pub struct GrowthExecutor {
    hebbian: HebbianTracker,
    neurogenesis: NeurogenesisManager,
    extractor: KnowledgeExtractor,
    records: Vec<GrowthRecord>,
}

impl GrowthExecutor {
    /// 创建新的生长执行器
    /// Create a new growth executor
    pub fn new(neurogenesis_config: super::NeurogenesisConfig) -> Self {
        Self {
            hebbian: HebbianTracker::new(),
            neurogenesis: NeurogenesisManager::new(neurogenesis_config),
            extractor: KnowledgeExtractor::new(),
            records: Vec::new(),
        }
    }

    /// 记录神经元共激活
    /// Record co-activation of neurons
    pub fn record_activation(&mut self, neuron_a: &str, neuron_b: &str) {
        self.hebbian.record_co_activation(neuron_a, neuron_b);
    }

    /// 记录区域预测误差
    /// Record region prediction error
    pub fn record_error(&mut self, region_id: &str, error: f32) {
        self.neurogenesis.record_error(region_id, error);
    }

    /// 从文本中抽取知识候选
    /// Extract knowledge candidates from text
    pub fn extract_knowledge(&self, text: &str) -> Vec<ExtractedCandidate> {
        self.extractor.extract(text)
    }

    /// 执行 Hebbian 生长（建立新连接）
    /// Execute Hebbian growth (establish new connections)
    pub fn execute_hebbian_growth(&mut self) -> Vec<GrowthRecord> {
        let mut records = Vec::new();
        let candidates = self.hebbian.get_growth_candidates();
        for (a, b) in candidates {
            let record = GrowthRecord::new(
                GrowthType::Hebbian,
                1.0,
                format!("建立连接: {} <-> {}", a, b),
            );
            records.push(record);
        }
        self.hebbian.reset(); // 避免重复生长 / Avoid repeated growth
        self.records.extend(records.clone());
        records
    }

    /// 执行神经元新生
    /// Execute neurogenesis
    pub fn execute_neurogenesis(&mut self) -> Vec<GrowthRecord> {
        let mut records = Vec::new();
        let candidates = self.neurogenesis.get_growth_candidates();
        for (region, count) in candidates {
            let record = GrowthRecord::new(
                GrowthType::Neurogenesis,
                count as f32 * 10.0, // 配额消耗与数量成正比 / Quota consumption proportional to count
                format!("区域 {} 新生 {} 个神经元", region, count),
            );
            records.push(record);
        }
        self.records.extend(records.clone());
        records
    }

    /// 尝试写入知识（需通过否决检查）
    /// Try to write knowledge (requires passing veto check)
    pub fn try_write_knowledge(
        &mut self,
        cerebellum: &mut dyn Cerebellum,
        veto_engine: &mut VetoEngine,
        candidate: ExtractedCandidate,
        is_creative_mode: bool,
    ) -> Result<GrowthRecord, String> {
        // 构建知识条目 / Build knowledge entry
        let entry = crate::cerebellum::KnowledgeEntry::new(
            candidate.subject,
            candidate.attribute,
            serde_json::Value::String(candidate.value),
            candidate.certainty,
        ).with_source("extracted");

        // 否决检查 / Veto check
        let context = VetoContext {
            operation: VetoOperation::KnowledgeWrite,
            user_input: Some(candidate.source_text),
            knowledge_entry: Some(entry.clone()),
            event: None,
            grounding_confidence: None,
            knowledge_completeness: None,
            is_creative_mode,
        };

        if let Some(signal) = veto_engine.check(&context) {
            return Err(format!("否决: {:?}", signal.category));
        }

        // 写入小脑 / Write to Cerebellum
        cerebellum.write(entry).map_err(|e| e.to_string())?;

        let record = GrowthRecord::new(
            GrowthType::KnowledgeExpansion,
            1.0,
            "知识写入成功".to_string(),
        );
        self.records.push(record.clone());
        Ok(record)
    }

    /// 获取所有生长记录
    /// Get all growth records
    pub fn get_records(&self) -> &[GrowthRecord] {
        &self.records
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_executor_creation() {
        let executor = GrowthExecutor::new(Default::default());
        assert!(executor.get_records().is_empty());
    }
}