mod git_controller;
mod interactive;
mod setting_util;
mod tui;

use git_controller::GitController;
use std::env;
use std::fs;
use std::sync::Arc;
use std::thread;
use tui::{update_repo_status, RepoStatus, TuiApp};

fn main() {
    // Load settings
    let gitp_setting = match setting_util::load() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    };

    // Parse command line arguments
    let args: Vec<String> = env::args().skip(1).collect();

    // Interactive mode or one-shot mode
    if args.is_empty() {
        // Interactive mode
        loop {
            match interactive::run_interactive_mode() {
                Ok(cmd_args) => {
                    if cmd_args.is_empty() {
                        // User exited
                        break;
                    }
                    // Execute command
                    if let Err(e) = execute_command(&gitp_setting, &cmd_args) {
                        eprintln!("Error: {e}");
                    }
                }
                Err(e) => {
                    eprintln!("Interactive mode error: {e:?}");
                    break;
                }
            }
        }
    } else {
        // One-shot mode
        if let Err(e) = execute_command(&gitp_setting, &args) {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    }
}

fn execute_command(
    gitp_setting: &setting_util::GitpSetting,
    args: &[String],
) -> Result<(), String> {
    if args.is_empty() {
        return Ok(());
    }

    let command = &args[0];
    let is_serial = args.len() > 1 && args[1] == "serial";

    // Collect enabled repositories
    let enabled_repos: Vec<_> = gitp_setting.repos.iter().filter(|r| r.enabled).collect();

    if enabled_repos.is_empty() {
        println!("No enabled repositories found in configuration.");
        return Ok(());
    }

    // Extract repository names for TUI
    let repo_names: Vec<String> = enabled_repos
        .iter()
        .map(|r| extract_repo_name(&r.remote))
        .collect();

    // Create TUI app
    let mut tui_app = TuiApp::new(repo_names);
    let repos_handle = tui_app.get_repos_handle();

    // Spawn worker threads based on command
    match command.as_str() {
        "clone" | "clo" | "cl" => {
            spawn_clone_workers(gitp_setting, &enabled_repos, repos_handle.clone());
        }
        "pull" | "pul" | "pu" => {
            spawn_pull_workers(gitp_setting, &enabled_repos, repos_handle.clone());
        }
        "push" | "pus" | "ps" => {
            spawn_push_workers(gitp_setting, &enabled_repos, repos_handle.clone());
        }
        "config" | "conf" | "cfg" => {
            // Check for subcommand
            if args.len() < 2 {
                // No subcommand - apply all configs from YAML
                spawn_config_all_workers(gitp_setting, &enabled_repos, repos_handle.clone());
            } else {
                let subcommand = &args[1];
                if subcommand == "user" || subcommand == "u" || subcommand == "usr" {
                    // config user - apply user.name and user.email only
                    let is_serial_config = args.len() > 2 && args[2] == "serial";
                    spawn_config_user_workers(gitp_setting, &enabled_repos, repos_handle.clone());

                    if let Err(e) = tui_app.run(!is_serial_config) {
                        return Err(format!("TUI error: {e:?}"));
                    }
                    return Ok(());
                } else if subcommand == "serial" {
                    // config serial - apply all configs in serial mode
                    spawn_config_all_workers(gitp_setting, &enabled_repos, repos_handle.clone());

                    if let Err(e) = tui_app.run(false) {
                        return Err(format!("TUI error: {e:?}"));
                    }
                    return Ok(());
                } else {
                    return Err(format!("Unknown subcommand: config {subcommand}"));
                }
            }

            // Run TUI for config without subcommand
            if let Err(e) = tui_app.run(true) {
                return Err(format!("TUI error: {e:?}"));
            }
            return Ok(());
        }
        "help" | "?" => {
            show_help();
            return Ok(());
        }
        _ => {
            return Err(format!("Unknown command: {command}"));
        }
    }

    // Run TUI
    if let Err(e) = tui_app.run(!is_serial) {
        return Err(format!("TUI error: {e:?}"));
    }

    Ok(())
}

fn show_help() {
    println!("\n\x1b[1;36mgitp\x1b[0m - Git Multiple Repository Manager\n");
    println!("\x1b[1;36mUsage:\x1b[0m");
    println!("  gitp                   Start interactive mode");
    println!("  gitp <command> [opts]  Execute command and exit\n");
    println!("\x1b[1;36mCommands:\x1b[0m");
    println!("  \x1b[1;33mclone\x1b[0m [serial]        Clone all enabled repositories");
    println!("  \x1b[1;33mpull\x1b[0m [serial]         Pull all enabled repositories");
    println!("  \x1b[1;33mpush\x1b[0m [serial]         Push all enabled repositories");
    println!("  \x1b[1;33mconfig user\x1b[0m [serial]  Set user.name and user.email for all repositories");
    println!("  \x1b[1;33mhelp\x1b[0m                  Show this help message\n");
    println!("\x1b[1;36mOptions:\x1b[0m");
    println!(
        "  \x1b[1;33mserial\x1b[0m                 Execute sequentially (default: parallel)\n"
    );
    println!("\x1b[1;36mShortcuts:\x1b[0m");
    println!("  clo, cl  → clone");
    println!("  pul, pu  → pull");
    println!("  pus, ps  → push");
    println!("  conf, cfg → config");
    println!("  u, usr   → user (for config subcommand)\n");
}

fn spawn_clone_workers(
    setting: &setting_util::GitpSetting,
    repos: &[&setting_util::Repos],
    repos_handle: Arc<std::sync::Mutex<Vec<tui::RepoProgress>>>,
) {
    for repo in repos {
        let repo_clone = (*repo).clone();
        let user_name = setting.user.name.clone();
        let user_email = setting.user.email.clone();
        let repos_handle = Arc::clone(&repos_handle);
        let repo_name = extract_repo_name(&repo.remote);

        thread::spawn(move || {
            update_repo_status(
                &repos_handle,
                &repo_name,
                RepoStatus::Running,
                "Starting...",
                10,
            );

            let git = GitController::new();

            // Create group directory
            update_repo_status(
                &repos_handle,
                &repo_name,
                RepoStatus::Running,
                "Creating directory...",
                20,
            );
            if let Err(e) = fs::create_dir_all(&repo_clone.group) {
                update_repo_status(
                    &repos_handle,
                    &repo_name,
                    RepoStatus::Failed,
                    &format!("Error: {e}"),
                    100,
                );
                return;
            }

            // Change to group directory
            let original_dir = env::current_dir().unwrap();
            if let Err(e) = env::set_current_dir(&repo_clone.group) {
                update_repo_status(
                    &repos_handle,
                    &repo_name,
                    RepoStatus::Failed,
                    &format!("Error: {e}"),
                    100,
                );
                return;
            }

            // Clone
            update_repo_status(
                &repos_handle,
                &repo_name,
                RepoStatus::Running,
                "Cloning...",
                40,
            );
            let result = git.git_clone(&repo_clone.remote, &repo_clone.branch);

            // Change into cloned repo
            update_repo_status(
                &repos_handle,
                &repo_name,
                RepoStatus::Running,
                "Configuring...",
                80,
            );
            if let Err(e) = env::set_current_dir(extract_repo_name(&repo_clone.remote)) {
                update_repo_status(
                    &repos_handle,
                    &repo_name,
                    RepoStatus::Failed,
                    &format!("Error: {e}"),
                    100,
                );
                env::set_current_dir(original_dir).ok();
                return;
            }

            git.git_config(&user_name, &user_email);
            env::set_current_dir(original_dir).ok();

            if result.contains("fatal") || result.contains("error") {
                update_repo_status(&repos_handle, &repo_name, RepoStatus::Failed, "Failed", 100);
            } else {
                update_repo_status(&repos_handle, &repo_name, RepoStatus::Success, "Done", 100);
            }
        });
    }
}

fn spawn_pull_workers(
    setting: &setting_util::GitpSetting,
    repos: &[&setting_util::Repos],
    repos_handle: Arc<std::sync::Mutex<Vec<tui::RepoProgress>>>,
) {
    for repo in repos {
        let repo_clone = (*repo).clone();
        let user_name = setting.user.name.clone();
        let user_email = setting.user.email.clone();
        let repos_handle = Arc::clone(&repos_handle);
        let repo_name = extract_repo_name(&repo.remote);

        thread::spawn(move || {
            update_repo_status(
                &repos_handle,
                &repo_name,
                RepoStatus::Running,
                "Starting...",
                10,
            );

            let git = GitController::new();
            let repo_path = format!(
                "{}/{}",
                repo_clone.group,
                extract_repo_name(&repo_clone.remote)
            );

            let original_dir = env::current_dir().unwrap();
            if let Err(e) = env::set_current_dir(&repo_path) {
                update_repo_status(
                    &repos_handle,
                    &repo_name,
                    RepoStatus::Failed,
                    &format!("Error: {e}"),
                    100,
                );
                return;
            }

            update_repo_status(
                &repos_handle,
                &repo_name,
                RepoStatus::Running,
                "Configuring...",
                30,
            );
            git.git_config(&user_name, &user_email);

            update_repo_status(
                &repos_handle,
                &repo_name,
                RepoStatus::Running,
                "Pulling...",
                50,
            );
            let result = git.git_pull();

            env::set_current_dir(original_dir).ok();

            if result.contains("fatal") || result.contains("error") {
                update_repo_status(&repos_handle, &repo_name, RepoStatus::Failed, "Failed", 100);
            } else {
                update_repo_status(&repos_handle, &repo_name, RepoStatus::Success, "Done", 100);
            }
        });
    }
}

fn spawn_push_workers(
    setting: &setting_util::GitpSetting,
    repos: &[&setting_util::Repos],
    repos_handle: Arc<std::sync::Mutex<Vec<tui::RepoProgress>>>,
) {
    let commit_message = setting
        .comments
        .get("default")
        .unwrap_or(&"update.".to_string())
        .clone();

    for repo in repos {
        let repo_clone = (*repo).clone();
        let user_name = setting.user.name.clone();
        let user_email = setting.user.email.clone();
        let repos_handle = Arc::clone(&repos_handle);
        let repo_name = extract_repo_name(&repo.remote);
        let commit_msg = commit_message.clone();

        thread::spawn(move || {
            update_repo_status(
                &repos_handle,
                &repo_name,
                RepoStatus::Running,
                "Starting...",
                10,
            );

            let git = GitController::new();
            let repo_path = format!(
                "{}/{}",
                repo_clone.group,
                extract_repo_name(&repo_clone.remote)
            );

            let original_dir = env::current_dir().unwrap();
            if let Err(e) = env::set_current_dir(&repo_path) {
                update_repo_status(
                    &repos_handle,
                    &repo_name,
                    RepoStatus::Failed,
                    &format!("Error: {e}"),
                    100,
                );
                return;
            }

            update_repo_status(
                &repos_handle,
                &repo_name,
                RepoStatus::Running,
                "Configuring...",
                20,
            );
            git.git_config(&user_name, &user_email);

            update_repo_status(
                &repos_handle,
                &repo_name,
                RepoStatus::Running,
                "Adding files...",
                40,
            );
            update_repo_status(
                &repos_handle,
                &repo_name,
                RepoStatus::Running,
                "Committing...",
                60,
            );
            update_repo_status(
                &repos_handle,
                &repo_name,
                RepoStatus::Running,
                "Pushing...",
                80,
            );

            let result = git.git_push(&commit_msg);

            env::set_current_dir(original_dir).ok();

            if result.contains("fatal") || result.contains("error") {
                update_repo_status(&repos_handle, &repo_name, RepoStatus::Failed, "Failed", 100);
            } else {
                update_repo_status(&repos_handle, &repo_name, RepoStatus::Success, "Done", 100);
            }
        });
    }
}

fn spawn_config_all_workers(
    setting: &setting_util::GitpSetting,
    repos: &[&setting_util::Repos],
    repos_handle: Arc<std::sync::Mutex<Vec<tui::RepoProgress>>>,
) {
    for repo in repos {
        let repo_clone = (*repo).clone();
        let user_name = setting.user.name.clone();
        let user_email = setting.user.email.clone();
        let configs = setting.config.clone();
        let repos_handle = Arc::clone(&repos_handle);
        let repo_name = extract_repo_name(&repo.remote);

        thread::spawn(move || {
            update_repo_status(
                &repos_handle,
                &repo_name,
                RepoStatus::Running,
                "Starting...",
                10,
            );

            let git = GitController::new();
            let repo_path = format!(
                "{}/{}",
                repo_clone.group,
                extract_repo_name(&repo_clone.remote)
            );

            let original_dir = env::current_dir().unwrap();
            if let Err(e) = env::set_current_dir(&repo_path) {
                update_repo_status(
                    &repos_handle,
                    &repo_name,
                    RepoStatus::Failed,
                    &format!("Error: {e}"),
                    100,
                );
                return;
            }

            // Apply user.name and user.email
            update_repo_status(
                &repos_handle,
                &repo_name,
                RepoStatus::Running,
                "Setting user...",
                20,
            );
            git.git_config(&user_name, &user_email);

            // Apply all configs from YAML
            let total_configs = configs.len();
            for (i, (key, value)) in configs.iter().enumerate() {
                let progress = 20 + ((i + 1) * 70 / total_configs.max(1)) as u16;
                update_repo_status(
                    &repos_handle,
                    &repo_name,
                    RepoStatus::Running,
                    &format!("Setting {key}..."),
                    progress,
                );
                git.git_config_raw(key, value);
            }

            env::set_current_dir(original_dir).ok();

            update_repo_status(
                &repos_handle,
                &repo_name,
                RepoStatus::Success,
                "Configured",
                100,
            );
        });
    }
}

fn spawn_config_user_workers(
    setting: &setting_util::GitpSetting,
    repos: &[&setting_util::Repos],
    repos_handle: Arc<std::sync::Mutex<Vec<tui::RepoProgress>>>,
) {
    for repo in repos {
        let repo_clone = (*repo).clone();
        let user_name = setting.user.name.clone();
        let user_email = setting.user.email.clone();
        let repos_handle = Arc::clone(&repos_handle);
        let repo_name = extract_repo_name(&repo.remote);

        thread::spawn(move || {
            update_repo_status(
                &repos_handle,
                &repo_name,
                RepoStatus::Running,
                "Starting...",
                10,
            );

            let git = GitController::new();
            let repo_path = format!(
                "{}/{}",
                repo_clone.group,
                extract_repo_name(&repo_clone.remote)
            );

            let original_dir = env::current_dir().unwrap();
            if let Err(e) = env::set_current_dir(&repo_path) {
                update_repo_status(
                    &repos_handle,
                    &repo_name,
                    RepoStatus::Failed,
                    &format!("Error: {e}"),
                    100,
                );
                return;
            }

            update_repo_status(
                &repos_handle,
                &repo_name,
                RepoStatus::Running,
                "Setting user.name...",
                40,
            );
            git.git_config(&user_name, &user_email);

            update_repo_status(
                &repos_handle,
                &repo_name,
                RepoStatus::Running,
                "Setting user.email...",
                80,
            );

            env::set_current_dir(original_dir).ok();

            update_repo_status(
                &repos_handle,
                &repo_name,
                RepoStatus::Success,
                "Configured",
                100,
            );
        });
    }
}

fn extract_repo_name(remote_url: &str) -> String {
    let parts: Vec<&str> = remote_url.split('/').collect();
    let last_part = parts.last().unwrap_or(&"");
    last_part.trim_end_matches(".git").to_string()
}
