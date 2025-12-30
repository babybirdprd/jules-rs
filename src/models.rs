use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AutomationMode {
    AutomationModeUnspecified,
    AutoCreatePr,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SessionState {
    StateUnspecified,
    Queued,
    Planning,
    AwaitingPlanApproval,
    AwaitingUserFeedback,
    InProgress,
    Paused,
    Failed,
    Completed,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub prompt: String,
    pub source_context: SourceContext,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_plan_approval: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_mode: Option<AutomationMode>,
    #[serde(skip_serializing)]
    pub create_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing)]
    pub update_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing)]
    pub state: Option<SessionState>,
    #[serde(skip_serializing)]
    pub url: Option<String>,
    #[serde(skip_serializing)]
    pub outputs: Option<Vec<SessionOutput>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SourceContext {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub github_repo_context: Option<GitHubRepoContext>,
    pub source: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GitHubRepoContext {
    pub starting_branch: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SessionOutput {
    pub pull_request: Option<PullRequest>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PullRequest {
    pub url: String,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    pub name: String,
    pub id: String,
    pub description: Option<String>,
    pub create_time: DateTime<Utc>,
    pub originator: String,
    pub agent_messaged: Option<AgentMessaged>,
    pub user_messaged: Option<UserMessaged>,
    pub plan_generated: Option<PlanGenerated>,
    pub plan_approved: Option<PlanApproved>,
    pub progress_updated: Option<ProgressUpdated>,
    pub session_completed: Option<serde_json::Value>,
    pub session_failed: Option<SessionFailed>,
    pub artifacts: Option<Vec<Artifact>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AgentMessaged {
    pub agent_message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserMessaged {
    pub user_message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlanGenerated {
    pub plan: Plan,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Plan {
    pub id: String,
    pub steps: Vec<PlanStep>,
    pub create_time: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlanStep {
    pub id: String,
    pub title: String,
    pub description: String,
    pub index: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlanApproved {
    pub plan_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProgressUpdated {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SessionFailed {
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Artifact {
    pub change_set: Option<ChangeSet>,
    pub media: Option<Media>,
    pub bash_output: Option<BashOutput>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChangeSet {
    pub git_patch: Option<GitPatch>,
    pub source: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GitPatch {
    pub unidiff_patch: String,
    pub base_commit_id: String,
    pub suggested_commit_message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    pub data: String, // Base64 encoded
    pub mime_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BashOutput {
    pub command: String,
    pub output: String,
    pub exit_code: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    pub name: String,
    pub id: String,
    pub github_repo: Option<GitHubRepo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GitHubRepo {
    pub owner: String,
    pub repo: String,
    pub is_private: bool,
    pub default_branch: GitHubBranch,
    pub branches: Vec<GitHubBranch>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GitHubBranch {
    pub display_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListSessionsResponse {
    pub sessions: Vec<Session>,
    pub next_page_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListActivitiesResponse {
    pub activities: Vec<Activity>,
    pub next_page_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListSourcesResponse {
    pub sources: Vec<Source>,
    pub next_page_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendMessageRequest {
    pub prompt: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApprovePlanRequest {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Empty {}
