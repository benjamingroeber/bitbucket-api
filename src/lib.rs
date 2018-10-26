#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

//! # bitbucket-api
//!
//! `bitbucket-api` provides rust bindings to the BitBucket Api.
//!
//! The main goals this library are ease of use and expressiveness.
//!
//! Completeness and performance are explicitly not primary goals,
//! but may be reached eventually

/* TODO BEFORE PUBLISH
 * Remove Passwords from Commit History
 * Optional, Make Link Retrieval Function new Type Links = HashMap<String,Link>
*/

extern crate curl;
extern crate serde;
extern crate serde_json;
extern crate url;
#[macro_use]
extern crate log;
extern crate env_logger;
#[macro_use]
extern crate serde_derive;

#[allow(unused_imports)]
#[macro_use]
extern crate lazy_static;

pub mod api;
mod pullrequests;
mod repositories;
mod teams;
mod users;

#[cfg(test)]
mod tests {
    use super::*;
    use env_logger;
    use test_utils::*;

    lazy_static! {
        static ref ENV: TestEnv = {
            env_logger::init();
            get_test_env()
        };
        static ref API: api::Api = { api::Api::new(&ENV.user, &ENV.api_key) };
    }

    #[test]
    fn repositories_query() {
        let mut repos = API.repositories(&ENV.user);

        repos.filter("icinga2");
        let repolist = API.get(&repos);
        //        println!("{:#?}", repolist);

        assert!(repolist.is_ok());
    }

    #[test]
    fn pull_requests_query() {
        let prs = API
            .repositories(&ENV.team)
            .repo_slug(&ENV.repo)
            .pullrequests();
        let prlist = API.get(&prs);
        //        println!("{:#?}", prlist);
        assert!(prlist.is_ok());
    }

    #[test]
    fn branch_restriction_query() {
        let repos = API
            .repositories(&ENV.team)
            .repo_slug(&ENV.repo)
            .branch_restrictions();

        let response = API.get(&repos);

        assert!(response.is_ok());
    }

    #[test]
    fn rnd_repo_branch_restrictions() {
        let repositories = API.repositories(&ENV.team);
        let mut repos = API
            .get(&repositories)
            .expect("Repo list query must not fail");

        let repo = repos.pop().expect("Got no repo from TEST_ACCOUNT");

        let restrictions = API.get(&repositories.repo(&repo).branch_restrictions());
        assert!(
            restrictions.is_ok(),
            "Successfully retrieve branch restrictions from random repo"
        )
    }

    #[test]
    fn repo_slug_query() {
        let repo = API.repositories(&ENV.team).repo_slug(&ENV.repo);
        let repository = API.get(&repo);

        assert!(repository.is_ok())
    }

    #[test]
    fn get_my_prs() {
        let prs = API.pullrequests(&ENV.user);
        let pr_list = API.get(&prs);

        assert!(pr_list.is_ok())
    }

    #[test]
    fn get_user() {
        let user = API.user(&ENV.user);
        let test_user = API.get(&user);

        assert!(test_user.is_ok())
    }

    #[test]
    fn get_team() {
        let team = API.team(&ENV.team);
        let test_team = API.get(&team);

        assert!(test_team.is_ok())
    }

    #[test]
    fn get_members() {
        let members = API.team(&ENV.team).members();
        let team_members: Vec<users::User> = API.get(&members).expect("Get Team Members");
        assert!(team_members.len() > 0);
    }
}

#[allow(missing_docs)]
pub mod test_utils {
    #[derive(Debug, Clone)]
    pub struct TestEnv {
        pub user: String,
        pub team: String,
        pub repo: String,
        pub api_key: String,
    }

    use std::env;
    pub fn get_test_env() -> TestEnv {
        let user = _get_test_env_var("TEST_USER");
        let team = _get_test_env_var("TEST_TEAM");
        let repo = _get_test_env_var("TEST_REPO");
        let api_key = _get_test_env_var("TEST_API_KEY");
        TestEnv {
            user,
            team,
            repo,
            api_key,
        }
    }

    fn _get_test_env_var(key: &str) -> String {
        env::var(key).expect(&format!("Missing ENV {}", key)).into()
    }
}
