//! 模板管理
//! Template Management
//!
//! 管理兜底回复模板，用于生成失败或无事实可依赖时的降级输出。
//! Manages fallback response templates for degraded output when generation fails
//! or no facts are available.

use std::collections::HashMap;

/// 模板管理器
#[derive(Debug, Default)]
pub struct TemplateManager {
    templates: HashMap<String, String>,
}

impl TemplateManager {
    pub fn new() -> Self {
        let mut manager = Self::default();
        manager.init_default_templates();
        manager
    }

    fn init_default_templates(&mut self) {
        self.templates.insert("fallback_unknown".to_string(), "抱歉，我目前无法回答这个问题。".to_string());
        self.templates.insert("fallback_clarify".to_string(), "我不太确定您的意思，能再详细说明一下吗？".to_string());
        self.templates.insert("fallback_safety".to_string(), "抱歉，我无法回应这个请求。".to_string());
        self.templates.insert("greeting".to_string(), "你好！有什么可以帮助你的吗？".to_string());
    }

    /// 获取模板
    pub fn get(&self, key: &str) -> String {
        self.templates.get(key).cloned().unwrap_or_else(|| "抱歉，出了点问题。".to_string())
    }

    /// 获取澄清模板，填入选项
    pub fn get_clarification(&self, options: &[String]) -> String {
        if options.is_empty() {
            return self.get("fallback_clarify");
        }
        let options_str = options.join("、");
        format!("你是指 {} 吗？", options_str)
    }

    /// 添加自定义模板
    pub fn add_template(&mut self, key: String, template: String) {
        self.templates.insert(key, template);
    }
}