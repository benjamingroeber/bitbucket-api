pub mod members;

use api;
use api::BitBucketQuery;

#[derive(Clone, Debug)]
pub struct TeamsBuilder {
    teamname: String,
}

impl TeamsBuilder {
    pub(crate) fn new(username: &str) -> TeamsBuilder {
        TeamsBuilder {
            teamname: username.to_owned(),
        }
    }
    pub fn members(&self) -> members::MembersBuilder {
        members::MembersBuilder::new(&self.teamname)
    }
}

impl api::GetQueryBuilder for TeamsBuilder {
    type Item = Team;
    fn get_query(&self) -> BitBucketQuery {
        BitBucketQuery::new(format!("teams/{}", self.teamname,))
    }
}

use std::collections::HashMap;
#[derive(Debug, Clone, Deserialize)]
pub struct Team {
    pub username: String,
    pub display_name: String,
    pub uuid: String,
    pub links: HashMap<String, api::Link>,
}
