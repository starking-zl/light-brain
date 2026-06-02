# 光脑方案 v1.0 单机部署脚本 (Windows)
# Light-Brain Scheme v1.0 Single-Machine Deployment Script (Windows)

$ErrorActionPreference = "Stop"

Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "光脑方案 v1.0 单机部署" -ForegroundColor Cyan
Write-Host "Light-Brain Scheme v1.0 Deployment" -ForegroundColor Cyan
Write-Host "==========================================" -ForegroundColor Cyan

# 检查 Rust
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "错误: Rust 未安装，请先安装 Rust" -ForegroundColor Red
    Write-Host "访问 https://rustup.rs/ 安装"
    exit 1
}
Write-Host "✓ Rust 已安装" -ForegroundColor Green

# 检查 Python
if (-not (Get-Command python -ErrorAction SilentlyContinue)) {
    Write-Host "错误: Python 未安装" -ForegroundColor Red
    exit 1
}
Write-Host "✓ Python 已安装" -ForegroundColor Green

# 创建虚拟环境
Write-Host "创建 Python 虚拟环境..."
if (-not (Test-Path "venv")) {
    python -m venv venv
}
.\venv\Scripts\Activate.ps1
Write-Host "✓ 虚拟环境已激活" -ForegroundColor Green

# 安装 Python 依赖
Write-Host "安装 Python 依赖..."
pip install --upgrade pip
pip install maturin numpy
Write-Host "✓ Python 依赖安装完成" -ForegroundColor Green

# 编译 Rust 核心
Write-Host "编译 Rust 核心引擎..."
Set-Location rust-core
cargo build --release
Set-Location ..
Write-Host "✓ Rust 核心编译完成" -ForegroundColor Green

# 构建 Python 绑定
Write-Host "构建 Python 绑定..."
Set-Location python-binding
maturin develop --release
Set-Location ..
Write-Host "✓ Python 绑定构建完成" -ForegroundColor Green

# 初始化数据目录
Write-Host "初始化数据目录..."
New-Item -ItemType Directory -Force -Path data\hippocampus | Out-Null
New-Item -ItemType Directory -Force -Path data\models\thalamus_encoder | Out-Null
New-Item -ItemType Directory -Force -Path data\models\broca_slm | Out-Null
New-Item -ItemType Directory -Force -Path data\nirvana | Out-Null
New-Item -ItemType Directory -Force -Path data\dreams | Out-Null
Write-Host "✓ 数据目录创建完成" -ForegroundColor Green

# 初始化知识库
Write-Host "初始化种子知识库..."
python scripts\init_knowledge.py
Write-Host "✓ 种子知识库初始化完成" -ForegroundColor Green

Write-Host ""
Write-Host "==========================================" -ForegroundColor Green
Write-Host "部署成功！" -ForegroundColor Green
Write-Host "==========================================" -ForegroundColor Green
Write-Host ""
Write-Host "启动交互式对话："
Write-Host "  .\venv\Scripts\Activate.ps1"
Write-Host "  python scripts\run.py"