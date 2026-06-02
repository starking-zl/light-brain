# 布罗卡区子模块 - 语言生成
# Broca submodule - Language generation
# 负责将符号决策包转化为自然语言输出
# Responsible for converting symbolic decision packages into natural language output

import json
import os
from .planner import SemanticPlanner
from .generator import SLMGenerator
from .constraints import ConstraintGuider
from .guardrails import GuardrailManager

class Broca:
    """
    布罗卡区 - 光脑方案的语言生成中枢
    Broca's Area - Language generation center of Light-Brain Scheme
    采用"规划-生成-约束"三层架构
    Adopts "planning-generation-constraint" three-tier architecture
    """
    
    def __init__(self, model_path: str, templates_path: str, guardrail_config: dict = None):
        """
        初始化布罗卡区
        Initialize Broca's Area
        
        Args:
            model_path: SLM模型路径 / SLM model path
            templates_path: 兜底模板文件路径 / Fallback template file path
            guardrail_config: 护栏配置 / Guardrail configuration
        """
        self.planner = SemanticPlanner()
        self.generator = SLMGenerator(model_path)
        self.constraints = ConstraintGuider()
        self.guardrails = GuardrailManager(guardrail_config or {})
        
        # 加载兜底模板 / Load fallback templates
        with open(templates_path, 'r', encoding='utf-8') as f:
            self.templates = json.load(f)
    
    def generate(self, decision_package: dict) -> str:
        """
        根据符号决策包生成自然语言回复
        Generate natural language reply based on symbolic decision package
        
        Args:
            decision_package: 符号决策包，包含意图、事实、风格等 / Symbolic decision package
            
        Returns:
            自然语言回复 / Natural language reply
        """
        # 1. 语义规划：构建抽象骨架 / Semantic planning: build abstract skeleton
        skeleton = self.planner.plan(decision_package)
        
        # 2. 神经生成：调用SLM生成候选文本 / Neural generation: call SLM to generate candidates
        style = decision_package.get("style", "normal")
        candidates = self.generator.generate(skeleton, style, num_candidates=3)
        
        # 3. 护栏检查：过滤违规候选 / Guardrail check: filter violating candidates
        candidates = self.guardrails.filter(candidates, decision_package)
        
        # 4. 约束引导：选择最忠实的候选 / Constraint guiding: select most faithful candidate
        best = self.constraints.guide(candidates, decision_package)
        
        # 5. 兜底：若生成失败，使用模板 / Fallback: if generation fails, use template
        if best is None:
            best = self._fallback_template(decision_package)
        
        return best
    
    def _fallback_template(self, decision_package: dict) -> str:
        """使用兜底模板生成回复 / Generate reply using fallback template"""
        intent = decision_package.get("intent", "Unknown")
        style = decision_package.get("style", "normal")
        fact = decision_package.get("fact")
        
        # 尝试匹配意图+风格 / Try to match intent+style
        key = f"{intent}_{style}"
        if key in self.templates:
            templates = self.templates[key].get("templates", [])
        elif f"{intent}_normal" in self.templates:
            templates = self.templates[f"{intent}_normal"].get("templates", [])
        else:
            templates = self.templates.get("fallback", {}).get("templates", ["我不确定该如何回答。"])
        
        import random
        template = random.choice(templates) if templates else "我不确定该如何回答。"
        
        if fact:
            return template.format(
                subject=fact.get("subject", ""),
                attribute=fact.get("attribute", ""),
                value=fact.get("value", ""),
                description=fact.get("description", "")
            )
        return template