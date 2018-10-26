use api;
use pullrequests::PullRequest;
use repositories::pullrequest;

#[derive(Clone, Debug)]
pub struct PullrequestsBuilder {
    username: String,
    repo_slug: String,
}

impl PullrequestsBuilder {
    pub(crate) fn new(username: &str, repo_slug: &str) -> PullrequestsBuilder {
        PullrequestsBuilder {
            username: username.to_owned(),
            repo_slug: repo_slug.to_owned(),
        }
    }

    pub fn id(&self, id: &str) -> pullrequest::PullrequestBuilder {
        pullrequest::PullrequestBuilder::new(&self.username, &self.repo_slug, id)
    }
}

impl api::GetQueryBuilder for PullrequestsBuilder {
    type Item = PullRequest;
    fn get_query(&self) -> api::BitBucketQuery {
        let url_path = format!(
            r#"repositories/{}/{}/pullrequests?pagelen=50"#,
            self.username, self.repo_slug
        );
        api::BitBucketQuery::new(url_path)
    }
}
