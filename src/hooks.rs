use crate::config::{GitHooksConfig, GitHooksMode};
use crate::fs::{create_directory, write_file};

const HOOK_MULTI_MODE_SAMPLE_SCRIPT: &str = include_str!("scripts/multi-mode-sample-script");
const HOOK_MULTI_MODE_RUNNER: &str = include_str!("scripts/multi-mode-runner");
const HOOK_SINGLE_MODE_RUNNER: &str = include_str!("scripts/single-mode-runner");

const HOOK_NAMES: [&str; 19] = [
    "applypatch-msg",
    "pre-applypatch",
    "post-applypatch",
    "pre-commit",
    "pre-merge-commit",
    "prepare-commit-msg",
    "commit-msg",
    "post-commit",
    "pre-rebase",
    "post-checkout",
    "post-merge",
    "pre-push",
    "reference-transaction",
    "pre-auto-gc",
    "post-rewrite",
    "rebase",
    "sendemail-validate",
    "fsmonitor-watchman",
    "post-index-change",
];

fn is_valid_hook_name(hook_name: &str) -> bool {
    HOOK_NAMES.iter().any(|v| v == &hook_name)
}

pub fn add_hook(config: &GitHooksConfig, hook_name: &str) -> Result<(), String> {
    match is_valid_hook_name(hook_name) {
        true => (),
        false => return Err("invalid hook name".to_string()),
    };

    if !config.hooks_path.exists() {
        create_directory(&config.hooks_path).unwrap_or_else(|_| {
            panic!(
                "failed to create hooks directory: {}",
                &config.hooks_path.to_str().unwrap()
            )
        })
    }

    let hook_path = config.hooks_path.join(hook_name);

    if config.mode == GitHooksMode::Single && !hook_path.exists() {
        write_file(&hook_path, HOOK_SINGLE_MODE_RUNNER, true)?;
    }

    if config.mode == GitHooksMode::Multi {
        if !hook_path.exists() {
            write_file(&hook_path, HOOK_MULTI_MODE_RUNNER, true)?;
        }

        let scripts_path = config.hooks_path.join(format!(".{}", hook_name));

        if !scripts_path.exists() {
            create_directory(&scripts_path).unwrap_or_else(|_| {
                panic!("failed to create scripts directory for hook: {}", hook_name)
            });

            write_file(
                &scripts_path.join("sample-script"),
                HOOK_MULTI_MODE_SAMPLE_SCRIPT,
                true,
            )?;
        }
    }

    Ok(())
}
