use api;
use api::BitBucketQuery;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct UsersBuilder {
    username: String,
}

impl UsersBuilder {
    pub(crate) fn new(username: &str) -> UsersBuilder {
        UsersBuilder {
            username: username.to_owned(),
        }
    }
}

impl api::GetQueryBuilder for UsersBuilder {
    type Item = User;
    fn get_query(&self) -> BitBucketQuery {
        BitBucketQuery::new(format!("users/{}", self.username,))
    }
}

/// BitBucket data structure representing a single User
#[derive(Debug, Clone, Deserialize)]
#[allow(missing_docs)]
pub struct User {
    pub username: String,
    pub nickname: String,
    pub display_name: String,
    pub account_id: Option<String>,
    pub uuid: String,
    pub account_status: Option<String>,
    pub created_on: Option<String>,
    pub is_staff: Option<bool>,
    pub links: HashMap<String, api::Link>,
    pub location: Option<String>,
    pub website: Option<String>,
}
