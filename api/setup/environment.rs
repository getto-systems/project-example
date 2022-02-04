use std::{
    fs::{read_to_string, File},
    io::Write,
    path::{Path, PathBuf},
};

pub fn generate() {
    EnvironmentCodegen::new()
        .build()
        .expect("failed to build environment");
}

struct EnvironmentCodegen {
    file: PathBuf,
}

impl EnvironmentCodegen {
    fn new() -> Self {
        Self {
            file: Path::new("src/y_environment/api").join("env.rs"),
        }
    }

    fn build(&self) -> std::io::Result<()> {
        self.env()?;
        Ok(())
    }

    fn env(&self) -> std::io::Result<()> {
        let mut file = File::create(self.file.as_path())?;
        write!(file, "{}", env_content()?)?;
        file.flush()
    }
}

fn env_content() -> std::io::Result<String> {
    let version = read_to_string("api/VERSION")?;
    Ok(CONTENT.replace("{VERSION}", version.trim_end()))
}

const CONTENT: &'static str = r#####"pub const VERSION: &'static str = "{VERSION}";
"#####;
