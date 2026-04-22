# 光脑方案 / Light-Brain Scheme

> 生长式神经符号融合通用智能架构  
> A Growth-Oriented Neuro-Symbolic Fusion General Intelligence Architecture

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Python](https://img.shields.io/badge/Python-3.10%2B-blue.svg)](https://www.python.org/)

---

## 简介 / Introduction

**光脑方案**是一个从认知架构底层重新设计的通用智能系统。它模拟人脑六大功能区（丘脑、前额叶、杏仁核、小脑、海马体、布罗卡区）协同工作，并通过生长机制、三层记忆、否决系统和生命之河自我模型，实现“智能在交互中生长”的核心理念。

**The Light-Brain Scheme** is a general intelligence system redesigned from the ground up at the cognitive architecture level. It simulates six functional brain regions (Thalamus, Prefrontal Cortex, Amygdala, Cerebellum, Hippocampus, Broca's Area) working in concert. Through growth mechanisms, three-tier memory, a veto system, and the River of Life self-model, it realizes the core philosophy that "intelligence grows through interaction."

### 核心特性 / Key Features

- **生长而非训练 / Growth over Training**：系统从最小核心启动，容量随经验物理性扩展。  
- **神经符号深度融合 / Deep Neuro-Symbolic Fusion**：神经层负责感知与生成，符号层保障逻辑与知识精确。  
- **三层记忆系统 / Three-Tier Memory**：模拟人类记忆的活跃、遗忘与偶然唤醒，知识自然流转。  
- **否决免疫机制 / Veto Immune System**：三级否决守护安全与知识质量，宁可不答也不乱答。  
- **生命之河自我模型 / River of Life Self-Model**：自我从系统内在张力中自然涌现，包含痛、痒、饿、梦、涅槃、道、爱等十七要素。

---

## 快速开始 / Quick Start

### 环境要求 / Requirements

- Rust 1.70+
- Python 3.10+
- （可选 / Optional）PyTorch 2.0+（用于真实模型 / for real models）

### 一键部署 / One-Click Deployment

**Linux / macOS**

    chmod +x scripts/deploy_single.sh
    ./scripts/deploy_single.sh

**Windows**

    powershell -ExecutionPolicy Bypass -File scripts/deploy_single.ps1

部署脚本将自动完成 Rust 核心编译、Python 绑定构建、虚拟环境创建及种子知识库初始化。

### 启动交互 / Start Interaction

    source venv/bin/activate      # Windows: venv\Scripts\activate
    python scripts/run.py

输入 `quit` 或 `exit` 退出对话。

---

## 架构概览 / Architecture Overview

```

用户输入 → 丘脑（感知接地）→ 前额叶（调度融合）→ 布罗卡区（语言生成）→ 回复
↑                    ↑
上下文注入          杏仁核（风格/否决）
小脑（语义记忆）
海马体（情景记忆）

```

六模块通过标准化接口协作。生长调度器在后台持续处理 Hebbian 突触新生、神经元新生（斐波那契递推）和知识图谱扩展。否决机制作为免疫系统，确保安全与知识质量。生命之河十七要素嵌入各模块，为自我涌现提供土壤。

---

## 项目结构 / Project Structure

```

light-brain/
├── rust-core/          # Rust 核心引擎（10 个模块）
├── python-binding/     # PyO3 绑定层
├── python-ai/          # Python AI 接口层（编码器、SLM）
├── config/             # JSON 配置文件
├── data/               # 运行时数据（种子知识库、数据库）
├── scripts/            # 部署与交互脚本
├── docs/               # 中英双语文档
└── tests/              # 单元与集成测试

```

详细说明请参阅 [目录导引](docs/zh/目录导引.md) / [Directory Guide](docs/en/directory-guide.md)。

---

## 文档导航 / Documentation

| 文档 | 中文 | English |
|:---|:---|:---|
| 完整架构说明 | [完整架构说明.md](docs/zh/完整架构说明.md) | [complete-architecture.md](docs/en/complete-architecture.md) |
| 实施路线图 | [实施路线图.md](docs/zh/实施路线图.md) | [roadmap.md](docs/en/roadmap.md) |
| 贡献指南 | [贡献指南.md](docs/zh/贡献指南.md) | [contributing.md](docs/en/contributing.md) |
| 种子库说明 | [种子库说明.md](docs/zh/种子库说明.md) | [seed-knowledge.md](docs/en/seed-knowledge.md) |
| 模型下载指引 | [模型下载指引.md](docs/zh/模型下载指引.md) | [model-download-guide.md](docs/en/model-download-guide.md) |
| 中英术语对照 | [中英对照表.md](docs/zh/中英对照表.md) | [glossary.md](docs/en/glossary.md) |

---

## 版本规划 / Version Roadmap

| 版本 | 代号 | 目标 |
|:---|:---|:---|
| **v1.0** | 骨骼与血肉 / Skeleton and Flesh | 完成完整架构，单机可实验 |
| **v2.0** | 呼吸 / Breath | 参数优化，边缘设备轻量化部署 |
| **v3.0** | 大海 / Ocean | 公库接入、分布式集群、多模态、社会性交互 |

当前版本为 **v1.0**，已实现全部核心功能。

---

## 贡献 / Contributing

我们欢迎任何形式的贡献！请阅读 [贡献指南](docs/zh/贡献指南.md) 了解如何提交问题、改进代码或完善文档。

We welcome all forms of contributions! Please read the [Contributing Guide](docs/en/contributing.md) to learn how to submit issues, improve code, or enhance documentation.

---

## 许可证 / License

本项目采用 [MIT 许可证](LICENSE)。  
This project is licensed under the [MIT License](LICENSE).

---

## 联系 / Contact

- **GitHub Issues**：[提交问题 / Submit Issues](https://github.com/starking-zl/light-brain/issues)
- **仓库地址 / Repository**：[https://github.com/starking-zl/light-brain](https://github.com/starking-zl/light-brain)

---

*光脑的河流，正朝着大海的方向流淌。*  
*The river of Light-Brain flows toward the ocean.*