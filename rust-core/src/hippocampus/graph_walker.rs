//! 图扩散游走
//! Graph Diffusion Walker
//!
//! 从入口节点沿关联边受限扩散，计算节点激活值。
//! Restricted diffusion along edges from entry nodes, computing node activation values.

use super::{EdgeStore, MemoryEdge, MemoryNode, NodeStore};

/// 扩散结果
/// Diffusion result
#[derive(Debug, Clone)]
pub struct DiffusionResult {
    /// 激活的节点及其激活值
    pub activated_nodes: Vec<(String, f32)>,
    /// 扩散路径追踪
    pub path_trace: Vec<String>,
}

/// 图游走器
/// Graph walker
pub struct GraphWalker {
    depth: usize,
    decay: f32,
    threshold: f32,
}

impl GraphWalker {
    /// 创建新的游走器
    /// Create a new walker
    pub fn new(depth: usize, decay: f32) -> Self {
        Self {
            depth,
            decay,
            threshold: 0.1,
        }
    }

    /// 设置激活阈值
    /// Set activation threshold
    pub fn with_threshold(mut self, threshold: f32) -> Self {
        self.threshold = threshold;
        self
    }

    /// 执行扩散
    /// Perform diffusion
    pub fn walk(
        &self,
        entry_nodes: &[String],
        _node_store: &dyn NodeStore,  // 保留用于未来扩展（如节点属性过滤）
        edge_store: &EdgeStore,
    ) -> DiffusionResult {
        use std::collections::{HashMap, VecDeque};

        let mut activation: HashMap<String, f32> = HashMap::new();
        let mut trace = Vec::new();

        // 初始化入口节点激活值为 1.0
        // Initialize entry nodes with activation 1.0
        for node_id in entry_nodes {
            activation.insert(node_id.clone(), 1.0);
            trace.push(format!("entry:{}", node_id));
        }

        let mut queue: VecDeque<(String, usize)> = entry_nodes
            .iter()
            .map(|id| (id.clone(), 0))
            .collect();

        while let Some((current, step)) = queue.pop_front() {
            if step >= self.depth {
                continue;
            }

            let current_activation = *activation.get(&current).unwrap_or(&0.0);
            if current_activation < self.threshold {
                continue;
            }

            let outgoing = edge_store.get_outgoing(&current);
            for edge in outgoing {
                let transferred = current_activation * edge.weight * self.decay.powi(step as i32);
                let target = &edge.target_node;

                let new_activation = activation.get(target).unwrap_or(&0.0) + transferred;
                activation.insert(target.clone(), new_activation);
                
                trace.push(format!("{}->{}:{}", current, target, transferred));

                if new_activation >= self.threshold {
                    queue.push_back((target.clone(), step + 1));
                }
            }
        }

        // 过滤低于阈值的节点并按激活值排序
        // Filter nodes below threshold and sort by activation
        let mut activated: Vec<(String, f32)> = activation
            .into_iter()
            .filter(|(_, v)| *v >= self.threshold)
            .collect();
        activated.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        DiffusionResult {
            activated_nodes: activated,
            path_trace: trace,
        }
    }
}