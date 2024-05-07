mod git_util;
mod setting_util;
// use setting_util::GitpSetting;

fn main() {
    // let mut gitp_setting = GitpSetting::default();
    // gitp_setting = setting_util::load(gitp_setting).unwrap();
    let gitp_setting = setting_util::load().unwrap();
    println!("{:#?}", gitp_setting);

    let result = git_util::git_clone("hoge");
    println!("{}", result);

    let result = git_util::git_status();
    println!("{}", result);
}
