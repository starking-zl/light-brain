# 布罗卡区约束引导层模块
# Broca constraint guiding layer module
# 筛选逻辑最忠实的候选，确保输出忠于符号决策包
# Filters the most logically faithful candidate, ensuring output fidelity to the symbolic decision package

from typing import List

class ConstraintGuider:
    """
    约束引导器
    Constraint guider
    从候选文本中选择最忠实于事实的选项
    Selects the most factually faithful option from candidate texts
    """
    
    def guide(self, candidates: List[str], decision_package: dict) -> str:
        """
        引导选择最佳候选
        Guide selection of the best candidate
        
        Args:
            candidates: 候选文本列表 / List of candidate texts
            decision_package: 符号决策包 / Symbolic decision package
            
        Returns:
            最佳候选文本 / Best candidate text
        """
        if not candidates:
            return None
        
        fact = decision_package.get("fact")
        if not fact:
            return candidates[0] if candidates else None
        
        # 提取必须包含的关键实体 / Extract key entities that must be included
        required_entities = [
            fact.get("subject", ""),
            fact.get("value", "")
        ]
        required_entities = [e for e in required_entities if e]
        
        # 评分：包含的必需实体越多越好 / Scoring: more required entities included is better
        best_candidate = None
        best_score = -1
        
        for candidate in candidates:
            score = sum(1 for e in required_entities if e in candidate)
            if score > best_score:
                best_score = score
                best_candidate = candidate
        
        # 如果没有任何候选包含必需实体，返回第一个 / If no candidate contains required entities, return the first
        if best_score == 0:
            return candidates[0]
        
        return best_candidate