use crate::auth::user::password::kernel::infra::PlainPassword;

#[tokio::test]
async fn invalid_password() {
    let result = PlainPassword::convert("a".repeat(100 + 1));
    assert_eq!(format!("{}", result.err().unwrap()), "password: too long");
}
