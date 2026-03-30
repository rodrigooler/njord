# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Event-driven monitoring for macOS (NWPathMonitor)
- Linux and Windows support
- Configuration file support
- Improved detection algorithms

## [0.1.1] - 2024-01-02

### Changed
- Improved error handling with descriptive expect messages
- Added configuration constants for URL, timeout, and monitoring interval
- Enhanced test reliability with proper failure simulation
- Code quality improvements: fixed Clippy warnings, applied rustfmt

### Fixed
- Removed unwrap() calls that could cause panics
- Better mutex lock error handling
- More robust connectivity testing

## [0.1.0] - 2024-01-01

### Added
- Project initialization
- Basic connectivity check using HTTP requests
- System tray integration for macOS
- Notification system for connectivity changes

### TODO
- Event-driven monitoring
- Cross-platform support (Linux, Windows)
- Improved detection algorithms