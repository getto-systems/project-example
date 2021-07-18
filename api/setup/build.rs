mod protobuf;
mod grpc;
mod environment;

fn main() {
    grpc::generate("auth.auth_ticket");
    protobuf::generate("auth.auth_ticket");
    protobuf::generate("auth.password");
    protobuf::generate("auth.password.reset");
    environment::generate();
}
