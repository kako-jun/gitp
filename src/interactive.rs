use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Editor, Helper};
use std::borrow::Cow;

pub struct GitpHelper {
    commands: Vec<String>,
}

impl GitpHelper {
    pub fn new() -> Self {
        GitpHelper {
            commands: vec![
                "clone".to_string(),
                "pull".to_string(),
                "push".to_string(),
                "config user".to_string(),
                "help".to_string(),
                "exit".to_string(),
                "quit".to_string(),
            ],
        }
    }
}

impl Completer for GitpHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        let input = &line[..pos];
        let mut candidates = Vec::new();

        for cmd in &self.commands {
            if cmd.starts_with(input) {
                candidates.push(Pair {
                    display: cmd.clone(),
                    replacement: cmd.clone(),
                });
            }
        }

        Ok((0, candidates))
    }
}

impl Hinter for GitpHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Option<String> {
        if pos < line.len() {
            return None;
        }

        for cmd in &self.commands {
            if cmd.starts_with(line) && cmd != line {
                return Some(cmd[line.len()..].to_string());
            }
        }

        None
    }
}

impl Highlighter for GitpHelper {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        // Simple syntax highlighting
        if self.commands.iter().any(|c| c == line) {
            Cow::Owned(format!("\x1b[1;36m{}\x1b[0m", line)) // Cyan for valid commands
        } else {
            Cow::Borrowed(line)
        }
    }

    fn highlight_char(&self, _line: &str, _pos: usize, _forced: bool) -> bool {
        true
    }
}

impl Validator for GitpHelper {}

impl Helper for GitpHelper {}

pub fn run_interactive_mode() -> rustyline::Result<Vec<String>> {
    let helper = GitpHelper::new();
    let mut rl = Editor::new()?;
    rl.set_helper(Some(helper));

    // Load history
    let history_file = dirs::home_dir()
        .map(|mut path| {
            path.push(".gitp_history");
            path
        })
        .unwrap_or_else(|| std::path::PathBuf::from(".gitp_history"));

    let _ = rl.load_history(&history_file);

    println!("\x1b[1;36mgitp\x1b[0m - Git Multiple Repository Manager");
    println!("Type '\x1b[1;33mhelp\x1b[0m' for available commands, '\x1b[1;33mexit\x1b[0m' to quit\n");

    loop {
        let readline = rl.readline("\x1b[1;36mgitp>\x1b[0m ");
        match readline {
            Ok(line) => {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }

                rl.add_history_entry(trimmed)?;

                match trimmed {
                    "exit" | "quit" => {
                        println!("Goodbye!");
                        break;
                    }
                    "help" | "?" => {
                        show_help();
                    }
                    cmd => {
                        // Parse and return command
                        let parts: Vec<String> = cmd.split_whitespace().map(String::from).collect();

                        // Save history before executing command
                        let _ = rl.save_history(&history_file);

                        return Ok(parts);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("^D");
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }

    let _ = rl.save_history(&history_file);
    Ok(vec![])
}

fn show_help() {
    println!("\n\x1b[1;36mAvailable Commands:\x1b[0m");
    println!("  \x1b[1;33mclone\x1b[0m [serial]        Clone all enabled repositories");
    println!("  \x1b[1;33mpull\x1b[0m [serial]         Pull all enabled repositories");
    println!("  \x1b[1;33mpush\x1b[0m [serial]         Push all enabled repositories");
    println!("  \x1b[1;33mconfig user\x1b[0m [serial]  Set user.name and user.email for all repositories");
    println!("  \x1b[1;33mhelp\x1b[0m, \x1b[1;33m?\x1b[0m              Show this help message");
    println!("  \x1b[1;33mexit\x1b[0m, \x1b[1;33mquit\x1b[0m           Exit interactive mode");
    println!("\n\x1b[1;36mOptions:\x1b[0m");
    println!("  \x1b[1;33mserial\x1b[0m                 Execute sequentially (default: parallel)");
    println!("\n\x1b[1;36mTips:\x1b[0m");
    println!("  - Use \x1b[1;33mTab\x1b[0m for auto-completion");
    println!("  - Use \x1b[1;33m↑/↓\x1b[0m arrows for command history");
    println!("  - Type partial commands: '\x1b[1;33mclo\x1b[0m' + Tab → 'clone'");
    println!();
}
