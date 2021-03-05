use std::collections::HashMap;
use std::process::Command;

const SHELL: &str = "sh";
const SHELL_ARG: &str = "-c";

pub fn run_command(
    cmd: &str,
    cwd: Option<&str>,
    env: Option<&HashMap<String, String>>,
) -> Result<Option<String>, Option<String>> {
    let mut command = Command::new(SHELL);

    command
        .current_dir(cwd.unwrap_or("."))
        .args(&[SHELL_ARG, cmd]);

    if let Some(vars) = env {
        command.envs(vars);
    }

    match command.output() {
        Ok(output) => {
            if output.status.success() {
                Ok(Some(
                    String::from_utf8(output.stdout)
                        .unwrap()
                        .trim_end_matches('\n')
                        .to_string(),
                ))
            } else {
                Err(Some(format!(
                    "{}\n{}",
                    String::from_utf8(output.stderr).unwrap(),
                    String::from_utf8(output.stdout).unwrap(),
                )))
            }
        }
        Err(error) => panic!("command failed: {}", error),
    }
}
