# Model Download Guide

> **Version**: v1.0  
> **Last Updated**: April 2026

## 1. Default Behavior in v1.0

The core objective of Light-Brain Scheme v1.0 is to validate the feasibility and self-consistency of the complete architecture. To lower the deployment barrier and accelerate startup, v1.0 adopts the following simplified implementations by default:

| Module | Default Implementation | Description |
|:---|:---|:---|
| **Thalamus Encoder** | Deterministic pseudo-vectors (based on text hash) | No model download required; ready to use immediately. Output vectors are used for prototype matching and grounding calculations. |
| **Broca's Area SLM** | Template-based generation | Preset response templates are filled based on intent and facts; no model inference required. |

This means you **do not need to download any neural network models** to fully run Light-Brain v1.0, perform end-to-end conversations, and test the growth mechanism and veto mechanism.

## 2. Enabling Real Neural Networks

If you wish to experience the semantic understanding and generation capabilities provided by real neural networks, you can replace the default implementations following the guidelines below.

### 2.1 Thalamus Encoder

**Recommended Model**: `paraphrase-multilingual-MiniLM-L12-v2` (sentence-transformers)

- **Parameter Count**: approximately 118M
- **Vector Dimension**: 384 (requires corresponding update of vector dimensions in `config/thalamus_prototypes.json` and `ThalamusConfig.vector_dim`)
- **Language Support**: Chinese, English, and multilingual

**Replacement Steps**:

1. Install dependencies:
    ```
    pip install sentence-transformers
    ```

2. Modify `python-ai/light_brain/thalamus_encoder/encoder.py`:

    ```python
    from sentence_transformers import SentenceTransformer

    class ThalamusEncoder:
        def __init__(self, model_name: str = "paraphrase-multilingual-MiniLM-L12-v2", vector_dim: int = 384):
            self.model = SentenceTransformer(model_name)
            self.vector_dim = vector_dim
            # ...

        def encode(self, text: str) -> np.ndarray:
            return self.model.encode(text, normalize_embeddings=True)
    ```

3. Update `vector_dim` in `ThalamusConfig` to `384` (adjustable via configuration file or code).

**Other Optional Models**:

| Model | Parameters | Dimension | Notes |
|:---|:---|:---|:---|
| `all-MiniLM-L6-v2` | ~22M | 384 | Lightweight, primarily English |
| `bert-base-uncased` | ~110M | 768 | Classic English BERT |
| `bert-base-chinese` | ~110M | 768 | Chinese BERT |

### 2.2 Broca's Area SLM

**Recommended Model**: `Qwen/Qwen2.5-0.5B-Instruct`

- **Parameter Count**: approximately 0.5B
- **Features**: Bilingual Chinese/English, instruction-tuned, suitable for generation tasks
- **Minimum VRAM**: approximately 2GB (FP16)

**Replacement Steps**:

1. Install dependencies:
    ```
    pip install transformers torch
    ```

2. Modify `python-ai/light_brain/broca_slm/generator.py`:

    ```python
    from transformers import AutoModelForCausalLM, AutoTokenizer

    class BrocaSLM:
        def __init__(self, model_name: str = "Qwen/Qwen2.5-0.5B-Instruct"):
            self.tokenizer = AutoTokenizer.from_pretrained(model_name)
            self.model = AutoModelForCausalLM.from_pretrained(
                model_name,
                torch_dtype="auto",
                device_map="auto"
            )
            # ...

        def generate(self, prompt: str, max_length: int = 128, temperature: float = 0.8) -> str:
            inputs = self.tokenizer(prompt, return_tensors="pt").to(self.model.device)
            outputs = self.model.generate(
                **inputs,
                max_new_tokens=max_length,
                temperature=temperature,
                do_sample=True
            )
            return self.tokenizer.decode(outputs[0], skip_special_tokens=True)
    ```

**Other Optional Models**:

| Model | Parameters | Notes |
|:---|:---|:---|
| `microsoft/phi-2` | ~2.7B | Strong reasoning, primarily English |
| `TinyLlama/TinyLlama-1.1B-Chat-v1.0` | ~1.1B | Lightweight conversational model |
| `google/gemma-2b-it` | ~2B | Google's lightweight instruction model |

## 3. Model File Storage Location

Downloaded model weights are cached by default in the following directories (managed by HuggingFace Transformers):

- Linux/macOS: `~/.cache/huggingface/hub/`
- Windows: `%USERPROFILE%\.cache\huggingface\hub\`

To specify a custom storage path, you may set the `cache_dir` parameter when loading the model.

The Light-Brain data directory `data/models/` reserves the following subdirectories for storing self-downloaded or fine-tuned models:

- `data/models/thalamus_encoder/`
- `data/models/broca_slm/`

## 4. Lightweighting Recommendations for Edge Devices (v2.0 Preview)

v2.0 will provide formal edge deployment solutions. The following are preview recommendations:

| Strategy | Description |
|:---|:---|
| **Model Quantization** | Use INT8 or INT4 quantization to compress a 0.5B model to under 500MB |
| **Model Distillation** | Use a smaller distilled model (e.g., `Qwen2.5-0.1B`) |
| **Local Caching** | Use offline after the first download; no network required thereafter |

## 5. Auxiliary Script

The project provides the `scripts/download_models.py` script to check model download status and output guidance. Run it with:
```

python scripts/download_models.py

```


This script does not automatically download models; instead, it provides specific download and replacement instructions based on the current configuration.

## 6. Frequently Asked Questions

**Q: Why doesn't v1.0 integrate real models directly?**

A: The core value of the Light-Brain Scheme lies in the architecture itself—growth mechanism, three-tier memory, veto system, River of Life. Using simplified encoders and generators allows:
- Lower deployment barriers; runs without a GPU.
- Deterministic behavior for easier debugging and testing.
- Integration of real models as a clear evolutionary step, consistent with the "from skeleton to flesh" version philosophy.

**Q: After replacing with a real model, do I need to retrain the prototype vectors?**

A: Yes. Prototype vectors need to be aligned with the semantic space of the new encoder. You can accomplish this in one of two ways:
- Use the new encoder to encode a small set of labeled samples and reinitialize the prototype vectors.
- Keep the existing prototypes and gradually adjust them through interactive calibration (using the `calibrate` method).

**Q: Can I use a mixed setup—real model for Thalamus and templates for Broca's Area?**

A: Absolutely. Each module is independent; you can replace them incrementally as needed.

---

*For a complete description of the Light-Brain architecture, please refer to `complete-architecture.md`.*