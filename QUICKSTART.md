# ⚡ Quick Start Guide

Get up and running with Git Commit Helper in 3 minutes!

## 📋 Prerequisites

- Git installed
- Claude Desktop app installed

## 🚀 5 Simple Steps

### Step 1: Install Rust (One-Time Only)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Restart your terminal after installation.

### Step 2: Clone and Build

```bash
git clone https://github.com/tarekbadrsh/git-commit-helper.git
cd git-commit-helper
cargo build --release
```

⏱️ This takes 2-3 minutes on first build (Rust downloads and compiles dependencies).

### Step 3: Get the Full Path

You need the **absolute path** to your binary:

**macOS/Linux:**

```bash
echo "$(pwd)/target/release/git-commit-helper"
```

**Windows PowerShell:**

```powershell
echo "$pwd\target\release\git-commit-helper.exe"
```

📝 **Copy this path** - you'll need it in the next step!

### Step 4: Configure Claude Desktop

Open your Claude Desktop config file:

**macOS:**

```bash
open ~/Library/Application\ Support/Claude/claude_desktop_config.json
```

**Windows:**

```powershell
notepad %APPDATA%\Claude\claude_desktop_config.json
```

**Linux:**

```bash
nano ~/.config/Claude/claude_desktop_config.json
```

Add this configuration (replace with your path from Step 3):

```json
{
  "mcpServers": {
    "git-commit-helper": {
      "command": "/your/absolute/path/to/git-commit-helper"
    }
  }
}
```

**Example for macOS:**

```json
{
  "mcpServers": {
    "git-commit-helper": {
      "command": "/Users/alice/git-commit-helper/target/release/git-commit-helper"
    }
  }
}
```

**Example for Windows:**

```json
{
  "mcpServers": {
    "git-commit-helper": {
      "command": "C:\\Users\\Alice\\git-commit-helper\\target\\release\\git-commit-helper.exe"
    }
  }
}
```

### Step 5: Restart Claude Desktop

**Important:** Completely quit and restart Claude Desktop:

- **macOS:** Press `Cmd+Q` then reopen
- **Windows:** Right-click taskbar icon → Exit, then reopen
- **Linux:** Quit completely then reopen

## ✅ Test It Out

Open Claude Desktop and try these queries:

1. **Navigate to a git repository:**

   ```
   cd /path/to/your/git/project
   ```

2. **Ask Claude:**
   - "What files have I changed?"
   - "Show me the last 5 commits"
   - "I've staged some changes, help me write a commit message"

## 🎯 Example Usage

```
You: I'm in my project directory and I've made some changes. 
     Can you show me what I've modified and help me write a commit message?

Claude: Let me check your changes.
[Uses git_status and git_diff_staged]

I see you've modified app.py to add input validation and updated tests. 
Here's a suggested commit:

feat(validation): add input validation for user forms

- Add email format validation
- Add phone number format checking
- Update tests to cover new validation logic
- Improve error messages for invalid input

Would you like me to adjust this?
```

## 🐛 Common Issues

### macOS: "Cannot be opened because developer cannot be verified"

```bash
xattr -d com.apple.quarantine /path/to/git-commit-helper
```

### Linux: "Permission denied"

```bash
chmod +x /path/to/git-commit-helper
```

### Claude doesn't see the server

1. Check you used the **absolute path** (not `./git-commit-helper`)
2. Make sure you **completely quit** Claude Desktop (not just closed)
3. Verify the config file location is correct for your OS

### "Not a git repository"

Make sure you're in a directory that has been initialized with git:

```bash
git init
# or clone an existing repo
git clone https://github.com/user/repo.git
```

## 📚 More Information

For detailed documentation, see:

- **[README.md](README.md)** - Full documentation
- **[RELEASE.md](RELEASE.md)** - Creating releases
- **[PUBLISHING.md](PUBLISHING.md)** - Publishing to stores

## 💡 Pro Tips

1. **Stay in your project directory** - The tools work in your current directory
2. **Stage changes first** - Use `git add` before asking for commit messages
3. **Ask for style matching** - "Look at recent commits and match that style"
4. **Be specific** - "Write a conventional commit message for my API changes"

---

**You're ready! 🎉** Start chatting with Claude about your git repositories!
