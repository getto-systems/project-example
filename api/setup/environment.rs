use io::Write;
use std::{fs, io, path};

pub fn generate() {
    EnvironmentCodegen::new()
        .build()
        .expect("failed to build environment");
}

struct EnvironmentCodegen {
    file: path::PathBuf,
}

impl EnvironmentCodegen {
    fn new() -> Self {
        Self {
            file: path::Path::new("src/y_environment/_api").join("env.rs"),
        }
    }

    fn build(&self) -> io::Result<()> {
        self.cleanup()?;
        self.env()?;
        Ok(())
    }

    fn cleanup(&self) -> io::Result<()> {
        if self.file.exists() {
            fs::remove_file(self.file.as_path())?;
        }
        Ok(())
    }
    fn env(&self) -> io::Result<()> {
        let mut file = fs::File::create(self.file.as_path())?;
        write!(file, "{}", env_content()?)?;
        file.flush()
    }
}

fn env_content() -> io::Result<String> {
    let version = fs::read_to_string("api/VERSION")?;
    Ok(CONTENT.replace("{VERSION}", version.trim_end()))
}

const CONTENT: &'static str = r#####"pub const VERSION: &'static str = "{VERSION}";
"#####;
