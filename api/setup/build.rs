mod environment;
mod grpc;

use std::env::var;

fn main() {
    if code_build_required() {
        grpc::generate("auth.ticket");
        grpc::generate("auth.user");
        grpc::generate("auth.user.account");
        grpc::generate("auth.user.password");
        grpc::generate("auth.user.password.reset");
        grpc::generate("avail.unexpected_error");
        grpc::generate("example.outline");

        environment::generate();
    }
}

fn code_build_required() -> bool {
    match var("RUN_CODE_BUILDER") {
        Ok(build) => build == "TRUE".to_string(),
        Err(_) => false,
    }
}
