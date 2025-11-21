# Release Guide

This guide explains how to create and publish releases of Git Commit Helper.

## 📋 Prerequisites

- Git installed and configured
- Rust toolchain installed
- GitHub repository with push access
- All changes committed and pushed to main branch

## 🚀 Release Process

### Step 1: Update Version Numbers

Update the version in these files:

**1. `Cargo.toml`:**

```toml
[package]
version = "1.0.0"  # Update this
```

**2. `src/main.rs`:**

```rust
ServerInfo {
    name: "git-commit-helper".to_string(),
    version: "1.0.0".to_string(),  // Update this
}
```

**3. `CHANGELOG.md`:**
Add a new section at the top:

```markdown
## [1.1.0] - 2024-11-21

### Added
- New feature description

### Fixed
- Bug fix description
```

Commit these changes:

```bash
git add Cargo.toml src/main.rs CHANGELOG.md
git commit -m "chore: bump version to 1.0.0"
git push origin main
```

### Step 2: Build Release Binaries

Build for all platforms using the provided script:

```bash
./build.sh --all
```

This creates binaries in:

- `target/x86_64-unknown-linux-gnu/release/git-commit-helper`
- `target/x86_64-apple-darwin/release/git-commit-helper`
- `target/aarch64-apple-darwin/release/git-commit-helper`
- `target/x86_64-pc-windows-gnu/release/git-commit-helper.exe`

**Verify each binary works:**

```bash
# Test Linux binary
./target/x86_64-unknown-linux-gnu/release/git-commit-helper --help

# Test macOS Intel binary
./target/x86_64-apple-darwin/release/git-commit-helper --help

# etc.
```

### Step 3: Create Descriptive Binary Names

Rename binaries to include platform information:

```bash
mkdir -p releases/v1.0.0

# Linux
cp target/x86_64-unknown-linux-gnu/release/git-commit-helper \
   releases/v1.0.0/git-commit-helper-v1.0.0-linux-x86_64

# macOS Intel
cp target/x86_64-apple-darwin/release/git-commit-helper \
   releases/v1.0.0/git-commit-helper-v1.0.0-macos-x86_64

# macOS ARM
cp target/aarch64-apple-darwin/release/git-commit-helper \
   releases/v1.0.0/git-commit-helper-v1.0.0-macos-arm64

# Windows
cp target/x86_64-pc-windows-gnu/release/git-commit-helper.exe \
   releases/v1.0.0/git-commit-helper-v1.0.0-windows-x86_64.exe
```

### Step 4: Generate Checksums

Create checksums for verification:

```bash
cd releases/v1.0.0

# Linux/macOS:
shasum -a 256 * > checksums.txt

# Or use sha256sum on Linux:
sha256sum * > checksums.txt

# Windows PowerShell:
Get-ChildItem -File | ForEach-Object {
    (Get-FileHash $_.Name -Algorithm SHA256).Hash + "  " + $_.Name
} | Out-File checksums.txt
```

### Step 5: Create Git Tag

Create and push a version tag:

```bash
git tag -a v1.0.0 -m "Release version 1.0.0"
git push origin v1.0.0
```

### Step 6: Create GitHub Release

1. **Navigate to GitHub releases page:**

   ```
   https://github.com/tarekbadrsh/git-commit-helper/releases/new
   ```

2. **Select your tag:**
   - Choose the tag you just pushed (v1.0.0)

3. **Set release title:**

   ```
   v1.0.0 - Initial Release
   ```

4. **Write release notes:**

   Use your CHANGELOG.md content and format it nicely:

   ```markdown
   ## 🎉 Initial Release

   Git Commit Helper is an MCP server that helps Claude write professional commit messages 
   by reading git repository information.

   ### ✨ Features

   - **Four Git Tools**: status, diff staged, diff all, and log
   - **Cross-Platform**: Runs on Linux, macOS (Intel & ARM), and Windows
   - **Standalone Binary**: No dependencies - just download and run
   - **Lightning Fast**: Starts in under 1ms
   - **Privacy First**: Runs entirely locally
   - **Small Size**: Only ~3MB per binary
   - **Official SDK**: Built with Anthropic's Rust MCP SDK

   ### 📦 Installation

   1. Download the binary for your platform below
   2. Make it executable (Linux/macOS: `chmod +x git-commit-helper`)
   3. Configure Claude Desktop (see [QUICKSTART.md](QUICKSTART.md))

   ### 🔍 Checksums (SHA-256)

   See `checksums.txt` in the release assets for file verification.

   ### 📚 Documentation

   - [README.md](README.md) - Full documentation
   - [QUICKSTART.md](QUICKSTART.md) - 3-minute setup guide
   - [CHANGELOG.md](CHANGELOG.md) - Version history

   ### 🐛 Known Issues

   None currently reported.

   ### 🙏 Thanks

   Thanks to Anthropic for the excellent MCP SDK and the Rust community for great tooling!
   ```

5. **Attach binaries:**
   - Click "Attach binaries by dropping them here or selecting them"
   - Upload all files from `releases/v1.0.0/`:
     - `git-commit-helper-v1.0.0-linux-x86_64`
     - `git-commit-helper-v1.0.0-macos-x86_64`
     - `git-commit-helper-v1.0.0-macos-arm64`
     - `git-commit-helper-v1.0.0-windows-x86_64.exe`
     - `checksums.txt`

6. **Publish the release:**
   - Check "Set as the latest release"
   - Click "Publish release"

### Step 7: Verify Release

1. Visit your releases page:

   ```
   https://github.com/tarekbadrsh/git-commit-helper/releases
   ```

2. Verify:
   - All binaries are attached and downloadable
   - Checksums file is present
   - Release notes are clear and well-formatted
   - Download links work

3. Test downloading and running a binary:

   ```bash
   # Download
   curl -L -o git-commit-helper \
     https://github.com/tarekbadrsh/git-commit-helper/releases/download/v1.0.0/git-commit-helper-v1.0.0-linux-x86_64

   # Make executable
   chmod +x git-commit-helper

   # Test
   ./git-commit-helper
   ```

## 🏷️ Version Numbering

Follow [Semantic Versioning](https://semver.org/):

- **Major (X.0.0)**: Breaking changes, incompatible API changes
- **Minor (1.X.0)**: New features, backwards compatible
- **Patch (1.0.X)**: Bug fixes, backwards compatible

Examples:

- `1.0.0` → `1.0.1` - Bug fixes only
- `1.0.0` → `1.1.0` - Added new git tool
- `1.0.0` → `2.0.0` - Changed MCP protocol version (breaking)

## 📝 Release Checklist

Use this checklist for each release:

- [ ] All tests pass
- [ ] Documentation updated
- [ ] CHANGELOG.md updated with all changes
- [ ] Version bumped in Cargo.toml and src/main.rs
- [ ] Changes committed and pushed to main
- [ ] Built binaries for all platforms with `./build.sh --all`
- [ ] Binaries renamed with version and platform
- [ ] Checksums generated
- [ ] Git tag created and pushed
- [ ] GitHub release created with proper notes
- [ ] All binaries attached to release
- [ ] Release published and verified
- [ ] Download links tested
- [ ] Checksums verified

## 🔄 Post-Release

After publishing:

1. **Announce the release:**
   - Update README.md with latest version number
   - Post in relevant communities (Discord, Reddit, etc.)
   - Submit to MCP directories (see [PUBLISHING.md](PUBLISHING.md))

2. **Monitor issues:**
   - Watch for bug reports
   - Respond to user questions
   - Plan next release if issues found

3. **Update documentation:**
   - Ensure all docs reference latest version
   - Update examples if needed

## ⚠️ Troubleshooting

### Build Fails for a Platform

Some platforms require specific setup:

**macOS cross-compilation from Linux:**
Requires OSX cross toolchain - may be easier to build on actual macOS hardware.

**Windows cross-compilation from Linux:**

```bash
sudo apt-get install mingw-w64
rustup target add x86_64-pc-windows-gnu
```

**Solution:** Build on native platform or use GitHub Actions for automated builds.

### Binary Too Large

If binary size exceeds 5MB, check:

- Profile settings in Cargo.toml (should use `opt-level = "z"`)
- Strip symbols: `strip = true`
- LTO enabled: `lto = true`

### GitHub Upload Limit

GitHub has a 2GB limit per file. Our binaries are ~3MB, well under limit.

## 🤖 Automation Ideas

Consider automating releases with GitHub Actions:

```yaml
# .github/workflows/release.yml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
          - os: macos-latest
            target: x86_64-apple-darwin
          - os: macos-latest
            target: aarch64-apple-darwin
          - os: windows-latest
            target: x86_64-pc-windows-gnu

    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: ${{ matrix.target }}
      - uses: actions-rs/cargo@v1
        with:
          command: build
          args: --release --target ${{ matrix.target }}
      # Upload artifacts and create release...
```

This workflow would automatically build and release when you push a tag.

---

**Happy Releasing! 🚀**
