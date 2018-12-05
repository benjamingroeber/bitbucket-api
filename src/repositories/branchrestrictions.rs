use api;

use serde_json;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct BranchRestrictionBuilder {
    username: String,
    repo_slug: String,
}

impl BranchRestrictionBuilder {
    pub fn new(username: &str, repo_slug: &str) -> BranchRestrictionBuilder {
        BranchRestrictionBuilder {
            username: username.to_owned(),
            repo_slug: repo_slug.to_owned(),
        }
    }
}

impl api::GetQueryBuilder for BranchRestrictionBuilder {
    type Item = BranchPermission;
    fn get_query(&self) -> api::BitBucketQuery {
        let url_path = format!(
            "repositories/{}/{}/branch-restrictions",
            self.username, self.repo_slug,
        );
        api::BitBucketQuery::new(url_path)
    }
}

impl api::PostQueryBuilder for BranchRestrictionBuilder {
    type Item = NewBranchPermission;
    fn get_query(&self) -> api::BitBucketQuery {
        let url_path = format!(
            "repositories/{}/{}/branch-restrictions",
            self.username, self.repo_slug,
        );
        api::BitBucketQuery::new(url_path)
    }
}

/// BitBucket data structure representing a single Repository
#[derive(Debug, Clone, Deserialize)]
#[allow(missing_docs)]
pub struct BranchPermission {
    pub id: u64,
    pub kind: String,
    pub links: HashMap<String, api::Link>,
    pub pattern: String,
    #[serde(rename = "type")]
    pub bb_type: String,
    pub users: serde_json::Value,
    pub groups: serde_json::Value,
    pub value: Option<serde_json::Value>,
}

///branch
#[derive(Debug, Clone, Serialize)]
pub struct NewBranchPermission {
    pub id: u64,
    pub kind: String,
    //pub links: HashMap<String, api::Link>,
    pub pattern: String,
    #[serde(rename = "type")]
    pub bb_type: String,
    pub users: serde_json::Value, // ??
    pub groups: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum Kind {
    require_tasks_to_be_completed,
    require_passing_builds_to_merge,
    force,
    require_all_dependencies_merged,
    push,
    require_approvals_to_merge,
    enforce_merge_checks,
    restrict_merges,
    reset_pullrequest_approvals_on_change,
    delete
}