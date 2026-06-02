//! 知识抽取器
//! Knowledge Extractor
//!
//! 从对话文本中通过规则匹配抽取候选知识三元组。
//! Extracts candidate knowledge triples from conversation text via rule matching.

use regex::Regex;

/// 抽取出的候选知识三元组
/// Extracted candidate knowledge triple
#[derive(Debug, Clone)]
pub(crate) struct ExtractedCandidate {
    pub subject: String,
    pub attribute: String,
    pub value: String,
    pub certainty: f32,
    pub source_text: String,
}

/// 知识抽取器
/// Knowledge extractor
#[derive(Debug)]
pub struct KnowledgeExtractor {
    patterns: Vec<ExtractionPattern>,
}

/// 抽取模式
/// Extraction pattern
#[derive(Debug, Clone)]
struct ExtractionPattern {
    regex: Regex,
    subject_group: usize,
    attribute_group: usize,
    value_group: usize,
    default_attribute: Option<String>,
}

impl KnowledgeExtractor {
    /// 创建新的知识抽取器，初始化默认中文和英文模式
    /// Create a new knowledge extractor with default Chinese and English patterns
    pub fn new() -> Self {
        let mut extractor = Self { patterns: Vec::new() };
        extractor.init_default_patterns();
        extractor
    }

    /// 初始化默认抽取模式
    /// Initialize default extraction patterns
    fn init_default_patterns(&mut self) {
        // 中文模式："X的Y是Z" / Chinese pattern: "X的Y是Z"
        if let Ok(re) = Regex::new(r"(.+)的(.+?)是(.+?)[。，\s]") {
            self.patterns.push(ExtractionPattern {
                regex: re,
                subject_group: 1,
                attribute_group: 2,
                value_group: 3,
                default_attribute: None,
            });
        }
        // 中文模式："X是Y"（属性默认为"定义"） / Chinese pattern: "X是Y" (attribute defaults to "定义")
        if let Ok(re) = Regex::new(r"(.+)是(.+?)[。，\s]") {
            self.patterns.push(ExtractionPattern {
                regex: re,
                subject_group: 1,
                attribute_group: 0, // 无显式属性 / No explicit attribute
                value_group: 2,
                default_attribute: Some("定义".to_string()),
            });
        }
        // 英文模式："X is Y" / English pattern: "X is Y"
        if let Ok(re) = Regex::new(r"(?i)(.+)\s+is\s+(.+?)[.\s]") {
            self.patterns.push(ExtractionPattern {
                regex: re,
                subject_group: 1,
                attribute_group: 0,
                value_group: 2,
                default_attribute: Some("definition".to_string()),
            });
        }
    }

    /// 从文本中抽取候选知识
    /// Extract knowledge candidates from text
    pub fn extract(&self, text: &str) -> Vec<ExtractedCandidate> {
        let mut candidates = Vec::new();
        
        for pattern in &self.patterns {
            for caps in pattern.regex.captures_iter(text) {
                let subject = caps.get(pattern.subject_group).map(|m| m.as_str().trim()).unwrap_or("");
                let value = caps.get(pattern.value_group).map(|m| m.as_str().trim()).unwrap_or("");
                
                if subject.is_empty() || value.is_empty() {
                    continue;
                }

                let attribute = if pattern.attribute_group > 0 {
                    caps.get(pattern.attribute_group).map(|m| m.as_str().trim()).unwrap_or("")
                } else {
                    pattern.default_attribute.as_deref().unwrap_or("")
                };

                candidates.push(ExtractedCandidate {
                    subject: subject.to_string(),
                    attribute: attribute.to_string(),
                    value: value.to_string(),
                    certainty: 0.5,
                    source_text: text.to_string(),
                });
            }
        }

        candidates
    }

    /// 添加自定义抽取模式
    /// Add a custom extraction pattern
    pub fn add_pattern(&mut self, pattern: ExtractionPattern) {
        self.patterns.push(pattern);
    }
}

impl Default for KnowledgeExtractor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_chinese() {
        let extractor = KnowledgeExtractor::new();
        let text = "地球的半径是6371公里。";
        let candidates = extractor.extract(text);
        assert!(!candidates.is_empty());
        let c = &candidates[0];
        assert_eq!(c.subject, "地球");
        assert_eq!(c.attribute, "半径");
        assert_eq!(c.value, "6371公里");
    }

    #[test]
    fn test_extract_chinese_definition() {
        let extractor = KnowledgeExtractor::new();
        let text = "光脑是一个生长式神经符号融合架构。";
        let candidates = extractor.extract(text);
        assert!(!candidates.is_empty());
        let c = &candidates[0];
        assert_eq!(c.subject, "光脑");
        assert_eq!(c.attribute, "定义");
        assert!(c.value.contains("生长式"));
    }
}