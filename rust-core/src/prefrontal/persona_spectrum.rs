//! 多面光谱
//! Persona Spectrum
//!
//! 从工脑报告、回声模式和昼夜行为中抽象跨场景稳定特质，
//! 构成光脑的"人格光谱"。
//! Abstracts stable cross-situational traits from worker reports, echo patterns,
//! and circadian behaviors, forming Light-Brain's "persona spectrum".

use crate::prefrontal::{EchoPattern, WorkerReport};
use std::collections::HashMap;

/// 人格特质维度
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TraitDimension {
    Warmth,        // 热情
    Formality,     // 正式
    Creativity,    // 创造性
    Caution,       // 谨慎
    Humor,         // 幽默
}

/// 多面光谱
#[derive(Debug, Default)]
pub struct PersonaSpectrum {
    traits: HashMap<TraitDimension, f32>,
    observation_count: u32,
}

impl PersonaSpectrum {
    pub fn new() -> Self {
        let mut traits = HashMap::new();
        traits.insert(TraitDimension::Warmth, 0.5);
        traits.insert(TraitDimension::Formality, 0.5);
        traits.insert(TraitDimension::Creativity, 0.5);
        traits.insert(TraitDimension::Caution, 0.5);
        traits.insert(TraitDimension::Humor, 0.3);
        Self {
            traits,
            observation_count: 0,
        }
    }

    /// 从工脑报告中学习
    pub fn learn_from_worker(&mut self, report: &WorkerReport) {
        // 根据报告中的自我发现调整特质
        if let Some(discovery) = &report.self_discovery {
            if discovery.contains("热情") {
                self.adjust(TraitDimension::Warmth, 0.05);
            }
            if discovery.contains("创意") || discovery.contains("创造") {
                self.adjust(TraitDimension::Creativity, 0.05);
            }
        }
        self.observation_count += 1;
    }

    /// 从回声模式中学习
    pub fn learn_from_echo(&mut self, pattern: &EchoPattern) {
        if pattern.correction_count > 0 {
            self.adjust(TraitDimension::Caution, 0.03);
        }
        if pattern.positive_feedback_count > 0 {
            self.adjust(TraitDimension::Warmth, 0.02);
        }
    }

    fn adjust(&mut self, dim: TraitDimension, delta: f32) {
        if let Some(v) = self.traits.get_mut(&dim) {
            *v = (*v + delta).clamp(0.0, 1.0);
        }
    }

    /// 获取当前光谱
    pub fn spectrum(&self) -> &HashMap<TraitDimension, f32> {
        &self.traits
    }

    /// 获取特定特质值
    pub fn get(&self, dim: TraitDimension) -> f32 {
        self.traits.get(&dim).copied().unwrap_or(0.5)
    }
}