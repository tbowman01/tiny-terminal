# Implementation Plan for tiny-terminal

## Overview
This document outlines the step-by-step implementation plan for the tiny-terminal Rust project, a hackable terminal toy with a Matrix rain default effect.

## Project Goals
- Create a minimal, performant terminal application in Rust
- Implement a Matrix-style rain effect as the default
- Support flexible configuration (project-level, user-level, CLI overrides)
- Enable easy extensibility for future effects
- Ensure cross-platform compatibility (Linux, macOS, Windows)

## Architecture Overview

### Core Components
1. **Configuration System** (`src/config.rs`)
   - Hierarchical config loading (CLI args > project file > user config > defaults)
   - TOML-based configuration files
   - Support for `.tiny-terminal.toml` discovery in project directories

2. **Effects System** (`src/effects/`)
   - Modular effect architecture
   - Feature-gated compilation for effects
   - Matrix rain as the default effect

3. **CLI Interface** (`src/main.rs`)
   - Clap-based argument parsing
   - Runtime parameter overrides
   - Effect selection

### Dependencies
- **crossterm**: Cross-platform terminal manipulation
- **clap**: Command-line argument parsing
- **serde/toml**: Configuration serialization
- **rand**: Random number generation for effects
- **directories**: Platform-specific directory paths
- **anyhow**: Error handling

## Implementation Phases

### Phase 1: Project Foundation (Tasks 1-4)

#### Task 1: Create Project Structure
Create the following directory layout:
```
tiny-terminal/
├─ .cargo/
├─ src/
│  └─ effects/
└─ examples/
   └─ project-config/
```

#### Task 2: Set Up Cargo.toml
- Define package metadata (name, version, authors, license)
- Add all required dependencies with appropriate versions
- Configure feature flags (default = ["effect-matrix"])
- Set up edition 2021

#### Task 3: Create .gitignore
- Ignore target/ directory
- Ignore IDE-specific files (.idea, .vscode)
- Ignore backup files (*.rs.bk)
- Ignore OS-specific files (.DS_Store)

#### Task 4: Create .cargo/config.toml
- Configure build optimizations
- Set up release profile with LTO and symbol stripping
- Configure codegen settings for smaller binaries

### Phase 2: Configuration System (Task 5)

#### Task 5: Implement src/config.rs
**Components:**
- `Config` struct with fields:
  - `fps`: Frame rate target (default: 60)
  - `column_width`: Character column width (default: 2)
  - `density`: Drop density 0.1-2.0 (default: 1.0)
  - `charset`: Character set string
  - `green`: Enable green color (default: true)

**Functionality:**
- `Config::default()`: Provide sensible defaults
- `Config::load(override_path)`: Hierarchical config loading
  1. Check explicit override path
  2. Search for `.tiny-terminal.toml` in current dir and parents
  3. Check user config at `~/.config/tiny-terminal/config.toml`
  4. Fall back to defaults
- `Config::from_file(path)`: Parse TOML config file
- `user_config_path()`: Locate platform-specific user config
- `find_project_file()`: Walk up directory tree for project config

**Error Handling:**
- Graceful fallback if config files are missing or invalid
- Use anyhow for error propagation

### Phase 3: Effects Module (Tasks 6-7)

#### Task 6: Create src/effects/mod.rs
- Conditionally export matrix module when `effect-matrix` feature is enabled
- Prepare structure for future effects

#### Task 7: Implement src/effects/matrix.rs
**Data Structures:**
- `Drop` struct:
  - `x`: Horizontal position (u16)
  - `y`: Vertical position (i16, allows negative for off-screen spawning)
  - `speed`: Fall speed (u16)

**Core Function: `run(cfg: &Config, cancel_key: Option<char>)`**

**Initialization:**
- Enter alternate screen (preserve user's terminal)
- Enable raw mode for direct key handling
- Initialize RNG and drop vector
- Calculate frame duration from FPS

**Main Loop:**
1. **Resize Detection**: Check terminal size, clear drops on resize
2. **Drop Spawning**:
   - Calculate column count based on terminal width and column_width
   - Spawn new drops based on density setting
   - Randomize position and speed
3. **Rendering**:
   - Hide cursor
   - Clear screen
   - Draw each drop with random character from charset
   - Apply green color if enabled
   - Flush output buffer
4. **Cleanup**: Remove off-screen drops
5. **Input Handling** (non-blocking):
   - Check for 'q', 'Esc', or Ctrl+C to exit
   - Check for custom cancel_key if provided
6. **Frame Timing**: Sleep to maintain target FPS

**Cleanup:**
- Show cursor
- Leave alternate screen
- Disable raw mode

**Helper Functions:**
- `pick_char()`: Select random character from charset

**Edge Cases:**
- Handle zero or very small terminal sizes
- Handle empty charset (fallback to '.')
- Prevent division by zero in FPS/column calculations

### Phase 4: Main Application (Task 8)

#### Task 8: Implement src/main.rs
**Components:**
- `Effect` enum (ValueEnum for clap):
  - `Matrix` variant (feature-gated)
  - Extensible for future effects

- `Args` struct (Parser):
  - `--effect`: Effect selection (optional, defaults to Matrix)
  - `--config/-c`: Override config file path
  - `--fps`: Runtime FPS override
  - `--density`: Runtime density override
  - `--cancel-key`: Custom exit key

**Main Function Flow:**
1. Parse command-line arguments
2. Load configuration (with optional path)
3. Apply CLI overrides (fps, density)
4. Match on selected effect and run
5. Propagate errors with anyhow

**Helper Functions:**
- `default_effect()`: Returns default effect (Matrix) when feature enabled

### Phase 5: Configuration Examples (Tasks 9-10)

#### Task 9: Create Root .tiny-terminal.toml
Example repository-level configuration:
- fps: 75
- column_width: 2
- density: 1.2
- Extended charset with more katakana
- green: true

Purpose: Demonstrate project-level overrides

#### Task 10: Create examples/project-config/.tiny-terminal.toml
Example dense configuration for demos:
- fps: 90
- density: 1.75
- Reduced charset for cleaner look

Purpose: Show how to create specialized configs

### Phase 6: Documentation (Task 11)

#### Task 11: Create README.md
**Sections:**
1. **Introduction**: Brief description of the project
2. **Quickstart**:
   - Basic build and run commands
   - Example usage with different parameters
   - Config file usage
3. **Keybindings**: Document exit keys
4. **Configuration**:
   - Explain hierarchy (CLI > project > user > defaults)
   - Show config file locations
   - Explain all configuration options
5. **Extending**:
   - Guide for adding new effects
   - Feature flag usage
   - Module structure
6. **Packaging**: Release build instructions
7. **License**: Dual Apache-2.0/MIT

### Phase 7: Validation & Testing (Task 12)

#### Task 12: Build and Test
**Build Verification:**
```bash
cargo build --release
```

**Functional Testing:**
1. Run with defaults: `cargo run --release`
2. Test CLI overrides: `cargo run --release -- --fps 90 --density 1.5`
3. Test config file: `cargo run --release -- --config examples/project-config/.tiny-terminal.toml`
4. Test keybindings: Verify 'q', 'Esc', Ctrl+C exit properly
5. Test resize handling: Resize terminal while running

**Validation Checklist:**
- [ ] Application builds without warnings
- [ ] Matrix effect renders correctly
- [ ] Terminal cleanup happens on exit (no leftover artifacts)
- [ ] Config files are read and applied correctly
- [ ] CLI overrides work as expected
- [ ] No memory leaks or performance issues
- [ ] Cross-platform compatibility (if possible to test)

## Success Criteria
- ✅ Application builds successfully with `cargo build --release`
- ✅ Matrix effect renders with smooth animation
- ✅ Configuration loading works in correct priority order
- ✅ All exit methods work (q, Esc, Ctrl+C, custom key)
- ✅ Terminal state is properly restored on exit
- ✅ README provides clear usage instructions
- ✅ Code follows Rust best practices (clippy-clean)

## Future Enhancements (Post-MVP)
- Additional effects (starfield, snow, clock, system dashboard)
- More comprehensive testing (unit tests, integration tests)
- CI/CD pipeline (GitHub Actions)
- Clippy and rustfmt configuration
- Performance profiling and optimization
- Color scheme customization
- Sound effects (optional)
- Recording/playback of effects

## Technical Considerations

### Performance
- Target 60 FPS default for smooth animation
- Use release builds for actual usage (debug is significantly slower)
- LTO and symbol stripping reduce binary size
- Efficient drop culling prevents memory growth

### Cross-Platform
- crossterm handles platform differences
- Truecolor support with graceful fallback
- Platform-specific config paths via directories crate

### Error Handling
- anyhow for convenient error propagation
- Graceful fallback for missing/invalid configs
- Proper cleanup even on errors

### Code Quality
- Use rustfmt for consistent formatting
- Run clippy for linting
- Follow Rust naming conventions
- Document public APIs

## Timeline Estimate
- **Phase 1** (Foundation): 15-20 minutes
- **Phase 2** (Config System): 15-20 minutes
- **Phase 3** (Effects): 20-25 minutes
- **Phase 4** (Main App): 10-15 minutes
- **Phase 5** (Examples): 5-10 minutes
- **Phase 6** (Documentation): 10-15 minutes
- **Phase 7** (Testing): 15-20 minutes

**Total Estimated Time**: 90-125 minutes

## Risk Mitigation
- **Risk**: Terminal doesn't restore properly
  - **Mitigation**: Ensure cleanup code runs even on panic

- **Risk**: Platform-specific issues
  - **Mitigation**: Use well-tested crossterm library

- **Risk**: Performance issues on slower systems
  - **Mitigation**: Configurable FPS, efficient algorithms

- **Risk**: Config parsing errors
  - **Mitigation**: Graceful fallback to defaults

## Conclusion
This implementation plan provides a clear roadmap for building the tiny-terminal application. Each phase builds upon the previous one, ensuring a systematic and maintainable development process. The modular architecture allows for easy extension with new effects while keeping the core simple and focused.
