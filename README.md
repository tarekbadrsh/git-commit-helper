# Git Commit Helper

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20macOS%20%7C%20Windows-blue.svg)](https://github.com/yourusername/git-commit-helper)
[![MCP Compatible](https://img.shields.io/badge/MCP-Compatible-green.svg)](https://modelcontextprotocol.io)

An MCP (Model Context Protocol) server that helps Claude write professional commit messages by reading git repository information. Built with the official Anthropic Rust SDK for maximum performance and reliability.

## 🌟 Features

✨ **Standalone Binary** - No Node, Python, Bun, or any runtime dependencies required  
⚡ **Lightning Fast** - Starts in under 1 millisecond  
🔒 **Privacy First** - Runs entirely on your local machine  
🌍 **Cross-Platform** - Works on Linux, macOS (Intel & ARM), and Windows  
🎯 **Smart Commit Messages** - Claude analyzes your changes and suggests clear, conventional commits  
📦 **Tiny Size** - Just ~3MB binary  
🛠️ **Official SDK** - Built with Anthropic's official Rust MCP SDK for reliability

## 💡 Use Cases

This tool is perfect for:

- **Solo Developers** who want consistent, professional commit messages
- **Teams** maintaining commit message standards across projects
- **Open Source Contributors** writing clear commits for community projects
- **Code Reviewers** who need to quickly understand what changed
- **Git Learners** who want to see examples of good commit messages

## 🎬 Demo

Here's how it works in practice:

```
You: I've made some changes and staged them. Can you write me a commit message?

Claude: Let me check what you've staged.
[Uses git_diff_staged tool]

I can see you've added user authentication with JWT tokens and password hashing. 
Here's a suggested commit message:

feat(auth): implement JWT-based user authentication

- Add JWT token generation and verification
- Implement bcrypt password hashing for security
- Create login and registration endpoints
- Add middleware for protected routes

This follows conventional commit format. Want me to adjust anything?
```

## 🚀 Setup Instructions

### Prerequisites

You need Rust installed. If you don't have it:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Visit [rustup.rs](https://rustup.rs) for more installation options.

### Build from Source

1. **Clone and build:**
   ```bash
   git clone https://github.com/yourusername/git-commit-helper.git
   cd git-commit-helper
   cargo build --release
   ```

2. **Find your binary:**
   ```bash
   # The binary will be at:
   # target/release/git-commit-helper
   ```

3. **Optional - Install globally:**
   ```bash
   # Copy to a directory in your PATH
   sudo cp target/release/git-commit-helper /usr/local/bin/
   # Or on Windows:
   copy target\release\git-commit-helper.exe C:\Windows\System32\
   ```

### Configure Claude Desktop

You need to tell Claude Desktop where to find the binary.

**📁 Config File Locations:**

- **macOS:** `~/Library/Application Support/Claude/claude_desktop_config.json`
- **Windows:** `%APPDATA%\Claude\claude_desktop_config.json`
- **Linux:** `~/.config/Claude/claude_desktop_config.json`

**Edit the config file and add:**

```json
{
  "mcpServers": {
    "git-commit-helper": {
      "command": "/absolute/path/to/git-commit-helper"
    }
  }
}
```

**⚠️ Important:** Use the **absolute path** to your binary!

**Platform-specific examples:**

**macOS/Linux:**
```json
{
  "mcpServers": {
    "git-commit-helper": {
      "command": "/Users/yourname/git-commit-helper/target/release/git-commit-helper"
    }
  }
}
```

**Windows:**
```json
{
  "mcpServers": {
    "git-commit-helper": {
      "command": "C:\\Users\\YourName\\git-commit-helper\\target\\release\\git-commit-helper.exe"
    }
  }
}
```

**💡 Tip:** To get the absolute path:
```bash
# macOS/Linux:
cd git-commit-helper
echo "$(pwd)/target/release/git-commit-helper"

# Windows PowerShell:
cd git-commit-helper
echo "$pwd\target\release\git-commit-helper.exe"
```

### Restart Claude Desktop

**Completely quit and restart** Claude Desktop (not just close the window - use Cmd+Q on macOS or fully exit on Windows/Linux).

## 📝 Usage Examples

Try these natural language queries with Claude:

### Generate Commit Messages
- "I have staged changes, write me a commit message"
- "Review my changes and suggest a conventional commit message"
- "Help me write a commit message for these changes"

### Review Changes
- "Show me what I'm about to commit"
- "What files have I modified?"
- "Show me all my changes in this repository"

### Understand Context
- "Show me the last 10 commits"
- "What's the commit message style in this project?"
- "Show me recent commits so I can match the style"

### Combined Workflows
- "Look at my staged changes and recent commits, then write a commit message that matches our style"
- "Show me what I've changed and help me decide what to commit first"

## 🛠️ Available Tools

The server provides four tools that Claude can use:

### 1. `git_status`
Shows the current state of the repository - modified, staged, and untracked files.

**Parameters:**
- `repo_path` (optional): Path to git repository

**Example:** "Show me what files I've changed"

### 2. `git_diff_staged`
Shows line-by-line changes for files staged with `git add`. This is what will be committed.

**Parameters:**
- `repo_path` (optional): Path to git repository

**Example:** "Show me what I'm about to commit"

### 3. `git_diff_all`
Shows all changes in the repository, both staged and unstaged. Can optionally include untracked files.

**Parameters:**
- `repo_path` (optional): Path to git repository
- `include_untracked` (optional, default: false): Include untracked files

**Example:** "Show me all my changes including untracked files"

### 4. `git_log`
Shows recent commit history to understand commit message conventions.

**Parameters:**
- `repo_path` (optional): Path to git repository
- `limit` (optional, default: 10, max: 50): Number of commits to show

**Example:** "Show me the last 5 commits"

## 🔧 Troubleshooting

### macOS: "cannot be opened because the developer cannot be verified"

macOS Gatekeeper may block the binary. To fix:

```bash
xattr -d com.apple.quarantine /path/to/git-commit-helper
```

### Linux: "Permission denied"

Make the binary executable:

```bash
chmod +x /path/to/git-commit-helper
```

### "Not a git repository" Error

Make sure you're running commands in a directory that has been initialized with `git init` or is a cloned repository.

### Claude Desktop Not Finding the Server

1. Check that the config file path is correct for your OS
2. Verify you're using an **absolute path** to the binary (not relative like `./git-commit-helper`)
3. Make sure you **completely restarted** Claude Desktop (not just closed the window)
4. Check Claude Desktop logs for errors (usually in same folder as config)

### Windows: "git is not recognized"

Make sure git is installed and in your PATH:

```bash
# Test git installation
git --version
```

If not installed, download from [git-scm.com](https://git-scm.com/)

## 🏗️ Cross-Compilation

Build for all platforms at once:

```bash
./build.sh --all
```

This creates binaries for:
- Linux x86_64: `target/x86_64-unknown-linux-gnu/release/git-commit-helper`
- macOS Intel: `target/x86_64-apple-darwin/release/git-commit-helper`
- macOS ARM: `target/aarch64-apple-darwin/release/git-commit-helper`
- Windows: `target/x86_64-pc-windows-gnu/release/git-commit-helper.exe`

**Manual cross-compilation:**

```bash
# Add target (one-time setup)
rustup target add x86_64-unknown-linux-gnu

# Build for that target
cargo build --release --target x86_64-unknown-linux-gnu
```

**Available targets:**
- `x86_64-unknown-linux-gnu` - Linux x86_64
- `x86_64-apple-darwin` - macOS Intel
- `aarch64-apple-darwin` - macOS Apple Silicon
- `x86_64-pc-windows-gnu` - Windows x86_64

## 🤔 Why Rust?

**Single Binary Distribution**  
No npm, pip, or runtime installation needed. Users just download and run.

**Instant Startup**  
Under 1ms startup time vs 100-500ms for Node.js/Python.

**Small Size**  
~3MB vs 30-50MB for Node.js-based solutions.

**Type Safety**  
Compile-time guarantees prevent runtime errors.

**Official SDK**  
Built with Anthropic's official Rust MCP SDK for long-term reliability.

## 📖 Documentation

- **[QUICKSTART.md](QUICKSTART.md)** - 3-minute setup guide
- **[RELEASE.md](RELEASE.md)** - How to create GitHub releases
- **[PUBLISHING.md](PUBLISHING.md)** - How to submit to MCP stores
- **[CHANGELOG.md](CHANGELOG.md)** - Version history

## 🤝 Contributing

Contributions are welcome! This is a simple, focused tool - please keep it that way:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with the official [Anthropic Rust MCP SDK](https://github.com/modelcontextprotocol/rust-sdk)
- Inspired by the [Model Context Protocol](https://modelcontextprotocol.io)
- Thanks to the Rust community for excellent tooling

## 🔗 Links

- [Model Context Protocol](https://modelcontextprotocol.io)
- [Anthropic MCP Documentation](https://docs.anthropic.com/en/docs/build-with-claude/mcp)
- [Rust MCP SDK](https://github.com/modelcontextprotocol/rust-sdk)

---

**Made with ❤️ using Rust and the official Anthropic MCP SDK**
