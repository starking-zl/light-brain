//! 斐波那契数列生成器
//! Fibonacci Sequence Generator
//!
//! 用于神经元新生的数量递推，避免过度生长。
//! Used for neuron count progression in neurogenesis to prevent excessive growth.

/// 斐波那契数列生成器
/// Fibonacci sequence generator
#[derive(Debug, Clone)]
pub struct FibonacciSequence {
    prev: u32,
    curr: u32,
}

impl FibonacciSequence {
    /// 创建新的斐波那契数列生成器
    /// Create a new Fibonacci sequence generator
    ///
    /// 默认起始于 1, 1
    /// Default starts at 1, 1
    pub fn new() -> Self {
        Self { prev: 1, curr: 1 }
    }

    /// 从指定起始值创建
    /// Create with custom starting values
    pub fn with_start(prev: u32, curr: u32) -> Self {
        Self { prev, curr }
    }

    /// 获取当前值
    /// Get current value
    pub fn current(&self) -> u32 {
        self.curr
    }

    /// 获取下一个值并推进数列
    /// Get next value and advance the sequence
    pub fn next(&mut self) -> u32 {
        let next = self.prev + self.curr;
        self.prev = self.curr;
        self.curr = next;
        next
    }

    /// 仅查看下一个值，不推进
    /// Peek at next value without advancing
    pub fn peek_next(&self) -> u32 {
        self.prev + self.curr
    }

    /// 重置到初始状态 (1, 1)
    /// Reset to initial state (1, 1)
    pub fn reset(&mut self) {
        self.prev = 1;
        self.curr = 1;
    }
}

impl Default for FibonacciSequence {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_sequence() {
        let mut fib = FibonacciSequence::new();
        assert_eq!(fib.current(), 1);
        assert_eq!(fib.next(), 2);
        assert_eq!(fib.current(), 2);
        assert_eq!(fib.next(), 3);
        assert_eq!(fib.next(), 5);
        assert_eq!(fib.next(), 8);
        assert_eq!(fib.next(), 13);
    }

    #[test]
    fn test_peek_next() {
        let fib = FibonacciSequence::new();
        assert_eq!(fib.peek_next(), 2);
    }

    #[test]
    fn test_reset() {
        let mut fib = FibonacciSequence::new();
        fib.next(); // 2
        fib.next(); // 3
        fib.reset();
        assert_eq!(fib.current(), 1);
    }
}