# Seed Knowledge Base Description

> **Version**: v1.0  
> **Last Updated**: April 2026

## 1. Positioning of the Seed Knowledge Base

The core proposition of the Light-Brain Scheme is "growth over training." The seed knowledge base is not a preloaded "common sense repository" but rather Light-Brain's **minimal self-manual**. It contains only meta-knowledge about Light-Brain itself—statements concerning "who I am," "what my principles are," and "how I work." Any general common sense, logical rules, or mathematical axioms that can be obtained from public knowledge bases or user instruction are intentionally omitted.

This design ensures that:
- Light-Brain starts in a state of "emptiness," leaving pure soil for interactive growth.
- Users perceive Light-Brain's learning nature from the very first conversation, rather than facing an "omniscient" system.
- The decay mechanism of the three-tier memory system is not crowded out by large amounts of preloaded knowledge.

## 2. Contents of the Seed Knowledge Base

The seed knowledge base is located at `data/knowledge/seed.json` and adopts a structure that separates logical entries from language variants. Each logical knowledge entry contains both a Chinese (`zh`) and an English (`en`) version. The initialization script expands these into individual `KnowledgeEntry` records and writes them into the Cerebellum.

### Current Seed Entries

| Logical ID | Chinese Content | English Content | Certainty | Core Flag |
|:---|:---|:---|:---|:---|
| `light_brain_definition` | Definition of Light-Brain | Light-Brain definition | 1.0 | core |
| `light_brain_principle` | Core principle: Peace and Love | Core principle: Peace and Love | 1.0 | core |
| `light_brain_version` | Current version: v1.0 | Current version: v1.0 | 1.0 | non-core |
| `light_brain_capability` | Current capability description | Current capability description | 1.0 | non-core |
| `light_brain_memory` | Memory feature (learns and forgets) | Memory feature (learns and forgets) | 1.0 | non-core |

### Field Descriptions

| Field | Description |
|:---|:---|
| `logical_id` | Unique identifier for the logical knowledge, used to associate multilingual versions |
| `zh` / `en` | Language variants, containing `subject`, `attribute`, `value`, and `description` |
| `certainty` | Certainty score (0.0~1.0); seed knowledge is uniformly 1.0 |
| `core` | Whether this is core knowledge. `true` means never decays, never downgrades, and is protected by the veto mechanism |
| `tags` | List of tags; `lang:zh` or `lang:en` is automatically appended during initialization |

## 3. Special Protection for Core Knowledge

Knowledge marked with `core: true` enjoys the following privileges:
- **Never decays**: The decay formula does not apply; weight remains 1.0.
- **Never downgrades**: Will not be moved to Dormant or Garbage tiers.
- **Veto protection**: Any new knowledge conflicting with it will be blocked by Knowledge Error Veto.

The "definition" and "core principle" of Light-Brain are marked `core: true` as anchors of the system's nature. Version, capability, and memory features, although having certainty 1.0, may change in the future and are therefore not marked as core, allowing updates during version upgrades.

## 4. Initialization Script

Run `scripts/init_knowledge.py` to import the seed knowledge base into the Cerebellum. The script logic is as follows:

1. Read `data/knowledge/seed.json`.
2. Iterate through each logical knowledge entry:
   - If a `zh` variant exists, construct a Chinese `KnowledgeEntry`, add the tag `lang:zh`, and invoke Cerebellum write.
   - If an `en` variant exists, construct an English `KnowledgeEntry`, add the tag `lang:en`, and invoke Cerebellum write.
3. Output import statistics.

### Usage
```

cd light-brain
python scripts/init_knowledge.py

```


The single-machine deployment script `deploy_single.sh` automatically executes this step.

## 5. Extending the Seed Knowledge Base

When adding new seed knowledge, please strictly adhere to the following principles:

**Permitted additions**:
- Meta-knowledge about Light-Brain itself (e.g., new capability descriptions, interaction guidance).
- Value statements consistent with the "Peace and Love" principle.

**Prohibited additions**:
- Any general common sense (e.g., Earth's radius, historical events).
- Any logical rules or mathematical axioms.
- Any knowledge obtainable from user instruction or public knowledge bases.

### Steps to Add

1. Add a new logical entry to `seed.json` with a unique `logical_id`.
2. Provide `zh` and `en` variants (at least one is required).
3. Determine the `core` flag based on content: set to `true` only if the knowledge is an immutable part of the system's nature.
4. Run `python scripts/init_knowledge.py` to re-import.

## 6. Relationship with Public Knowledge Base Integration

The minimalist design of the seed knowledge base reserves a clear boundary for public knowledge base integration in v3.0:
- Seed is responsible solely for "self-cognition."
- Public knowledge bases are responsible for "world knowledge."
- User instruction is responsible for "personalized knowledge."

The three have distinct responsibilities without overlap. Light-Brain's growth begins precisely from this minimal "self" and gradually enriches through interaction with the world and the user.

---

*For a complete description of the Light-Brain architecture, please refer to `complete-architecture.md`.*