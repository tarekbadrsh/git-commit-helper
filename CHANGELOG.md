# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2024-11-21

### Added

- Initial release of Git Commit Helper MCP server
- `git_status` tool - Show current repository state with modified, staged, and untracked files
- `git_diff_staged` tool - Display line-by-line changes for staged files
- `git_diff_all` tool - Show all changes including unstaged modifications with optional untracked files
- `git_log` tool - View recent commit history with configurable limit (up to 50 commits)
- Cross-platform support for Linux (x86_64), macOS (Intel and ARM), and Windows (x86_64)
- Standalone binary with zero runtime dependencies
- Fast startup time (under 1 millisecond)
- Built with official Anthropic Rust MCP SDK (rmcp 0.8)
- Comprehensive documentation including README, QUICKSTART, and publishing guides
- Cross-compilation build script for all supported platforms
- User-friendly error messages with actionable guidance
- Optimized binary size (~3MB) with LTO and size optimizations

### Technical Details

- Uses stdio transport for communication with Claude Desktop
- Implements ServerHandler trait for MCP protocol compliance
- Async/await architecture with Tokio runtime
- Proper error handling without panics
- Clean separation between tool logic and command execution

[1.0.0]: https://github.com/tarekbadrsh/git-commit-helper/releases/tag/v1.0.0
