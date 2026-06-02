"""
设备检测工具
Device Detection Utility
"""

import torch


def get_device() -> str:
    """获取当前可用的计算设备"""
    if torch.cuda.is_available():
        return "cuda"
    elif hasattr(torch.backends, "mps") and torch.backends.mps.is_available():
        return "mps"
    else:
        return "cpu"


def get_device_info() -> dict:
    """获取设备详细信息"""
    device = get_device()
    info = {"device": device}
    if device == "cuda":
        info["cuda_version"] = torch.version.cuda
        info["device_count"] = torch.cuda.device_count()
        info["device_name"] = torch.cuda.get_device_name(0)
    return info