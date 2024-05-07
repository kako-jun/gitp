mod git_util;

fn main() {
    let result = git_util::git_clone("hoge");
    println!("{}", result);

    let result = git_util::git_status();
    println!("{}", result);
}
