# Claude Configuration for Njord Project

## Project Context
Njord is a cross-platform network connectivity monitor written in Rust. It provides real-time notifications when internet connection status changes, with a focus on detecting scenarios where you're connected to WiFi but lack actual internet access.

## Development Guidelines

### Based in Brazil: All context-less questions regarding law, society, or systems should assume Brazilian standards.

### Git Commits
- NEVER add "Co-Authored-By" trailers to commit messages.
- Do not include any Claude/Anthropic attribution in commits.
- Use conventional commit format: `feat:`, `fix:`, `docs:`, `chore:`, etc.

### Documentation
- Always choose to write explanations in the conversation channel instead of creating markdown files unless explicitly told.
- Prefer inline comments and conversation for clarifications.

### Fixes and Changes
- If asked to fix something (unless it's a simple typo), always ask as many clarifying questions as needed to fully understand the context, especially for architectural changes.
- Prioritize understanding requirements before implementing solutions.

### Code Style
- Follow Rust standard formatting (`cargo fmt`)
- Use meaningful variable and function names
- Write comprehensive tests (aim for high coverage)
- Document public APIs with clear examples

### Project Structure
- `src/lib.rs`: Core library functionality
- `src/main.rs`: Application entry point
- `src/connectivity.rs`: Network checking logic
- Tests in respective modules

### Dependencies
- Core: `tokio`, `reqwest`, `tao`, `notify-rust`
- macOS specific: `system-configuration`
- Keep dependencies minimal and well-maintained

### Testing
- Unit tests for all logic
- Integration tests for full functionality
- Run `cargo test` before commits

### Releases
- Use semantic versioning (v0.1.0, v0.2.0, etc.)
- Create GitHub releases with binaries
- Document changes in CHANGELOG.md

### Security
- Never commit secrets or keys
- Use secure HTTP clients (reqwest with TLS)
- Validate all inputs

### Performance
- Optimize for low resource usage
- Use async where appropriate
- Avoid blocking operations in UI threads

### Cross-Platform
- Design with Linux and Windows in mind
- Use conditional compilation for platform-specific code
- Test on all target platforms when possible

## Communication
- Be direct and technical in responses
- Use concise language
- Provide code examples when explaining
- Ask for clarification when requirements are ambiguous