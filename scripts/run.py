#!/usr/bin/env python3
"""
光脑方案交互主入口
Light-Brain Scheme Interactive Entry Point
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent / "python-ai"))

from light_brain.thalamus_encoder import ThalamusEncoder, PrototypeStore
from light_brain.broca_slm import BrocaSLM, SemanticPlanner

try:
    import light_brain_binding as lb
    RUST_ENABLED = True
except ImportError:
    print("警告: Rust 绑定未安装，使用纯 Python 降级模式")
    RUST_ENABLED = False


def main():
    print("光脑方案 v1.0 - 交互式对话")
    print("输入 'quit' 退出\n")

    # 初始化 Python 侧组件
    encoder = ThalamusEncoder()
    prototype_store = PrototypeStore()
    proto_path = Path(__file__).parent.parent / "config" / "thalamus_prototypes.json"
    if proto_path.exists():
        prototype_store.load_from_file(str(proto_path))

    slm = BrocaSLM()
    planner = SemanticPlanner()

    if RUST_ENABLED:
        # 将 Python 编码器传递给 Rust 绑定层
        thalamus = lb.PyThalamus(encoder)
        prefrontal = lb.PyPrefrontal()
        amygdala = lb.PyAmygdala()
        cerebellum = lb.PyCerebellum()
        hippocampus = lb.PyHippocampus("data/hippocampus/events.db")
        broca = lb.PyBroca()

    while True:
        try:
            user_input = input("You: ").strip()
            if user_input.lower() in ("quit", "exit", "q"):
                break
            if not user_input:
                continue

            if RUST_ENABLED:
                # 使用 Rust 核心引擎
                perception_json = thalamus.perceive(user_input, None)
                package_json = prefrontal.schedule(perception_json)
                response_json = broca.generate(package_json, "daily")
                response = json.loads(response_json).get("text", "")
            else:
                # 纯 Python 降级模式
                feat, recon = encoder.encode_with_context(user_input)
                response = "这是一个降级模式下的回复。请安装 Rust 绑定以获得完整体验。"

            print(f"光脑: {response}\n")

        except KeyboardInterrupt:
            break
        except Exception as e:
            print(f"错误: {e}")

    print("再见！")


if __name__ == "__main__":
    main()