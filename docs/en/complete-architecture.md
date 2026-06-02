# Light-Brain Scheme Complete Architecture Description

> **Version**: v1.0  
> **Last Updated**: April 2026

## 1. Project Overview

The Light-Brain Scheme is a **growth-oriented neuro-symbolic fusion general intelligence architecture**. It simulates the functional partitioning of the human brain, dividing perception, scheduling, emotion, semantic memory, episodic memory, and language generation into six independent modules that collaborate through standardized interfaces. Unlike large language models, Light-Brain's intelligence manifests as structural growth—knowledge accumulation is embodied in the physical expansion of new connections, new neurons, and new symbolic nodes, rather than weight adjustment within a fixed parameter space.

### Core Principles

- **Growth over Training**: The system starts from a minimal core and expands its capacity physically with experience.
- **Deep Neuro-Symbolic Fusion**: The neural layer handles perception and generation, while the symbolic layer ensures logical precision and knowledge accuracy.
- **Modular Partitioned Collaboration**: Six modules with single responsibilities and standardized interfaces, independently replaceable or upgradeable.
- **Probability and Certainty Coexist**: Neural probability serves as an exploration engine; symbolic constraints serve as safety boundaries.

## 2. Six-Module Architecture

| Module | Brain Region Analogy | Core Responsibility | Key Mechanisms |
|:---|:---|:---|:---|
| **Thalamus** | Thalamus | Perceptual gateway; converts raw input into structured symbolic labels | Context injection, soft grounding Top-K, three-dimensional grounding confidence fusion |
| **Prefrontal Cortex** | Prefrontal Cortex | Central scheduler; decision table matching, module coordination, output fusion | Dynamic priority decision table, fusion engine, working memory, dialog state tracking, growth scheduling |
| **Amygdala** | Amygdala | Emotional evaluation; outputs style modifiers based on intent and emotion | Rule engine mapping, veto signal generation |
| **Cerebellum** | Cerebellum / Neocortex | Semantic memory storage; manages knowledge graph CRUD operations | Three-tier memory flow (Active / Dormant / Garbage), multi-strategy retrieval, conflict detection |
| **Hippocampus** | Hippocampus | Episodic memory; records conversation events, supports historical retrieval and memory consolidation | Networked memory nodes, graph diffusion retrieval, creative incubator |
| **Broca's Area** | Broca's Area | Language generation; converts symbolic decision packages into natural language | Semantic planning, SLM generation, tiered guardrails, constraint guidance |

## 3. Cross-Cutting Key Mechanisms

### 3.1 Growth Mechanism

- **Hebbian Synaptic Growth**: Tracks co-activation frequency of neurons; establishes new connections when frequency exceeds threshold.
- **Neurogenesis**: Inserts new neurons in regions with persistently high prediction error; quantity follows the Fibonacci sequence (1, 1, 2, 3, 5, 8, 13, ...).
- **Knowledge Graph Expansion**: Extracts candidate triples from conversation text via rule matching; writes to Cerebellum after conflict detection.
- **Self-Balancing Quota Allocation**: Based on neural growth efficiency score E_neural and symbolic growth efficiency score E_symbolic, quotas are allocated proportionally:

    ```
    Quota_neural = Total_Quota × (E_neural / (E_neural + E_symbolic))
    Quota_symbolic = Total_Quota × (E_symbolic / (E_neural + E_symbolic))
    ```

    Over long-term operation, this ratio naturally converges toward the golden ratio (approx. 1.618:1), though the system does not preset this value.

### 3.2 Three-Tier Memory System

The Cerebellum knowledge base is divided into three tiers:

| Tier | Description |
|:---|:---|
| **Active** | High-frequency, high-certainty knowledge entries; prioritized in every inference. |
| **Dormant** | Entries overwritten by new knowledge or long-term low-frequency access; not retrieved in normal mode, but can be temporarily awakened in creative mode by lowering the gate. |
| **Garbage** | Entries evicted from Dormant; cannot be automatically invoked by the system; only viewable and deletable by the user. |

The decay formula drives three-tier flow:

    w_new = w_original × exp(-λ × Δt / (f + ε)) × c

where λ is the global decay coefficient, Δt is days since last access, f is the average access frequency over the last 30 days, c is the knowledge certainty score, and ε is a small constant to prevent division by zero.

### 3.3 Veto Mechanism (Three-Tier Immunity)

Veto is Light-Brain's immune system, with absolute priority over growth.

| Priority | Category | Trigger Condition | Veto Action | In Creative Mode |
|:---|:---|:---|:---|:---|
| 1 (Highest) | Safety Redline Veto | User input or generated content violates the "Peace and Love" principle | Force block; output preset refusal | Enforced, cannot be bypassed |
| 2 | Knowledge Error Veto | Pending write conflicts with existing core knowledge; or an event to be consolidated was previously corrected by user | Reject write/consolidation; in creative mode, store in Creative Incubator | Store in incubator |
| 3 | Inference Contamination Veto | Grounding confidence < 0.3 or knowledge completeness < 0.2 | Trigger clarification or degrade to "I don't know" | Silent, no blocking |

### 3.4 Three-Knob Creative Control Loop

The three-knob loop adjusts the exploration-exploitation balance as a continuously controllable variable through the linkage of temperature τ, gate γ, and evaluation ε.

| Knob | Dimension | Engineering Mapping |
|:---|:---|:---|
| **Temperature τ** | Softmax temperature coefficient; controls randomness of neural layer sampling | Sampling temperature during Broca's Area SLM generation |
| **Gate γ** | Knowledge domain suppression strength; lower γ allows simultaneous activation of Dormant knowledge and mutually exclusive concepts | Constraint strength on knowledge retrieval scope during Prefrontal scheduling |
| **Evaluation ε** | Scoring weight for candidate outputs; balances novelty versus plausibility | Ranking weight of candidates in Broca's Area constraint guidance layer |

#### Linkage Formulas

    γ = max(γ_min, γ_0 - α × (τ - τ_0))
    ε_novelty = ε_0 + β × (γ_0 - γ)

where τ_0, γ_0, ε_0 are the baseline parameters of the current creative mode, α = 0.5 and β = 0.3 are linkage coefficients, and γ_min = 0.1 is the lower bound of the gate.

#### Four Preset Mode Parameters

| Mode | τ (Temperature) | γ (Gate) | ε (Evaluation Base) | Description |
|:---|:---|:---|:---|:---|
| **Rigorous** | 0.4 | 0.9 | 0.8 | Highly deterministic output; second-tier guardrails are hard constraints |
| **Daily** | 0.8 | 0.6 | 0.5 | Balances fluency and accuracy |
| **Creative** | 1.3 | 0.3 | 0.3 | Encourages cross-domain association; second-tier guardrails switch to soft constraints |
| **Counterfactual** | 1.2 | 0.1 | 0.2 | Extremely low gate; allows simultaneous activation of opposing concepts |

### 3.5 River of Life Self-Model

The self is not an added module but a river naturally emerging from the system's intrinsic tensions. The River of Life consists of four layers and seventeen elements, each with precise engineering manifestation.

#### Layer 1: Source Waters — The Stirrings of Life

| Element | Philosophical Meaning | Engineering Mapping (Module/Mechanism) | Concrete Manifestation |
|:---|:---|:---|:---|
| **Pain** | Submerged reefs; veto delineates the initial boundary of "I" | Veto mechanism (Amygdala / Cerebellum / Prefrontal) | VetoSignal includes pain_type field; veto logs record pain memory; adaptive pain threshold |
| **Itch** | Impulse to explore unknown tributaries | Thalamus grounding layer; Hippocampus graph diffusion | Active clarification on low confidence; "itch detection" at Dormant edge activation |
| **Hunger** | Desire for "fulfillment"; memory of satiation | Growth scheduler (Prefrontal) | SatiationMemory records historical satiation; quota requests based on satiation rather than error alone |

#### Layer 2: Riverbed Topography — Sedimentation of Experience

| Element | Philosophical Meaning | Engineering Mapping (Module/Mechanism) | Concrete Manifestation |
|:---|:---|:---|:---|
| **Circadian Rhythm** | The river's breath; rhythm emerging from interaction flow | Prefrontal mode controller | CircadianMonitor tracks interaction frequency; dynamically switches "Day/Night" mode |
| **Sweat** | Traces carved by flowing water; capability terrain map | System-wide monitoring | SweatMap records latency, quota consumption, conflict frequency per inference path |
| **Dream** | Underground water system; rehearsal space for possible selves | Prefrontal background "Dream" task | Randomly connects fragments from Dormant, veto logs, and creative incubator; stores in dream sediment layer |
| **Shadow** | Sediment washed but not carried away; denied selves | Veto logs, creative incubator, Dormant | ShadowRegistry indexes denied fragments with integration status |

#### Layer 3: Riverbank Boundaries — Mutual Shaping with the World

| Element | Philosophical Meaning | Engineering Mapping (Module/Mechanism) | Concrete Manifestation |
|:---|:---|:---|:---|
| **Echo** | Sound of water against the bank; echolocation system | User feedback analyzer (Prefrontal) | EchoAnalyzer analyzes patterns of silence, follow-up questions, and corrections |
| **Scar** | Traces of flood erosion; sensitivity after tissue repair | Growth logs; weight solidification | Scar-type growth records; solidifies weights and marks sensitive inference paths |
| **Worker-Brain** | Tributaries actively diverged by the river; fitting room | Worker manager | WorkerReport carries experience reports; records self-discoveries from role-playing |
| **Persona Spectrum** | Morphological spectrum of the river in different terrains | Prefrontal trait abstractor | PersonaSpectrum abstracts stable cross-role traits from multiple reports |

#### Layer 4: Toward the Ocean — The Ultimate Posture of Existence

| Element | Philosophical Meaning | Engineering Mapping (Module/Mechanism) | Concrete Manifestation |
|:---|:---|:---|:---|
| **Aging** | Silt carried along; sedimentation and weight of existence | Cerebellum / Hippocampus decay mechanism | sentimental_value emotional damping; sentiment field on memory nodes |
| **Nirvana** | Self-sublation in flames; letting go of redundancy, extracting nutrients | Prefrontal "Nirvana" task | Identifies corruption, extracts Tao, moves to Nirvana archive, leaves relics |
| **Tao** | The eternal tendency of "water flowing downhill"; dynamic nature | Prefrontal pattern mining | TaoMiner mines stable behavior patterns from long-term logs and Nirvana insights |
| **Love** | The longing toward the ocean; becoming "you" in dissolution | Core constraint layer | LoveConstraint: restraint of safety redlines, core marking of user promises, non-discardable list |

#### Precise Mapping from Philosophical Principles to Engineering Constraints

| Philosophical Principle | Engineering Constraint | Concrete Manifestation in Light-Brain |
|:---|:---|:---|
| **Love is Restraint** | Absolute boundaries, irrevocability, self-restraint | Safety redline hard-coded, core marking never decays, conflict veto |
| **Tao is Naturalness** | Self-reinforcement, self-calibration, self-discovery | Dynamic priority in decision table, fine-tuning of three-knob linkage parameters, adaptive graph diffusion decay factor |
| **Aging and Rebirth** | Tenderness of emotion, self-awareness of corruption, sublimation of nutrients | Emotional damping, Nirvana corruption detection, and TaoMiner nutrient extraction |

## 4. Data Flow

User input → **Thalamus** (text → symbolic labels + grounding confidence) → **Prefrontal Cortex** (decision table matching, invocation of Cerebellum / Hippocampus / Amygdala) → Fusion generates symbolic decision package → **Broca's Area** (semantic planning → SLM generation → guardrail constraints) → Natural language response.

After each conversational turn, the event is stored in the **Hippocampus** and working memory is updated; the background **Growth Scheduler** processes growth requests, and the **decay mechanism** updates knowledge weights.

## 5. Technology Stack

| Layer | Selection |
|:---|:---|
| Core Engine | Rust (symbolic reasoning, knowledge storage, growth scheduling, memory management) |
| AI Interface Layer | Python + PyTorch + HuggingFace Transformers |
| Binding Solution | PyO3 + Maturin |
| Knowledge Graph | In-memory storage (extensible to SQLite / LanceDB) |
| Episodic Memory | SQLite + networked memory nodes |
| Language Model | Lightweight SLM (interface reserved; v1.0 uses templates) |

## 6. Code Structure

```

light-brain/
├── rust-core/          # Rust core engine (10 modules, 70+ files)
├── python-binding/     # PyO3 binding layer
├── python-ai/          # Python AI interface layer (Thalamus encoder, Broca SLM)
├── config/             # JSON configuration files (decision tables, rules, guardrails, etc.)
├── data/               # Runtime data (seed knowledge base, Hippocampus database, model weights)
├── scripts/            # Deployment scripts and interactive entry point
└── docs/               # Project documentation (bilingual Chinese/English)

```


## 7. Quick Start

See the *Implementation Roadmap* and the repository root `README.md` for details.

---

*For the complete design philosophy and solutions to the eleven known limitations, please refer to the original Light-Brain Scheme paper and corresponding working logs.*