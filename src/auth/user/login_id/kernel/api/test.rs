use crate::auth::user::login_id::kernel::data::LoginId;

#[tokio::test]
async fn invalid_login_id() {
    let result = LoginId::convert("a".repeat(100 + 1));
    assert_eq!(format!("{}", result.err().unwrap()), "login-id: too long");
}
