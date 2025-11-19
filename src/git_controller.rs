use std::process::Command;

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

        GitController { encoding }
    }

    #[allow(dead_code)]
    pub fn git_status(&self) -> String {
        self.exec_git_command("status")
    }

    pub fn git_clone(&self, repo_name: &str, branch: &str) -> String {
        let sub_command: String = format!("clone {repo_name} -b {branch}");
        self.exec_git_command(&sub_command)
    }

    pub fn git_pull(&self) -> String {
        self.exec_git_command("pull")
    }

    pub fn git_push(&self, commit_message: &str) -> String {
        self.exec_git_command("add -A");
        let commit_cmd = format!("commit -m \"{commit_message}\"");
        self.exec_git_command(&commit_cmd);
        self.exec_git_command("push")
    }

    pub fn git_config(&self, name: &str, email: &str) {
        self.exec_git_command(&format!("config user.name \"{name}\""));
        self.exec_git_command(&format!("config user.email \"{email}\""));
    }

    pub fn git_config_raw(&self, key: &str, value: &str) {
        self.exec_git_command(&format!("config {key} \"{value}\""));
    }

    fn exec_git_command(&self, sub_command: &str) -> String {
        let sub_commands = sub_command.split(' ').collect();
        self.exec_command("git", sub_commands)
    }

    pub fn exec_command(&self, cmd: &str, args: Vec<&str>) -> String {
        println!("{cmd}");

        let output = Command::new(cmd).args(args).output().unwrap();

        let (stdout_result, _, _) = self.encoding.decode(&output.stdout);
        let (stderr_result, _, _) = self.encoding.decode(&output.stderr);

        let mut result = stdout_result.to_string();
        result.push_str(stderr_result.as_ref());

        result
    }
}
