use crate::error::{JulesError, Result};
use crate::models::*;
use futures_util::{StreamExt, stream::Stream};
use reqwest::{Client, Method, RequestBuilder};
use serde::Deserialize;
use std::pin::Pin;
use url::Url;

pub struct JulesClient {
    http: Client,
    base_url: Url,
    token: String,
}

impl JulesClient {
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
            .bearer_auth(&self.token)
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

    pub async fn create_session(&self, session: &Session) -> Result<Session> {
        let rb = self.request(Method::POST, "sessions").json(session);
        self.execute(rb).await
    }

    pub async fn get_session(&self, name: &str) -> Result<Session> {
        self.execute(self.request(Method::GET, name)).await
    }

    pub async fn delete_session(&self, name: &str) -> Result<()> {
        let _: Empty = self.execute(self.request(Method::DELETE, name)).await?;
        Ok(())
    }

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

    pub async fn approve_plan(&self, session_name: &str) -> Result<()> {
        let path = format!("{}:approvePlan", session_name);
        let body = ApprovePlanRequest {};
        let _: Empty = self
            .execute(self.request(Method::POST, &path).json(&body))
            .await?;
        Ok(())
    }

    // --- Activities API ---

    pub async fn get_activity(&self, name: &str) -> Result<Activity> {
        self.execute(self.request(Method::GET, name)).await
    }

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

    pub async fn get_source(&self, name: &str) -> Result<Source> {
        self.execute(self.request(Method::GET, name)).await
    }

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
