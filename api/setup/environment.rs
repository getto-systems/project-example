use std::{
    fs::{read_to_string, File},
    io::Write,
    path::PathBuf,
};

pub fn generate() {
    build().expect("failed to build environment");
}

fn build() -> std::io::Result<()> {
    let version = read_to_string("api/VERSION")?;

    let mut file = File::create(PathBuf::from("src/y_environment/api/env.rs"))?;
    write!(file, "{}", CONTENT.replace("{VERSION}", version.trim_end()))?;
    file.flush()
}

const CONTENT: &'static str = r#####"pub const VERSION: &'static str = "{VERSION}";
"#####;
