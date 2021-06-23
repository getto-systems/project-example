use io::Write;
use std::{fs, io, path};

use protobuf_codegen_pure::Codegen;

pub fn generate(root: &str) {
    ProtobufCodegen::new(root)
        .build()
        .expect(format!("failed to build protobuf; {}", root).as_str());
}

struct ProtobufTarget {
    dist: path::PathBuf,
    source: path::PathBuf,
    index: path::PathBuf,
}

impl ProtobufTarget {
    fn new(target: &path::Path) -> Self {
        let source = target.join("z_protobuf");
        let dist = target.join("_api/y_protobuf");
        let index = dist.join("mod.rs");

        Self {
            source,
            dist,
            index,
        }
    }
}

struct ProtobufCodegen {
    target: ProtobufTarget,
}

impl ProtobufCodegen {
    fn new(root: &str) -> Self {
        Self {
            target: ProtobufTarget::new(path::Path::new(root)),
        }
    }

    fn build(&self) -> io::Result<()> {
        self.cleanup()?;
        self.protobuf()?;
        self.module_index()?;
        Ok(())
    }

    fn cleanup(&self) -> io::Result<()> {
        if self.target.dist.exists() {
            fs::remove_dir_all(self.target.dist.as_path())?;
        }
        fs::create_dir(self.target.dist.as_path())?;

        Ok(())
    }
    fn module_index(&self) -> io::Result<()> {
        let mut file = fs::File::create(self.target.index.as_path())?;
        write!(
            file,
            "{}",
            self.source_proto_basename()?
                .map(|name| format!("pub mod {};\n", name))
                .collect::<String>()
        )?;
        file.flush()
    }
    fn protobuf(&self) -> io::Result<()> {
        Codegen::new()
            .out_dir(self.target.dist.as_path())
            .inputs(self.source_proto()?)
            .include(self.target.source.as_path())
            .run()
    }

    fn source_proto(&self) -> io::Result<impl Iterator<Item = path::PathBuf>> {
        Ok(fs::read_dir(self.target.source.as_path())?.filter_map(filter_proto))
    }
    fn source_proto_basename(&self) -> io::Result<impl Iterator<Item = String>> {
        Ok(self.source_proto()?.filter_map(pickup_basename))
    }
}

fn filter_proto(result: Result<fs::DirEntry, io::Error>) -> Option<path::PathBuf> {
    result.ok().and_then(|entry| {
        entry
            .path()
            .file_name()
            .and_then(|name| match name.to_str() {
                Some("api.proto") => Some(entry.path()),
                _ => None,
            })
    })
}
fn pickup_basename(file: path::PathBuf) -> Option<String> {
    file.file_stem()
        .and_then(|name| name.to_str())
        .map(|name| name.to_string())
}
