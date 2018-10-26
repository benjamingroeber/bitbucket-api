use api;
use api::BitBucketQuery;
use users::User;

#[derive(Clone, Debug)]
pub struct MembersBuilder {
    teamname: String,
}

impl MembersBuilder {
    pub(crate) fn new(username: &str) -> MembersBuilder {
        MembersBuilder {
            teamname: username.to_owned(),
        }
    }
}

impl api::GetQueryBuilder for MembersBuilder {
    type Item = User;
    fn get_query(&self) -> BitBucketQuery {
        BitBucketQuery::new(format!("teams/{}/members", self.teamname,))
    }
}
