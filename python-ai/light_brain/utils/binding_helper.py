# Rust绑定辅助函数
# Rust binding helper functions

import importlib
from typing import Optional

_RUST_BINDING = None

def get_rust_binding():
    """
    获取Rust绑定模块
    Get Rust binding module
    
    Returns:
        Rust绑定模块，若未找到则返回None / Rust binding module, or None if not found
    """
    global _RUST_BINDING
    if _RUST_BINDING is None:
        try:
            _RUST_BINDING = importlib.import_module("light_brain_binding")
        except ImportError:
            print("警告: Rust绑定模块 'light_brain_binding' 未找到，请先编译 / Warning: Rust binding module 'light_brain_binding' not found, please compile first")
            _RUST_BINDING = None
    return _RUST_BINDING