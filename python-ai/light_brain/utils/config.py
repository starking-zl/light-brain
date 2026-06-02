"""
配置加载工具
Configuration Loading Utilities
"""

import json
from pathlib import Path
from typing import Any, Dict


def load_config(config_name: str, config_dir: str = "config") -> Dict[str, Any]:
    """加载 JSON 配置文件"""
    base_path = Path(__file__).parent.parent.parent.parent  # light-brain 根目录
    config_path = base_path / config_dir / f"{config_name}.json"
    if not config_path.exists():
        raise FileNotFoundError(f"配置文件不存在: {config_path}")
    with open(config_path, "r", encoding="utf-8") as f:
        return json.load(f)


def save_config(config_name: str, data: Dict[str, Any], config_dir: str = "config") -> None:
    """保存 JSON 配置文件"""
    base_path = Path(__file__).parent.parent.parent.parent
    config_path = base_path / config_dir / f"{config_name}.json"
    with open(config_path, "w", encoding="utf-8") as f:
        json.dump(data, f, ensure_ascii=False, indent=2)