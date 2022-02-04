use std::{
    env::var,
    ffi::OsString,
    fs::{read_dir, read_to_string, File},
    io::Write,
    path::PathBuf,
};

use regex::{escape, Regex};
use tonic_build::configure;

pub fn generate() {
    build().expect("failed to build grpc");
}

fn build() -> std::io::Result<()> {
    let files = find_proto(PathBuf::from("src"), &vec!["service.proto".into()])?;
    configure().compile(&files, &["src/"])?;
    rewrite(&files)?;
    Ok(())
}

fn find_proto(root: PathBuf, targets: &[OsString]) -> std::io::Result<Vec<PathBuf>> {
    let mut files: Vec<PathBuf> = vec![];
    for entry in read_dir(root)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            find_proto(entry.path(), targets)?
                .into_iter()
                .for_each(|file| files.push(file));
        }
        if file_type.is_file() {
            if let Some(file_name) = entry.path().file_name() {
                if targets.iter().any(|target| file_name == target) {
                    files.push(entry.path());
                }
            }
        }
    }
    Ok(files)
}

fn rewrite(files: &[PathBuf]) -> std::io::Result<()> {
    // rust の package として y_protobuf を追加しているので、書き換えないといけない
    let out_dir = var("OUT_DIR").expect("OUT_DIR is not defined");
    files
        .iter()
        .filter_map(|file| {
            match (
                dist(file),
                package(file),
                file.file_stem().and_then(|name| name.to_str()),
            ) {
                (Some(file), Some(package), Some(name)) => Some((file, package, name)),
                _ => None,
            }
        })
        .fold(Ok(()), |acc, (file, package, name)| {
            acc?;

            let pattern = Regex::new(&format!("super::(.*)::{}::", escape(name))).unwrap();
            let content = read_to_string(format!("{}/{}.rs", out_dir, package))?;
            let mut file = File::create(file)?;
            write!(
                file,
                "{}",
                pattern.replace_all(
                    &content,
                    format!("super::super::$1::y_protobuf::{}::", name)
                ),
            )?;
            file.flush()
        })
}

fn dist(path: &PathBuf) -> Option<PathBuf> {
    match (path.parent(), path.file_stem()) {
        (Some(parent), Some(name)) => {
            let mut dist = path.clone();
            dist.pop();
            if parent.ends_with("z_protobuf") {
                dist.pop();
                dist.push("y_protobuf");
            }
            dist.push(name);
            dist.set_extension("rs");
            Some(dist)
        }
        _ => None,
    }
}
fn package(path: &PathBuf) -> Option<String> {
    match (path.parent(), path.file_stem()) {
        (Some(parent), Some(name)) => {
            let mut package = path.clone();
            package.pop();
            if parent.ends_with("z_protobuf") {
                package.pop();
            }
            package.push(name);
            package
                .strip_prefix("src/")
                .ok()
                .and_then(|path| path.to_str())
                .map(|path| path.to_owned().replace("/", "."))
        }
        _ => None,
    }
}
