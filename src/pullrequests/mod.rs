use api;

use std::fmt;
use serde_json;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct PullrequestsBuilder {
    username: String,
}

impl PullrequestsBuilder {
    pub(crate) fn new(username: &str) -> PullrequestsBuilder {
        PullrequestsBuilder {
            username: username.to_owned(),
        }
    }
}

impl api::GetQueryBuilder for PullrequestsBuilder {
    type Item = PullRequest;
    fn get_query(&self) -> api::BitBucketQuery {
        let url_path = format!(r#"pullrequests/{}?pagelen=50"#, self.username);
        api::BitBucketQuery::new(url_path)
    }
}

/// BitBucket data structure representing a single Pullrequest
#[derive(Debug, Clone, Deserialize)]
#[allow(missing_docs)]
pub struct PullRequest {
    pub id: u64,
    pub title: String,
    pub state: PullRequestState,
    // TODO structure for PR summary
    pub summary: serde_json::Value,
    pub description: String,
    pub author: api::User,
    pub close_source_branch: bool,
    pub closed_by: serde_json::Value,
    pub comment_count: u32,
    pub created_on: String,
    pub destination: api::Sourctination,
    pub source: api::Sourctination,
    pub links: HashMap<String, api::Link>,
    pub merge_commit: serde_json::Value,
    pub reason: String,
    pub task_count: u32,
    pub updated_on: String,
}

/// BitBucket data structure representing all possible states for a PullRequest
#[derive(Debug, Copy, Clone, Deserialize)]
pub enum PullRequestState {
    #[allow(missing_docs)]
    MERGED,
    #[allow(missing_docs)]
    SUPERSEDED,
    #[allow(missing_docs)]
    OPEN,
    #[allow(missing_docs)]
    DECLINED,
}

impl fmt::Display for PullRequestState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PullRequestState::MERGED => write!(f, "Merged"),
            PullRequestState::SUPERSEDED => write!(f, "Superseeded"),
            PullRequestState::OPEN => write!(f, "Open"),
            PullRequestState::DECLINED => write!(f, "Declined"),
        }
    }
}