#!/usr/bin/env python3
"""
模型下载指引脚本
Model Download Guidance Script

提供丘脑编码器和布罗卡区 SLM 模型的下载说明。
Provides download instructions for Thalamus encoder and Broca's Area SLM models.
"""

import sys
from pathlib import Path

def main():
    print("=" * 50)
    print("光脑方案 v1.0 模型下载指引")
    print("Light-Brain Scheme v1.0 Model Download Guide")
    print("=" * 50)
    print()
    print("v1.0 默认使用伪向量编码器和模板生成器，无需下载真实模型。")
    print("如需启用真实神经网络，请参考以下指引：")
    print()
    print("【丘脑编码器】")
    print("  推荐模型: paraphrase-multilingual-MiniLM-L12-v2 (sentence-transformers)")
    print("  下载方式:")
    print("    pip install sentence-transformers")
    print("  然后在 python-ai/light_brain/thalamus_encoder/encoder.py 中")
    print("  将 model_name 改为 'real' 并加载模型。")
    print()
    print("【布罗卡区 SLM】")
    print("  推荐模型: Qwen/Qwen2.5-0.5B-Instruct")
    print("  下载方式:")
    print("    pip install transformers torch")
    print("  模型将自动从 HuggingFace 下载。")
    print()
    print("=" * 50)


if __name__ == "__main__":
    main()