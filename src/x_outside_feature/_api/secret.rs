use std::env::var;

pub trait Secret {
    fn load(&self, key: &'static str) -> String;
}

// 環境変数から secret をロードする
// k8s の secret 読み込みで SECRET_ の接頭辞をつけて読み込むことを想定
pub struct EnvSecret {}

impl EnvSecret {
    pub fn new() -> Self {
        Self {}
    }
}

impl Secret for EnvSecret {
    fn load(&self, key: &'static str) -> String {
        var(format!("SECRET_{}", key))
            .expect(format!("secret not specified: SECRET_{}", key).as_str())
    }
}
