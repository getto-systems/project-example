use std::{
    env::var,
    fs::{create_dir, read_dir, read_to_string, remove_dir_all, DirEntry, File},
    io::{Error as IoError, Result as IoResult, Write},
    path::{Path, PathBuf},
};

use regex::{escape, Regex};
use tonic_build::configure;

pub fn generate(package: &str) {
    GrpcBuilder::new(GrpcTarget::new(package.into()))
        .build()
        .expect(format!("failed to build grpc; {}", package).as_str());
}

struct GrpcTarget {
    package: String,
    dist: PathBuf,
    source: PathBuf,
    index: PathBuf,
}

impl GrpcTarget {
    fn new(package: String) -> Self {
        let target = format!("src/{}", package.replace(".", "/"));
        let target = Path::new(&target);

        let source = target.join("z_protobuf");
        let dist = target.join("_auth/y_protobuf"); // TODO _auth の部分を指定できるようにしたい
        let index = dist.join("mod.rs");

        Self {
            package,
            source,
            dist,
            index,
        }
    }
}

struct GrpcBuilder {
    target: GrpcTarget,
}

impl GrpcBuilder {
    fn new(target: GrpcTarget) -> Self {
        Self { target }
    }

    fn build(&self) -> IoResult<()> {
        self.cleanup()?;
        self.protobuf()?;
        self.module_index()?;
        Ok(())
    }

    fn cleanup(&self) -> IoResult<()> {
        if self.target.dist.exists() {
            remove_dir_all(self.target.dist.as_path())?;
        }
        create_dir(self.target.dist.as_path())?;

        Ok(())
    }
    fn module_index(&self) -> IoResult<()> {
        let mut file = File::create(self.target.index.as_path())?;
        write!(
            file,
            "{}",
            self.source_proto_basename()?
                .map(|name| format!("pub mod {};\n", name))
                .collect::<String>()
        )?;
        file.flush()
    }
    fn protobuf(&self) -> IoResult<()> {
        let out_dir = var("OUT_DIR").expect("OUT_DIR is not defined");

        let inputs: Vec<PathBuf> = self.source_proto()?.collect();

        configure().compile(&inputs, &[Path::new("src/").into()])?;

        // 他の proto は _auth::y_protobuf を追加して参照しないといけない
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
                    format!("super::super::super::$1::_auth::y_protobuf::{}::", name)
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
                Some("service.proto") => Some(entry.path()),
                _ => None,
            })
    })
}
fn pickup_basename(file: PathBuf) -> Option<String> {
    file.file_stem()
        .and_then(|name| name.to_str())
        .map(|name| name.to_string())
}
