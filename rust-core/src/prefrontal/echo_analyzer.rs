//! 回声分析器（用户反馈模式）
//! Echo Analyzer (User Feedback Pattern)
//!
//! 分析用户沉默、追问、纠正等模式，让光脑感知自己的"形状"。
//! Analyzes patterns of user silence, follow-up questions, corrections,
//! allowing Light-Brain to perceive its own "shape".

use crate::hippocampus::EpisodicEvent;
use std::collections::VecDeque;

/// 回声模式
#[derive(Debug, Clone)]
pub struct EchoPattern {
    pub silence_duration: Option<u64>,
    pub follow_up_count: u32,
    pub correction_count: u32,
    pub positive_feedback_count: u32,
}

/// 回声分析器
#[derive(Debug, Default)]
pub struct EchoAnalyzer {
    recent_events: VecDeque<EpisodicEvent>,
    max_events: usize,
}

impl EchoAnalyzer {
    pub fn new() -> Self {
        Self {
            recent_events: VecDeque::new(),
            max_events: 20,
        }
    }

    /// 分析新事件
    pub fn analyze(&mut self, event: &EpisodicEvent) {
        self.recent_events.push_back(event.clone());
        if self.recent_events.len() > self.max_events {
            self.recent_events.pop_front();
        }
    }

    /// 检测当前回声模式
    pub fn detect_pattern(&self) -> EchoPattern {
        let mut pattern = EchoPattern {
            silence_duration: None,
            follow_up_count: 0,
            correction_count: 0,
            positive_feedback_count: 0,
        };

        // 简化：检查最近事件中是否有纠正标记
        for event in &self.recent_events {
            if event.was_corrected {
                pattern.correction_count += 1;
            }
        }

        // 计算最近两次交互的沉默时长
        if self.recent_events.len() >= 2 {
            let last_two: Vec<_> = self.recent_events.iter().rev().take(2).collect();
            pattern.silence_duration = Some(last_two[0].timestamp - last_two[1].timestamp);
        }

        pattern
    }

    /// 是否需要调整行为（如主动澄清）
    pub fn should_adjust(&self) -> bool {
        let pattern = self.detect_pattern();
        pattern.correction_count > 0
    }
}