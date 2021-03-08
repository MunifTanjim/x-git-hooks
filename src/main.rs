#![warn(clippy::all)]

use std::path::PathBuf;
use structopt::StructOpt;

mod config;
mod fs;
mod git;
mod hooks;
mod shell;

#[derive(StructOpt, Debug)]
struct GitHooks {
    #[structopt(long, short)]
    verbose: bool,
    #[structopt(subcommand)]
    cmd: Cmd,
}

#[derive(StructOpt, Debug)]
enum Cmd {
    /// initialize git hooks
    Init,
    /// deinitialize git hooks
    Deinit,
    /// add a git hook
    Add(AddOpts),
}

#[derive(StructOpt, Debug)]
pub struct InitOpts {}

#[derive(StructOpt, Debug)]
pub struct AddOpts {
    hook_name: String,
}

pub fn init(git_root: &PathBuf, config: &config::GitHooksConfig) -> Result<(), String> {
    let current_hooks_path = git::get_hooks_path(&git_root)?;

    if config.hooks_path.exists() && config.hooks_path != current_hooks_path {
        git::set_hooks_path(&git_root, &config.hooks_path)?;
    }

    Ok(())
}

pub fn deinit(git_root: &PathBuf, config: &config::GitHooksConfig) -> Result<(), String> {
    let current_hooks_path = git::get_hooks_path(&git_root)?;

    if config.hooks_path == current_hooks_path {
        git::unset_hooks_path(&git_root)?;
    }

    Ok(())
}

pub fn add(config: &config::GitHooksConfig, opts: AddOpts) -> Result<(), String> {
    match hooks::add_hook(config, &opts.hook_name) {
        Ok(_) => (),
        Err(error) => return Err(error),
    };

    Ok(())
}

fn main() {
    let opt = GitHooks::from_args();

    let git_root = git::get_root_path(None).expect("failed to get root path");
    let config = config::get_config(&git_root).unwrap();

    let result = match opt.cmd {
        Cmd::Init => init(&git_root, &config),
        Cmd::Deinit => deinit(&git_root, &config),
        Cmd::Add(opts) => add(&config, opts),
    };

    match result {
        Ok(_) => (),
        Err(error) => panic!(error),
    }
}
