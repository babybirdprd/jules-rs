//! # jules-rs
//!
//! A production-grade Rust client for the [Jules API](https://jules.google.com).
//!
//! Jules is Google's AI coding agent that can understand, plan, and execute
//! coding tasks on your GitHub repositories.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use jules_rs::{JulesClient, Session, SourceContext, GitHubRepoContext};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a client with your API key from jules.google.com/settings
//!     let client = JulesClient::new("YOUR_API_KEY")?;
//!     
//!     // List all sessions
//!     let response = client.list_sessions(Some(10), None).await?;
//!     for session in response.sessions {
//!         println!("Session: {:?} - {:?}", session.id, session.title);
//!     }
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Features
//!
//! - **Sessions**: Create, list, get, and delete coding sessions
//! - **Activities**: Track session activities and progress updates
//! - **Sources**: List and query connected GitHub repositories
//! - **Streaming**: Paginate through results with async streams
//! - **Type-safe**: Full Rust types for all API models
//!
//! ## Authentication
//!
//! Obtain an API key from [jules.google.com/settings](https://jules.google.com/settings).
//!
//! ## Example: Create a Session
//!
//! ```rust,no_run
//! use jules_rs::{JulesClient, Session, SourceContext, GitHubRepoContext};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = JulesClient::new("YOUR_OAUTH_TOKEN")?;
//!
//! let session = Session {
//!     name: None,
//!     id: None,
//!     prompt: "Fix the bug in the login handler".to_string(),
//!     source_context: SourceContext {
//!         source: "sources/my-repo-id".to_string(),
//!         github_repo_context: Some(GitHubRepoContext {
//!             starting_branch: "main".to_string(),
//!         }),
//!     },
//!     title: Some("Fix login bug".to_string()),
//!     require_plan_approval: Some(true),
//!     automation_mode: None,
//!     create_time: None,
//!     update_time: None,
//!     state: None,
//!     url: None,
//!     outputs: None,
//! };
//!
//! let created = client.create_session(&session).await?;
//! println!("Created session: {}", created.name.unwrap());
//! # Ok(())
//! # }
//! ```
//!
//! ## Example: Stream All Sessions
//!
//! ```rust,no_run
//! use jules_rs::JulesClient;
//! use futures_util::StreamExt;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = JulesClient::new("YOUR_API_KEY")?;
//!
//! let mut stream = client.stream_sessions();
//! while let Some(result) = stream.next().await {
//!     let session = result?;
//!     println!("Session: {:?}", session.title);
//! }
//! # Ok(())
//! # }
//! ```

pub mod client;
pub mod error;
pub mod models;

pub use client::JulesClient;
pub use error::{JulesError, Result};
pub use models::*;
