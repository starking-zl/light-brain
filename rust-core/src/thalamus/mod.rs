//! 丘脑模块
//! Thalamus Module
//!
//! 丘脑是光脑方案的感知入口，负责将原始文本输入转化为结构化的符号标签。
//! 支持上下文注入、软接地Top-K输出和接地置信度三维融合。
//! The Thalamus is the perceptual gateway of the Light-Brain Scheme,
//! responsible for converting raw text input into structured symbolic labels.
//! Supports context injection, soft grounding Top-K output, and three-dimensional grounding confidence fusion.

mod encoder;
mod prototype;
mod grounding;
mod confidence;
mod clarification;

pub use encoder::*;
pub use prototype::*;
pub use grounding::*;
pub use confidence::*;
pub use clarification::*;

use serde::{Deserialize, Serialize};

/// 丘脑 trait
/// Thalamus trait
///
/// 定义丘脑对外提供的标准接口。
/// Defines the standard interface exposed by the Thalamus.
pub trait Thalamus: Send + Sync {
    /// 感知用户输入，输出结构化标签
    /// Perceive user input and output structured labels
    ///
    /// # 参数 / Arguments
    /// * `text` - 用户输入文本 / user input text
    /// * `context_vector` - 可选的上下文向量（来自前额叶工作记忆）/ optional context vector from prefrontal working memory
    ///
    /// # 返回 / Returns
    /// 丘脑输出，包含候选标签及接地置信度。
    /// Thalamus output containing candidate labels and grounding confidence.
    fn perceive(&self, text: &str, context_vector: Option<&[f32]>) -> ThalamusOutput;

    /// 在线校准原型向量
    /// Online calibration of prototype vectors
    ///
    /// 当用户纠正感知结果时调用，更新对应标签的原型向量。
    /// Called when user corrects perception result, updates the prototype vector for the label.
    fn calibrate(&mut self, label: &str, feature_vector: &[f32], success: bool);
}

/// 丘脑输出
/// Thalamus output
#[derive(Debug, Clone)]
pub struct ThalamusOutput {
    /// 候选标签列表（按概率降序）
    /// Candidate labels list (sorted by probability descending)
    pub candidates: Vec<LabelGroundingInfo>,
    /// 是否需要澄清
    /// Whether clarification is needed
    pub need_clarification: bool,
    /// 推荐的探索路径数 K
    /// Recommended number of exploration paths K
    pub recommended_k: usize,
}

/// 单个标签的接地信息
/// Grounding information for a single label
#[derive(Debug, Clone)]
pub struct LabelGroundingInfo {
    /// 标签名称（如意图类别）
    /// Label name (e.g., intent category)
    pub label: String,
    /// 概率置信度（softmax 输出）
    /// Probability confidence (softmax output)
    pub probability: f32,
    /// 综合接地置信度
    /// Composite grounding confidence
    pub grounding_confidence: f32,
    /// 重构相似度（向量解码后与输入向量的余弦相似度）
    /// Reconstruction similarity (cosine similarity between decoded and input vector)
    pub recon_similarity: f32,
    /// 语义一致性（与小脑知识描述向量的相似度）
    /// Semantic consistency (similarity with cerebellum knowledge description vector)
    pub semantic_similarity: f32,
    /// 情感极性 (-1.0 ~ 1.0)
    /// Emotional polarity (-1.0 ~ 1.0)
    pub polarity: f32,
    /// 话题领域
    /// Topic domain
    pub domain: String,
    /// 提取的关键词
    /// Extracted keywords
    pub keywords: Vec<String>,
    /// 提取的实体
    /// Extracted entities
    pub entities: Vec<String>,
}

impl Default for LabelGroundingInfo {
    fn default() -> Self {
        Self {
            label: String::new(),
            probability: 0.0,
            grounding_confidence: 0.0,
            recon_similarity: 0.0,
            semantic_similarity: 0.0,
            polarity: 0.0,
            domain: String::new(),
            keywords: Vec::new(),
            entities: Vec::new(),
        }
    }
}

/// 感知标签（简化版，用于兼容旧接口）
/// Perception labels (simplified version for backward compatibility)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerceptionLabels {
    /// 意图类别
    pub intent: String,
    /// 情感极性
    pub polarity: f32,
    /// 话题领域
    pub domain: String,
    /// 关键词
    pub keywords: Vec<String>,
    /// 实体
    pub entities: Vec<String>,
    /// 接地置信度
    pub grounding_confidence: f32,
}

impl From<&LabelGroundingInfo> for PerceptionLabels {
    fn from(info: &LabelGroundingInfo) -> Self {
        Self {
            intent: info.label.clone(),
            polarity: info.polarity,
            domain: info.domain.clone(),
            keywords: info.keywords.clone(),
            entities: info.entities.clone(),
            grounding_confidence: info.grounding_confidence,
        }
    }
}

/// 丘脑配置
/// Thalamus configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThalamusConfig {
    /// 上下文注入是否启用
    pub context_injection_enabled: bool,
    /// 向量维度
    pub vector_dim: usize,
    /// 默认门控参数（上下文更新时的衰减）
    pub default_gate: f32,
    /// 融合方式（"concat" 或 "add"）
    pub fusion_method: String,
    /// 是否归一化输出
    pub normalize_output: bool,
    /// 软接地默认 K 值
    pub default_k: usize,
    /// 高价值领域列表（医疗、法律等，需要更多探索路径）
    pub high_value_domains: Vec<String>,
    /// 创意模式 K 值
    pub creative_mode_k: usize,
    /// 冷启动成熟度阈值
    pub cold_start_maturity_threshold: f32,
    /// 概率阈值（正常模式）
    pub probability_threshold_normal: f32,
    /// 概率阈值（创意模式）
    pub probability_threshold_creative: f32,
}

impl Default for ThalamusConfig {
    fn default() -> Self {
        Self {
            context_injection_enabled: true,
            vector_dim: 768,
            default_gate: 0.7,
            fusion_method: "concat".to_string(),
            normalize_output: true,
            default_k: 2,
            high_value_domains: vec!["医疗".to_string(), "法律".to_string(), "金融".to_string()],
            creative_mode_k: 4,
            cold_start_maturity_threshold: 0.5,
            probability_threshold_normal: 0.05,
            probability_threshold_creative: 0.02,
        }
    }
}

/// 丘脑标准实现
/// Standard Thalamus implementation
#[derive(Debug)]
pub struct StandardThalamus {
    config: ThalamusConfig,
    encoder: Box<dyn NeuralEncoder>,
    prototype_store: PrototypeStore,
    grounding_engine: GroundingEngine,
    clarification_trigger: ClarificationTrigger,
}

impl StandardThalamus {
    /// 创建新的丘脑实例
    /// Create a new Thalamus instance
    pub fn new(config: ThalamusConfig, encoder: Box<dyn NeuralEncoder>) -> Self {
        let prototype_store = PrototypeStore::new(config.vector_dim);
        let grounding_engine = GroundingEngine::new(config.clone());
        let clarification_trigger = ClarificationTrigger::new();

        Self {
            config,
            encoder,
            prototype_store,
            grounding_engine,
            clarification_trigger,
        }
    }

    /// 获取原型存储的可变引用（用于初始化或加载）
    /// Get mutable reference to prototype store (for initialization or loading)
    pub fn prototype_store_mut(&mut self) -> &mut PrototypeStore {
        &mut self.prototype_store
    }
}

impl Thalamus for StandardThalamus {
    fn perceive(&self, text: &str, context_vector: Option<&[f32]>) -> ThalamusOutput {
        // 1. 神经编码（调用 Python 侧）
        let (feature_vector, reconstructed_vector) = self.encoder.encode_with_context(text, context_vector);

        // 2. 原型匹配，获取 Top-K 候选
        let candidates = self.prototype_store.match_top_k(
            &feature_vector,
            self.config.default_k * 2, // 获取更多候选供后续过滤
        );

        // 3. 计算每个候选的接地置信度
        let mut candidates_with_confidence: Vec<LabelGroundingInfo> = candidates
            .into_iter()
            .map(|(label, prob, proto_vec)| {
                self.grounding_engine.compute_grounding_info(
                    label,
                    prob,
                    &feature_vector,
                    &reconstructed_vector,
                    &proto_vec,
                )
            })
            .collect();

        // 4. 按综合接地置信度排序
        candidates_with_confidence
            .sort_by(|a, b| b.grounding_confidence.partial_cmp(&a.grounding_confidence).unwrap());

        // 5. 动态决定 K 值
        let recommended_k = self.grounding_engine.dynamic_k(
            &candidates_with_confidence,
            &self.config,
        );

        // 6. 截取前 K 个候选
        candidates_with_confidence.truncate(recommended_k);

        // 7. 判断是否需要澄清
        let need_clarification = self.clarification_trigger.should_clarify(
            &candidates_with_confidence,
            &self.config,
        );

        ThalamusOutput {
            candidates: candidates_with_confidence,
            need_clarification,
            recommended_k,
        }
    }

    fn calibrate(&mut self, label: &str, feature_vector: &[f32], success: bool) {
        self.prototype_store.calibrate(label, feature_vector, success);
    }
}

// 注意：不提供 Default 实现，必须通过 new 显式注入编码器。
// Note: No Default implementation provided; encoder must be explicitly injected via new.