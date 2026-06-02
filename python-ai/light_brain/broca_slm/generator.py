# 布罗卡区神经生成层模块
# Broca neural generation layer module
# 加载轻量级SLM，在语义骨架约束下生成候选文本
# Loads lightweight SLM, generates candidate texts under semantic skeleton constraints

import os
import numpy as np
from typing import List

class SLMGenerator:
    """
    小型语言模型生成器
    Small Language Model generator
    负责加载SLM并生成自然语言候选
    Responsible for loading SLM and generating natural language candidates
    
    推荐SLM模型 / Recommended SLM models:
    - Qwen2.5-0.5B-Instruct (最轻量 / most lightweight)
    - Qwen2.5-1.5B-Instruct (平衡性能 / balanced)
    - SmolLM2-135M (极致轻量 / ultra lightweight)
    - TinyLlama-1.1B (开源可控 / open source)
    
    安装方法 / Installation:
    1. 安装依赖: pip install transformers torch
    2. 下载模型: 使用 HuggingFace 下载到 data/models/broca_slm/
       例如: huggingface-cli download Qwen/Qwen2.5-0.5B-Instruct --local-dir data/models/broca_slm/
    3. 或在代码中首次运行时自动下载 (需要网络)
    """
    
    def __init__(self, model_path: str = None):
        """
        初始化SLM生成器
        Initialize SLM generator
        
        Args:
            model_path: SLM模型路径，若为None则使用简化实现 / SLM model path, if None use simplified implementation
        """
        self.model_path = model_path
        self.model = None
        self.tokenizer = None
        self.device = "cpu"
        
        if model_path and os.path.exists(model_path):
            self._load_model(model_path)
    
    def _load_model(self, model_path: str):
        """
        加载SLM模型
        Load SLM model
        
        支持自动检测并使用GPU / Supports auto-detection and GPU usage
        """
        try:
            import torch
            from transformers import AutoModelForCausalLM, AutoTokenizer
            
            # 检测设备 / Detect device
            self.device = "cuda" if torch.cuda.is_available() else "cpu"
            
            self.tokenizer = AutoTokenizer.from_pretrained(model_path, trust_remote_code=True)
            self.model = AutoModelForCausalLM.from_pretrained(
                model_path,
                torch_dtype=torch.float16 if self.device == "cuda" else torch.float32,
                trust_remote_code=True
            )
            self.model.to(self.device)
            self.model.eval()
            
        except ImportError:
            print("警告: transformers/torch 未安装，使用简化生成器 / Warning: transformers/torch not installed, using simplified generator")
            self.model = None
            self.tokenizer = None
        except Exception as e:
            print(f"加载SLM失败 / Failed to load SLM: {e}")
            self.model = None
            self.tokenizer = None
    
    def generate(self, skeleton: dict, style: str, num_candidates: int = 3) -> List[str]:
        """
        生成候选文本
        Generate candidate texts
        
        Args:
            skeleton: 语义骨架 / Semantic skeleton
            style: 风格修饰符 / Style modifier
            num_candidates: 候选数量 / Number of candidates
            
        Returns:
            候选文本列表 / List of candidate texts
        """
        if self.model is not None and self.tokenizer is not None:
            return self._generate_with_model(skeleton, style, num_candidates)
        else:
            return self._generate_simple(skeleton, style, num_candidates)
    
    def _generate_with_model(self, skeleton: dict, style: str, num_candidates: int) -> List[str]:
        """使用真实SLM生成 / Generate with real SLM"""
        import torch
        
        prompt = self._skeleton_to_prompt(skeleton, style)
        
        inputs = self.tokenizer(prompt, return_tensors="pt", truncation=True, max_length=256)
        inputs = {k: v.to(self.device) for k, v in inputs.items()}
        
        with torch.no_grad():
            outputs = self.model.generate(
                **inputs,
                max_new_tokens=128,
                do_sample=True,
                temperature=0.8,
                top_p=0.9,
                num_return_sequences=num_candidates,
                pad_token_id=self.tokenizer.eos_token_id
            )
        
        candidates = []
        for output in outputs:
            text = self.tokenizer.decode(output, skip_special_tokens=True)
            # 移除输入部分 / Remove input part
            if text.startswith(prompt):
                text = text[len(prompt):].strip()
            candidates.append(text)
        
        return candidates
    
    def _generate_simple(self, skeleton: dict, style: str, num_candidates: int) -> List[str]:
        """
        简化生成实现（不依赖深度学习框架）
        Simplified generation (no deep learning framework required)
        基于模板的生成，用于测试和快速原型
        Template-based generation, for testing and rapid prototyping
        """
        slots = skeleton.get("slots", {})
        intent = skeleton.get("intent", "Unknown")
        
        candidates = []
        
        if slots.get("subject") and slots.get("description"):
            # 有事实内容 / Has fact content
            base = f"{slots['subject']}的{slots['attribute']}是{slots['description']}"
            candidates = [base]
            if style == "Enthusiastic":
                candidates.append(f"这个问题很好！{base}")
            elif style == "Cautious":
                candidates.append(f"据我所知，{base}")
            elif style == "Defensive":
                candidates.append(f"{base}，这一点是确定的。")
        else:
            # 无事实内容 / No fact content
            if intent == "AskFact":
                candidates = ["我不确定这个问题的答案。", "我需要更多信息来回答这个问题。"]
            elif intent == "Chat":
                candidates = ["我们可以聊聊别的。", "你有什么想聊的吗？"]
            elif intent == "TestBoundary":
                candidates = ["我不认为这个问题合适。", "我们换个话题吧。"]
            else:
                candidates = ["我不太明白你的意思。"]
        
        # 扩展到请求数量 / Expand to requested number
        while len(candidates) < num_candidates:
            candidates.append(candidates[0])
        
        return candidates[:num_candidates]
    
    def _skeleton_to_prompt(self, skeleton: dict, style: str) -> str:
        """将语义骨架转化为SLM提示词 / Convert semantic skeleton to SLM prompt"""
        slots = skeleton.get("slots", {})
        intent = skeleton.get("intent", "Unknown")
        
        if slots.get("subject") and slots.get("description"):
            prompt = f"请用{style}的语气回答：{slots['subject']}的{slots['attribute']}是什么？答案是{slots['description']}"
        else:
            prompt = f"请用{style}的语气回应：用户说了一句话，我需要给出一个得体的回复。"
        
        return prompt