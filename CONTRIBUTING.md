# Contributing to Network Monitor

Thank you for your interest in contributing to Network Monitor! We welcome contributions from everyone.

## Code of Conduct

This project follows a code of conduct to ensure a welcoming environment. See [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) for details.

## How to Contribute

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass (`cargo test`)
6. Update documentation if needed
7. Commit your changes (`git commit -m 'Add amazing feature'`)
8. Push to the branch (`git push origin feature/amazing-feature`)
9. Open a Pull Request

## Development Setup

### Prerequisites

- Rust 1.70+
- Git

### Setup

```bash
git clone https://github.com/yourusername/njord.git
cd njord
cargo build
cargo test
```

## Coding Standards

- Follow Rust standard formatting (`cargo fmt`)
- Run clippy for linting (`cargo clippy`)
- Write comprehensive tests (aim for 100% coverage)
- Use meaningful commit messages
- Document public APIs

## Testing

- Write unit tests for all new code
- Integration tests for cross-platform functionality
- Run `cargo test` before submitting PRs

## Reporting Issues

When reporting bugs:
- Use the issue template
- Include OS and Rust version
- Describe steps to reproduce
- Include error messages/logs

## Feature Requests

For new features:
- Check if it's already planned in issues
- Describe the use case clearly
- Consider implementation complexity

## Questions

For questions, use GitHub Discussions or join our community chat.

Thank you for contributing! 🎉