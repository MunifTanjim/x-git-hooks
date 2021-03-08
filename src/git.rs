use std::path::PathBuf;

use crate::shell;

fn is_valid_hooks_path(git_root: &PathBuf, hooks_path: &PathBuf) -> bool {
    git_root == hooks_path.parent().unwrap()
}

pub fn get_root_path(cwd: Option<&str>) -> Result<PathBuf, String> {
    match shell::run_command("git rev-parse --show-toplevel", cwd, None) {
        Ok(Some(path)) => Ok(PathBuf::from(path)),
        Err(Some(error)) => Err(error),
        _ => Err(String::from("failed to get git root")),
    }
}

pub fn get_hooks_path(git_root: &PathBuf) -> Result<PathBuf, String> {
    match shell::run_command("git rev-parse --git-path hooks", git_root.to_str(), None) {
        Ok(Some(path)) => Ok(git_root.join(path)),
        Err(Some(error)) => Err(error),
        _ => panic!(),
    }
}

pub fn set_hooks_path(git_root: &PathBuf, hooks_path: &PathBuf) -> Result<(), String> {
    match is_valid_hooks_path(git_root, hooks_path) {
        true => (),
        false => return Err("invalid hooks path".to_string()),
    };

    let path = match hooks_path.is_absolute() {
        true => hooks_path.strip_prefix(git_root).unwrap(),
        false => hooks_path,
    };

    match shell::run_command(
        &format!("git config core.hooksPath {}", path.to_str().unwrap()),
        git_root.to_str(),
        None,
    ) {
        Ok(_) => Ok(()),
        _ => Err(String::from("failed to set git hooks path")),
    }
}

pub fn unset_hooks_path(git_root: &PathBuf) -> Result<(), String> {
    match shell::run_command("git config --unset core.hooksPath", git_root.to_str(), None) {
        Ok(_) => Ok(()),
        _ => Err(String::from("failed to unset git hooks path")),
    }
}
