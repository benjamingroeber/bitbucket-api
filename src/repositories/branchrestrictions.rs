use api;

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

use serde_json;
use std::collections::HashMap;
#[derive(Debug, Clone, Deserialize)]
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
