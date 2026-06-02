# Contributing Guide

> **Version**: v1.0  
> **Last Updated**: April 2026

Thank you for your interest in the Light-Brain Scheme! We welcome contributions of all forms, including but not limited to code submissions, documentation improvements, issue reports, and feature suggestions. The core principle of Light-Brain is "Peace and Love," and we expect community interactions to follow the same spirit.

## Code of Conduct

Please maintain a friendly, respectful, and professional atmosphere in communication. Harassment, insults, or discriminatory behavior of any kind will not be tolerated. We are committed to creating an open and inclusive collaborative environment for everyone.

## How to Contribute

### Reporting Issues

If you discover a bug or have a feature suggestion, please submit it via GitHub Issues. When submitting, please include the following information:

- A clear description of the issue
- Steps to reproduce (if applicable)
- Expected behavior versus actual behavior
- Environment information (operating system, Rust/Python versions, etc.)

### Code Contribution Workflow

1. **Fork the project**: Click the Fork button in the top-right corner of the GitHub repository.
2. **Clone the repository**:
    ```
    git clone https://github.com/your-username/light-brain.git
    ```
3. **Create a branch**:
    ```
    git checkout -b feature/your-feature-name
    ```
4. **Make changes**: Ensure the code adheres to existing style guidelines and add necessary tests and comments.
5. **Run tests**: Ensure all tests pass (both Rust and Python tests).
6. **Commit changes**: Use clear commit messages.
7. **Push the branch**:
    ```
    git push origin feature/your-feature-name
    ```
8. **Create a Pull Request**: Initiate a PR on GitHub, describing the changes and their motivation.

### Code Style

- **Rust**: Follow standard Rust style (use `cargo fmt` to format) and pass `cargo clippy` checks.
- **Python**: Adhere to PEP 8 guidelines (use `black` to format).
- **Comments**: All public APIs and critical logic must include **bilingual Chinese/English comments**, with Chinese above and English below.

Example:
```

/// 计算衰减后的权重
/// Calculate decayed weight
pub fn calculate_decayed_weight(...) -> f32 { ... }

```

### Documentation Contributions

Documentation is located in the `docs/zh/` and `docs/en/` directories. When modifying or adding documentation, please ensure that both Chinese and English versions are updated synchronously. Documentation is written in Markdown format; maintain clear structure and accurate language.

### Testing

- Rust unit tests:
    ```
    cd rust-core && cargo test --all
    ```
- Rust integration tests:
    ```
    cargo test --test '*'
    ```
- Python tests:
    ```
    pytest tests/
    ```

### Commit Message Guidelines

Please use the following prefixes in accordance with Conventional Commits:

| Prefix | Description |
|:---|:---|
| `feat` | New feature |
| `fix` | Bug fix |
| `docs` | Documentation update |
| `style` | Code formatting adjustment (no logic change) |
| `refactor` | Refactoring |
| `test` | Test-related changes |
| `chore` | Build or tooling changes |

Example: `feat(prefrontal): add dynamic priority to decision table`

## Development Environment Setup

1. Install Rust (https://rustup.rs/) and Python 3.10+.
2. Clone the repository and navigate to the project directory.
3. Compile the Rust core:
    ```
    cd rust-core && cargo build
    ```
4. Build the Python bindings:
    ```
    cd python-binding && maturin develop
    ```
5. Install Python dependencies:
    ```
    pip install -r requirements.txt
    ```

## Getting Help

If you have any questions, please feel free to ask via GitHub Issues or Discussions. We look forward to your participation!