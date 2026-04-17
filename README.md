光脑方案 · Light-Brain Scheme

光脑方案是一个生长式神经符号融合通用智能架构。

English Version: ./en/README.md

---

项目结构

本仓库包含两个主要部分：

· 光脑方案MVP/ — 最小可行产品，可立即运行的 Python 原型，展示核心架构流程。
· light-brain-full/ — 当前目录，光脑方案完整版代码（Rust 核心 + Python AI 层）。

MVP 文档已内嵌于 光脑方案MVP/ 目录中，包含中英文说明、术语对照、实施路线图等。

---

快速开始

运行 MVP

cd 光脑方案MVP
python main.py

构建完整版（开发中）

完整版采用 Rust + Python 混合架构，构建步骤详见 完整架构说明.md。

---

文档导航

· 完整架构说明：./完整架构说明.md
· 实施路线图：./实施路线图.md
· 贡献指南：./贡献指南.md
· 中英术语对照：./GLOSSARY_ZH_EN.md
· MVP 解释说明：./光脑方案MVP/MVP的解释.md

英文文档位于 ./en/ 目录下。

---

技术栈

· 核心引擎：Rust（符号推理、知识存储、生长调度）
· AI 接口层：Python + PyTorch + Hugging Face Transformers
· 绑定方案：PyO3 + Maturin
· 知识存储：OverGraph（图数据库 + 向量检索）
· 情景记忆：SQLite

---

许可证

本项目采用 MIT License，详见 ./LICENSE 文件。

