mod environment;
mod grpc;

use std::env::var;

fn main() {
    if code_build_required() {
        grpc::generate();

        environment::generate();
    }
}

fn code_build_required() -> bool {
    match var("RUN_CODE_BUILDER") {
        Ok(build) => build == "TRUE".to_string(),
        Err(_) => true,
    }
}
