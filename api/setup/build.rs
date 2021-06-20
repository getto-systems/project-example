mod protobuf;
mod environment;

fn main() {
    protobuf::generate("src/auth");
    environment::generate();
}
