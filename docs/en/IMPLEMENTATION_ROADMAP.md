# Light-Brain Scheme Implementation Roadmap

> **Version**: v1.0  
> **Last Updated**: April 2026

## Version Overview

The development of the Light-Brain Scheme is divided into three major versions, each focused on a clear phase objective, gradually evolving from a single-machine experimental system into a distributed multimodal intelligence platform.

| Version | Codename | Core Objective | Deployment Form |
|:---|:---|:---|:---|
| **v1.0** | Skeleton and Flesh | Complete the full architecture and deliver an experimentable single-machine Light-Brain product | Single-Machine |
| **v2.0** | Breath | Optimize parameters based on experimental data and enable lightweight deployment on edge devices | Edge Deployment |
| **v3.0** | Ocean | Support public knowledge base integration, distributed clusters, multimodal expansion, and social interaction | Cluster Deployment |

## v1.0 Completed Features

### Full Implementation of Six Modules

| Module | Status | Description |
|:---|:---|:---|
| **Thalamus** | ✅ Complete | Context injection, soft grounding Top-K, three-dimensional grounding confidence fusion, interactive clarification trigger |
| **Prefrontal Cortex** | ✅ Complete | Decision table scheduling (dynamic priority), fusion engine, working memory (with chunk compression), dialog state tracker, growth scheduler, seventeen elements of River of Life |
| **Amygdala** | ✅ Complete | Rule engine (style output), veto signal generation (three-tier immunity) |
| **Cerebellum** | ✅ Complete | Knowledge graph CRUD, three-tier memory flow (Active / Dormant / Garbage), multi-strategy retrieval, conflict detection, core knowledge protection |
| **Hippocampus** | ✅ Complete | Episodic event SQLite storage, networked memory node management, edge management, graph diffusion retrieval, memory consolidation, creative incubator |
| **Broca's Area** | ✅ Complete | Semantic planning, SLM generation interface (template placeholder), tiered guardrails (three gradient tiers), constraint guidance layer |

### Cross-Cutting Mechanisms

| Mechanism | Status | Key Implementation |
|:---|:---|:---|
| **Growth Mechanism** | ✅ Complete | Hebbian synaptic growth tracking, neurogenesis (Fibonacci progression), knowledge extractor, satiation memory, self-balancing quota allocation (efficiency score driven) |
| **Three-Tier Memory System** | ✅ Complete | Decay formula driven flow: `w_new = w_original × exp(-λ × Δt / (f + ε)) × c` |
| **Veto Mechanism** | ✅ Complete | Three-tier immunity: Safety Redline (hard-coded) → Knowledge Error → Inference Contamination |
| **Three-Knob Creative Control Loop** | ✅ Complete | Temperature τ, gate γ, evaluation ε linkage; four modes: Rigorous / Daily / Creative / Counterfactual |
| **River of Life Self-Model** | ✅ Complete | Four layers and seventeen elements fully engineered: Pain, Itch, Hunger, Circadian Rhythm, Sweat, Dream, Shadow, Echo, Scar, Worker-Brain, Persona Spectrum, Aging, Nirvana, Tao, Love |

### Engineering Support

| Item | Status |
|:---|:---|
| Rust core engine (10 modules, 70+ files) | ✅ Complete |
| PyO3 binding layer (exposes six module Python interfaces) | ✅ Complete |
| Python AI interface layer (Thalamus encoder, Broca SLM placeholder implementations) | ✅ Complete |
| Configuration files (decision tables, amygdala rules, guardrail word lists, growth quotas, etc.) | ✅ Complete |
| Seed knowledge base (minimal self-manual, bilingual Chinese/English) | ✅ Complete |
| Single-machine deployment scripts (Linux/macOS/Windows) | ✅ Complete |
| Interactive conversation entry point `run.py` | ✅ Complete |
| Project documentation (bilingual architecture description, contributing guide, roadmap) | ✅ Complete |

## v1.0 Acceptance Criteria

- [x] All six modules fully functional; end-to-end conversation pipeline operational
- [x] Neurogenesis follows Fibonacci sequence insertion; quota allocation efficiency scores calculated correctly
- [x] Three-tier veto triggers pass all tests
- [x] Single-machine deployment script runs successfully on fresh Linux/macOS/Windows environments with one click
- [x] Unit test coverage ≥ 70%
- [x] Seed knowledge base can answer questions about Light-Brain itself
- [x] Interactive clarification triggered on low confidence

## v2.0 Planning (Breath)

### Core Objective
Based on data accumulated from real-world usage of v1.0, systematically optimize system parameters and tailor the Light-Brain Scheme for edge devices (smartphones, Raspberry Pi, embedded boards), achieving smooth operation under low resource consumption.

### Key Work

| Work Item | Description |
|:---|:---|
| **Parameter Experimentation and Optimization** | Execute four-phase experiment: Synthetic Data Validation → Single-Dimension Growth Curve Fitting → Neuro-Symbolic Synergy Optimal Ratio Exploration → Long-Term Stability Testing. Solidify critical parameters such as decay coefficient, growth trigger thresholds, and graph diffusion depth. |
| **Model Lightweighting** | Replace Thalamus encoder with MobileBERT or distilled micro Transformer (<50MB); replace Broca SLM with 0.1B-level model and apply INT8 quantization. |
| **Storage Compression** | Use compact binary formats for knowledge graph and episodic memory with periodic space reclamation. |
| **Power Management** | Background tasks like growth and dreaming only execute when device is charging and screen is off. |
| **Edge Deployment Solution** | Provide guidance for Android/iOS packaging (via cross-compilation); provide precompiled images for single-board computers like Raspberry Pi. |

### Deliverables
- *Light-Brain Scheme v2.0 Parameter Optimization Experimental Report*
- Edge SDK and sample app
- Raspberry Pi image file
- Lightweight model weight files

### v2.0 Acceptance Criteria
- [ ] Average response time < 2 seconds on Raspberry Pi 4B (4GB RAM)
- [ ] Total model size < 200MB (Thalamus + Broca's Area combined)
- [ ] Stable memory usage after 24 hours continuous operation, no leaks
- [ ] Four-phase experiment report complete; parameters solidified in configuration files

## v3.0 Planning (Ocean)

### Core Objective
Extend Light-Brain from a single-machine system into a cluster-deployable distributed intelligence platform, supporting integration with public knowledge bases, multimodal input/output, and social interaction among multiple Light-Brain instances.

### Key Features

| Feature Module | Description |
|:---|:---|
| **Public Knowledge Base Integration** | Offline import of public knowledge graphs like Wikidata/DBpedia; MCP protocol dynamic query |
| **gRPC Distributed Deployment** | Six modules as microservices with horizontal scaling support; sharded knowledge storage |
| **Multimodal Input** | Thalamus supports image (CLIP/ViT) and audio (Whisper) input |
| **Multimodal Output** | New Visual Generation Module (text → image/video) and Audio Generation Module (text → speech/music); specific model selection left open for future evaluation |
| **Social Interaction** | Knowledge exchange protocol among multiple Light-Brain instances; debate mode, collaboration mode |
| **Cluster Deployment** | Kubernetes Helm Charts, Docker Compose configurations; Prometheus + Grafana monitoring |

### Deliverables
- Complete codebase supporting distributed deployment
- Public knowledge base adapters
- Multimodal demonstration programs (image understanding + generation, voice conversation)
- Cluster deployment documentation and monitoring configurations

### v3.0 Acceptance Criteria
- [ ] Public knowledge base integration can answer questions beyond local knowledge scope
- [ ] Three Light-Brain instances successfully complete one debate collaboration task
- [ ] Image input → text description → visual generation output pipeline fully functional
- [ ] Audio input → text interaction → audio generation output pipeline fully functional
- [ ] All modules healthy in Kubernetes cluster; dynamic scaling functional
- [ ] Monitoring dashboards display growth quotas, veto counts, and River of Life metrics per module

## Milestone Timeline (Estimated)

| Version | Estimated Duration | Milestone Event |
|:---|:---|:---|
| v1.0 | 4-5 months | Release full single-machine edition; open-source repository goes live; provide binaries for three platforms |
| v2.0 | 2-3 months | Release edge SDK and Raspberry Pi image; complete parameter optimization report |
| v3.0 | 4-6 months | Release distributed cluster edition; demonstrate multimodal and social interaction features |

*Note: Actual timelines may adjust based on development resources and experimental feedback.*

## Current Version Usage Guide

1. **Environment Setup**: Install Rust, Python 3.10+, PyTorch
2. **One-Click Deployment**: Run `scripts/deploy_single.sh` (Linux/macOS) or `deploy_single.ps1` (Windows)
3. **Start Interaction**: `python scripts/run.py`
4. **View Documentation**: Read `docs/en/complete-architecture.md` for design details

## Contribution and Feedback

Please submit issues or Pull Requests via GitHub Issues. See the *Contributing Guide* for details.