use encoding_rs::SHIFT_JIS;
use std::io::Read;
use std::process::{Command, Stdio};

pub fn git_status() -> String {
    let result = exec_git_command("status");
    result
}

pub fn git_clone(repo_name: &str) -> String {
    let sub_cmd = format!("clone {}", repo_name);
    let result = exec_git_command(&sub_cmd);
    result
}

pub fn exec_git_command(sub_cmd: &str) -> String {
    let full_cmd = format!("git {}", sub_cmd);
    let result = exec_command(&full_cmd);
    result
}

pub fn exec_command(cmd: &str) -> String {
    let mut child = Command::new("cmd")
        .args(["/C", cmd])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let stdout = child.stdout.as_mut().unwrap();
    let stderr = child.stderr.as_mut().unwrap();

    let mut stdout_buffer = Vec::new();
    let mut stderr_buffer = Vec::new();

    loop {
        let stdout_available = stdout.read_to_end(&mut stdout_buffer).unwrap();
        let stderr_available = stderr.read_to_end(&mut stderr_buffer).unwrap();
        if stdout_available == 0 && stderr_available == 0 {
            break;
        }
    }

    let (stdout_result, _, _) = SHIFT_JIS.decode(&stdout_buffer);
    let (stderr_result, _, _) = SHIFT_JIS.decode(&stderr_buffer);

    let mut result = stdout_result.to_string();
    result.push_str(&stderr_result.to_string());

    result
}
