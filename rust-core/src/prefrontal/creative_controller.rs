//! 三旋钮创造性控制回路
//! Three-Knob Creative Control Loop
//!
//! 通过温度τ、门控γ、评估ε三个参数的联动，调节系统的探索-利用平衡。
//! 支持严谨推理、日常对话、头脑风暴、反事实想象四种预设模式。
//! Regulates the exploration-exploitation balance through the linkage of three parameters:
//! temperature τ, gate γ, and evaluation ε.
//! Supports four preset modes: Rigorous, Daily, Creative, Counterfactual.

use serde::{Deserialize, Serialize};

/// 创造性模式
/// Creative mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CreativeMode {
    /// 严谨推理：低温度，高门控
    Rigorous,
    /// 日常对话：中等温度，中等门控
    Daily,
    /// 头脑风暴：高温度，低门控
    Creative,
    /// 反事实想象：高温度，极低门控
    Counterfactual,
}

impl Default for CreativeMode {
    fn default() -> Self {
        CreativeMode::Daily
    }
}

impl CreativeMode {
    /// 获取温度参数 τ (0.0 ~ 2.0)
    pub fn temperature(&self) -> f32 {
        match self {
            CreativeMode::Rigorous => 0.4,
            CreativeMode::Daily => 0.8,
            CreativeMode::Creative => 1.3,
            CreativeMode::Counterfactual => 1.2,
        }
    }

    /// 获取门控参数 γ (0.0 ~ 1.0)
    pub fn gate(&self) -> f32 {
        match self {
            CreativeMode::Rigorous => 0.9,
            CreativeMode::Daily => 0.6,
            CreativeMode::Creative => 0.3,
            CreativeMode::Counterfactual => 0.1,
        }
    }

    /// 获取评估参数 ε 的基础值 (0.0 ~ 1.0)
    pub fn evaluation_base(&self) -> f32 {
        match self {
            CreativeMode::Rigorous => 0.8,
            CreativeMode::Daily => 0.5,
            CreativeMode::Creative => 0.3,
            CreativeMode::Counterfactual => 0.2,
        }
    }

    /// 是否为软约束模式（创意/反事实模式下第二级护栏切换为软约束）
    pub fn is_soft_constraint_mode(&self) -> bool {
        matches!(self, CreativeMode::Creative | CreativeMode::Counterfactual)
    }
}

/// 三旋钮控制器
/// Three-Knob Controller
///
/// 管理温度τ、门控γ、评估ε的联动关系，支持根据上下文动态微调。
/// Manages the linkage among τ, γ, ε, supporting dynamic fine-tuning based on context.
#[derive(Debug, Clone)]
pub struct CreativeController {
    mode: CreativeMode,
    /// 联动系数 α (用于 γ = max(γ_min, γ_0 - α × (τ - τ_0)))
    alpha: f32,
    /// 联动系数 β (用于 ε_novelty = ε_0 + β × (γ_0 - γ))
    beta: f32,
    /// 门控最小值
    gamma_min: f32,
}

impl CreativeController {
    pub fn new(mode: CreativeMode) -> Self {
        Self {
            mode,
            alpha: 0.5,
            beta: 0.3,
            gamma_min: 0.1,
        }
    }

    /// 设置当前模式
    pub fn set_mode(&mut self, mode: CreativeMode) {
        self.mode = mode;
    }

    /// 获取当前模式
    pub fn mode(&self) -> CreativeMode {
        self.mode
    }

    /// 获取当前温度 τ
    pub fn temperature(&self) -> f32 {
        self.mode.temperature()
    }

    /// 获取当前门控 γ（基础值）
    pub fn gate(&self) -> f32 {
        self.mode.gate()
    }

    /// 获取动态门控 γ（根据实际温度微调）
    /// γ = max(γ_min, γ_0 - α × (τ - τ_0))
    pub fn dynamic_gate(&self, actual_temperature: f32) -> f32 {
        let tau_0 = self.mode.temperature();
        let gamma_0 = self.mode.gate();
        let gamma = gamma_0 - self.alpha * (actual_temperature - tau_0);
        gamma.max(self.gamma_min)
    }

    /// 获取评估参数 ε（基础值）
    pub fn evaluation(&self) -> f32 {
        self.mode.evaluation_base()
    }

    /// 获取动态评估参数 ε（根据门控微调）
    /// ε_novelty = ε_0 + β × (γ_0 - γ)
    pub fn dynamic_evaluation(&self, actual_gate: f32) -> f32 {
        let gamma_0 = self.mode.gate();
        let epsilon_0 = self.mode.evaluation_base();
        epsilon_0 + self.beta * (gamma_0 - actual_gate)
    }

    /// 获取三旋钮参数的完整快照
    pub fn snapshot(&self) -> KnobSnapshot {
        KnobSnapshot {
            mode: self.mode,
            temperature: self.temperature(),
            gate: self.gate(),
            evaluation: self.evaluation(),
        }
    }

    /// 根据对话上下文微调参数（预留接口）
    pub fn adapt_to_context(&mut self, context_complexity: f32, user_preference: f32) {
        // 复杂语境可适当降低门控，鼓励联想
        // 用户偏好可调节基础模式
        // 具体实现留作后续实验校准
        let _ = (context_complexity, user_preference);
    }
}

impl Default for CreativeController {
    fn default() -> Self {
        Self::new(CreativeMode::default())
    }
}

/// 旋钮参数快照
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnobSnapshot {
    pub mode: CreativeMode,
    pub temperature: f32,
    pub gate: f32,
    pub evaluation: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mode_parameters() {
        assert_eq!(CreativeMode::Rigorous.temperature(), 0.4);
        assert_eq!(CreativeMode::Rigorous.gate(), 0.9);
        assert_eq!(CreativeMode::Creative.temperature(), 1.3);
        assert_eq!(CreativeMode::Counterfactual.gate(), 0.1);
    }

    #[test]
    fn test_dynamic_gate() {
        let controller = CreativeController::new(CreativeMode::Daily);
        // τ_0 = 0.8, γ_0 = 0.6, α = 0.5
        // 若实际温度升高到 1.0，γ 应降低
        let dynamic = controller.dynamic_gate(1.0);
        assert!(dynamic < 0.6);
        assert!(dynamic >= controller.gamma_min);
    }

    #[test]
    fn test_soft_constraint_flag() {
        assert!(!CreativeMode::Rigorous.is_soft_constraint_mode());
        assert!(CreativeMode::Creative.is_soft_constraint_mode());
    }
}