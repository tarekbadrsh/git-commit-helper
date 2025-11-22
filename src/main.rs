use anyhow::{Context, Result};
use rmcp::{
    handler::server::{tool::ToolRouter, wrapper::Parameters},
    model::*,
    serve_server,
    tool,
    tool_router,
    ErrorData as McpError,
    ServerHandler,
};
use serde::{Deserialize, Serialize};
use std::process::Command;

/// Git Commit Helper - MCP server for helping Claude write commit messages
#[derive(Debug, Clone)]
struct GitCommitHelper {
    tool_router: ToolRouter<Self>,
}

impl GitCommitHelper {
    fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }
}

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
                    Err(
                        "Not a git repository. Make sure you're in a git repository directory."
                            .to_string(),
                    )
                } else if error.trim().is_empty() {
                    Err("Git command failed with no error message".to_string())
                } else {
                    Err(error)
                }
            }
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                Err(
                    "Git is not installed or not found in PATH. Please install git first."
                        .to_string(),
                )
            } else {
                Err(format!("Failed to execute git command: {}", e))
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ::schemars::JsonSchema)]
struct GitStatusParams {
    /// Path to the git repository (optional, defaults to current directory)
    #[serde(skip_serializing_if = "Option::is_none")]
    repo_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ::schemars::JsonSchema)]
struct GitDiffStagedParams {
    /// Path to the git repository (optional, defaults to current directory)
    #[serde(skip_serializing_if = "Option::is_none")]
    repo_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ::schemars::JsonSchema)]
struct GitDiffAllParams {
    /// Path to the git repository (optional, defaults to current directory)
    #[serde(skip_serializing_if = "Option::is_none")]
    repo_path: Option<String>,
    /// Include untracked files in the output (default: false)
    #[serde(skip_serializing_if = "Option::is_none")]
    include_untracked: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, ::schemars::JsonSchema)]
struct GitLogParams {
    /// Path to the git repository (optional, defaults to current directory)
    #[serde(skip_serializing_if = "Option::is_none")]
    repo_path: Option<String>,
    /// Maximum number of commits to show (default: 10, max: 50)
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i32>,
}

#[tool_router]
impl GitCommitHelper {
    /// Get git status
    #[tool(
        name = "git_status",
        description = "Shows the current state of a git repository including modified files, staged files, and untracked files. Use this when you need to see what files have changed or when asked questions like 'Show me what files I've changed' or 'What's the current status of my repo?'"
    )]
    async fn git_status(
        &self,
        params: Parameters<GitStatusParams>,
    ) -> Result<CallToolResult, McpError> {
        match run_git_command(&["status"], params.0.repo_path.as_deref()) {
            Ok(output) => Ok(CallToolResult::success(vec![Content::text(output)])),
            Err(error) => Ok(CallToolResult::error(vec![Content::text(error)])),
        }
    }

    /// Get staged changes (what will be committed)
    #[tool(
        name = "git_diff_staged",
        description = "Shows the line-by-line changes for files that have been staged with 'git add'. This is what will be included in the next commit. Particularly useful when generating commit messages. Use this when asked 'Show me what I'm about to commit' or 'Generate a commit message for my staged changes'."
    )]
    async fn git_diff_staged(
        &self,
        params: Parameters<GitDiffStagedParams>,
    ) -> Result<CallToolResult, McpError> {
        match run_git_command(&["diff", "--cached"], params.0.repo_path.as_deref()) {
            Ok(result) => {
                let text = if result.trim().is_empty() {
                    "No staged changes found. Use 'git add' to stage changes first.".to_string()
                } else {
                    result
                };
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            Err(error) => Ok(CallToolResult::error(vec![Content::text(error)])),
        }
    }

    /// Get all changes (both staged and unstaged)
    #[tool(
        name = "git_diff_all",
        description = "Shows all changes in the repository, including both staged and unstaged modifications. Optionally includes a list of untracked files. Use this when you need to see everything that's changed, not just what's staged. Users might ask 'Show me all my changes' or 'What have I modified in this repository?'"
    )]
    async fn git_diff_all(
        &self,
        params: Parameters<GitDiffAllParams>,
    ) -> Result<CallToolResult, McpError> {
        match run_git_command(&["diff", "HEAD"], params.0.repo_path.as_deref()) {
            Ok(mut result) => {
                // If include_untracked is true, append untracked files
                if params.0.include_untracked.unwrap_or(false) {
                    if let Ok(untracked) = run_git_command(
                        &["ls-files", "--others", "--exclude-standard"],
                        params.0.repo_path.as_deref(),
                    ) {
                        if !untracked.trim().is_empty() {
                            result.push_str("\n\n--- Untracked files ---\n");
                            result.push_str(&untracked);
                        }
                    }
                }

                let text = if result.trim().is_empty() {
                    "No changes found in the repository.".to_string()
                } else {
                    result
                };
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            Err(error) => Ok(CallToolResult::error(vec![Content::text(error)])),
        }
    }

    /// Get recent commit history
    #[tool(
        name = "git_log",
        description = "Shows recent commit history so you can understand the project's commit message style and conventions. Formatted as: 'hash - author, time : message'. Use this when asked 'Show me recent commits for context' or 'What's the commit message style in this project?'"
    )]
    async fn git_log(&self, params: Parameters<GitLogParams>) -> Result<CallToolResult, McpError> {
        // Validate and clamp limit
        let limit = params.0.limit.unwrap_or(10).clamp(1, 50);
        let limit_str = limit.to_string();

        match run_git_command(
            &[
                "log",
                &format!("-{}", limit_str),
                "--pretty=format:%h - %an, %ar : %s",
            ],
            params.0.repo_path.as_deref(),
        ) {
            Ok(result) => {
                let text = if result.trim().is_empty() {
                    "No commits found in this repository.".to_string()
                } else {
                    result
                };
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            Err(error) => Ok(CallToolResult::error(vec![Content::text(error)])),
        }
    }
}

// Implement ServerHandler trait
impl ServerHandler for GitCommitHelper {}

#[tokio::main]
async fn main() -> Result<()> {
    // Log startup message to stderr (not stdout, which is used for MCP protocol)
    eprintln!("Git Commit Helper MCP Server v1.0.0 starting...");

    // Create the server instance
    let server = GitCommitHelper::new();

    // Set up stdio transport
    let transport = (tokio::io::stdin(), tokio::io::stdout());

    // Start the server
    eprintln!("Server ready and listening on stdio");
    serve_server(server, transport)
        .await
        .context("Failed to start MCP server")?;

    Ok(())
}
