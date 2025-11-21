use anyhow::{Context, Result};
use rmcp::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::process::Command;

/// Git Commit Helper - MCP server for helping Claude write commit messages
#[derive(Debug, Clone)]
struct GitCommitHelper;

impl GitCommitHelper {
    /// Helper function to execute git commands safely
    fn run_git_command(args: &[&str], repo_path: Option<&str>) -> Result<String, String> {
        let mut cmd = Command::new("git");
        
        // Set working directory if provided
        if let Some(path) = repo_path {
            cmd.current_dir(path);
        }
        
        // Add all arguments
        cmd.args(args);
        
        // Execute the command
        match cmd.output() {
            Ok(output) => {
                if output.status.success() {
                    Ok(String::from_utf8_lossy(&output.stdout).to_string())
                } else {
                    let error = String::from_utf8_lossy(&output.stderr).to_string();
                    
                    // Handle common errors with user-friendly messages
                    if error.contains("not a git repository") {
                        Err("Not a git repository. Make sure you're in a git repository directory.".to_string())
                    } else if error.trim().is_empty() {
                        Err("Git command failed with no error message".to_string())
                    } else {
                        Err(error)
                    }
                }
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    Err("Git is not installed or not found in PATH. Please install git first.".to_string())
                } else {
                    Err(format!("Failed to execute git command: {}", e))
                }
            }
        }
    }

    /// Tool 1: Get git status
    #[tool(
        description = "Shows the current state of a git repository including modified files, staged files, and untracked files. Use this when you need to see what files have changed or when asked questions like 'Show me what files I've changed' or 'What's the current status of my repo?'"
    )]
    async fn git_status(
        #[argument(description = "Path to the git repository (optional, defaults to current directory)")]
        repo_path: Option<String>,
    ) -> Result<String, String> {
        Self::run_git_command(&["status"], repo_path.as_deref())
    }

    /// Tool 2: Get staged changes (what will be committed)
    #[tool(
        description = "Shows the line-by-line changes for files that have been staged with 'git add'. This is what will be included in the next commit. Particularly useful when generating commit messages. Use this when asked 'Show me what I'm about to commit' or 'Generate a commit message for my staged changes'."
    )]
    async fn git_diff_staged(
        #[argument(description = "Path to the git repository (optional, defaults to current directory)")]
        repo_path: Option<String>,
    ) -> Result<String, String> {
        let result = Self::run_git_command(&["diff", "--cached"], repo_path.as_deref())?;
        
        if result.trim().is_empty() {
            Ok("No staged changes found. Use 'git add' to stage changes first.".to_string())
        } else {
            Ok(result)
        }
    }

    /// Tool 3: Get all changes (both staged and unstaged)
    #[tool(
        description = "Shows all changes in the repository, including both staged and unstaged modifications. Optionally includes a list of untracked files. Use this when you need to see everything that's changed, not just what's staged. Users might ask 'Show me all my changes' or 'What have I modified in this repository?'"
    )]
    async fn git_diff_all(
        #[argument(description = "Path to the git repository (optional, defaults to current directory)")]
        repo_path: Option<String>,
        #[argument(description = "Include untracked files in the output (default: false)")]
        include_untracked: Option<bool>,
    ) -> Result<String, String> {
        let mut result = Self::run_git_command(&["diff", "HEAD"], repo_path.as_deref())?;
        
        // If include_untracked is true, append untracked files
        if include_untracked.unwrap_or(false) {
            let untracked = Self::run_git_command(
                &["ls-files", "--others", "--exclude-standard"], 
                repo_path.as_deref()
            )?;
            
            if !untracked.trim().is_empty() {
                result.push_str("\n\n--- Untracked files ---\n");
                result.push_str(&untracked);
            }
        }
        
        if result.trim().is_empty() {
            Ok("No changes found in the repository.".to_string())
        } else {
            Ok(result)
        }
    }

    /// Tool 4: Get recent commit history
    #[tool(
        description = "Shows recent commit history so you can understand the project's commit message style and conventions. Formatted as: 'hash - author, time : message'. Use this when asked 'Show me recent commits for context' or 'What's the commit message style in this project?'"
    )]
    async fn git_log(
        #[argument(description = "Path to the git repository (optional, defaults to current directory)")]
        repo_path: Option<String>,
        #[argument(description = "Maximum number of commits to show (default: 10, max: 50)")]
        limit: Option<i32>,
    ) -> Result<String, String> {
        // Validate and clamp limit
        let limit = limit.unwrap_or(10).clamp(1, 50);
        let limit_str = limit.to_string();
        
        let result = Self::run_git_command(
            &["log", &format!("-{}", limit_str), "--pretty=format:%h - %an, %ar : %s"],
            repo_path.as_deref(),
        )?;
        
        if result.trim().is_empty() {
            Ok("No commits found in this repository.".to_string())
        } else {
            Ok(result)
        }
    }
}

// Register all tools using the tool_box macro
tool_box!(
    GitCommitHelper,
    [git_status, git_diff_staged, git_diff_all, git_log]
);

// Implement the ServerHandler trait
impl ServerHandler for GitCommitHelper {
    async fn server_info(&self) -> ServerInfo {
        ServerInfo {
            name: "git-commit-helper".to_string(),
            version: "1.0.0".to_string(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Log startup message to stderr (not stdout, which is used for MCP protocol)
    eprintln!("Git Commit Helper MCP Server v1.0.0 starting...");
    
    // Create the server instance
    let server = GitCommitHelper;
    
    // Set up stdio transport
    let transport = TokioStdio::new();
    
    // Start the server
    eprintln!("Server ready and listening on stdio");
    server.serve(transport).await
        .context("Failed to start MCP server")?;
    
    Ok(())
}
