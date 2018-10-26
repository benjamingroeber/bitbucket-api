# Bitbucket API
This crate provides (incomplete) Rust bindings to the BitBucket API.
The idea is to provide an easy to use, idiomatic Rust API, which makes using the original API easy.

This is my first experiment with Rust, and should also be treated like that.

In this first version only some GET requests are possible and filtering is limited.

## Running Tests
As the Tests will actually call the BitBucket API, you need to set the following environment variables: 

```
TEST_USER="<USERNAME>"
TEST_TEAM="<TEAM>"
TEST_REPO="<REPO>"
TEST_API_KEY="<APP_PASSWORD>"
```
It is suggested to generate a dedicated app password, and never use your BitBucket account password.
