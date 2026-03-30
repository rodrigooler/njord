# Njord

[![Build Status](https://github.com/rodrigooler/njord/actions/workflows/ci.yml/badge.svg)](https://github.com/rodrigooler/njord/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org/)

A highly efficient, cross-platform network connectivity monitor written in Rust. Named after Njord, the Norse god of sea, ships, and commerce - representing connections and navigation. Inspired by modern macOS network monitoring practices, this tool provides real-time notifications when your internet connection status changes, with special attention to detecting scenarios where you're connected to WiFi but lack actual internet access (e.g., connected to router but no WAN).

## Description

This project addresses the common issue where devices appear connected to a WiFi network (interface active) but can't reach the internet (e.g., router connected to modem but ISP down). Unlike polling-based solutions that waste battery and CPU, it aims for event-driven monitoring:

- **macOS**: Uses NWPathMonitor (Network framework) for efficient, callback-based network state changes
- **Cross-platform**: Extensible to Linux (netlink) and Windows (WMI) with similar event-driven approaches

The app runs discreetly in the system tray/menu bar, sending push notifications only when connectivity status actually changes.

## Features

- 🖥️ System tray icon with status indication
- 🔔 Push notifications on connectivity changes
- 🔄 Event-driven monitoring (planned)
- 🌐 Cross-platform support (macOS first, then Linux/Windows)
- ⚡ Low resource usage (no constant polling)
- 🎯 Smart detection: Distinguishes "connected to network" from "internet access available"

## How It Works

The monitor checks network path status:

1. **Path Status**: Monitors if there's a viable network path
2. **Connectivity Verification**: For robustness, periodically verifies real internet access
3. **Notifications**: Alerts user only on status changes
4. **Battery-Friendly**: Uses system callbacks instead of polling where possible

## Platforms

- ✅ **macOS**: Full implementation with system tray and notifications
- 🚧 **Linux**: Planned (netlink-based event monitoring)
- 🚧 **Windows**: Planned (WMI-based monitoring)

## Installation

### Prerequisites

- Rust 1.70+
- macOS 10.14+ (for current implementation)

### Build from Source

```bash
git clone https://github.com/rodrigooler/njord.git
cd njord
cargo build --release
```

## Usage

```bash
cargo run --release
```

The app will:
- Appear in your macOS menu bar
- Monitor network connectivity in the background
- Send notifications when internet status changes

## Configuration

Currently, no configuration needed. Future versions may include:
- Custom check endpoints
- Notification preferences
- Check intervals

## Development

### Testing

Run tests with full coverage:

```bash
cargo test -- --nocapture
```

### Code Coverage

We aim for 100% code coverage:

```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## Roadmap

- [ ] Implement event-driven monitoring on macOS (NWPathMonitor)
- [ ] Add Linux support with netlink
- [ ] Add Windows support
- [ ] Improve "no internet" detection algorithm
- [ ] Add configuration file support
- [ ] Localization support

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

Inspired by Apple's Network framework and various open-source connectivity libraries.