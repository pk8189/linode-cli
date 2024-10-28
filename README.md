
# Linode API Rust SDK


## Overview
[Read the API documentation](https://techdocs.akamai.com/linode-api/reference/api).


### Example Client Initialization

```rust
let client = linode_api::Client::default()
    .with_oauth(&std::env::var("API_TOKEN").unwrap())
    .with_personal_access_token(&std::env::var("API_TOKEN").unwrap());
```

### SDK Usage 
 See [SDK Examples](SDK_EXAMPLES.md) for example usage of all SDK functionality