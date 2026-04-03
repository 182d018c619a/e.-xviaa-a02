use std::sync::Arc;
use std::time::Duration;

use futures::stream::{self, StreamExt};
use reqwest::{Client, Response};
use tokio::sync::{mpsc, Semaphore};

use crate::signatures::{scan_content, Finding, Severity};

/// =============================
/// CONFIG
/// =============================
const MAX_CONCURRENT_REQUESTS: usize = 50;
const MAX_BODY_SIZE: usize = 100_000; // limit read (anti memory blow)

/// =============================
/// MESSAGE TYPE (CHANNEL)
/// =============================
#[derive(Debug)]
pub enum ScanMessage {
    Finding(FindingResult),
    Progress,
}

/// =============================
/// FINAL RESULT STRUCT
/// =============================
#[derive(Debug, Clone)]
pub struct FindingResult {
    pub url: String,
    pub name: String,
    pub severity: Severity,
    pub matched: String,
}

/// =============================
/// BUILD HTTP CLIENT
/// =============================
pub fn build_client() -> Arc<Client> {
    Arc::new(
        Client::builder()
            .timeout(Duration::from_secs(10))
            .danger_accept_invalid_certs(true)
            .user_agent("RustySentinel/3.0 (Pentest Scanner)")
            .build()
            .expect("Failed to build client"),
    )
}

/// =============================
/// MAIN SCAN FUNCTION
/// =============================
pub async fn run_scan(
    base_url: String,
    wordlist: Vec<&str>,
    tx: mpsc::Sender<ScanMessage>,
) {
    let client = build_client();
    let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_REQUESTS));

    stream::iter(wordlist)
        .map(|path| {
            let client = client.clone();
            let sem = semaphore.clone();
            let tx = tx.clone();

            let url = format!("{}/{}", base_url, path);

            async move {
                let _permit = sem.acquire().await.unwrap();

                match fetch(&client, &url).await {
                    Some(resp) => {
                        process_response(resp, &url, &tx).await;
                    }
                    None => {}
                }

                let _ = tx.send(ScanMessage::Progress).await;
            }
        })
        .buffer_unordered(MAX_CONCURRENT_REQUESTS)
        .collect::<()>()
        .await;
}

/// =============================
/// FETCH URL
/// =============================
async fn fetch(client: &Client, url: &str) -> Option<Response> {
    match client.get(url).send().await {
        Ok(resp) if resp.status().is_success() => Some(resp),
        _ => None,
    }
}

/// =============================
/// PROCESS RESPONSE
/// =============================
async fn process_response(
    resp: Response,
    url: &str,
    tx: &mpsc::Sender<ScanMessage>,
) {
    // --- HEADER CHECK (MISCONFIG)
    check_headers(&resp, url, tx).await;

    // --- BODY SCAN
    if let Ok(text) = resp.text().await {
        let limited: String = text.chars().take(MAX_BODY_SIZE).collect();

        let findings = scan_content(&limited);

        for f in findings {
            let _ = tx.send(ScanMessage::Finding(FindingResult {
                url: url.to_string(),
                name: f.name.to_string(),
                severity: f.severity,
                matched: mask(&f.matched),
            })).await;
        }
    }
}

/// =============================
/// HEADER ANALYSIS (IMPORTANT)
/// =============================
async fn check_headers(
    resp: &Response,
    url: &str,
    tx: &mpsc::Sender<ScanMessage>,
) {
    let headers = resp.headers();

    if let Some(server) = headers.get("server") {
        let _ = tx.send(ScanMessage::Finding(FindingResult {
            url: url.to_string(),
            name: "Server Header Exposure".into(),
            severity: Severity::Low,
            matched: format!("{:?}", server),
        })).await;
    }

    if let Some(powered) = headers.get("x-powered-by") {
        let _ = tx.send(ScanMessage::Finding(FindingResult {
            url: url.to_string(),
            name: "X-Powered-By Exposure".into(),
            severity: Severity::Low,
            matched: format!("{:?}", powered),
        })).await;
    }
}

/// =============================
/// MASK SENSITIVE DATA
/// =============================
fn mask(value: &str) -> String {
    if value.len() <= 8 {
        return "****".into();
    }

    let start = &value[..4];
    let end = &value[value.len() - 4..];

    format!("{}****{}", start, end)
}
