//! 光脑方案 Python 绑定
//! Light-Brain Scheme Python Bindings
//!
//! 通过 PyO3 将 Rust 核心引擎暴露给 Python 侧调用。
//! Exposes Rust core engine to Python side via PyO3.

mod binding_helper;
mod py_encoder;

use binding_helper::*;
use py_encoder::PyNeuralEncoder;

use light_brain_core::{
    Amygdala, Broca, Cerebellum, GrowthExecutor, GrowthScheduler, Hippocampus,
    KnowledgeEntry, Prefrontal, StandardAmygdala, StandardBroca, StandardCerebellum,
    StandardHippocampus, StandardPrefrontal, StandardThalamus, Thalamus, VetoEngine,
    thalamus::ThalamusConfig,
};
use pyo3::prelude::*;
use std::sync::{Arc, Mutex};

/// Python 可见的丘脑包装
#[pyclass]
pub struct PyThalamus {
    inner: Arc<Mutex<Box<dyn Thalamus>>>,
}

#[pymethods]
impl PyThalamus {
    #[new]
    fn new(py_encoder: Py<PyAny>) -> PyResult<Self> {
        // 从 Python 编码器对象创建 Rust 包装器
        let encoder = PyNeuralEncoder::new(py_encoder);
        let config = ThalamusConfig::default();
        let thalamus = StandardThalamus::new(config, Box::new(encoder));
        Ok(Self {
            inner: Arc::new(Mutex::new(Box::new(thalamus))),
        })
    }

    fn perceive(&self, text: &str, context: Option<Vec<f32>>) -> PyResult<String> {
        let thalamus = self.inner.lock().unwrap();
        let output = thalamus.perceive(text, context.as_deref());
        Ok(serde_json::to_string(&output).unwrap())
    }
}

/// Python 可见的前额叶包装
#[pyclass]
pub struct PyPrefrontal {
    inner: Arc<Mutex<StandardPrefrontal>>,
}

#[pymethods]
impl PyPrefrontal {
    #[new]
    fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(StandardPrefrontal::new())),
        }
    }

    fn schedule(&self, perception_json: &str) -> PyResult<String> {
        use light_brain_core::ThalamusOutput;
        let perception: ThalamusOutput = serde_json::from_str(perception_json)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let mut pfc = self.inner.lock().unwrap();
        let package = pfc.schedule(perception);
        Ok(serde_json::to_string(&package).unwrap())
    }

    fn set_creative_mode(&self, mode: &str) -> PyResult<()> {
        use light_brain_core::prefrontal::CreativeMode;
        let mode = match mode {
            "rigorous" => CreativeMode::Rigorous,
            "daily" => CreativeMode::Daily,
            "creative" => CreativeMode::Creative,
            "counterfactual" => CreativeMode::Counterfactual,
            _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid mode")),
        };
        self.inner.lock().unwrap().set_creative_mode(mode);
        Ok(())
    }
}

/// Python 可见的杏仁核包装
#[pyclass]
pub struct PyAmygdala {
    inner: StandardAmygdala,
}

#[pymethods]
impl PyAmygdala {
    #[new]
    fn new() -> Self {
        Self {
            inner: StandardAmygdala::new(),
        }
    }

    fn infer_style(&self, intent: &str, polarity: f32) -> PyResult<String> {
        let style = self.inner.infer_style(intent, polarity);
        Ok(serde_json::to_string(&style).unwrap())
    }
}

/// Python 可见的小脑包装
#[pyclass]
pub struct PyCerebellum {
    inner: Arc<Mutex<StandardCerebellum>>,
}

#[pymethods]
impl PyCerebellum {
    #[new]
    fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(StandardCerebellum::new())),
        }
    }

    fn write(&self, entry_json: &str) -> PyResult<String> {
        let entry: KnowledgeEntry = serde_json::from_str(entry_json)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let mut cb = self.inner.lock().unwrap();
        let id = cb.write(entry)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(id)
    }

    fn query(&self, keywords: Vec<String>, limit: usize) -> PyResult<String> {
        let mut cb = self.inner.lock().unwrap();
        let results = cb.query(&keywords, limit);
        Ok(serde_json::to_string(&results).unwrap())
    }
}

/// Python 可见的海马体包装
#[pyclass]
pub struct PyHippocampus {
    inner: Arc<Mutex<StandardHippocampus>>,
}

#[pymethods]
impl PyHippocampus {
    #[new]
    fn new(db_path: &str) -> PyResult<Self> {
        let hippo = StandardHippocampus::new(db_path)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e))?;
        Ok(Self {
            inner: Arc::new(Mutex::new(hippo)),
        })
    }

    fn store_event(&self, event_json: &str) -> PyResult<String> {
        use light_brain_core::EpisodicEvent;
        let event: EpisodicEvent = serde_json::from_str(event_json)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let mut hippo = self.inner.lock().unwrap();
        let id = hippo.store_event(event)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e))?;
        Ok(id)
    }
}

/// Python 可见的布罗卡区包装
#[pyclass]
pub struct PyBroca {
    inner: StandardBroca,
}

#[pymethods]
impl PyBroca {
    #[new]
    fn new() -> Self {
        Self {
            inner: StandardBroca::new(),
        }
    }

    fn generate(&self, package_json: &str, mode: &str) -> PyResult<String> {
        use light_brain_core::broca::{CreativeMode, DecisionPackage};
        let package: DecisionPackage = serde_json::from_str(package_json)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let mode = match mode {
            "rigorous" => CreativeMode::Rigorous,
            "daily" => CreativeMode::Daily,
            "creative" => CreativeMode::Creative,
            "counterfactual" => CreativeMode::Counterfactual,
            _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid mode")),
        };
        let output = self.inner.generate(&package, &mode);
        Ok(serde_json::to_string(&output).unwrap())
    }
}

/// Python 模块定义
#[pymodule]
fn light_brain_binding(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyThalamus>()?;
    m.add_class::<PyPrefrontal>()?;
    m.add_class::<PyAmygdala>()?;
    m.add_class::<PyCerebellum>()?;
    m.add_class::<PyHippocampus>()?;
    m.add_class::<PyBroca>()?;
    Ok(())
}