//! 神经编码器接口
//! Neural Encoder Interface
//!
//! 定义丘脑与 Python 侧神经网络编码器的接口。
//! Defines the interface between Thalamus and Python-side neural network encoder.

/// 神经编码器 trait
/// Neural encoder trait
///
/// 实际实现位于 Python 侧，通过 PyO3 绑定调用。
/// 要求实现者具备 'static 生命周期，以便安全地跨线程持有。
/// Actual implementation resides in Python side, invoked via PyO3 bindings.
/// Requires 'static lifetime for safe cross-thread holding.
pub trait NeuralEncoder: Send + Sync + 'static {
    /// 对文本进行编码，可选上下文注入
    /// Encode text with optional context injection
    ///
    /// # 参数 / Arguments
    /// * `text` - 输入文本 / input text
    /// * `context_vector` - 可选的上下文向量 / optional context vector
    ///
    /// # 返回 / Returns
    /// (特征向量, 重构向量) 元组。
    /// (feature vector, reconstructed vector) tuple.
    /// 特征向量用于原型匹配，重构向量用于计算重构相似度。
    /// Feature vector for prototype matching, reconstructed vector for reconstruction similarity.
    fn encode_with_context(&self, text: &str, context_vector: Option<&[f32]>) -> (Vec<f32>, Vec<f32>);
}