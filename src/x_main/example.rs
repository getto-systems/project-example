use std::{ffi::OsString, fs::read_dir, path::PathBuf};

fn main() {
    println!(
        "{:?}",
        find_proto(PathBuf::from("src"), &vec!["service.proto".into()])
    );
}

fn find_proto(root: PathBuf, targets: &Vec<OsString>) -> std::io::Result<Vec<PathBuf>> {
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
