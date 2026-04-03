use std::{io, sync::Arc, time::Duration};

use tokio::sync::{mpsc, Semaphore};
use futures::stream::{self, StreamExt};

use reqwest::Client;
use regex::Regex;
use once_cell::sync::Lazy;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

/// =============================
/// CONFIG
/// =============================
const MAX_CONCURRENT_REQUESTS: usize = 50;

/// =============================
/// SEVERITY
/// =============================
#[derive(Debug, Clone, Copy)]
enum Severity {
    Critical,
    High,
    Medium,
    Low,
}

/// =============================
/// SIGNATURE STRUCT
/// =============================
struct Signature {
    name: &'static str,
    pattern: &'static str,
    severity: Severity,
}

/// COMPILED SIGNATURE
struct CompiledSignature {
    name: &'static str,
    regex: Regex,
    severity: Severity,
}

/// FINDING RESULT
#[derive(Clone)]
struct Finding {
    url: String,
    name: String,
    severity: Severity,
}

/// =============================
/// SIGNATURE DATABASE
/// =============================
fn load_signatures() -> Vec<Signature> {
    vec![
        Signature {
            name: "AWS Key",
            pattern: r"AKIA[0-9A-Z]{16}",
            severity: Severity::Critical,
        },
        Signature {
            name: "Google API",
            pattern: r"AIza[0-9A-Za-z\-_]{35}",
            severity: Severity::High,
        },
        Signature {
            name: "DB URL",
            pattern: r"(mysql|postgres|mongodb)://[^\\s]+",
            severity: Severity::Critical,
        },
        Signature {
            name: "Env Secret",
            pattern: r"(?i)(DB_PASS|SECRET_KEY)=",
            severity: Severity::High,
        },
        Signature {
            name: "Private Key",
            pattern: r"-----BEGIN RSA PRIVATE KEY-----",
            severity: Severity::Critical,
        },
    ]
}

/// =============================
/// REGEX CACHE (IMPORTANT)
/// =============================
static SIGNATURES: Lazy<Vec<CompiledSignature>> = Lazy::new(|| {
    load_signatures()
        .into_iter()
        .filter_map(|s| {
            Regex::new(s.pattern).ok().map(|r| CompiledSignature {
                name: s.name,
                regex: r,
                severity: s.severity,
            })
        })
        .collect()
});

/// =============================
/// SCAN LOGIC
/// =============================
fn scan_content(body: &str, url: &str) -> Vec<Finding> {
    let mut results = Vec::new();

    for sig in SIGNATURES.iter() {
        if sig.regex.is_match(body) {
            results.push(Finding {
                url: url.to_string(),
                name: sig.name.to_string(),
                severity: sig.severity,
            });
        }
    }

    results
}

/// =============================
/// UI STATE
/// =============================
struct App {
    findings: Vec<Finding>,
    scanned: usize,
    total: usize,
}

impl App {
    fn new(total: usize) -> Self {
        Self {
            findings: vec![],
            scanned: 0,
            total,
        }
    }
}

/// =============================
/// MAIN
/// =============================
#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let target = "http://target-web.com";

    let wordlist = vec![
        ".env",
        ".git/config",
        "wp-config.php.bak",
        "id_rsa",
    ];

    let client = Arc::new(Client::new());
    let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_REQUESTS));

    let (tx, mut rx) = mpsc::channel(100);

    /// SPAWN SCANNER
    tokio::spawn({
        let client = client.clone();
        let semaphore = semaphore.clone();
        let wordlist = wordlist.clone();
        let tx = tx.clone();
        let target = target.to_string();

        async move {
            stream::iter(wordlist)
                .map(|path| {
                    let client = client.clone();
                    let sem = semaphore.clone();
                    let tx = tx.clone();
                    let url = format!("{}/{}", target, path);

                    async move {
                        let _permit = sem.acquire().await.unwrap();

                        if let Ok(resp) = client.get(&url).send().await {
                            if let Ok(text) = resp.text().await {
                                let findings = scan_content(&text, &url);

                                for f in findings {
                                    let _ = tx.send(f).await;
                                }
                            }
                        }

                        // progress tick
                        let _ = tx.send(Finding {
                            url: "__progress__".into(),
                            name: "".into(),
                            severity: Severity::Low,
                        }).await;
                    }
                })
                .buffer_unordered(MAX_CONCURRENT_REQUESTS)
                .collect::<()>()
                .await;
        }
    });

    /// START UI
    start_ui(wordlist.len(), &mut rx).await?;

    Ok(())
}

/// =============================
/// UI LOOP
/// =============================
async fn start_ui(total: usize, rx: &mut mpsc::Receiver<Finding>) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(total);

    loop {
        while let Ok(msg) = rx.try_recv() {
            if msg.url == "__progress__" {
                app.scanned += 1;
            } else {
                app.findings.push(msg);
            }
        }

        terminal.draw(|f| draw_ui(f, &app))?;

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

/// =============================
/// DRAW UI
/// =============================
fn draw_ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(5),
        ])
        .split(f.size());

    // HEADER
    let header = Paragraph::new("🛡️ RUSTY SENTINEL")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(header, chunks[0]);

    // PROGRESS
    let progress = Paragraph::new(format!("Scanned: {}/{}", app.scanned, app.total))
        .block(Block::default().title("Progress").borders(Borders::ALL));

    f.render_widget(progress, chunks[1]);

    // FINDINGS
    let items: Vec<ListItem> = app
        .findings
        .iter()
        .rev()
        .take(20)
        .map(|f| {
            let color = match f.severity {
                Severity::Critical => Color::Red,
                Severity::High => Color::LightRed,
                Severity::Medium => Color::Yellow,
                Severity::Low => Color::Blue,
            };

            ListItem::new(format!("{} | {}", f.name, f.url))
                .style(Style::default().fg(color))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().title("Findings").borders(Borders::ALL));

    f.render_widget(list, chunks[2]);
}
