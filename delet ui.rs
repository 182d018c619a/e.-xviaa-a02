use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Paragraph},
    layout::{Layout, Constraint, Direction},
    style::{Style, Color, Modifier},
};
use crossterm::{event::{self, Event, KeyCode}, execute};
use tokio::sync::mpsc;
use std::io;
use std::time::Duration;

pub async fn start_ui(rx: &mut mpsc::Receiver<super::scanner::ScanMessage>, total: usize) -> io::Result<()> {
    use crossterm::terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut findings: Vec<super::scanner::FindingResult> = Vec::new();
    let mut scanned = 0;

    loop {
        // Receive scan results or progress updates
        while let Ok(msg) = rx.try_recv() {
            match msg {
                super::scanner::ScanMessage::Finding(finding) => {
                    findings.push(finding);
                }
                super::scanner::ScanMessage::Progress => {
                    scanned += 1;
                }
            }
        }

        // Draw UI
        terminal.draw(|f| {
            let size = f.size();
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Length(3), Constraint::Min(5)])
                .split(size);

            // Header
            let header = Paragraph::new("🛡️ Rusty Sentinel - File Scanner")
                .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(header, layout[0]);

            // Progress
            let progress = Paragraph::new(format!("Scanned: {}/{}", scanned, total))
                .block(Block::default().title("Progress").borders(Borders::ALL));
            f.render_widget(progress, layout[1]);

            // Findings List
            let items: Vec<ListItem> = findings.iter()
                .rev()
                .take(20) // Only show top 20 latest findings
                .map(|f| ListItem::new(format!(
                    "{} | {} | {}",
                    match f.severity {
                        super::scanner::Severity::Critical => format!("[CRITICAL] {}", f.name).red(),
                        super::scanner::Severity::High => format!("[HIGH] {}", f.name).light_red(),
                        super::scanner::Severity::Medium => format!("[MEDIUM] {}", f.name).yellow(),
                        super::scanner::Severity::Low => format!("[LOW] {}", f.name).blue(),
                    },
                    f.severity,
                    f.matched,
                )).style(Style::default().fg(match f.severity {
                    super::scanner::Severity::Critical => Color::Red,
                    super::scanner::Severity::High => Color::LightRed,
                    super::scanner::Severity::Medium => Color::Yellow,
                    super::scanner::Severity::Low => Color::Blue,
                })))
                .collect();

            let list = List::new(items)
                .block(Block::default().title("Findings").borders(Borders::ALL));

            f.render_widget(list, layout[2]);
        })?;

        // Handle user input for exiting
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
