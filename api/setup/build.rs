mod protobuf;
mod environment;

fn main() {
    protobuf::generate("auth.auth_ticket");
    protobuf::generate("auth.password");
    protobuf::generate("auth.password.reset");
    environment::generate();
}
