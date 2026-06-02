//! Python 编码器回调包装
//! Python Encoder Callback Wrapper
//!
//! 实现 NeuralEncoder trait，将调用转发给 Python 对象的方法。
//! Implements the NeuralEncoder trait by forwarding calls to a Python object's methods.

use light_brain_core::thalamus::NeuralEncoder;
use pyo3::prelude::*;
use pyo3::types::{PyList, PyTuple};

/// Python 编码器包装器
/// Python encoder wrapper
pub struct PyNeuralEncoder {
    /// Python 编码器对象
    py_encoder: Py<PyAny>,
}

impl PyNeuralEncoder {
    /// 从 Python 对象创建包装器
    /// Create wrapper from a Python object
    pub fn new(py_encoder: Py<PyAny>) -> Self {
        Self { py_encoder }
    }
}

// 安全声明：Python 对象的访问受 GIL 保护，跨线程传递安全
// Safety declaration: Python object access is protected by GIL, safe for cross-thread passing
unsafe impl Send for PyNeuralEncoder {}
unsafe impl Sync for PyNeuralEncoder {}

impl NeuralEncoder for PyNeuralEncoder {
    fn encode_with_context(&self, text: &str, context_vector: Option<&[f32]>) -> (Vec<f32>, Vec<f32>) {
        Python::with_gil(|py| {
            // 准备参数：text 和可选的 context_vector（转换为 Python 列表）
            let ctx_list = match context_vector {
                Some(v) => {
                    let list = PyList::new_bound(py, v.iter().map(|&x| x));
                    list.into()
                }
                None => py.None(),
            };

            // 调用 Python 对象的 encode_with_context 方法
            let result = self.py_encoder
                .call_method1(py, "encode_with_context", (text, ctx_list))
                .map_err(|e| {
                    log::error!("Python encoder call failed: {}", e);
                    e
                })
                .unwrap_or_else(|_| {
                    // 出错时返回全零向量（维度由配置决定，但此处无法获取配置，采用常见 768 维）
                    let zero_list = PyList::new_bound(py, vec![0.0f32; 768]);
                    let tuple = PyTuple::new_bound(py, &[zero_list.clone(), zero_list]);
                    tuple.into()
                });

            // 解析返回的元组 (feature_vector, reconstructed_vector)
            let tuple = result.downcast::<PyTuple>()
                .expect("encode_with_context must return a tuple of two lists");

            let feat_list: &PyList = tuple.get_item(0)
                .expect("first element missing")
                .downcast()
                .expect("first element must be a list");
            let recon_list: &PyList = tuple.get_item(1)
                .expect("second element missing")
                .downcast()
                .expect("second element must be a list");

            let feat_vec: Vec<f32> = feat_list.iter().map(|v| v.extract::<f32>().unwrap_or(0.0)).collect();
            let recon_vec: Vec<f32> = recon_list.iter().map(|v| v.extract::<f32>().unwrap_or(0.0)).collect();

            (feat_vec, recon_vec)
        })
    }
}