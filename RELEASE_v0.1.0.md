# Release v0.1.0 - Manual Release Instructions

## Binary Package Created

**Release Package:** `tiny-terminal-v0.1.0-linux-x86_64.tar.gz`
**Size:** 547KB (compressed)
**Binary Size:** 1.2MB (uncompressed, stripped)
**SHA256:** `43dcbb6505426775e12557771bae5a533b55ed4f69302c784c81fb34a68a9500`

## Files Ready for Upload
```
tiny-terminal-v0.1.0-linux-x86_64.tar.gz
tiny-terminal-v0.1.0-linux-x86_64.tar.gz.sha256
```

## Creating the GitHub Release

### Option 1: Via GitHub Web Interface

1. **Navigate to Releases**
   - Go to: https://github.com/tbowman01/tiny-terminal/releases
   - Click "Draft a new release"

2. **Create Release Tag**
   - Tag version: `v0.1.0`
   - Target: `main` branch
   - Release title: `Release v0.1.0 - Initial Release`

3. **Add Release Notes**
   Copy the content from the "Release Notes Template" section below

4. **Upload Binary Assets**
   - Drag and drop or click to upload:
     - `tiny-terminal-v0.1.0-linux-x86_64.tar.gz`
     - `tiny-terminal-v0.1.0-linux-x86_64.tar.gz.sha256`

5. **Publish Release**
   - Click "Publish release"

### Option 2: Via GitHub CLI (gh)

If you have push permissions and `gh` CLI installed:

```bash
# Create the release
gh release create v0.1.0 \
  --title "Release v0.1.0 - Initial Release" \
  --notes-file RELEASE_NOTES.md \
  tiny-terminal-v0.1.0-linux-x86_64.tar.gz \
  tiny-terminal-v0.1.0-linux-x86_64.tar.gz.sha256
```

### Option 3: Via Git Tag Push

If you have push permissions:

```bash
# Push the tag (will trigger automated workflow if permissions allow)
git push origin v0.1.0
```

This will trigger the `.github/workflows/release.yml` workflow which will automatically build binaries for all 6 platforms.

---

## Release Notes Template

Use this template for the GitHub release notes:

```markdown
# tiny-terminal v0.1.0 - Initial Release

A tiny, hackable terminal toy written in Rust featuring a Matrix-style rain effect with smooth 60 FPS animation.

## ✨ Features

- 🎨 **Matrix rain visual effect** with customizable parameters
- ⚙️ **Hierarchical configuration system** (CLI args → project file → user config → defaults)
- 🖥️ **Cross-platform support** (Linux, macOS, Windows)
- 🎯 **Feature-gated modular architecture** for easy extensibility
- ⚡ **Optimized builds** with LTO and symbol stripping (~1.2MB binary)
- 🎮 **Non-blocking input** with multiple exit methods (q, Esc, Ctrl+C)
- 🔧 **Comprehensive Makefile** with 30+ development targets
- 🤖 **Full CI/CD automation** with GitHub Actions

## 📦 Installation

### Linux / macOS

```bash
# Download and extract
wget https://github.com/tbowman01/tiny-terminal/releases/download/v0.1.0/tiny-terminal-v0.1.0-linux-x86_64.tar.gz
tar xzf tiny-terminal-v0.1.0-linux-x86_64.tar.gz

# Verify checksum (optional)
sha256sum -c tiny-terminal-v0.1.0-linux-x86_64.tar.gz.sha256

# Move to bin directory
sudo mv tiny-terminal /usr/local/bin/
sudo chmod +x /usr/local/bin/tiny-terminal

# Or install to user directory
mkdir -p ~/.local/bin
mv tiny-terminal ~/.local/bin/
```

### From Source

```bash
git clone https://github.com/tbowman01/tiny-terminal.git
cd tiny-terminal
make install
```

## 🚀 Quick Start

```bash
# Run with defaults
tiny-terminal

# Customize settings
tiny-terminal --density 1.5 --fps 90

# Use custom config
tiny-terminal --config /path/to/config.toml
```

## ⚙️ Configuration

Create `.tiny-terminal.toml` in your project or `~/.config/tiny-terminal/config.toml`:

```toml
fps = 75
column_width = 2
density = 1.2
charset = "ｱｲｳｴｵｶｷｸｹｺｻｼｽｾｿﾀﾁﾂﾃﾄﾅﾆﾇﾈﾉ0123456789@#$%&*"
green = true
```

## 📊 Technical Details

- **Language:** Rust (Edition 2021)
- **Binary Size:** 1.2MB (stripped)
- **Memory Usage:** 2-5MB during runtime
- **CPU Usage:** <5% at 60 FPS on modern hardware
- **Platforms:** Linux, macOS, Windows

## 🔧 Dependencies

- crossterm ^0.27 - Cross-platform terminal manipulation
- clap ^4 - Command-line argument parsing
- rand ^0.8 - Random number generation
- serde ^1 - Serialization
- toml ^0.8 - Configuration parsing
- directories ^5 - Platform-specific paths
- anyhow ^1 - Error handling

## 📝 What's Included

- Complete Rust implementation with Matrix rain effect
- Hierarchical configuration system
- Comprehensive README with usage examples
- Makefile with 30+ development commands
- GitHub Actions CI/CD workflows
- Example configurations
- Full documentation

## 🙏 Acknowledgments

- Inspired by the classic Matrix digital rain effect
- Built with amazing Rust ecosystem tools

## 📄 License

Dual-licensed under MIT OR Apache-2.0

## 🔗 Links

- **Documentation:** [README.md](https://github.com/tbowman01/tiny-terminal/blob/main/README.md)
- **Changelog:** [CHANGELOG.md](https://github.com/tbowman01/tiny-terminal/blob/main/CHANGELOG.md)
- **Issues:** [GitHub Issues](https://github.com/tbowman01/tiny-terminal/issues)

---

**SHA256 Checksum:**
```
43dcbb6505426775e12557771bae5a533b55ed4f69302c784c81fb34a68a9500  tiny-terminal-v0.1.0-linux-x86_64.tar.gz
```

**Verify:**
```bash
echo "43dcbb6505426775e12557771bae5a533b55ed4f69302c784c81fb34a68a9500  tiny-terminal-v0.1.0-linux-x86_64.tar.gz" | sha256sum -c
```
```

---

## Download Binary Now

The binary is ready in the repository:
- Location: `./tiny-terminal-v0.1.0-linux-x86_64.tar.gz`
- Checksum: `./tiny-terminal-v0.1.0-linux-x86_64.tar.gz.sha256`

You can download it directly from the repository or create the GitHub release using one of the methods above.

---

## Future Automated Releases

Once the v0.1.0 tag is pushed to GitHub (when permissions allow), the automated release workflow will build binaries for:

1. **Linux x86_64 (GNU)** - Standard Linux distribution
2. **Linux x86_64 (musl)** - Static linked, fully portable
3. **Linux aarch64** - ARM64/Raspberry Pi
4. **macOS x86_64** - Intel Macs
5. **macOS aarch64** - Apple Silicon (M1/M2/M3)
6. **Windows x86_64** - Windows 10/11

Each with SHA256 checksums for verification.

---

## Testing the Binary

```bash
# Extract
tar xzf tiny-terminal-v0.1.0-linux-x86_64.tar.gz

# Test
./tiny-terminal --version
# Output: tiny-terminal 0.1.0

./tiny-terminal --help
# Shows all available options

# Run (press q to quit)
./tiny-terminal
```

---

## Next Steps

1. Create the GitHub release using one of the methods above
2. Upload the binary package files
3. Share the download link with users
4. For future releases, simply push new version tags (e.g., `v0.2.0`)

**Release is ready! 🎉**
