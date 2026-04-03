use reqwest::{Client, StatusCode};
use std::sync::Arc;
use tokio::sync::Semaphore;
use futures::stream::{self, StreamExt};
use colored::*;

// Konfigurasi performa
const MAX_CONCURRENT_REQUESTS: usize = 100; // Menembak 100 URL sekaligus

#[tokio::main]
async fn main() {
    let target_base = "http://target-web.com";
    
    // List wordlist yang sangat banyak (bisa ribuan)
    let wordlist = vec![
        ".env", ".git/config", "app/config/database.yml", 
        "storage/logs/laravel.log", "wp-config.php.bak"
    ];

    let client = Arc::new(
        Client::builder()
            .danger_accept_invalid_certs(true)
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) RustySentinel/1.0")
            .build()
            .unwrap()
    );

    // Semaphore untuk mengontrol traffic agar tidak dianggap DDoS
    let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_REQUESTS));

    println!("{}", "[-] Sentinel Mode: Aggressive Scanning Initialized...".yellow());

    let scan_tasks = stream::iter(wordlist).map(|path| {
        let client = Arc::clone(&client);
        let sem = Arc::clone(&semaphore);
        let url = format!("{}/{}", target_base, path);

        async move {
            let _permit = sem.acquire().await.unwrap(); // Tunggu antrian jika sudah 100 request berjalan
            match client.get(&url).send().await {
                Ok(resp) => {
                    let status = resp.status();
                    if status.is_success() {
                        handle_success(&url, resp).await;
                    } else if status == StatusCode::FORBIDDEN {
                        println!("[{}] Access Denied: {}", "403".yellow(), url);
                    }
                }
                Err(e) => eprintln!("[{}] Connection Error: {}", "ERR".red(), e),
            }
        }
    });

    // Jalankan semua task secara paralel
    scan_tasks.buffer_unordered(MAX_CONCURRENT_REQUESTS).collect::<()>().await;
}

async fn handle_success(url: &str, response: reqwest::Response) {
    println!("[{}] POTENTIAL LEAK: {}", "200 OK".green().bold(), url);
    
    // Kompleksitas tambahan: Jika ketemu 200 OK, langsung scan isinya (Content Inspection)
    if let Ok(body) = response.text().await {
        if body.contains("AWS_SECRET") || body.contains("DB_PASSWORD") {
            println!("    {} Sensitive data detected inside the file!", "-> ALERT:".red().blink());
        }
    }
}
