# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2025-01-XX

### Added

- Initial release of jules-rs
- `JulesClient` for authenticated API access
- **Sessions API**
  - `create_session` - Create a new coding session
  - `get_session` - Retrieve session details
  - `list_sessions` - List sessions with pagination
  - `delete_session` - Delete a session
  - `stream_sessions` - Async stream over all sessions
  - `send_message` - Send a message to a session
  - `approve_plan` - Approve a generated plan
- **Activities API**
  - `get_activity` - Get activity details
  - `list_activities` - List session activities
- **Sources API**
  - `get_source` - Get source repository details
  - `list_sources` - List connected repositories
- Comprehensive error handling with `JulesError`
- Full type definitions for all API models
