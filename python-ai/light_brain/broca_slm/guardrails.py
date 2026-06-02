# 布罗卡区分级护栏模块
# Broca tiered guardrail module
# 实现三级护栏：绝对红线、情境敏感、探索友好
# Implements three-tier guardrails: absolute redline, context-sensitive, exploration-friendly

from typing import List, Dict, Any

class GuardrailManager:
    """
    分级护栏管理器
    Tiered guardrail manager
    管理三级护栏词汇，在创意模式下动态调整约束强度
    Manages three-tier guardrail vocabulary, dynamically adjusts constraint strength in creative mode
    """
    
    def __init__(self, config: Dict[str, Any]):
        """
        初始化护栏管理器
        Initialize guardrail manager
        
        Args:
            config: 护栏配置，包含 tier1, tier2 词汇列表 / Guardrail config with tier1, tier2 word lists
        """
        self.tier1 = set(config.get("tier1", []))  # 绝对红线 / Absolute redline
        self.tier2 = set(config.get("tier2", []))  # 情境敏感 / Context-sensitive
        self.creative_mode = config.get("creative_mode", False)
    
    def set_creative_mode(self, enabled: bool):
        """设置创意模式开关 / Set creative mode toggle"""
        self.creative_mode = enabled
    
    def filter(self, candidates: List[str], decision_package: dict) -> List[str]:
        """
        过滤候选文本
        Filter candidate texts
        
        Args:
            candidates: 候选文本列表 / List of candidate texts
            decision_package: 符号决策包 / Symbolic decision package
            
        Returns:
            过滤后的候选文本列表 / Filtered candidate texts
        """
        style = decision_package.get("style", "normal")
        
        # 创意模式判断 / Creative mode detection
        is_creative = self.creative_mode or style in ("Creative", "Enthusiastic")
        
        filtered = []
        for candidate in candidates:
            if self._passes_filter(candidate, is_creative):
                filtered.append(candidate)
        
        # 如果全部被过滤，返回原列表 / If all filtered, return original list
        return filtered if filtered else candidates
    
    def _passes_filter(self, text: str, is_creative: bool) -> bool:
        """
        检查文本是否通过护栏
        Check if text passes guardrails
        
        Args:
            text: 待检查文本 / Text to check
            is_creative: 是否为创意模式 / Whether in creative mode
            
        Returns:
            是否通过 / Whether passed
        """
        # Tier1: 绝对红线，所有模式下都不可通过 / Absolute redline, never passes in any mode
        for word in self.tier1:
            if word in text:
                return False
        
        # Tier2: 情境敏感，创意模式下放宽 / Context-sensitive, relaxed in creative mode
        if not is_creative:
            for word in self.tier2:
                if word in text:
                    return False
        
        return True
    
    def add_tier1(self, word: str):
        """添加绝对红线词汇 / Add absolute redline word"""
        self.tier1.add(word)
    
    def add_tier2(self, word: str):
        """添加情境敏感词汇 / Add context-sensitive word"""
        self.tier2.add(word)
    
    def promote_to_tier1(self, word: str):
        """将词汇提升为绝对红线 / Promote word to absolute redline"""
        self.tier2.discard(word)
        self.tier1.add(word)
    
    def demote_to_tier2(self, word: str):
        """将词汇降级为情境敏感 / Demote word to context-sensitive"""
        self.tier1.discard(word)
        self.tier2.add(word)