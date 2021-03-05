use serde::Deserialize;
use serde_yaml::from_str;
use std::env::var;
use std::fs::read_to_string;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct ConfigFile {
    mode: Option<String>,
    hooks_path: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum GitHooksMode {
    Single,
    Multi,
}

#[derive(Debug)]
pub struct GitHooksConfig {
    pub mode: GitHooksMode,
    pub hooks_path: PathBuf,
}

fn get_mode(config: &ConfigFile) -> Result<GitHooksMode, ()> {
    let mode = var("GIT_HOOKS_MODE");

    if mode.is_ok() {
        return match mode.unwrap_or_default().as_ref() {
            "single" => Ok(GitHooksMode::Single),
            "multi" => Ok(GitHooksMode::Multi),
            _ => Err(()),
        };
    }

    if let Some(mode) = &config.mode {
        return match mode.as_ref() {
            "single" => Ok(GitHooksMode::Single),
            "multi" => Ok(GitHooksMode::Multi),
            _ => Err(()),
        };
    }

    Ok(GitHooksMode::Single)
}

fn get_hooks_path(config: &ConfigFile, git_root: &PathBuf) -> Result<PathBuf, ()> {
    let hooks_path = var("GIT_HOOKS_PATH");

    if hooks_path.is_ok() {
        return Ok(git_root.join(hooks_path.unwrap()));
    }

    if let Some(hooks_path) = &config.hooks_path {
        return Ok(git_root.join(hooks_path));
    }

    Ok(git_root.join(".git-hooks"))
}

pub fn get_config(git_root: &PathBuf) -> Result<GitHooksConfig, serde_yaml::Error> {
    let config_path = git_root.join(".git-hooks.yml");

    let config_blob = match config_path.exists() {
        true => read_to_string(config_path).expect("error reading config file!"),
        false => String::from("mode: single"),
    };

    let config: ConfigFile = from_str(&config_blob)?;

    let mode = get_mode(&config).expect("invalid config: mode");
    let hooks_path = get_hooks_path(&config, git_root).expect("invalid config: hooks_path");

    Ok(GitHooksConfig { mode, hooks_path })
}
