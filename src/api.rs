//! The api module provides the general traits and serves as main interaction point with BitBucket.

use curl::easy::Easy;
use serde;
use serde_json;
use std::collections::HashMap;
use std::fmt;

use pullrequests::PullrequestsBuilder;
use repositories::RepositoriesBuilder;
use std::error;
use teams::TeamsBuilder;
use users::UsersBuilder;

/// The GetQueryBuilder Trait defines the datastructure returned when passed to API::get.
/// Implementing it requires also the definition of the exact Rest Endpoint Called by the Query.
pub trait GetQueryBuilder {
    /// The associated type Item defines the data structure returned by the BitBucketQuery
    type Item: serde::de::DeserializeOwned + fmt::Debug;
    /// The get_query method defines the exact Rest API Endpoint to be called
    /// relative to the API base url https://api.bitbucket.org/v2/
    fn get_query(&self) -> BitBucketQuery;
}

/// Api is used to handle authentication. This part may be subject to change very soon.
#[derive(Debug, Clone)]
pub struct Api {
    user: String,
    key: String,
}

impl Api {
    /// Get a new API Object by passing Username and Bitbucket App Password.
    /// Please do NOT use your account password.
    pub fn new(user: &str, key: &str) -> Api {
        Api {
            user: user.to_owned(),
            key: key.to_owned(),
        }
    }
    /// Operations on Repositories Endpoint
    pub fn repositories(&self, username: &str) -> RepositoriesBuilder {
        RepositoriesBuilder::new(username)
    }
    /// Operations on Pullrequests Endpoint
    pub fn pullrequests(&self, username: &str) -> PullrequestsBuilder {
        PullrequestsBuilder::new(username)
    }
    /// Operations on User Endpoint
    pub fn user(&self, username: &str) -> UsersBuilder {
        UsersBuilder::new(username)
    }
    /// Operations on Team Endpoint
    pub fn team(&self, teamname: &str) -> TeamsBuilder {
        TeamsBuilder::new(teamname)
    }

    /// This is the main Interface for GET requests between Rust Code and the BitBucket API.
    /// ```
    /// use bitbucket_api::api::Api;
    ///
    /// # use bitbucket_api::test_utils;
    /// # let env = test_utils::get_test_env();
    ///
    /// let user = env.user;
    /// let api_key = env.api_key;
    ///
    /// let api = Api::new(&user, &api_key);
    /// let query = api.user(&user);
    ///
    /// let user = api.get(&query);
    /// assert!(user.is_ok());
    /// ```
    pub fn get<T>(&self, query: &GetQueryBuilder<Item = T>) -> Result<Vec<T>, Box<error::Error>>
    where
        T: serde::de::DeserializeOwned + fmt::Debug,
    {
        let query = query.get_query();
        let data = self.get_curl_data(&query)?;
        debug!("{}", String::from_utf8_lossy(&data));
        let content: BitBucketResponse<T> = serde_json::from_slice(&data)?;

        let data = match content {
            BitBucketResponse::Item(content) => vec![content],
            BitBucketResponse::Paged { values, next, .. } => {
                let mut v = values;
                self.append_paged_data(&mut v, next.to_owned())?;
                v
            }
        };
        Ok(data)
    }

    fn append_paged_data<T>(
        &self,
        data: &mut Vec<T>,
        next: Option<String>,
    ) -> Result<(), Box<error::Error>>
    where
        T: serde::de::DeserializeOwned,
    {
        let mut next_url = next;
        while let Some(url) = next_url {
            let request = BitBucketQuery::new(url.to_owned());
            let mut raw_data = self.get_curl_data(&request)?;
            debug!("{}\n", String::from_utf8_lossy(&raw_data));

            let paged_content: BitBucketResponse<T> = serde_json::from_slice(&raw_data)?;
            match paged_content {
                BitBucketResponse::Paged {
                    mut values, next, ..
                } => {
                    data.append(&mut values);
                    next_url = next.to_owned()
                }
                BitBucketResponse::Item(_) => panic!("Single value on next of multi value query"),
            }
        }
        Ok(())
    }

    fn get_curl_data(&self, query: &BitBucketQuery) -> Result<Vec<u8>, Box<error::Error>> {
        let url = query.get_url();
        info!("CALLING {}\n", url);
        let mut handle = self.get_curl_handle(&url)?;
        let mut buf = Vec::new();
        fetch_curl_data(&mut handle, &mut buf)?;
        Ok(buf)
    }

    fn get_curl_handle(&self, url: &str) -> Result<Easy, Box<error::Error>> {
        let mut handle = Easy::new();
        handle.accept_encoding("application/json")?;
        handle.username(&self.user)?;
        handle.password(&self.key)?;
        handle.url(url)?;
        Ok(handle)
    }
}

fn fetch_curl_data(handle: &mut Easy, buf: &mut Vec<u8>) -> Result<(), Box<error::Error>> {
    let mut transfer = handle.transfer();
    transfer.write_function(|data| {
        buf.extend_from_slice(data);
        Ok(data.len())
    })?;
    transfer.perform()?;
    Ok(())
}

/// This is a simple structure for constructing a Rest API Endpoint URL/URI in String Format.
#[derive(Debug, Clone)]
pub struct BitBucketQuery {
    url_path: String,
}

impl BitBucketQuery {
    pub(crate) fn new(url_path: String) -> BitBucketQuery {
        BitBucketQuery { url_path }
    }

    /// Appends relative URLs to the API base URL, or returns absolute URLs as is.
    pub fn get_url(&self) -> String {
        if self.url_path.starts_with("http://") || self.url_path.starts_with("https://") {
            self.url_path.to_owned()
        } else {
            format!("https://api.bitbucket.org/2.0/{}", self.url_path)
        }
    }
}

/// This is the main wrapper type for (successful) BitBucket API Responses.
/// For now it handles either single Item Responses, as well as Paged Multi-Item Responses.
/// This will most likely be subject to change very soon.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub(crate) enum BitBucketResponse<T> {
    /// The Paged enum variant corresponds to a paged BitBucket API Response.
    /// page and size are currently unused values. All values are gathered following the
    /// next value which contains the URL to the next page or null.
    Paged {
        /// Number of current page
        page: Option<usize>,
        /// Number of Items on current page
        pagelen: Option<usize>,
        /// Total number of Items in the Query
        size: Option<usize>,
        /// Values
        values: Vec<T>,
        /// URL to the next page containing the rest of the Items or nothing.
        next: Option<String>,
    },
    /// The Item enum variant corresponds to a single Item BitBucket API Response.
    Item(T),
}

/*
 * Common BitBucket JSON structures
 */

// Reexport Specific DS
pub use pullrequests::PullRequest;
pub use repositories::branchrestrictions::BranchPermission;
pub use repositories::Repository;
pub use users::User;

/// BitBucket data structure representing multiple kinds of Links
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Link {
    /// Some Links have multiple values e.g. Repository Clone has HTTPS and SSH.
    Multi(Vec<Link>),
    /// Single URL with optional name
    #[allow(missing_docs)]
    Link { href: String, name: Option<String> },
}

/// Branch defined by its name
#[derive(Debug, Clone, Deserialize)]
pub struct Branch {
    /// Name of the Branch
    pub name: String,
}

/// Commit hash with Links
#[derive(Debug, Clone, Deserialize)]
pub struct Commit {
    /// Digest representing a specific commit commit
    pub hash: String,
    /// Various Links related to a specific commit
    pub links: HashMap<String, Link>,
}

/// Represents PullRequest data structure, Pointing to Source/Destination Branch,Commit,Repo
/// This will probably be moved into pullrequests
#[derive(Debug, Clone, Deserialize)]
#[allow(missing_docs)]
pub struct Sourctination {
    pub branch: Branch,
    pub commit: Commit,
    pub repository: serde_json::Value,
}
