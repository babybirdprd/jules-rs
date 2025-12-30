# jules-rs

[![Crates.io](https://img.shields.io/crates/v/jules-rs.svg)](https://crates.io/crates/jules-rs)
[![Documentation](https://docs.rs/jules-rs/badge.svg)](https://docs.rs/jules-rs)
[![CI](https://github.com/babybirdprd/jules-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/babybirdprd/jules-rs/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A production-grade Rust client for the [Jules API](https://jules.google.com).

Jules is Google's AI coding agent that can understand, plan, and execute coding tasks on your GitHub repositories.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
jules-rs = "0.1"
tokio = { version = "1", features = ["full"] }
futures-util = "0.3"
```

## Quick Start

```rust
use jules_rs::{JulesClient, Session, SourceContext, GitHubRepoContext};
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = JulesClient::new("YOUR_OAUTH_TOKEN")?;

    // List all sessions
    let response = client.list_sessions(Some(10), None).await?;
    for session in response.sessions {
        println!("Session: {:?} - {:?}", session.id, session.title);
    }

    Ok(())
}
```

## Features

- **Sessions**: Create, list, get, and delete coding sessions
- **Activities**: Track session activities and progress updates  
- **Sources**: List and query connected GitHub repositories
- **Streaming**: Paginate through results with async streams
- **Type-safe**: Full Rust types for all API models
- **Async/await**: Built on `reqwest` and `futures`

## Examples

### Create a Session

```rust
use jules_rs::{JulesClient, Session, SourceContext, GitHubRepoContext};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = JulesClient::new("YOUR_OAUTH_TOKEN")?;

    let session = Session {
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

    let created = client.create_session(&session).await?;
    println!("Created session: {}", created.name.unwrap());

    Ok(())
}
```

### Stream All Sessions

```rust
use jules_rs::JulesClient;
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = JulesClient::new("YOUR_OAUTH_TOKEN")?;

    let mut stream = client.stream_sessions();
    while let Some(result) = stream.next().await {
        let session = result?;
        println!("Session: {:?} - State: {:?}", session.title, session.state);
    }

    Ok(())
}
```

### Approve a Plan

```rust
use jules_rs::JulesClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = JulesClient::new("YOUR_OAUTH_TOKEN")?;

    // When a session is in AWAITING_PLAN_APPROVAL state
    client.approve_plan("sessions/abc123").await?;
    
    Ok(())
}
```

## Authentication

1. Go to [jules.google.com/settings](https://jules.google.com/settings)
2. Click **Generate API Key** (or copy an existing one)
3. Store your API key securely — it won't be shown again
4. Use the API key with `JulesClient::new(api_key)`

## API Coverage

| Endpoint | Method | Status |
|----------|--------|--------|
| Sessions | Create | ✅ |
| Sessions | Get | ✅ |
| Sessions | List | ✅ |
| Sessions | Delete | ✅ |
| Sessions | Stream | ✅ |
| Sessions | Send Message | ✅ |
| Sessions | Approve Plan | ✅ |
| Activities | Get | ✅ |
| Activities | List | ✅ |
| Sources | Get | ✅ |
| Sources | List | ✅ |

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.