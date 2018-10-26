use api;
use repositories::branchrestrictions;
use repositories::pullrequests;

#[derive(Clone, Debug)]
pub struct RepositoryBuilder {
    username: String,
    repo_slug: String,
}

impl RepositoryBuilder {
    pub(crate) fn new(username: &str, repo_slug: &str) -> RepositoryBuilder {
        RepositoryBuilder {
            username: username.to_owned(),
            repo_slug: repo_slug.to_owned(),
        }
    }

    pub fn pullrequests(&self) -> pullrequests::PullrequestsBuilder {
        pullrequests::PullrequestsBuilder::new(&self.username, &self.repo_slug)
    }

    pub fn branch_restrictions(&self) -> branchrestrictions::BranchRestrictionBuilder {
        branchrestrictions::BranchRestrictionBuilder::new(&self.username, &self.repo_slug)
    }
}

use repositories;
impl api::GetQueryBuilder for RepositoryBuilder {
    type Item = repositories::Repository;
    fn get_query(&self) -> api::BitBucketQuery {
        let url_path = format!("repositories/{}/{}", self.username, self.repo_slug,);
        api::BitBucketQuery::new(url_path)
    }
}
