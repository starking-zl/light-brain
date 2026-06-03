# 丘脑子模块 - 感知与符号接地
# Thalamus submodule - Perception and symbol grounding
# 负责将原始文本转化为结构化符号标签
# Responsible for converting raw text into structured symbolic labels

from .encoder import ThalamusEncoder
from .grounding import GroundingLayer

__all__ = ["ThalamusEncoder", "GroundingLayer"]


class PrototypeStore:
    """
    原型向量存储
    Prototype vector store
    简化实现：从JSON配置加载原型向量
    """
    
    def __init__(self):
        self.prototypes = {}
    
    def load_from_file(self, filepath: str):
        """从JSON文件加载原型向量"""
        import json
        from pathlib import Path
        if Path(filepath).exists():
            with open(filepath, 'r', encoding='utf-8') as f:
                self.prototypes = json.load(f)
    
    def get_prototypes(self):
        return self.prototypes


class Thalamus:
    """
    丘脑 - 光脑方案的感知入口
    Thalamus - Perception entry of Light-Brain Scheme
    将用户输入转化为意图、情感、领域等符号标签
    Converts user input into symbolic labels such as intent, polarity, domain
    """
    
    def __init__(self, prototypes: dict = None):
        """
        初始化丘脑模块
        Initialize Thalamus module
        
        Args:
            prototypes: 原型向量配置 / Prototype vector configuration
        """
        self.encoder = ThalamusEncoder()
        self.grounding = GroundingLayer(prototypes or {})
    
    def perceive(self, text: str) -> dict:
        """
        感知用户输入，输出符号标签
        Perceive user input and output symbolic labels
        
        Args:
            text: 用户输入文本 / User input text
            
        Returns:
            包含意图、情感、领域、关键词、确定性的字典
            Dictionary containing intent, polarity, domain, keywords, certainty
        """
        # 1. 编码为连续向量 / Encode to continuous vector
        vector = self.encoder.encode(text)
        
        # 2. 符号接地：向量 -> 标签概率分布 / Symbol grounding: vector -> label probability distribution
        intent_dist = self.grounding.classify(vector, "intent")
        polarity_dist = self.grounding.classify(vector, "polarity")
        domain_dist = self.grounding.classify(vector, "domain")
        
        # 3. 提取关键词（简化实现，使用分词） / Extract keywords (simplified, using tokenization)
        keywords = self._extract_keywords(text)
        
        # 4. 取最大概率标签 / Take maximum probability label
        intent = max(intent_dist, key=intent_dist.get) if intent_dist else "Unknown"
        polarity = max(polarity_dist, key=polarity_dist.get) if polarity_dist else "Neutral"
        domain = max(domain_dist, key=domain_dist.get) if domain_dist else "Daily"
        certainty = intent_dist.get(intent, 0.5) if intent_dist else 0.5
        
        return {
            "intent": intent,
            "polarity": polarity,
            "domain": domain,
            "keywords": keywords,
            "certainty": certainty
        }
    
    def _extract_keywords(self, text: str) -> list:
        """
        从文本中提取关键词
        Extract keywords from text
        简化实现：按空格/标点分词，取前5个非停用词
        Simplified: tokenize by space/punctuation, take first 5 non-stopwords
        """
        # 简单分词 / Simple tokenization
        import re
        words = re.findall(r'[\u4e00-\u9fff]+|[a-zA-Z]+', text)
        # 停用词过滤（极简） / Stopword filtering (minimal)
        stopwords = {'的', '了', '是', '在', '我', '你', '他', '她', '它', '这', '那', '吗', '呢', '吧'}
        keywords = [w for w in words if w.lower() not in stopwords]
        return keywords[:5]