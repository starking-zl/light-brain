"""
丘脑神经编码器
Thalamus Neural Encoder

使用轻量 Transformer 将文本编码为向量。v1.0 采用简化确定性伪向量实现，
真实模型接口已预留，可替换为 BERT/SBERT 等。
Encodes text into vectors using a lightweight Transformer. v1.0 uses simplified
deterministic pseudo-vectors; real model interface is reserved for BERT/SBERT.
"""

import numpy as np
from typing import Optional, List, Tuple
import hashlib


class ThalamusEncoder:
    """丘脑神经编码器 / Thalamus Neural Encoder"""

    def __init__(self, model_name: str = "pseudo", vector_dim: int = 768):
        self.model_name = model_name
        self.vector_dim = vector_dim
        self._context_gate = 0.7

    def encode(self, text: str) -> np.ndarray:
        """
        将文本编码为向量
        Encode text into a vector
        """
        if self.model_name == "pseudo":
            return self._pseudo_encode(text)
        else:
            # 预留真实模型接口
            # Reserved for real model integration
            raise NotImplementedError("Real model not yet integrated")

    def encode_with_context(
        self, text: str, context_vector: Optional[List[float]] = None
    ) -> Tuple[List[float], List[float]]:
        """
        带上下文注入的编码
        Encode with context injection

        Args:
            text: 输入文本 / input text
            context_vector: 可选的上下文向量（Python列表） / optional context vector as Python list

        Returns:
            (feature_vector, reconstructed_vector) 两个列表
        """
        # 将 context_vector 转换为 numpy 数组（如果提供）
        ctx = None
        if context_vector is not None:
            ctx = np.array(context_vector, dtype=np.float32)

        feat = self.encode(text)
        if ctx is not None and len(ctx) == self.vector_dim:
            # 上下文注入：加权求和
            feat = self._context_gate * ctx + (1.0 - self._context_gate) * feat

        # 重构向量简化实现：与特征向量相同
        recon = feat.copy()

        # 返回 Python 列表，符合 Rust 侧期望
        return feat.tolist(), recon.tolist()

    def _pseudo_encode(self, text: str) -> np.ndarray:
        """
        伪向量编码：基于文本哈希生成确定性向量
        Pseudo-vector encoding: generate deterministic vector based on text hash
        """
        hash_bytes = hashlib.sha256(text.encode("utf-8")).digest()
        seed = int.from_bytes(hash_bytes[:8], "big") / (2**64)
        np.random.seed(int(seed * 2**32) % 2**32)
        vec = np.random.randn(self.vector_dim).astype(np.float32)
        norm = np.linalg.norm(vec)
        if norm > 0:
            vec = vec / norm
        return vec