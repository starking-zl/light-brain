//! 工脑管理器
//! Worker Manager
//!
//! 管理工脑（分身）的创建、执行与回收。工脑是光脑的"试衣间"，
//! 在隔离环境中探索角色扮演和创造性任务，成功后产出体验报告。
//! Manages the creation, execution, and recycling of Worker-Brains (avatars).
//! Worker-Brains are Light-Brain's "fitting room", exploring role-playing and
//! creative tasks in an isolated environment, returning experience reports on success.

use crate::broca::DecisionPackage;
use crate::utils::{current_timestamp, generate_uuid};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 工脑任务类型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkerTaskType {
    RolePlay,
    CreativeExploration,
    ConservativeQuery,
}

/// 工脑体验报告
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerReport {
    pub id: String,
    pub task_type: WorkerTaskType,
    pub role_description: String,
    pub output: String,
    pub self_discovery: Option<String>,
    pub user_feedback: Option<String>,
    pub success: bool,
    pub timestamp: u64,
}

/// 工脑实例
#[derive(Debug)]
struct WorkerInstance {
    id: String,
    task_type: WorkerTaskType,
    created_at: u64,
    // 实际实现中应包含独立的推理上下文
}

/// 工脑管理器
#[derive(Debug, Default)]
pub struct WorkerManager {
    active_workers: HashMap<String, WorkerInstance>,
    reports: Vec<WorkerReport>,
    max_concurrent: usize,
}

impl WorkerManager {
    pub fn new() -> Self {
        Self {
            active_workers: HashMap::new(),
            reports: Vec::new(),
            max_concurrent: 5,
        }
    }

    /// 创建新的工脑分身
    pub fn spawn(&mut self, task_type: WorkerTaskType, role_description: &str) -> String {
        if self.active_workers.len() >= self.max_concurrent {
            // 回收最老的工脑
            if let Some(oldest) = self.active_workers.values().min_by_key(|w| w.created_at) {
                self.recycle(&oldest.id.clone());
            }
        }

        let id = format!("worker_{}", generate_uuid());
        let worker = WorkerInstance {
            id: id.clone(),
            task_type: task_type.clone(),
            created_at: current_timestamp(),
        };
        self.active_workers.insert(id.clone(), worker);
        id
    }

    /// 工脑执行任务并返回报告
    pub fn execute(&mut self, worker_id: &str, decision: &DecisionPackage) -> WorkerReport {
        // 实际实现中，工脑会使用独立的上下文执行生成，此处简化为直接生成报告
        let report = WorkerReport {
            id: worker_id.to_string(),
            task_type: WorkerTaskType::CreativeExploration,
            role_description: "探索角色".to_string(),
            output: format!("对 {} 的探索性回应", decision.intent),
            self_discovery: Some("发现了新的表达方式".to_string()),
            user_feedback: None,
            success: true,
            timestamp: current_timestamp(),
        };
        self.reports.push(report.clone());
        report
    }

    /// 回收工脑
    pub fn recycle(&mut self, worker_id: &str) -> Option<WorkerInstance> {
        self.active_workers.remove(worker_id)
    }

    /// 获取所有体验报告
    pub fn reports(&self) -> &[WorkerReport] {
        &self.reports
    }

    /// 从报告中提取自我发现
    pub fn extract_self_discoveries(&self) -> Vec<String> {
        self.reports
            .iter()
            .filter_map(|r| r.self_discovery.clone())
            .collect()
    }
}