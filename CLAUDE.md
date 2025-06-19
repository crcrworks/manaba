# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Common Commands

### Build and Development
```bash
# Build the entire workspace
cargo build

# Build specific crate
cargo build -p manaba-cli
cargo build -p manaba-sdk

# Run the CLI (must be built first)
cargo run -p manaba-cli -- <command>

# Install locally for testing
cargo install --path crates/manaba-cli

# Run tests
cargo test
cargo test -p manaba-sdk  # SDK tests only
```

### Testing the CLI
```bash
# Test different commands
manaba check        # List all assignments
manaba report       # List reports only
manaba exam         # List exams only
manaba timetable    # Show timetable
manaba browse       # Open manaba in browser
manaba config-path  # Show config file location

# Test with flags
manaba report --all   # Show all reports
manaba report --warn  # Show only approaching deadlines
```

## Architecture

This is a Rust workspace with two main crates:

### manaba-sdk (crates/manaba-sdk/)
- Core library for interacting with the manaba learning management system
- Handles web scraping, cookie management, and data parsing
- Key modules:
  - `client.rs`: HTTP client with cookie-based authentication
  - `cookie.rs`: Browser cookie extraction using the `rookie` crate
  - `scrape/`: HTML parsing for courses, exams, and reports
  - `assignment.rs`: Common assignment data structures

### manaba-cli (crates/manaba-cli/)
- CLI application that uses manaba-sdk
- Handles configuration, colors, and command-line interface
- Key modules:
  - `main.rs`: App initialization, config loading, and global state
  - `cmd.rs`: Command parsing and routing using `clap`
  - `cmd/`: Individual command implementations (exam, report, timetable)
  - `app_config.rs`: Configuration management with TOML
  - `color.rs`: Customizable color theming system

### Key Design Patterns
- **Cookie-based Authentication**: Uses browser cookies extracted via `rookie` crate
- **Web Scraping**: HTML parsing with `scraper` crate for data extraction
- **Global State**: Uses `OnceLock` for app config and color schemes
- **Error Handling**: Custom error types with `thiserror` for both crates
- **Configuration**: TOML-based config with environment variable support

### Configuration System
- Config file location: `~/.config/manaba/settings.toml`
- Auto-creates default config if missing
- Supports color customization and timetable formatting
- Base URL and cookie domain configurable for different institutions