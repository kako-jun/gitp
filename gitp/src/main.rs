mod git_controller;
mod setting_util;
use git_controller::GitController;
// use setting_util::GitpSetting;

fn main() {
    let git_controller = GitController::new();
    let result = git_controller.exec_command("ls", Vec::from(["-alF"]));
    println!("{}", result);

    // let mut gitp_setting = GitpSetting::default();
    // gitp_setting = setting_util::load(gitp_setting).unwrap();
    let gitp_setting = setting_util::load().unwrap();
    println!("{:#?}", gitp_setting);

    println!("{:#?}", gitp_setting.repos[0].remote);
    

    let result = git_controller.git_clone(&gitp_setting.repos[0].remote);
    println!("{}", result);

    let result = git_controller.git_status();
    println!("{}", result);
}
