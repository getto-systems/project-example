mod environment;
mod grpc;
mod protobuf;

fn main() {
    grpc::generate("auth.ticket");
    grpc::generate("auth.user");
    grpc::generate("auth.password");
    grpc::generate("auth.password.reset");
    grpc::generate("avail.unexpected_error");
    grpc::generate("example.outline");
    protobuf::generate("auth.ticket");
    protobuf::generate("auth.password");
    protobuf::generate("auth.password.reset");
    protobuf::generate("avail.unexpected_error");
    protobuf::generate("example.outline");
    environment::generate();
}
