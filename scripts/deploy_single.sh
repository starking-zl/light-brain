#!/bin/bash
# 光脑方案 v1.0 单机部署脚本
# Light-Brain Scheme v1.0 Single-Machine Deployment Script

set -e

echo "=========================================="
echo "光脑方案 v1.0 单机部署"
echo "Light-Brain Scheme v1.0 Deployment"
echo "=========================================="

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 检查 Rust
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}错误: Rust 未安装，请先安装 Rust${NC}"
    echo "访问 https://rustup.rs/ 安装"
    exit 1
fi
echo -e "${GREEN}✓ Rust 已安装${NC}"

# 检查 Python
if ! command -v python3 &> /dev/null; then
    echo -e "${RED}错误: Python3 未安装${NC}"
    exit 1
fi
echo -e "${GREEN}✓ Python3 已安装${NC}"

# 创建虚拟环境
echo "创建 Python 虚拟环境..."
if [ ! -d "venv" ]; then
    python3 -m venv venv
fi
source venv/bin/activate
echo -e "${GREEN}✓ 虚拟环境已激活${NC}"

# 安装 Python 依赖
echo "安装 Python 依赖..."
pip install --upgrade pip
pip install maturin numpy
echo -e "${GREEN}✓ Python 依赖安装完成${NC}"

# 编译 Rust 核心
echo "编译 Rust 核心引擎..."
cd rust-core
cargo build --release
cd ..
echo -e "${GREEN}✓ Rust 核心编译完成${NC}"

# 构建 Python 绑定
echo "构建 Python 绑定..."
cd python-binding
maturin develop --release
cd ..
echo -e "${GREEN}✓ Python 绑定构建完成${NC}"

# 初始化数据目录
echo "初始化数据目录..."
mkdir -p data/hippocampus
mkdir -p data/models/thalamus_encoder
mkdir -p data/models/broca_slm
mkdir -p data/nirvana
mkdir -p data/dreams
echo -e "${GREEN}✓ 数据目录创建完成${NC}"

# 初始化知识库
echo "初始化种子知识库..."
python3 scripts/init_knowledge.py
echo -e "${GREEN}✓ 种子知识库初始化完成${NC}"

echo ""
echo -e "${GREEN}=========================================="
echo "部署成功！"
echo "==========================================${NC}"
echo ""
echo "启动交互式对话："
echo "  source venv/bin/activate"
echo "  python3 scripts/run.py"
echo ""