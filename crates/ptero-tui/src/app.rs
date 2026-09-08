use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ptero_config::Config;
use ptero_core::{
    AccountService, AppContext, ApplicationService, ConsoleEvent, ServerService, TerminalService,
};
use ptero_protocol::server::PowerSignal;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::{self, Stdout};
use std::time::Duration;

use crate::ui;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Home,
    Servers,
    Account,
    AppUsers,
    AppNodes,
    Help,
    Terminal,
}

pub struct App {
    pub screen: Screen,
    pub ctx: Option<AppContext>,
    pub config: Config,
    pub status: String,
    pub servers: Vec<(String, String)>, // identifier, name
    pub account_text: String,
    pub users: Vec<String>,
    pub nodes: Vec<String>,
    pub selected: usize,
    pub input: String,
    pub term_lines: Vec<String>,
    pub term_server: Option<String>,
    pub should_quit: bool,
}

impl App {
    pub fn new(ctx: Option<AppContext>, config: Config) -> Self {
        let status = if ctx.is_some() {
            format!("Connected: {}", config.panel_url)
        } else if config.panel_url.is_empty() {
            "Not configured. Press 5 for help. Set URL via `ptero config set-url`.".into()
        } else {
            format!("Config present but context failed for {}", config.panel_url)
        };
        Self {
            screen: Screen::Home,
            ctx,
            config,
            status,
            servers: Vec::new(),
            account_text: String::new(),
            users: Vec::new(),
            nodes: Vec::new(),
            selected: 0,
            input: String::new(),
            term_lines: Vec::new(),
            term_server: None,
            should_quit: false,
        }
    }
}

pub async fn run_app(ctx: Option<AppContext>, config: Config) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(ctx, config);
    let res = event_loop(&mut terminal, &mut app).await;

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    res
}

async fn event_loop(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app: &mut App,
) -> Result<()> {
    refresh_current(app).await;

    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                match key.code {
                    KeyCode::Char('q') if app.screen != Screen::Terminal => {
                        app.should_quit = true;
                    }
                    KeyCode::Char('0') => {
                        app.screen = Screen::Home;
                    }
                    KeyCode::Char('1') => {
                        app.screen = Screen::Servers;
                        refresh_current(app).await;
                    }
                    KeyCode::Char('2') => {
                        app.screen = Screen::Account;
                        refresh_current(app).await;
                    }
                    KeyCode::Char('3') => {
                        app.screen = Screen::AppUsers;
                        refresh_current(app).await;
                    }
                    KeyCode::Char('4') => {
                        app.screen = Screen::AppNodes;
                        refresh_current(app).await;
                    }
                    KeyCode::Char('5') | KeyCode::Char('h') | KeyCode::Char('?') => {
                        app.screen = Screen::Help;
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        let len = list_len(app);
                        if len > 0 {
                            app.selected = (app.selected + 1).min(len - 1);
                        }
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        if app.selected > 0 {
                            app.selected -= 1;
                        }
                    }
                    KeyCode::Enter if app.screen == Screen::Servers => {
                        if let Some((id, _)) = app.servers.get(app.selected).cloned() {
                            app.term_server = Some(id.clone());
                            app.screen = Screen::Terminal;
                            app.term_lines.clear();
                            app.status = format!("Terminal {id}");
                            attach_and_drain(app).await;
                        }
                    }
                    KeyCode::Char('o') if app.screen == Screen::Servers => {
                        power_action(app, PowerSignal::Start).await;
                    }
                    KeyCode::Char('s') if app.screen == Screen::Servers => {
                        power_action(app, PowerSignal::Stop).await;
                    }
                    KeyCode::Char('r') if app.screen == Screen::Servers => {
                        power_action(app, PowerSignal::Restart).await;
                    }
                    KeyCode::Char('R') if app.screen != Screen::Terminal => {
                        refresh_current(app).await;
                    }
                    KeyCode::Esc if app.screen == Screen::Terminal => {
                        app.screen = Screen::Servers;
                    }
                    KeyCode::Char(c) if app.screen == Screen::Terminal => {
                        app.input.push(c);
                    }
                    KeyCode::Backspace if app.screen == Screen::Terminal => {
                        app.input.pop();
                    }
                    KeyCode::Enter if app.screen == Screen::Terminal => {
                        let cmd = std::mem::take(&mut app.input);
                        send_term_command(app, &cmd).await;
                    }
                    _ => {}
                }
            }
        }

        if app.should_quit {
            break;
        }
    }
    Ok(())
}

fn list_len(app: &App) -> usize {
    match app.screen {
        Screen::Servers => app.servers.len(),
        Screen::AppUsers => app.users.len(),
        Screen::AppNodes => app.nodes.len(),
        _ => 0,
    }
}

async fn refresh_current(app: &mut App) {
    let Some(ctx) = app.ctx.clone() else {
        app.status = "No authenticated context".into();
        return;
    };
    match app.screen {
        Screen::Servers => match ServerService(&ctx).list(&[]).await {
            Ok(list) => {
                app.servers = list
                    .data
                    .into_iter()
                    .map(|i| (i.attributes.identifier.clone(), i.attributes.name.clone()))
                    .collect();
                app.status = format!("{} servers", app.servers.len());
                app.selected = app.selected.min(app.servers.len().saturating_sub(1));
            }
            Err(e) => app.status = format!("servers error: {e}"),
        },
        Screen::Account => match AccountService(&ctx).get().await {
            Ok(acc) => {
                let a = acc.attributes;
                app.account_text = format!(
                    "id={} username={} email={} admin={}",
                    a.id, a.username, a.email, a.admin
                );
                app.status = "account loaded".into();
            }
            Err(e) => {
                app.account_text = format!("error: {e}");
                app.status = "account error".into();
            }
        },
        Screen::AppUsers => match ApplicationService(&ctx).list_users(&[]).await {
            Ok(list) => {
                app.users = list
                    .data
                    .into_iter()
                    .map(|u| {
                        format!(
                            "{}\t{}\t{}",
                            u.attributes.id, u.attributes.username, u.attributes.email
                        )
                    })
                    .collect();
                app.status = format!("{} users", app.users.len());
            }
            Err(e) => app.status = format!("users error: {e}"),
        },
        Screen::AppNodes => match ApplicationService(&ctx).list_nodes(&[]).await {
            Ok(list) => {
                app.nodes = list
                    .data
                    .into_iter()
                    .map(|n| {
                        format!(
                            "{}\t{}\t{}",
                            n.attributes.id, n.attributes.name, n.attributes.fqdn
                        )
                    })
                    .collect();
                app.status = format!("{} nodes", app.nodes.len());
            }
            Err(e) => app.status = format!("nodes error: {e}"),
        },
        _ => {}
    }
}

async fn power_action(app: &mut App, signal: PowerSignal) {
    let Some(ctx) = app.ctx.clone() else {
        app.status = "No context".into();
        return;
    };
    let Some((id, _)) = app.servers.get(app.selected).cloned() else {
        return;
    };
    match ServerService(&ctx).power(&id, signal).await {
        Ok(()) => app.status = format!("{} → {}", id, signal.as_str()),
        Err(e) => app.status = format!("power error: {e}"),
    }
}

async fn attach_and_drain(app: &mut App) {
    let Some(ctx) = app.ctx.clone() else {
        app.term_lines.push("No context".into());
        return;
    };
    let Some(server) = app.term_server.clone() else {
        return;
    };
    match TerminalService(&ctx).attach(&server).await {
        Ok(mut console) => {
            app.term_lines.push(format!("[attached {server}]"));
            // Drain a burst of initial events (non-interactive snapshot).
            for _ in 0..50 {
                match tokio::time::timeout(Duration::from_millis(50), console.next()).await {
                    Ok(Some(ConsoleEvent::ConsoleOutput(line))) => {
                        app.term_lines.push(line);
                        if app.term_lines.len() > 500 {
                            app.term_lines.drain(0..100);
                        }
                    }
                    Ok(Some(ConsoleEvent::Status(s))) => {
                        app.term_lines.push(format!("[status] {s}"));
                    }
                    Ok(Some(ConsoleEvent::AuthSuccess)) => {
                        app.term_lines.push("[auth] ok".into());
                    }
                    Ok(Some(ConsoleEvent::Closed)) | Ok(None) => break,
                    Ok(_) => {}
                    Err(_) => break,
                }
            }
            // Keep console open for subsequent Enter sends via re-attach each command
            // (simple TUI model — full duplex session would need stored console).
            let _ = console.close().await;
            app.status = "Terminal snapshot loaded; Enter sends via re-attach".into();
        }
        Err(e) => {
            app.term_lines.push(format!("attach failed: {e}"));
            app.status = format!("attach error: {e}");
        }
    }
}

async fn send_term_command(app: &mut App, cmd: &str) {
    if cmd.is_empty() {
        return;
    }
    let Some(ctx) = app.ctx.clone() else {
        return;
    };
    let Some(server) = app.term_server.clone() else {
        return;
    };
    // Prefer Client HTTP command endpoint for reliability from TUI.
    match ServerService(&ctx).command(&server, cmd).await {
        Ok(()) => {
            app.term_lines.push(format!("> {cmd}"));
            app.status = "command sent".into();
        }
        Err(e) => {
            app.term_lines.push(format!("command error: {e}"));
            app.status = format!("command error: {e}");
        }
    }
}
