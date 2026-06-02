//! 交互式澄清触发
//! Interactive Clarification Trigger
//!
//! 判断是否需要对用户进行澄清提问。
//! Determines whether clarification questions need to be asked to the user.

use super::{LabelGroundingInfo, ThalamusConfig};

/// 澄清触发器
/// Clarification trigger
#[derive(Debug, Default)]
pub struct ClarificationTrigger {
    consecutive_low_confidence: u32,
}

impl ClarificationTrigger {
    pub fn new() -> Self {
        Self::default()
    }

    /// 判断是否需要澄清
    /// Determine if clarification is needed
    pub fn should_clarify(
        &mut self,
        candidates: &[LabelGroundingInfo],
        config: &ThalamusConfig,
    ) -> bool {
        if candidates.is_empty() {
            self.consecutive_low_confidence += 1;
            return self.consecutive_low_confidence >= 3;
        }

        let best = &candidates[0];
        
        // 最佳候选接地置信度过低
        if best.grounding_confidence < 0.5 {
            self.consecutive_low_confidence += 1;
            return self.consecutive_low_confidence >= 3;
        }

        // 最佳与次佳的差距过小（歧义）
        if candidates.len() >= 2 {
            let second = &candidates[1];
            if best.probability - second.probability < 0.1 {
                return true;
            }
        }

        self.consecutive_low_confidence = 0;
        false
    }

    /// 重置连续低置信度计数
    pub fn reset(&mut self) {
        self.consecutive_low_confidence = 0;
    }
}