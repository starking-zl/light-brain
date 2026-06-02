# Light-Brain Scheme Directory Guide

> **Version**: v1.0  
> **Last Updated**: April 2026

This document helps you quickly understand the file structure of the Light-Brain Scheme project and the purpose of each directory.

## Project Root Directory

```

light-brain/
├── README.md                  # Project homepage (bilingual Chinese/English)
├── LICENSE                    # MIT License
├── .gitignore                 # Git ignore rules
├── Cargo.toml                 # Rust workspace configuration
│
├── docs/                      # Design documentation (bilingual Chinese/English)
├── rust-core/                 # Rust core engine
├── python-binding/            # PyO3 binding layer
├── python-ai/                 # Python AI interface layer
├── config/                    # Configuration files
├── data/                      # Runtime data
├── scripts/                   # Script tools
├── tests/                     # Test code
├── experiments/               # Experiment framework (four-phase directory reserved)
├── extensions/                # Extension modules
├── proto/                     # gRPC interface definitions (reserved)
└── deployments/               # Deployment configurations

```

## Core Code Directories

### `rust-core/` — Rust Core Engine

The symbolic reasoning, knowledge storage, growth scheduling, and memory management of the Light-Brain Scheme are all implemented in Rust. This directory is an independent Cargo project.

```

rust-core/
├── Cargo.toml                 # Core library dependency configuration
└── src/
├── lib.rs                 # Library entry point, module exports
├── utils/                 # Utility modules (time, decay, metrics, ID generation)
├── thalamus/              # Thalamus module (perception, grounding, clarification)
├── prefrontal/            # Prefrontal cortex module (scheduling, fusion, working memory, DST, growth scheduling, River of Life)
├── amygdala/              # Amygdala module (style inference, veto)
├── cerebellum/            # Cerebellum module (semantic memory, three-tier flow, conflict detection)
├── hippocampus/           # Hippocampus module (episodic memory, networked nodes, graph diffusion)
├── broca/                 # Broca's area module (language generation, guardrails)
├── growth/                # Growth mechanism (Hebbian, neurogenesis, knowledge extraction)
├── memory/                # Three-tier memory management
└── veto/                  # Veto mechanism

```

### `python-binding/` — PyO3 Binding Layer

Exposes the functionality of the Rust core engine as Python-callable modules.

```

python-binding/
├── Cargo.toml                 # Binding library dependency configuration
└── src/
├── lib.rs                 # PyO3 module entry point, defines Python classes
├── binding_helper.rs      # Binding helper functions
└── py_encoder.rs          # Python encoder callback wrapper

```

### `python-ai/` — Python AI Interface Layer

Contains Python implementations of the Thalamus neural network encoder and Broca's area language model.

```

python-ai/light_brain/
├── init.py                # Package entry point
├── thalamus_encoder/          # Thalamus encoder (Transformer, prototype matching)
│   ├── encoder.py
│   ├── grounding.py
│   └── calibration.py
├── broca_slm/                 # Broca's area SLM
│   ├── generator.py
│   ├── planner.py
│   ├── guardrails.py
│   └── constraint.py
└── utils/                     # Python utilities (configuration, logging, device)
├── config.py
├── logging.py
└── device.py

```

## Configuration and Data Directories

### `config/` — Configuration Files

| File | Purpose |
|:---|:---|
| `prefrontal_decisions.json` | Prefrontal decision table |
| `amygdala_rules.json` | Amygdala style rules |
| `broca_templates.json` | Broca's area fallback templates |
| `creative_profiles.json` | Three-knob mode preset parameters |
| `thalamus_prototypes.json` | Thalamus prototype vectors |
| `guardrail_tiers.json` | Tiered guardrail word list |
| `growth_config.json` | Growth quota configuration |
| `metacognition_config.json` | Metacognition configuration (thresholds, etc.) |
| `love_constraint.json` | Non-discardable list |

### `data/` — Runtime Data

| Path | Purpose |
|:---|:---|
| `knowledge/seed.json` | Seed knowledge base (minimal self-manual) |
| `hippocampus/` | Hippocampus SQLite database (generated at runtime) |
| `models/thalamus_encoder/` | Storage directory for Thalamus encoder model weights |
| `models/broca_slm/` | Storage directory for Broca's area SLM model weights |
| `nirvana/` | Nirvana archive (sublated traces, generated at runtime) |
| `dreams/` | Dream sediment layer (generated at runtime) |

## Scripts and Tools

### `scripts/` — Script Tools

| Script | Purpose |
|:---|:---|
| `run.py` | Main entry point, starts interactive conversation |
| `init_knowledge.py` | Initializes seed knowledge base, imports into Cerebellum |
| `deploy_single.sh` | Linux/macOS single-machine one-click deployment script |
| `deploy_single.ps1` | Windows single-machine one-click deployment script |
| `download_models.py` | Model download guidance script |

## Documentation Directory

### `docs/` — Design Documentation

| Path | Content |
|:---|:---|
| `zh/complete-architecture.md` | Complete description of Light-Brain architecture, six modules, cross-cutting mechanisms, River of Life (Chinese) |
| `zh/roadmap.md` | v1.0 / v2.0 / v3.0 version planning and progress (Chinese) |
| `zh/contributing.md` | Code contribution workflow, style guidelines, testing requirements (Chinese) |
| `zh/seed-knowledge.md` | Design principles and usage of the seed knowledge base (Chinese) |
| `zh/model-download-guide.md` | Guidance for replacing with real neural network models (Chinese) |
| `zh/glossary.md` | Chinese-English glossary of core terms (Chinese) |
| `zh/directory-guide.md` | This document (Chinese) |
| `en/` | English documentation, structurally identical to the Chinese version |

## Testing and Experiments

### `tests/` — Test Code

```

tests/
├── rust_tests/                # Rust unit tests and integration tests
└── python_tests/              # Python end-to-end tests

```

### `experiments/` — Experiment Framework

Reserved four-phase experiment directories for v2.0 parameter optimization:
- `phase1_component/`: Component-level validation
- `phase2_curve/`: Single-dimension growth curve fitting
- `phase3_synergy/`: Neuro-symbolic synergy validation
- `phase4_stability/`: Long-term stability testing

## Extensions and Deployment

### `extensions/` — Extension Modules

```

extensions/
└── public_kg/                 # Public knowledge base integration extension (to be completed in v3.0)

```

### `proto/` — gRPC Interface Definitions

Reserved directory for Protocol Buffer definition files, intended for v3.0 distributed deployment.

### `deployments/` — Deployment Configurations

```

deployments/
└── docker/                    # Docker-related configurations (Dockerfile, compose)

```

## Quick File Reference

| To learn about... | See file |
|:---|:---|
| Overall architecture | `docs/en/complete-architecture.md` |
| Six-module collaboration | `rust-core/src/prefrontal/decision_table.rs`, `fusion.rs` |
| Growth mechanism | `rust-core/src/growth/scheduler.rs`, `neurogenesis.rs` |
| Veto and guardrails | `rust-core/src/amygdala/veto.rs`, `python-ai/light_brain/broca_slm/guardrails.py` |
| River of Life self-model | `rust-core/src/prefrontal/` files: `circadian.rs`, `dream_task.rs`, `nirvana_task.rs`, `tao_miner.rs`, etc. |
| Python calling Rust | `python-binding/src/lib.rs` |
| Main entry point | `scripts/run.py` |

---

*The directory structure will be updated as the version evolves.*