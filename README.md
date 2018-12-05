# Bitbucket API
This crate provides (incomplete) Rust bindings to the BitBucket API.
The idea is to provide an easy to use, idiomatic Rust API, which makes using the original API easy.

This project covers mainly my personal use cases with BitBucket, andis my first experiment with Rust,
 thus does not (yet :) ) cover all endpoints and should also be treated like a beginner project.

In this first version only some GET requests are possible and filtering is limited, suggestions and 
contributions are very much appreciated.

## Running Tests
As the Tests will actually call the BitBucket API. For now the test_utils module is public such that
doc tests can use it. For running the tests successfully, you need to set the following
environment variables: 

```
TEST_USER="<USERNAME>"
TEST_TEAM="<TEAM>"
TEST_REPO="<REPO>"
TEST_API_KEY="<APP_PASSWORD>"
```
Please note that it is strongly suggested to generate a dedicated app password, and 
**NEVER** use your personal BitBucket account password.
