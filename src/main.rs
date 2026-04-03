use std::{env, process};
use tokio::sync::mpsc;
use colored::*;

// Mendaftarkan modul-modul internal
mod scanner;
mod ui;
mod signatures;
mod report;

#[tokio::main]
async fn main() {
    // 1. Mengambil URL target dari argumen CLI
    // Contoh: cargo run -- https://example.com
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("\n{}", "===============================================".cyan());
        println!("{}", "   e-xviaa-a02 Security Scanner".bold().yellow());
        println!("{}", "===============================================".cyan());
        println!("Usage: cargo run -- <TARGET_URL>");
        println!("Example: cargo run -- https://testphp.vulnweb.com");
        process::exit(1);
    }

    let target_base = args[1].trim_end_matches('/').to_string();

    // 2. Daftar path sensitif (Wordlist)
    let wordlist = vec![
        ".env", 
        ".git/config", 
        "app/config/database.yml",
        "storage/logs/laravel.log", 
        "wp-config.php.bak", 
        "id_rsa",
        ".htaccess",
        "robots.txt"
    ];

    // 3. Membuat MPSC Channel untuk komunikasi antar thread
    // tx: Transmitter (pengirim dari scanner)
    // rx: Receiver (penerima di UI)
    let (tx, mut rx) = mpsc::channel(100);

    println!("\n{} Target: {}", "[*]".blue(), target_base.bold());
    println!("{} Loaded {} paths to scan\n", "[*]".blue(), wordlist.len());

    // 4. Spawning Thread UI (Task untuk menampilkan progres)
    let ui_handle = tokio::spawn(async move {
        ui::start_ui(&mut rx, wordlist.len()).await;
    });

    // 5. Spawning Thread Scanner (Task untuk melakukan HTTP Request)
    let scan_tx = tx.clone();
    let scan_handle = tokio::spawn(async move {
        scanner::run_scan(target_base, wordlist, scan_tx).await;
    });

    // 6. Menunggu kedua task selesai (Join)
    let _ = tokio::join!(ui_handle, scan_handle);

    println!("\n{}", "===============================================".cyan());
    println!("{}", "Scan Completed! Check results.txt for findings.".green().bold());
    println!("{}", "===============================================".cyan());
    
    process::exit(0);
}
