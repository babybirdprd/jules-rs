//! Data models for the Jules API.
//!
//! This module contains all the types used to represent Jules API objects,
//! including sessions, activities, sources, and their related types.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// The automation mode for a session.
///
/// Controls whether certain actions are performed automatically.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AutomationMode {
    /// No automation (default).
    AutomationModeUnspecified,
    /// Automatically create a pull request when code changes are ready.
    AutoCreatePr,
}

/// The current state of a session.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SessionState {
    /// State is not specified.
    StateUnspecified,
    /// Session is queued and waiting to start.
    Queued,
    /// Agent is generating a plan.
    Planning,
    /// Waiting for user to approve the plan.
    AwaitingPlanApproval,
    /// Waiting for user feedback or input.
    AwaitingUserFeedback,
    /// Agent is actively working on the task.
    InProgress,
    /// Session is paused.
    Paused,
    /// Session failed due to an error.
    Failed,
    /// Session completed successfully.
    Completed,
}

/// A coding session with the Jules agent.
///
/// Sessions represent a contiguous amount of work within the same context.
/// Each session has a prompt describing the task and a source context
/// specifying which repository to work on.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    /// The full resource name (e.g., `sessions/{session}`). Output only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The session ID. Output only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The prompt describing the coding task.
    pub prompt: String,
    /// The source repository and context for this session.
    pub source_context: SourceContext,
    /// Optional title for the session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Whether plan approval is required before the agent starts work.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_plan_approval: Option<bool>,
    /// The automation mode for this session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_mode: Option<AutomationMode>,
    /// When the session was created. Output only.
    #[serde(skip_serializing)]
    pub create_time: Option<DateTime<Utc>>,
    /// When the session was last updated. Output only.
    #[serde(skip_serializing)]
    pub update_time: Option<DateTime<Utc>>,
    /// The current state of the session. Output only.
    #[serde(skip_serializing)]
    pub state: Option<SessionState>,
    /// URL to view the session in the Jules web app. Output only.
    #[serde(skip_serializing)]
    pub url: Option<String>,
    /// Outputs produced by the session (e.g., pull requests). Output only.
    #[serde(skip_serializing)]
    pub outputs: Option<Vec<SessionOutput>>,
}

/// Context for using a source in a session.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SourceContext {
    /// GitHub-specific context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub github_repo_context: Option<GitHubRepoContext>,
    /// The source resource name (e.g., `sources/{source}`).
    pub source: String,
}

/// GitHub repository context for a session.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GitHubRepoContext {
    /// The branch to start the session from.
    pub starting_branch: String,
}

/// An output produced by a session.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SessionOutput {
    /// A pull request created by the session.
    pub pull_request: Option<PullRequest>,
}

/// A pull request created by Jules.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PullRequest {
    /// The URL of the pull request.
    pub url: String,
    /// The title of the pull request.
    pub title: String,
    /// The description/body of the pull request.
    pub description: String,
}

/// An activity within a session.
///
/// Activities represent individual units of work or events that occur
/// during a session, such as messages, plan generation, and progress updates.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    /// The full resource name.
    pub name: String,
    /// The activity ID.
    pub id: String,
    /// Description of this activity.
    pub description: Option<String>,
    /// When the activity was created.
    pub create_time: DateTime<Utc>,
    /// Who originated this activity (user, agent, or system).
    pub originator: String,
    /// The agent posted a message.
    pub agent_messaged: Option<AgentMessaged>,
    /// The user posted a message.
    pub user_messaged: Option<UserMessaged>,
    /// A plan was generated.
    pub plan_generated: Option<PlanGenerated>,
    /// A plan was approved.
    pub plan_approved: Option<PlanApproved>,
    /// Progress was updated.
    pub progress_updated: Option<ProgressUpdated>,
    /// The session completed.
    pub session_completed: Option<serde_json::Value>,
    /// The session failed.
    pub session_failed: Option<SessionFailed>,
    /// Artifacts produced by this activity.
    pub artifacts: Option<Vec<Artifact>>,
}

/// Agent message activity.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AgentMessaged {
    /// The message content.
    pub agent_message: String,
}

/// User message activity.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserMessaged {
    /// The message content.
    pub user_message: String,
}

/// Plan generation activity.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlanGenerated {
    /// The generated plan.
    pub plan: Plan,
}

/// A plan consisting of steps to complete the task.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Plan {
    /// Unique ID for this plan within the session.
    pub id: String,
    /// The steps in the plan.
    pub steps: Vec<PlanStep>,
    /// When the plan was created.
    pub create_time: DateTime<Utc>,
}

/// A step in a plan.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlanStep {
    /// Unique ID for this step within the plan.
    pub id: String,
    /// Title of the step.
    pub title: String,
    /// Description of what this step will do.
    pub description: String,
    /// 0-based index of this step.
    pub index: i32,
}

/// Plan approval activity.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlanApproved {
    /// ID of the approved plan.
    pub plan_id: String,
}

/// Progress update activity.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProgressUpdated {
    /// Title of the progress update.
    pub title: String,
    /// Description of current progress.
    pub description: String,
}

/// Session failure activity.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SessionFailed {
    /// The reason for failure.
    pub reason: String,
}

/// An artifact produced during a session.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Artifact {
    /// A set of code changes.
    pub change_set: Option<ChangeSet>,
    /// A media file (image, video, etc.).
    pub media: Option<Media>,
    /// Output from a bash command.
    pub bash_output: Option<BashOutput>,
}

/// A set of code changes.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChangeSet {
    /// The changes in git patch format.
    pub git_patch: Option<GitPatch>,
    /// The source this applies to.
    pub source: String,
}

/// A git patch representing code changes.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GitPatch {
    /// The patch in unified diff format.
    pub unidiff_patch: String,
    /// The base commit this patch applies to.
    pub base_commit_id: String,
    /// Suggested commit message.
    pub suggested_commit_message: Option<String>,
}

/// A media artifact.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    /// Base64-encoded media data.
    pub data: String,
    /// MIME type of the media.
    pub mime_type: String,
}

/// Output from a bash command.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BashOutput {
    /// The command that was executed.
    pub command: String,
    /// Combined stdout and stderr output.
    pub output: String,
    /// Exit code of the command.
    pub exit_code: i32,
}

/// A source repository.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    /// The full resource name.
    pub name: String,
    /// The source ID.
    pub id: String,
    /// GitHub repository details.
    pub github_repo: Option<GitHubRepo>,
}

/// A GitHub repository.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GitHubRepo {
    /// Repository owner (user or organization).
    pub owner: String,
    /// Repository name.
    pub repo: String,
    /// Whether the repository is private.
    pub is_private: bool,
    /// The default branch.
    pub default_branch: GitHubBranch,
    /// Available branches.
    pub branches: Vec<GitHubBranch>,
}

/// A GitHub branch.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GitHubBranch {
    /// The branch name.
    pub display_name: String,
}

/// Response from listing sessions.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListSessionsResponse {
    /// The sessions.
    pub sessions: Vec<Session>,
    /// Token for the next page, if any.
    pub next_page_token: Option<String>,
}

/// Response from listing activities.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListActivitiesResponse {
    /// The activities.
    pub activities: Vec<Activity>,
    /// Token for the next page, if any.
    pub next_page_token: Option<String>,
}

/// Response from listing sources.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListSourcesResponse {
    /// The sources.
    pub sources: Vec<Source>,
    /// Token for the next page, if any.
    pub next_page_token: Option<String>,
}

/// Request to send a message to a session.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendMessageRequest {
    /// The message prompt to send.
    pub prompt: String,
}

/// Request to approve a plan.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApprovePlanRequest {}

/// Empty response type for operations with no response body.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Empty {}
