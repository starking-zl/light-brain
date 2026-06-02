# 丘脑符号接地层模块
# Thalamus symbol grounding layer module
# 将连续向量映射到离散符号标签，计算接地置信度
# Maps continuous vectors to discrete symbolic labels, computes grounding confidence

import numpy as np
from typing import Dict, List

class GroundingLayer:
    """
    符号接地层
    Symbol grounding layer
    基于原型向量计算标签概率分布
    Computes label probability distribution based on prototype vectors
    """
    
    def __init__(self, prototypes: dict):
        """
        初始化接地层
        Initialize grounding layer
        
        Args:
            prototypes: 原型向量配置，格式为 {"intent": [...], "polarity": [...], "domain": [...]}
        """
        self.prototypes = prototypes
    
    def classify(self, vector: np.ndarray, category: str) -> Dict[str, float]:
        """
        对指定类别进行分类
        Classify for specified category
        
        Args:
            vector: 输入向量 / Input vector
            category: 类别名称 ("intent", "polarity", "domain") / Category name
            
        Returns:
            标签到概率的映射 / Mapping from label to probability
        """
        if category not in self.prototypes:
            return {"Unknown": 1.0}
        
        proto_list = self.prototypes[category]
        if not proto_list:
            return {"Unknown": 1.0}
        
        # 计算与所有原型的余弦相似度 / Compute cosine similarity with all prototypes
        similarities = []
        for proto in proto_list:
            proto_vector = np.array(proto.get("vector", []))
            if len(proto_vector) == 0:
                similarities.append(0.0)
            else:
                sim = self._cosine_similarity(vector, proto_vector)
                similarities.append(sim)
        
        # Softmax归一化 / Softmax normalization
        probs = self._softmax(np.array(similarities))
        
        # 构建结果字典 / Build result dictionary
        result = {}
        for proto, prob in zip(proto_list, probs):
            label = proto.get("label", "Unknown")
            result[label] = float(prob)
        
        return result
    
    @staticmethod
    def _cosine_similarity(a: np.ndarray, b: np.ndarray) -> float:
        """计算余弦相似度 / Compute cosine similarity"""
        # 对齐维度 / Align dimensions
        min_len = min(len(a), len(b))
        a_aligned = a[:min_len]
        b_aligned = b[:min_len]
        
        dot = np.dot(a_aligned, b_aligned)
        norm_a = np.linalg.norm(a_aligned)
        norm_b = np.linalg.norm(b_aligned)
        
        if norm_a > 0 and norm_b > 0:
            return dot / (norm_a * norm_b)
        return 0.0
    
    @staticmethod
    def _softmax(x: np.ndarray) -> np.ndarray:
        """Softmax函数 / Softmax function"""
        x_max = np.max(x)
        e_x = np.exp(x - x_max)
        return e_x / np.sum(e_x)