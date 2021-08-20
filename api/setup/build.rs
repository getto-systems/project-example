mod environment;
mod grpc;
mod protobuf;

fn main() {
    grpc::generate("auth.auth_ticket");
    grpc::generate("auth.auth_user");
    grpc::generate("auth.password");
    grpc::generate("auth.password.reset");
    grpc::generate("outline");
    protobuf::generate("auth.auth_ticket");
    protobuf::generate("auth.password");
    protobuf::generate("auth.password.reset");
    protobuf::generate("avail.unexpected_error");
    protobuf::generate("outline");
    environment::generate();
}
