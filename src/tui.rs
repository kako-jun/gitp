use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::io;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Debug, Clone, PartialEq)]
pub enum RepoStatus {
    Pending,
    Running,
    Success,
    Failed,
}

#[derive(Debug, Clone)]
pub struct RepoProgress {
    pub name: String,
    pub status: RepoStatus,
    pub message: String,
    pub progress: u16, // 0-100
}

pub struct TuiApp {
    repos: Arc<Mutex<Vec<RepoProgress>>>,
}

impl TuiApp {
    pub fn new(repo_names: Vec<String>) -> Self {
        let repos = repo_names
            .into_iter()
            .map(|name| RepoProgress {
                name,
                status: RepoStatus::Pending,
                message: "Waiting...".to_string(),
                progress: 0,
            })
            .collect();

        TuiApp {
            repos: Arc::new(Mutex::new(repos)),
        }
    }

    pub fn get_repos_handle(&self) -> Arc<Mutex<Vec<RepoProgress>>> {
        Arc::clone(&self.repos)
    }

    pub fn run(&mut self, is_parallel: bool) -> Result<(), io::Error> {
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Run the app
        let res = self.run_app(&mut terminal, is_parallel);

        // Restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        if let Err(err) = res {
            println!("{err:?}");
        }

        Ok(())
    }

    fn run_app<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
        _is_parallel: bool,
    ) -> io::Result<()> {
        loop {
            terminal.draw(|f| self.ui(f))?;

            // Check if all repos are done
            let repos = self.repos.lock().unwrap();
            let all_done = repos
                .iter()
                .all(|r| r.status == RepoStatus::Success || r.status == RepoStatus::Failed);
            drop(repos);

            if all_done {
                // Show final state for 1 second
                std::thread::sleep(Duration::from_secs(1));
                break;
            }

            // Poll for events (non-blocking with timeout)
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if let KeyCode::Char('q') = key.code {
                        break;
                    }
                }
            }
        }

        Ok(())
    }

    fn ui(&self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(f.area());

        // Header
        let header = Paragraph::new(vec![Line::from(vec![
            Span::styled(
                "gitp",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" - Git Multiple Repository Manager"),
        ])])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default()),
        );
        f.render_widget(header, chunks[0]);

        // Repository list
        self.render_repos(f, chunks[1]);

        // Footer
        let repos = self.repos.lock().unwrap();
        let total = repos.len();
        let completed = repos
            .iter()
            .filter(|r| r.status == RepoStatus::Success || r.status == RepoStatus::Failed)
            .count();
        let success = repos
            .iter()
            .filter(|r| r.status == RepoStatus::Success)
            .count();
        let failed = repos
            .iter()
            .filter(|r| r.status == RepoStatus::Failed)
            .count();
        drop(repos);

        let footer = Paragraph::new(Line::from(vec![
            Span::styled("Total: ", Style::default().fg(Color::White)),
            Span::styled(format!("{total} "), Style::default().fg(Color::Cyan)),
            Span::raw("| "),
            Span::styled("Completed: ", Style::default().fg(Color::White)),
            Span::styled(format!("{completed} "), Style::default().fg(Color::Yellow)),
            Span::raw("| "),
            Span::styled("Success: ", Style::default().fg(Color::White)),
            Span::styled(format!("{success} "), Style::default().fg(Color::Green)),
            Span::raw("| "),
            Span::styled("Failed: ", Style::default().fg(Color::White)),
            Span::styled(format!("{failed} "), Style::default().fg(Color::Red)),
            Span::raw("| "),
            Span::styled(
                "Press 'q' to force quit",
                Style::default().fg(Color::DarkGray),
            ),
        ]))
        .block(Block::default().borders(Borders::ALL));
        f.render_widget(footer, chunks[2]);
    }

    fn render_repos(&self, f: &mut Frame, area: Rect) {
        let repos = self.repos.lock().unwrap();

        let mut lines = vec![];
        for repo in repos.iter() {
            let (status_icon, status_color) = match repo.status {
                RepoStatus::Pending => ("⏸", Color::DarkGray),
                RepoStatus::Running => ("⚙", Color::Yellow),
                RepoStatus::Success => ("✓", Color::Green),
                RepoStatus::Failed => ("✗", Color::Red),
            };

            lines.push(Line::from(vec![
                Span::styled(
                    format!(" {status_icon} "),
                    Style::default()
                        .fg(status_color)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("{:40}", repo.name),
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!(" {}", repo.message),
                    Style::default().fg(Color::White),
                ),
            ]));

            // Progress bar line
            let bar_width = 50;
            let filled = (bar_width as f32 * repo.progress as f32 / 100.0) as usize;
            let empty = bar_width - filled;
            let bar = format!(
                " [{}{}] {}%",
                "█".repeat(filled),
                "░".repeat(empty),
                repo.progress
            );

            lines.push(Line::from(Span::styled(
                bar,
                Style::default().fg(match repo.status {
                    RepoStatus::Success => Color::Green,
                    RepoStatus::Failed => Color::Red,
                    RepoStatus::Running => Color::Yellow,
                    RepoStatus::Pending => Color::DarkGray,
                }),
            )));

            lines.push(Line::from("")); // Empty line between repos
        }

        let paragraph = Paragraph::new(lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Repositories ")
                    .style(Style::default()),
            )
            .style(Style::default().bg(Color::Black));

        f.render_widget(paragraph, area);
    }
}

pub fn update_repo_status(
    repos: &Arc<Mutex<Vec<RepoProgress>>>,
    repo_name: &str,
    status: RepoStatus,
    message: &str,
    progress: u16,
) {
    let mut repos = repos.lock().unwrap();
    if let Some(repo) = repos.iter_mut().find(|r| r.name == repo_name) {
        repo.status = status;
        repo.message = message.to_string();
        repo.progress = progress;
    }
}
