#!/usr/bin/env python3
"""
种子知识库初始化脚本（支持中英文双语）
Seed Knowledge Base Initialization Script (Bilingual Chinese/English Support)

将 data/knowledge/seed.json 中的多语言知识条目展开并导入小脑。
Expands multilingual knowledge entries from data/knowledge/seed.json and imports them into Cerebellum.
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent / "python-ai"))

try:
    import light_brain_binding as lb
    RUST_ENABLED = True
except ImportError:
    RUST_ENABLED = False
    print("警告: Rust 绑定未安装，仅生成 JSON 文件")
    print("Warning: Rust binding not installed, only JSON file is generated")


def main():
    seed_path = Path(__file__).parent.parent / "data" / "knowledge" / "seed.json"
    
    if not seed_path.exists():
        print(f"错误: 种子文件 {seed_path} 不存在")
        print(f"Error: Seed file {seed_path} does not exist")
        return

    with open(seed_path, "r", encoding="utf-8") as f:
        logical_entries = json.load(f)

    if RUST_ENABLED:
        cerebellum = lb.PyCerebellum()
        imported_count = 0
        for logical in logical_entries:
            logical_id = logical.get("logical_id", "unknown")
            certainty = logical.get("certainty", 0.5)
            core = logical.get("core", False)
            common_tags = logical.get("tags", [])

            # 处理中文条目
            if "zh" in logical:
                zh = logical["zh"]
                entry_zh = {
                    "subject": zh["subject"],
                    "attribute": zh["attribute"],
                    "value": zh["value"],
                    "certainty": certainty,
                    "tags": common_tags + ["lang:zh"],
                    "description": zh.get("description", ""),
                    "source": f"seed:{logical_id}:zh",
                    "core": core
                }
                try:
                    entry_id = cerebellum.write(json.dumps(entry_zh))
                    print(f"✓ 写入中文知识: {zh['subject']} {zh['attribute']} ({entry_id})")
                    imported_count += 1
                except Exception as e:
                    print(f"✗ 中文写入失败: {zh['subject']} {zh['attribute']} - {e}")

            # 处理英文条目
            if "en" in logical:
                en = logical["en"]
                entry_en = {
                    "subject": en["subject"],
                    "attribute": en["attribute"],
                    "value": en["value"],
                    "certainty": certainty,
                    "tags": common_tags + ["lang:en"],
                    "description": en.get("description", ""),
                    "source": f"seed:{logical_id}:en",
                    "core": core
                }
                try:
                    entry_id = cerebellum.write(json.dumps(entry_en))
                    print(f"✓ Wrote English knowledge: {en['subject']} {en['attribute']} ({entry_id})")
                    imported_count += 1
                except Exception as e:
                    print(f"✗ English write failed: {en['subject']} {en['attribute']} - {e}")

        print(f"\n总计导入 {imported_count} 条知识条目")
        print(f"Total imported {imported_count} knowledge entries")
    else:
        print("Rust 绑定不可用，种子文件已就绪。")
        print("Rust binding unavailable, seed file is ready.")
        # 统计条目数
        count = sum(1 for e in logical_entries if "zh" in e) + sum(1 for e in logical_entries if "en" in e)
        print(f"种子知识逻辑条目数: {len(logical_entries)}，展开后语言条目数: {count}")
        print(f"Logical entries: {len(logical_entries)}, expanded language entries: {count}")


if __name__ == "__main__":
    main()