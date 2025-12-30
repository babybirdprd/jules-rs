//! HTTP client for the Jules API.
//!
//! This module provides the main [`JulesClient`] struct for interacting with
//! the Jules API endpoints.

use crate::error::{JulesError, Result};
use crate::models::*;
use futures_util::{StreamExt, stream::Stream};
use reqwest::{Client, Method, RequestBuilder};
use serde::Deserialize;
use std::pin::Pin;
use url::Url;

/// The main client for interacting with the Jules API.
///
/// `JulesClient` provides methods for all Jules API operations including
/// managing sessions, activities, and sources.
///
/// # Example
///
/// ```rust,no_run
/// use jules_rs::JulesClient;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = JulesClient::new("YOUR_OAUTH_TOKEN")?;
///
/// // List sessions
/// let response = client.list_sessions(Some(10), None).await?;
/// println!("Found {} sessions", response.sessions.len());
/// # Ok(())
/// # }
/// ```
pub struct JulesClient {
    http: Client,
    base_url: Url,
    token: String,
}

impl JulesClient {
    /// Creates a new Jules API client.
    ///
    /// # Arguments
    ///
    /// * `api_key` - An API key from [jules.google.com/settings](https://jules.google.com/settings).
    ///
    /// # Errors
    ///
    /// Returns an error if the base URL cannot be parsed (should not happen
    /// under normal circumstances).
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use jules_rs::JulesClient;
    ///
    /// let client = JulesClient::new("YOUR_API_KEY").unwrap();
    /// ```
    pub fn new(token: impl Into<String>) -> Result<Self> {
        Ok(Self {
            http: Client::new(),
            base_url: Url::parse("https://jules.googleapis.com/v1alpha/")?,
            token: token.into(),
        })
    }

    fn request(&self, method: Method, path: &str) -> RequestBuilder {
        let url = self.base_url.join(path).expect("Path joining failed");
        self.http
            .request(method, url)
            .header("X-Goog-Api-Key", &self.token)
            .header("Accept", "application/json")
    }

    async fn execute<T>(&self, builder: RequestBuilder) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let response = builder.send().await?;
        if !response.status().is_success() {
            let status = response.status();
            let message = response.text().await.unwrap_or_default();
            return Err(JulesError::Api { status, message });
        }
        Ok(response.json().await?)
    }

    // --- Sessions API ---

    /// Creates a new coding session.
    ///
    /// # Arguments
    ///
    /// * `session` - The session configuration including prompt and source context.
    ///
    /// # Returns
    ///
    /// The created session with server-generated fields populated (name, id, etc.).
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use jules_rs::{JulesClient, Session, SourceContext, GitHubRepoContext};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = JulesClient::new("TOKEN")?;
    /// let session = Session {
    ///     prompt: "Fix the bug".to_string(),
    ///     source_context: SourceContext {
    ///         source: "sources/repo-id".to_string(),
    ///         github_repo_context: Some(GitHubRepoContext {
    ///             starting_branch: "main".to_string(),
    ///         }),
    ///     },
    ///     // ... other fields set to None/default
    /// #   name: None, id: None, title: None, require_plan_approval: None,
    /// #   automation_mode: None, create_time: None, update_time: None,
    /// #   state: None, url: None, outputs: None,
    /// };
    /// let created = client.create_session(&session).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_session(&self, session: &Session) -> Result<Session> {
        let rb = self.request(Method::POST, "sessions").json(session);
        self.execute(rb).await
    }

    /// Gets a session by its resource name.
    ///
    /// # Arguments
    ///
    /// * `name` - The full resource name (e.g., `sessions/abc123`).
    pub async fn get_session(&self, name: &str) -> Result<Session> {
        self.execute(self.request(Method::GET, name)).await
    }

    /// Deletes a session.
    ///
    /// # Arguments
    ///
    /// * `name` - The full resource name of the session to delete.
    pub async fn delete_session(&self, name: &str) -> Result<()> {
        let _: Empty = self.execute(self.request(Method::DELETE, name)).await?;
        Ok(())
    }

    /// Lists sessions with pagination.
    ///
    /// # Arguments
    ///
    /// * `page_size` - Maximum number of sessions to return (1-100, default 30).
    /// * `page_token` - Token from a previous response for pagination.
    ///
    /// # Returns
    ///
    /// A response containing sessions and optionally a token for the next page.
    pub async fn list_sessions(
        &self,
        page_size: Option<i32>,
        page_token: Option<String>,
    ) -> Result<ListSessionsResponse> {
        let mut rb = self.request(Method::GET, "sessions");
        if let Some(ps) = page_size {
            rb = rb.query(&[("pageSize", ps)]);
        }
        if let Some(pt) = page_token {
            rb = rb.query(&[("pageToken", pt)]);
        }
        self.execute(rb).await
    }

    /// Returns an async stream over all sessions.
    ///
    /// This method automatically handles pagination, yielding sessions one at
    /// a time until all sessions have been retrieved.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use jules_rs::JulesClient;
    /// use futures_util::StreamExt;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = JulesClient::new("TOKEN")?;
    /// let mut stream = client.stream_sessions();
    ///
    /// while let Some(result) = stream.next().await {
    ///     let session = result?;
    ///     println!("Session: {:?}", session.title);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn stream_sessions(&self) -> Pin<Box<dyn Stream<Item = Result<Session>> + '_>> {
        Box::pin(
            futures_util::stream::unfold(Some("".to_string()), move |state| async move {
                let current_token = state?;
                let token_opt = if current_token.is_empty() {
                    None
                } else {
                    Some(current_token)
                };

                match self.list_sessions(Some(100), token_opt).await {
                    Ok(resp) => {
                        let next_token = resp.next_page_token.clone().unwrap_or_default();
                        let next_state = if next_token.is_empty() {
                            None
                        } else {
                            Some(next_token)
                        };
                        let items: Vec<Result<Session>> =
                            resp.sessions.into_iter().map(Ok).collect();
                        Some((futures_util::stream::iter(items), next_state))
                    }
                    Err(e) => {
                        let items: Vec<Result<Session>> = vec![Err(e)];
                        Some((futures_util::stream::iter(items), None))
                    }
                }
            })
            .flatten(),
        )
    }

    /// Sends a message to an active session.
    ///
    /// Use this to provide additional context or respond to the agent's
    /// questions during a session.
    ///
    /// # Arguments
    ///
    /// * `session_name` - The full resource name of the session.
    /// * `prompt` - The message to send.
    pub async fn send_message(&self, session_name: &str, prompt: &str) -> Result<()> {
        let path = format!("{}:sendMessage", session_name);
        let body = SendMessageRequest {
            prompt: prompt.to_string(),
        };
        let _: Empty = self
            .execute(self.request(Method::POST, &path).json(&body))
            .await?;
        Ok(())
    }

    /// Approves the current plan for a session.
    ///
    /// When a session is in the `AWAITING_PLAN_APPROVAL` state, call this
    /// method to approve the plan and allow the agent to proceed.
    ///
    /// # Arguments
    ///
    /// * `session_name` - The full resource name of the session.
    pub async fn approve_plan(&self, session_name: &str) -> Result<()> {
        let path = format!("{}:approvePlan", session_name);
        let body = ApprovePlanRequest {};
        let _: Empty = self
            .execute(self.request(Method::POST, &path).json(&body))
            .await?;
        Ok(())
    }

    // --- Activities API ---

    /// Gets an activity by its resource name.
    ///
    /// # Arguments
    ///
    /// * `name` - The full resource name (e.g., `sessions/123/activities/456`).
    pub async fn get_activity(&self, name: &str) -> Result<Activity> {
        self.execute(self.request(Method::GET, name)).await
    }

    /// Lists activities for a session with pagination.
    ///
    /// # Arguments
    ///
    /// * `session_name` - The full resource name of the session.
    /// * `page_size` - Maximum number of activities to return.
    /// * `page_token` - Token from a previous response for pagination.
    pub async fn list_activities(
        &self,
        session_name: &str,
        page_size: Option<i32>,
        page_token: Option<String>,
    ) -> Result<ListActivitiesResponse> {
        let path = format!("{}/activities", session_name);
        let mut rb = self.request(Method::GET, &path);
        if let Some(ps) = page_size {
            rb = rb.query(&[("pageSize", ps)]);
        }
        if let Some(pt) = page_token {
            rb = rb.query(&[("pageToken", pt)]);
        }
        self.execute(rb).await
    }

    // --- Sources API ---

    /// Gets a source by its resource name.
    ///
    /// # Arguments
    ///
    /// * `name` - The full resource name (e.g., `sources/abc123`).
    pub async fn get_source(&self, name: &str) -> Result<Source> {
        self.execute(self.request(Method::GET, name)).await
    }

    /// Lists available sources (connected repositories) with pagination.
    ///
    /// # Arguments
    ///
    /// * `filter` - Optional filter expression.
    /// * `page_size` - Maximum number of sources to return.
    /// * `page_token` - Token from a previous response for pagination.
    pub async fn list_sources(
        &self,
        filter: Option<String>,
        page_size: Option<i32>,
        page_token: Option<String>,
    ) -> Result<ListSourcesResponse> {
        let mut rb = self.request(Method::GET, "sources");
        if let Some(f) = filter {
            rb = rb.query(&[("filter", f)]);
        }
        if let Some(ps) = page_size {
            rb = rb.query(&[("pageSize", ps)]);
        }
        if let Some(pt) = page_token {
            rb = rb.query(&[("pageToken", pt)]);
        }
        self.execute(rb).await
    }
}
