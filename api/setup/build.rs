mod environment;
mod grpc;
mod protobuf;

use std::env::var;

fn main() {
    if code_build_required() {
        grpc::generate("auth.ticket");
        grpc::generate("auth.user");
        grpc::generate("auth.user.password");
        grpc::generate("auth.user.password.reset");
        grpc::generate("avail.unexpected_error");
        grpc::generate("example.outline");

        protobuf::generate("auth.ticket");
        protobuf::generate("auth.user.password");
        protobuf::generate("auth.user.password.reset");
        protobuf::generate("avail.unexpected_error");
        protobuf::generate("example.outline");

        environment::generate();
    }
}

fn code_build_required() -> bool {
    match var("RUN_CODE_BUILDER") {
        Ok(build) => build == "TRUE".to_string(),
        Err(_) => false,
    }
}
