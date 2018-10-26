#[derive(Clone, Debug)]
pub struct PullrequestBuilder {
    _username: String,
    _repo_slug: String,
    _id: String,
}

impl PullrequestBuilder {
    pub fn new(username: &str, repo_slug: &str, id: &str) -> PullrequestBuilder {
        PullrequestBuilder {
            _username: username.to_owned(),
            _repo_slug: repo_slug.to_owned(),
            _id: id.to_owned(),
        }
    }
}
