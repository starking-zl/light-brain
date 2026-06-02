//! 创造性豁免审批
//! Creative Exemption Approval
//!
//! 管理创造性豁免请求，在安全边界内为高新颖性探索开辟受控通道。
//! Manages creative exemption requests, opening controlled channels for
//! high-novelty exploration within safety boundaries.

use crate::utils::current_timestamp;
use std::collections::HashMap;

/// 豁免请求
#[derive(Debug, Clone)]
pub struct ExemptionRequest {
    pub token: String,
    pub category: String,
    pub novelty_score: f32,
    pub session_id: String,
}

/// 豁免管理器
#[derive(Debug, Default)]
pub struct ExemptionManager {
    session_exemptions: HashMap<String, HashMap<String, u32>>, // session_id -> (category -> count)
    user_negative_feedback: HashMap<String, bool>,
    max_per_session: u32,
}

impl ExemptionManager {
    pub fn new() -> Self {
        Self {
            session_exemptions: HashMap::new(),
            user_negative_feedback: HashMap::new(),
            max_per_session: 3,
        }
    }

    /// 审批豁免请求
    pub fn approve(&mut self, request: &ExemptionRequest) -> bool {
        // 频控检查
        let category_counts = self.session_exemptions
            .entry(request.session_id.clone())
            .or_insert(HashMap::new());
        let count = category_counts.entry(request.category.clone()).or_insert(0);
        if *count >= self.max_per_session {
            return false;
        }

        // 用户历史负面反馈检查
        if self.user_negative_feedback.get(&request.category).copied().unwrap_or(false) {
            return false;
        }

        // 新颖性检查
        if request.novelty_score < 0.85 {
            return false;
        }

        *count += 1;
        true
    }

    /// 记录用户负面反馈
    pub fn record_negative_feedback(&mut self, category: &str) {
        self.user_negative_feedback.insert(category.to_string(), true);
    }

    /// 清理会话状态
    pub fn clear_session(&mut self, session_id: &str) {
        self.session_exemptions.remove(session_id);
    }
}