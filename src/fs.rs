use std::fs::{create_dir_all, File};
use std::io::prelude::Write;
#[cfg(target_family = "unix")]
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

pub fn create_directory(path: &PathBuf) -> std::io::Result<()> {
    create_dir_all(&path)
}

fn create_file(path: &PathBuf) -> std::io::Result<File> {
    File::create(&path)
}

fn make_executable(file: &File) -> std::io::Result<()> {
    let metadata = file.metadata()?;

    if !metadata.is_file() {
        return Ok(());
    };

    let mut permissions = metadata.permissions();

    #[cfg(target_family = "unix")]
    permissions.set_mode(0o755);

    file.set_permissions(permissions)
}

pub fn write_file(file_path: &PathBuf, contents: &str, executable: bool) -> Result<(), String> {
    let mut file = create_file(file_path)
        .unwrap_or_else(|_| panic!("failed to create file {}", file_path.display()));

    if executable {
        make_executable(&file)
            .unwrap_or_else(|_| panic!("failed to make executable {}", file_path.display()));
    }

    match file.write_all(contents.as_bytes()) {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("failed to write file {}", file_path.display())),
    }
}
