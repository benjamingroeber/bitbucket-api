pub mod branchrestrictions;
pub mod pullrequest;
pub mod pullrequests;
pub mod repository;

use api;
use api::BitBucketQuery;

use std::collections::HashMap;
use url::percent_encoding::{utf8_percent_encode, DEFAULT_ENCODE_SET};

#[derive(Clone, Debug)]
pub struct RepositoriesBuilder {
    username: String,
    filter: Option<String>,
}

impl RepositoriesBuilder {
    pub(crate) fn new(username: &str) -> RepositoriesBuilder {
        RepositoriesBuilder {
            username: username.to_owned(),
            filter: None,
        }
    }

    pub fn repo_slug(&self, repo_slug: &str) -> repository::RepositoryBuilder {
        repository::RepositoryBuilder::new(&self.username, repo_slug)
    }

    pub fn repo(&self, repo: &Repository) -> repository::RepositoryBuilder {
        repository::RepositoryBuilder::new(&self.username, &repo.slug)
    }

    pub fn filter(&mut self, filter: &str) -> &mut RepositoriesBuilder {
        self.filter = Some(filter.to_owned());
        self
    }
}

impl api::GetQueryBuilder for RepositoriesBuilder {
    type Item = Repository;
    fn get_query(&self) -> BitBucketQuery {
        let url_path = if let Some(ref filter) = self.filter {
            let filter = format!(r#"name~"{}""#, filter);
            let iter = utf8_percent_encode(&filter, DEFAULT_ENCODE_SET);
            let encoded: String = iter.collect();
            format!(r#"repositories/{}?pagelen=75&q={}"#, self.username, encoded)
        } else {
            format!("repositories/{}?pagelen=75", self.username,)
        };
        BitBucketQuery::new(url_path)
    }
}

/// BitBucket data structure representing a single Repository
#[derive(Debug, Clone, Deserialize)]
#[allow(missing_docs)]
pub struct Repository {
    pub name: String,
    pub full_name: String,
    pub description: String,
    pub uuid: String,
    pub links: HashMap<String, api::Link>,
    pub slug: String,
    pub is_private: bool,
}
