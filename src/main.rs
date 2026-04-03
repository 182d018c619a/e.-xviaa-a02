mod signatures; // Menghubungkan ke file signatures.rs

use signatures::{load_signatures, Severity};
use reqwest::{Client, StatusCode};
use std::sync::Arc;
use tokio::sync::Semaphore;
use futures::stream::{self, StreamExt};
use colored::*;
use regex::Regex;

const MAX_CONCURRENT_REQUESTS: usize = 50; 

#[tokio::main]
async fn main() {
    let target_base = "http://target-web.com"; // Ganti dengan targetmu
    
    let wordlist = vec![
        ".env", ".git/config", "app/config/database.yml", 
        "storage/logs/laravel.log", "wp-config.php.bak", "id_rsa"
    ];

    let client = Arc::new(
        Client::builder()
            .danger_accept_invalid_certs(true)
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) RustySentinel/1.1")
            .build()
            .unwrap()
    );

    let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_REQUESTS));

    println!("\n{}", "===================================================".cyan());
    println!("{}", "       🛡️  RUSTY-SENTINEL: ADAPTIVE SCANNER       ".cyan().bold());
    println!("{}\n", "===================================================".cyan());

    let scan_tasks = stream::iter(wordlist).map(|path| {
        let client = Arc::clone(&client);
        let sem = Arc::clone(&semaphore);
        let url = format!("{}/{}", target_base, path);

        async move {
            let _permit = sem.acquire().await.unwrap();
            match client.get(&url).send().await {
                Ok(resp) => {
                    if resp.status().is_success() {
                        process_found_file(&url, resp).await;
                    }
                }
                Err(_) => {} // Abaikan error koneksi untuk kecepatan
            }
        }
    });

    scan_tasks.buffer_unordered(MAX_CONCURRENT_REQUESTS).collect::<()>().await;
    
    println!("\n{}", "[-] Scan Completed.".bright_black());
}

async fn process_found_file(url: &str, response: reqwest::Response) {
    println!("[{}] File Found: {}", "200".green().bold(), url);
    
    // Ambil isi file
    if let Ok(content) = response.text().await {
        let signatures = load_signatures();
        
        for sig in signatures {
            let re = Regex::new(sig.pattern).unwrap();
            
            if re.is_match(&content) {
                let sev_label = match sig.severity {
                    Severity::Critical => "CRITICAL".red().bold().blink(),
                    Severity::High => "HIGH".red(),
                    Severity::Medium => "MEDIUM".yellow(),
                    Severity::Low => "LOW".blue(),
                };

                println!(
                    "    {} {} detected!", 
                    "->".bright_white(), 
                    sev_label
                );
                println!("       Name: {}", sig.name.bright_white());
                println!("       Desc: {}", sig.description.italic().bright_black());
            }
        }
    }
}
