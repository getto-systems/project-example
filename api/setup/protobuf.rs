use std::{
    env::var,
    fs::{create_dir, read_dir, read_to_string, remove_file, DirEntry, File},
    io::{Error as IoError, Result as IoResult, Write},
    path::{Path, PathBuf},
};

use prost_build::compile_protos;
use regex::{escape, Regex};

pub fn generate(package: &str) {
    ProtobufBuilder::new(ProtobufTarget::new(package.into()))
        .build()
        .expect(format!("failed to build protobuf; {}", package).as_str());
}

struct ProtobufTarget {
    package: String,
    dist: PathBuf,
    source: PathBuf,
}

impl ProtobufTarget {
    fn new(package: String) -> Self {
        let target = format!("src/{}", package.replace(".", "/"));
        let target = Path::new(&target);

        let source = target.join("z_protobuf");
        let dist = target.join("remote/y_protobuf");

        Self {
            package,
            source,
            dist,
        }
    }
}

struct ProtobufBuilder {
    target: ProtobufTarget,
}

impl ProtobufBuilder {
    fn new(target: ProtobufTarget) -> Self {
        Self { target }
    }

    fn build(&self) -> IoResult<()> {
        self.cleanup()?;
        self.protobuf()?;
        Ok(())
    }

    fn cleanup(&self) -> IoResult<()> {
        if self.target.dist.exists() {
            self.source_proto_basename()?.fold(Ok(()), |acc, name| {
                acc?;
                let file = self.target.dist.join(name);
                if file.exists() {
                    remove_file(file)
                } else {
                    Ok(())
                }
            })?;
        } else {
            create_dir(self.target.dist.as_path())?;
        }

        Ok(())
    }
    fn protobuf(&self) -> IoResult<()> {
        let out_dir = var("OUT_DIR").expect("OUT_DIR is not defined");

        let inputs: Vec<PathBuf> = self.source_proto()?.collect();
        compile_protos(&inputs, &["src/"])?;

        // 他の proto は remote::y_protobuf を追加して参照しないといけない
        self.source_proto_basename()?.fold(Ok(()), |acc, name| {
            acc?;

            let import_ref_regex =
                Regex::new(&format!("super::(.*)::{}::", escape(&name))).unwrap();
            let content =
                read_to_string(format!("{}/{}.{}.rs", out_dir, self.target.package, name))?;
            let mut file = File::create(self.target.dist.join(format!("{}.rs", name)))?;
            write!(
                file,
                "{}",
                import_ref_regex.replace_all(
                    &content,
                    format!("super::super::super::$1::remote::y_protobuf::{}::", name)
                ),
            )?;
            file.flush()
        })
    }

    fn source_proto(&self) -> IoResult<impl Iterator<Item = PathBuf>> {
        Ok(read_dir(self.target.source.as_path())?.filter_map(filter_proto))
    }
    fn source_proto_basename(&self) -> IoResult<impl Iterator<Item = String>> {
        Ok(self.source_proto()?.filter_map(pickup_basename))
    }
}

fn filter_proto(result: Result<DirEntry, IoError>) -> Option<PathBuf> {
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
fn pickup_basename(file: PathBuf) -> Option<String> {
    file.file_stem()
        .and_then(|name| name.to_str())
        .map(|name| name.to_string())
}
