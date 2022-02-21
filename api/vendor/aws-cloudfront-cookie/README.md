# AWS CloudFront signed-cookie

sign and encode for AWS CloudFront signed-cookie

status: beta

###### Table of Contents

-   [Requirements](#Requirements)
-   [Usage](#Usage)
-   [License](#License)

## Requirements

-   rust: 1.51

## Usage

```rust
use chrono::Utc;
use aws_cloudfront_cookie::{CloudfrontKey, CloudfrontPolicy};

fn sign() {
    let key = CloudfrontKey::from_pem(PEM)
        .expect("failed to parse cloudfront private key");

    let resource = "https://secure.example.com/*";
    let expires = Utc::now();
    let policy = CloudfrontPolicy::from_resource(resource.into(), expires.timestamp());

    let content = key.sign(policy)
        .expect("failed to sign");

    println!("policy: {}", content.policy);
    println!("signature: {}", content.signature);
}

const PEM: &'static str = "-----BEGIN RSA PRIVATE KEY-----
...(private key is here)";
```

## License

[MIT](LICENSE) license.

Copyright &copy; shun-fix9
