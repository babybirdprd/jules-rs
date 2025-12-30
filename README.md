# Jules-rs

jules-rs is a Rust client for the Jules API.

## Usage

use jules_rs::{JulesClient, Session, SourceContext, GitHubRepoContext};
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = JulesClient::new("YOUR_OAUTH_TOKEN")?;

    // 1. Create a session
    let new_session = Session {
        name: None,
        id: None,
        prompt: "Refactor the error handling in the main module".to_string(),
        source_context: SourceContext {
            source: "sources/my-repo".to_string(),
            github_repo_context: Some(GitHubRepoContext {
                starting_branch: "main".to_string(),
            }),
        },
        title: Some("Refactor Task".to_string()),
        require_plan_approval: Some(true),
        automation_mode: None,
        create_time: None,
        update_time: None,
        state: None,
        url: None,
        outputs: None,
    };

    let session = client.create_session(&new_session).await?;
    println!("Created session: {}", session.name.unwrap());

    // 2. Stream all sessions
    let mut sessions = client.stream_sessions();
    while let Some(s) = sessions.next().await {
        println!("Found session: {:?}", s?.title);
    }

    Ok(())
}