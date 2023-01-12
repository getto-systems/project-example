use crate::common::api::validate::text::{
    check_text_empty, check_text_invalid_email, check_text_too_long,
};

#[test]
fn test_check_text_empty() {
    check_text_empty("text").expect("check");
    check_text_empty("").expect_err("check");
}

#[test]
fn test_check_text_too_long() {
    check_text_too_long(&"a".repeat(10), 10).expect("check");
    check_text_too_long(&"a".repeat(11), 10).expect_err("check");

    check_text_too_long("", 0).expect("check");
}

#[test]
fn test_check_text_invalid_email() {
    check_text_invalid_email("valid-email@example.com").expect("check");
    check_text_invalid_email("invalid-email").expect_err("check");
}
