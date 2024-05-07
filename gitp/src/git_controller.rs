use std::io::Read;
use std::process::{Command, Stdio};

// trait GitControllerInterface {
//     fn new() -> Self;
//     fn git_status() -> String;
//     fn git_clone(repo_name: &str) -> String;
//     fn exec_git_command(sub_command: &str) -> String;
//     fn exec_command(cmd: &str, args: Vec<&str>) -> String;
// }

pub struct GitController {
    // fields
    encoding: &'static encoding_rs::Encoding,
}

// impl GitControllerInterface for GitController {
impl GitController {
    // methods
    pub fn new() -> Self {
        let mut encoding = encoding_rs::UTF_8;
        if cfg!(target_os = "linux") {
            println!("Running on Linux.");
        } else if cfg!(target_os = "windows") {
            // Windowsの場合の処理
            println!("Running on Windows.");
            // ここにWindows用のコードを追加
            encoding = encoding_rs::SHIFT_JIS;
        } else {
            // 他のOSの場合の処理
            println!("Running on an unknown OS.");
        }

        GitController {
            encoding: encoding,
        }
    }

    pub fn git_status(&self) -> String {
        let sub_command = "status";
        let result = self.exec_git_command(sub_command);
        result
    }

    pub fn git_clone(&self, repo_name: &str) -> String {
        let sub_command: String = format!("clone {}", repo_name);
        let result = self.exec_git_command(&sub_command);
        result
    }

    fn exec_git_command(&self, sub_command: &str) -> String {
        let sub_commands = sub_command.split(' ').collect();
        let result = self.exec_command("git", sub_commands);
        result
    }

    pub fn exec_command(&self, cmd: &str, args: Vec<&str>) -> String {
        println!("{}", cmd);
        // let mut child = Command::new("cmd")
        //     .args(["/C", cmd])
        let mut child = Command::new(cmd)
            .args(args)
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

        let (stdout_result, _, _) = self.encoding.decode(&stdout_buffer);
        let (stderr_result, _, _) = self.encoding.decode(&stderr_buffer);

        let mut result = stdout_result.to_string();
        result.push_str(&stderr_result.to_string());

        result
    }
}
