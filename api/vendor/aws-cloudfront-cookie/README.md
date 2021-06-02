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
    let key = CloudfrontKey::pkcs1_sha512_from_pem(PEM)
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
MIIBPQIBAAJBAOsfi5AGYhdRs/x6q5H7kScxA0Kzzqe6WI6gf6+tc6IvKQJo5rQc
dWWSQ0nRGt2hOPDO+35NKhQEjBQxPh/v7n0CAwEAAQJBAOGaBAyuw0ICyENy5NsO
2gkT00AWTSzM9Zns0HedY31yEabkuFvrMCHjscEF7u3Y6PB7An3IzooBHchsFDei
AAECIQD/JahddzR5K3A6rzTidmAf1PBtqi7296EnWv8WvpfAAQIhAOvowIXZI4Un
DXjgZ9ekuUjZN+GUQRAVlkEEohGLVy59AiEA90VtqDdQuWWpvJX0cM08V10tLXrT
TTGsEtITid1ogAECIQDAaFl90ZgS5cMrL3wCeatVKzVUmuJmB/VAmlLFFGzK0QIh
ANJGc7AFk4fyFD/OezhwGHbWmo/S+bfeAiIh2Ss2FxKJ
-----END RSA PRIVATE KEY-----
";
```

## License

[MIT](LICENSE) license.

Copyright &copy; shun-fix9
