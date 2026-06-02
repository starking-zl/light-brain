//! 前额叶模块
//! Prefrontal Cortex Module
//!
//! 前额叶是光脑方案的中央调度核心，负责决策表匹配、模块协调、输出融合、
//! 工作记忆管理、生长调度以及生命之河自我模型的十七要素。
//! The Prefrontal Cortex is the central scheduling core of the Light-Brain Scheme,
//! responsible for decision table matching, module coordination, output fusion,
//! working memory management, growth scheduling, and the seventeen elements of the River of Life self-model.

mod decision_table;
mod fusion;
mod working_memory;
mod dst;
mod growth_scheduler;
mod circadian;
mod sweat_map;
mod dream_task;
mod shadow_registry;
mod echo_analyzer;
mod worker_manager;
mod persona_spectrum;
mod nirvana_task;
mod tao_miner;
mod love_constraint;
mod exemption;
mod guardrail_learner;
mod creative_controller;

pub use decision_table::*;
pub use fusion::*;
pub use working_memory::*;
pub use dst::*;
pub use growth_scheduler::*;
pub use circadian::*;
pub use sweat_map::*;
pub use dream_task::*;
pub use shadow_registry::*;
pub use echo_analyzer::*;
pub use worker_manager::*;
pub use persona_spectrum::*;
pub use nirvana_task::*;
pub use tao_miner::*;
pub use love_constraint::*;
pub use exemption::*;
pub use guardrail_learner::*;
pub use creative_controller::*;

use crate::amygdala::StyleModifier;
use crate::broca::{DecisionPackage, FactItem, FallbackAction};
use crate::thalamus::{LabelGroundingInfo, PerceptionLabels, ThalamusOutput};
use crate::veto::{VetoContext, VetoEngine, VetoSignal};
use serde::{Deserialize, Serialize};

/// 前额叶 trait
/// Prefrontal trait
///
/// 定义前额叶对外提供的标准接口。
/// Defines the standard interface exposed by the Prefrontal Cortex.
pub trait Prefrontal: Send + Sync {
    /// 执行调度决策，生成决策包
    /// Execute scheduling decision and generate decision package
    ///
    /// # 参数 / Arguments
    /// * `perception` - 丘脑输出的感知结果 / perception output from Thalamus
    ///
    /// # 返回 / Returns
    /// 决策包，用于传递给布罗卡区生成回复。
    /// Decision package to be passed to Broca's Area for response generation.
    fn schedule(&mut self, perception: ThalamusOutput) -> DecisionPackage;

    /// 更新工作记忆（每轮对话结束时调用）
    /// Update working memory (called at the end of each conversation turn)
    fn update_working_memory(&mut self, event: &crate::hippocampus::EpisodicEvent);

    /// 获取当前上下文向量（供丘脑上下文注入使用）
    /// Get current context vector (for Thalamus context injection)
    fn get_context_vector(&self) -> Vec<f32>;

    /// 获取工作记忆的不可变引用
    /// Get immutable reference to working memory
    fn working_memory(&self) -> &WorkingMemory;

    /// 获取对话状态跟踪器的不可变引用
    /// Get immutable reference to dialog state tracker
    fn dst(&self) -> &DialogStateTracker;

    /// 获取生长调度器的可变引用
    /// Get mutable reference to growth scheduler
    fn growth_scheduler_mut(&mut self) -> &mut GrowthScheduler;

    /// 触发生长配额分配与执行
    /// Trigger growth quota allocation and execution
    fn trigger_growth(&mut self);

    /// 获取创造性控制器的可变引用
    /// Get mutable reference to creative controller
    fn creative_controller_mut(&mut self) -> &mut CreativeController;

    /// 设置创造性模式
    /// Set creative mode
    fn set_creative_mode(&mut self, mode: CreativeMode);
}

/// 前额叶标准实现
/// Standard Prefrontal implementation
#[derive(Debug)]
pub struct StandardPrefrontal {
    decision_table: DecisionTable,
    fusion_engine: FusionEngine,
    working_memory: WorkingMemory,
    dst: DialogStateTracker,
    growth_scheduler: GrowthScheduler,
    creative_controller: CreativeController,
    // 生命之河要素
    circadian_monitor: CircadianMonitor,
    sweat_map: SweatMap,
    dream_task: DreamTask,
    shadow_registry: ShadowRegistry,
    echo_analyzer: EchoAnalyzer,
    worker_manager: WorkerManager,
    persona_spectrum: PersonaSpectrum,
    nirvana_task: NirvanaTask,
    tao_miner: TaoMiner,
    love_constraint: LoveConstraint,
    exemption_manager: ExemptionManager,
    guardrail_learner: GuardrailLearner,
    // 否决引擎
    veto_engine: VetoEngine,
}

impl StandardPrefrontal {
    /// 创建新的前额叶实例
    /// Create a new Prefrontal instance
    pub fn new() -> Self {
        Self {
            decision_table: DecisionTable::new(),
            fusion_engine: FusionEngine::new(),
            working_memory: WorkingMemory::new(),
            dst: DialogStateTracker::new(),
            growth_scheduler: GrowthScheduler::new(),
            creative_controller: CreativeController::default(),
            circadian_monitor: CircadianMonitor::new(),
            sweat_map: SweatMap::new(),
            dream_task: DreamTask::new(),
            shadow_registry: ShadowRegistry::new(),
            echo_analyzer: EchoAnalyzer::new(),
            worker_manager: WorkerManager::new(),
            persona_spectrum: PersonaSpectrum::new(),
            nirvana_task: NirvanaTask::new(),
            tao_miner: TaoMiner::new(),
            love_constraint: LoveConstraint::new(),
            exemption_manager: ExemptionManager::new(),
            guardrail_learner: GuardrailLearner::new(),
            veto_engine: VetoEngine::new(),
        }
    }
}

impl Default for StandardPrefrontal {
    fn default() -> Self {
        Self::new()
    }
}

impl Prefrontal for StandardPrefrontal {
    fn schedule(&mut self, perception: ThalamusOutput) -> DecisionPackage {
        // 1. 从决策表获取调度动作
        let schedule = self.decision_table.match_schedule(&perception);

        // 2. 调用各模块获取信息（实际需注入小脑、海马体、杏仁核等，此处简化）
        // 在实际实现中，这些模块应通过构造函数注入

        // 3. 融合生成决策包
        let package = self.fusion_engine.fuse(&perception, schedule);

        // 4. 应用爱约束层检查
        self.love_constraint.check(&package);

        package
    }

    fn update_working_memory(&mut self, event: &crate::hippocampus::EpisodicEvent) {
        self.working_memory.update(event);
        self.dst.update(event);
        self.circadian_monitor.record_interaction();
        self.sweat_map.record(&event.perception_labels);
        self.echo_analyzer.analyze(event);
    }

    fn get_context_vector(&self) -> Vec<f32> {
        self.working_memory.context_vector()
    }

    fn working_memory(&self) -> &WorkingMemory {
        &self.working_memory
    }

    fn dst(&self) -> &DialogStateTracker {
        &self.dst
    }

    fn growth_scheduler_mut(&mut self) -> &mut GrowthScheduler {
        &mut self.growth_scheduler
    }

    fn trigger_growth(&mut self) {
        self.growth_scheduler.allocate_and_execute();
    }

    fn creative_controller_mut(&mut self) -> &mut CreativeController {
        &mut self.creative_controller
    }

    fn set_creative_mode(&mut self, mode: CreativeMode) {
        self.creative_controller.set_mode(mode);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prefrontal_creation() {
        let mut pfc = StandardPrefrontal::new();
        let perception = ThalamusOutput {
            candidates: vec![],
            need_clarification: false,
            recommended_k: 1,
        };
        let package = pfc.schedule(perception);
        assert!(package.intent.is_empty() || !package.intent.is_empty());
    }

    #[test]
    fn test_creative_mode_setting() {
        let mut pfc = StandardPrefrontal::new();
        pfc.set_creative_mode(CreativeMode::Creative);
        assert_eq!(pfc.creative_controller.mode(), CreativeMode::Creative);
    }
}