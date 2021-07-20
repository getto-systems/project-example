mod environment;
mod grpc;
mod protobuf;

fn main() {
    grpc::generate("auth.auth_ticket");
    grpc::generate("auth.auth_user");
    grpc::generate("auth.password");
    protobuf::generate("auth.auth_ticket");
    protobuf::generate("auth.password");
    protobuf::generate("auth.password.reset");
    environment::generate();
}
