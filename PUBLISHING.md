# Publishing Guide

This guide explains how to publish Git Commit Helper to various MCP server directories and stores so users can discover it.

## 📋 Pre-Publishing Checklist

Before submitting to any directory, ensure:

- [ ] GitHub repository is **public**
- [ ] README.md is complete with clear installation instructions
- [ ] LICENSE file exists (MIT recommended)
- [ ] At least one release with compiled binaries attached
- [ ] CHANGELOG.md is up to date
- [ ] All four tools are documented with examples
- [ ] Repository has appropriate tags (git, mcp, claude, commit-message)
- [ ] Repository description is clear and concise
- [ ] Working examples are included in README
- [ ] Mentions it uses the official Anthropic Rust SDK

## 🌐 MCP Directories

Here are the major MCP server directories where you should publish:

### 1. Official Anthropic MCP Servers Repository

**URL:** <https://github.com/modelcontextprotocol/servers>

**Type:** GitHub Pull Request

**Authority:** ⭐⭐⭐⭐⭐ (Official Anthropic repository)

**Process:**

1. Fork the repository
2. Add your server to the appropriate section in `README.md`
3. Follow their format:

   ```markdown
   ### Git Commit Helper
   An MCP server that helps Claude write professional commit messages by 
   reading git repository information. Built with the official Rust SDK.
   
   - [GitHub](https://github.com/tarekbadrsh/git-commit-helper)
   - Transport: stdio
   - Language: Rust
   - Features: git status, diff, log analysis, commit message generation
   ```

4. Create a pull request with description:

   ```
   Add Git Commit Helper - Rust-based git commit assistant
   
   This MCP server helps Claude write professional commit messages by 
   analyzing git repository state, diffs, and history. Built with the 
   official Anthropic Rust SDK (rmcp).
   
   Features:
   - Standalone binary (no runtime dependencies)
   - Cross-platform (Linux, macOS, Windows)
   - Fast startup (<1ms)
   - Four git tools for comprehensive repository analysis
   ```

**Tips:**

- Keep description concise but informative
- Highlight that it uses the official SDK
- Mention unique features (Rust, standalone binary)
- Link to your releases with compiled binaries

### 2. MCP Store

**URL:** <https://mcpstore.site>

**Type:** Web Form Submission

**Authority:** ⭐⭐⭐⭐ (Popular community directory)

**Process:**

1. Visit <https://mcpstore.site/submit>
2. Fill out the submission form:
   - **Server Name:** Git Commit Helper
   - **Description:** MCP server that helps Claude write professional commit messages by reading git repository information
   - **GitHub URL:** <https://github.com/tarekbadrsh/git-commit-helper>
   - **Category:** Development Tools
   - **Tags:** git, commit-message, rust, developer-tools
   - **Transport:** stdio
   - **Language:** Rust
   - **Dependencies:** None (standalone binary)
3. Submit and wait for approval

**Required Information:**

- Clear, concise description (under 200 characters)
- Valid GitHub repository URL
- At least one release
- Working installation instructions

### 3. MCP Server Finder

**URL:** <https://mcpserverfinder.com>

**Type:** GitHub PR or Direct Submission

**Authority:** ⭐⭐⭐⭐ (Comprehensive search engine)

**Process:**

1. Check if submission form exists at <https://mcpserverfinder.com/submit>
2. If form exists, fill out:
   - Server name
   - Repository URL
   - Description
   - Categories
   - Features
3. If no form, create GitHub issue or PR in their repository

**Optimization Tips:**

- Use keywords users might search: "git", "commit", "message", "helper"
- Include "Rust" and "standalone" to highlight unique features
- Mention "official SDK" for credibility

### 4. MCPSERVER.WORK

**URL:** <https://mcpserver.work>

**Type:** Automated Indexing / Manual Submission

**Authority:** ⭐⭐⭐ (Automated directory)

**Process:**

1. This site may automatically index GitHub repositories with "mcp" topic
2. Ensure your repository has these GitHub topics:
   - `mcp`
   - `mcp-server`
   - `model-context-protocol`
   - `claude`
   - `git`
   - `rust`
3. If not auto-indexed, look for submission form on site

**How to add GitHub topics:**

1. Go to your GitHub repository
2. Click "About" section (gear icon)
3. Add topics in "Topics" field

## 📝 Writing Effective Descriptions

### Short Description (Under 200 characters)

```
MCP server that helps Claude write professional commit messages by analyzing 
git repositories. Standalone Rust binary with zero dependencies.
```

### Long Description (For README/Detailed Listings)

```
Git Commit Helper is an MCP (Model Context Protocol) server that enables Claude 
to help you write professional, consistent commit messages by reading and 
analyzing your git repository.

Built with the official Anthropic Rust MCP SDK, it compiles to a standalone 
binary with zero runtime dependencies, making it incredibly easy to install 
and use across platforms.

Features:
• Four comprehensive git tools (status, diff staged, diff all, log)
• Cross-platform support (Linux, macOS Intel/ARM, Windows)
• Instant startup (under 1 millisecond)
• Privacy-first (runs entirely locally)
• Tiny 3MB binary size
• Smart commit message generation
• Official Anthropic SDK for reliability

Perfect for:
- Developers wanting consistent commit messages
- Teams maintaining commit standards
- Open source contributors
- Code reviewers needing quick change understanding
- Anyone learning good git practices
```

## 🏷️ Recommended Tags and Keywords

Use these across all platforms:

**Primary Tags:**

- git
- commit-message
- mcp
- claude
- rust
- developer-tools

**Secondary Tags:**

- version-control
- ai-assistant
- cli-tool
- productivity
- code-review
- devops

**Keywords for Search Optimization:**

- git commit helper
- commit message generator
- mcp server
- claude ai
- rust mcp
- git analysis
- commit automation
- developer productivity

## 🎯 Target Audience

When writing descriptions, keep these audiences in mind:

1. **Solo Developers:** Looking for consistent commit messages
2. **Development Teams:** Need commit message standards
3. **Open Source Contributors:** Want professional commits
4. **Code Reviewers:** Need to understand changes quickly
5. **Git Learners:** Want to see good commit examples

## 📸 Screenshots and Demos

If directories support images, include:

1. **Terminal screenshot showing:**
   - Claude asking about changes
   - Git Commit Helper tools being used
   - Generated commit message

2. **Configuration example:**
   - Claude Desktop config JSON
   - Clear file paths

3. **Demo conversation:**
   - Natural language query
   - Tool usage
   - Final output

## 🔗 Essential Links to Include

Always provide these links when submitting:

- **GitHub Repository:** Main project page
- **Latest Release:** Direct link to download binaries
- **Quick Start Guide:** QUICKSTART.md
- **Full Documentation:** README.md
- **Issue Tracker:** For bug reports and feature requests

Example format:

```markdown
- 🏠 [GitHub Repository](https://github.com/tarekbadrsh/git-commit-helper)
- 📦 [Download Latest Release](https://github.com/tarekbadrsh/git-commit-helper/releases/latest)
- ⚡ [Quick Start (3 min)](https://github.com/tarekbadrsh/git-commit-helper/blob/main/QUICKSTART.md)
- 📖 [Full Documentation](https://github.com/tarekbadrsh/git-commit-helper/blob/main/README.md)
- 🐛 [Report Issues](https://github.com/tarekbadrsh/git-commit-helper/issues)
```

## ✅ Submission Checklist

Use this checklist when submitting to each directory:

- [ ] Repository is public and accessible
- [ ] README.md has clear installation instructions
- [ ] At least one release with binaries exists
- [ ] LICENSE file is present
- [ ] Repository has descriptive topics/tags
- [ ] Repository description is clear
- [ ] CHANGELOG.md is up to date
- [ ] All tools documented with examples
- [ ] Working configuration example provided
- [ ] Troubleshooting section included
- [ ] Mentions official Anthropic SDK
- [ ] Cross-platform support highlighted
- [ ] Binary installation advantages mentioned

## 📢 Post-Publishing Promotion

After publishing to directories:

### 1. Social Media

- Post on relevant subreddits (r/rust, r/git, r/programming)
- Share on Twitter/X with relevant hashtags
- Post in Discord communities (Anthropic, Rust)

### 2. Developer Communities

- Hacker News "Show HN"
- Dev.to article explaining the project
- LinkedIn post for professional network

### 3. Documentation Sites

- Add to Awesome MCP lists
- Contribute to MCP documentation with examples
- Write blog post about building MCP servers in Rust

### Example Social Post

```
🚀 Built a Git Commit Helper MCP server in Rust!

Helps Claude write professional commit messages by analyzing your git repos. 

✨ Standalone binary (no dependencies)
⚡ <1ms startup
🔒 Runs locally
📦 Only 3MB

Built with Anthropic's official Rust SDK.

GitHub: [your-link]
```

## 🔄 Keeping Listings Updated

After initial publication:

1. **Update directories when:**
   - New major version released
   - Significant features added
   - Installation process changes
   - Requirements change

2. **Respond promptly to:**
   - User questions in comments
   - Issues opened on GitHub
   - Pull requests from community

3. **Monitor:**
   - Download statistics
   - Star count trends
   - User feedback
   - Common issues

## 📊 Measuring Success

Track these metrics:

- **GitHub Stars:** Indicates interest
- **Download Count:** Shows actual usage
- **Issues/Questions:** Reveals pain points
- **Pull Requests:** Community contribution
- **Directory Rankings:** Visibility in searches

## 🤝 Community Engagement

Best practices:

1. **Respond quickly** to issues and PRs
2. **Thank contributors** in CHANGELOG.md
3. **Consider feedback** for future versions
4. **Be welcoming** to beginners
5. **Document thoroughly** to reduce support burden

## 🚫 What to Avoid

Don't:

- Spam directories with multiple submissions
- Over-promise capabilities
- Ignore user feedback
- Let documentation become outdated
- Be defensive about criticism
- Make breaking changes without major version bump

## 🎓 Learning from Popular Servers

Study successful MCP servers:

1. Check official Anthropic servers repository
2. Look at highly starred MCP projects
3. Read their documentation style
4. Notice their feature descriptions
5. See how they handle issues

## 📅 Publishing Timeline

Recommended timeline:

**Week 1:**

- Polish documentation
- Create first release
- Test on all platforms

**Week 2:**

- Submit to official Anthropic repository
- Submit to MCP Store
- Submit to MCP Server Finder

**Week 3:**

- Share on social media
- Write blog post
- Engage with early users

**Week 4:**

- Address any issues found
- Update based on feedback
- Plan next version

## 🆘 Getting Help

If you need help with publishing:

1. **Anthropic Discord:** Ask in #mcp channel
2. **GitHub Discussions:** Create discussion in your repo
3. **Stack Overflow:** Tag with `mcp` and `anthropic`
4. **Reddit:** r/ClaudeAI community

## 🏆 Success Story Template

Once published, share your story:

```
Built a Git Commit Helper MCP server - my experience

I created an MCP server that helps Claude write commit messages. 
Here's what I learned:

- Why I built it: [your motivation]
- Technology choices: Rust + official SDK
- Challenges faced: [specific problems]
- How I solved them: [solutions]
- What I learned: [insights]

The server is now published and getting positive feedback!

Key takeaways:
1. [insight 1]
2. [insight 2]
3. [insight 3]

GitHub: [your link]
```

---

**Good luck with your publishing! 🚀**

Remember: The goal is to help users discover and use your tool. Focus on clear
documentation and responsive support, and the community will help spread the word!
