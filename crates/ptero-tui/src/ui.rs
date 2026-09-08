use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Wrap};
use ratatui::Frame;

use crate::app::{App, Screen};

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(f.area());

    draw_header(f, chunks[0], app);
    match app.screen {
        Screen::Home => draw_home(f, chunks[1], app),
        Screen::Servers => draw_list(
            f,
            chunks[1],
            "Servers",
            &app.servers
                .iter()
                .map(|(id, name)| format!("{id}\t{name}"))
                .collect::<Vec<_>>(),
            app.selected,
        ),
        Screen::Account => draw_text(f, chunks[1], "Account", &app.account_text),
        Screen::AppUsers => draw_list(f, chunks[1], "App Users", &app.users, app.selected),
        Screen::AppNodes => draw_list(f, chunks[1], "App Nodes", &app.nodes, app.selected),
        Screen::Help => draw_help(f, chunks[1], app),
        Screen::Terminal => draw_terminal(f, chunks[1], app),
    }
    draw_status(f, chunks[2], app);
}

fn draw_header(f: &mut Frame, area: Rect, app: &App) {
    let title = match app.screen {
        Screen::Home => "Home",
        Screen::Servers => "Servers",
        Screen::Account => "Account",
        Screen::AppUsers => "App Users",
        Screen::AppNodes => "App Nodes",
        Screen::Help => "Help / Config",
        Screen::Terminal => "Terminal",
    };
    let tabs = " [0]Home [1]Servers [2]Account [3]Users [4]Nodes [5]Help  q=quit ";
    let p = Paragraph::new(vec![
        Line::from(Span::styled(
            format!(" ptero — {title}"),
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(tabs),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Pterodactyl "),
    );
    f.render_widget(p, area);
}

fn draw_home(f: &mut Frame, area: Rect, app: &App) {
    let body = format!(
        "Panel: {}\nClient: {}  Application: {}  Remote: {}\n\n\
         Keys: 1 servers · o/s/r power · Enter terminal · R refresh · 5 help",
        if app.config.panel_url.is_empty() {
            "(unset)"
        } else {
            &app.config.panel_url
        },
        yes(app.config.is_ready_client()),
        yes(app.config.is_ready_application()),
        yes(app.config.is_ready_remote()),
    );
    draw_text(f, area, "Home", &body);
}

fn draw_help(f: &mut Frame, area: Rect, app: &App) {
    let body = format!(
        "Config file: ~/.config/ptero/config.toml\n\
         panel_url / client_api_key / application_api_key / daemon_token\n\n\
         CLI: ptero config set-url|set-client-key|set-app-key|set-daemon-token\n\
         Remote uses Node daemon token id.secret (not ptla/ptlc).\n\n\
         Current URL: {}\n\
         client={} app={} remote={}",
        app.config.panel_url,
        yes(app.config.is_ready_client()),
        yes(app.config.is_ready_application()),
        yes(app.config.is_ready_remote()),
    );
    draw_text(f, area, "Help / Config", &body);
}

fn draw_text(f: &mut Frame, area: Rect, title: &str, text: &str) {
    let p = Paragraph::new(text.to_string())
        .wrap(Wrap { trim: false })
        .block(Block::default().borders(Borders::ALL).title(title));
    f.render_widget(p, area);
}

fn draw_list(f: &mut Frame, area: Rect, title: &str, items: &[String], selected: usize) {
    let list_items: Vec<ListItem> = items
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let style = if i == selected {
                Style::default().add_modifier(Modifier::REVERSED)
            } else {
                Style::default()
            };
            ListItem::new(s.as_str()).style(style)
        })
        .collect();
    let list = List::new(list_items).block(Block::default().borders(Borders::ALL).title(title));
    f.render_widget(list, area);
}

fn draw_terminal(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(3)])
        .split(area);
    let text = app.term_lines.join("\n");
    draw_text(f, chunks[0], "Console", &text);
    let input = Paragraph::new(format!("> {}", app.input)).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Input (Enter send, Esc back)"),
    );
    f.render_widget(input, chunks[1]);
}

fn draw_status(f: &mut Frame, area: Rect, app: &App) {
    let p = Paragraph::new(app.status.as_str())
        .block(Block::default().borders(Borders::ALL).title("Status"));
    f.render_widget(p, area);
}

fn yes(v: bool) -> &'static str {
    if v {
        "ok"
    } else {
        "—"
    }
}
