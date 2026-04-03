use std::{env, process};
use tokio::sync::mpsc;
use futures::executor::block_on;

mod scanner;
mod ui;
mod signatures;

#[tokio::main]
async fn main() {
    // Ambil URL target dan wordlist dari argumen atau hardcode
    let args: Vec<String> = env::args().collect();
    let target_base = if args.len() > 1 { &args[1] } else { "http://target-web.com" };
    let wordlist = vec![
        ".env", ".git/config", "app/config/database.yml",
        "storage/logs/laravel.log", "wp-config.php.bak", "id_rsa"
    ];

    // Membuat channel untuk komunikasi antara pemindaian dan UI
    let (tx, mut rx) = mpsc::channel(100);

    // Menjalankan UI (Thread UI)
    let ui_handle = tokio::spawn(async move {
        ui::start_ui(&mut rx, wordlist.len()).await.unwrap();
    });

    // Menjalankan Pemindaian (Thread Scan)
    let scan_handle = tokio::spawn(async move {
        // Mulai pemindaian
        scanner::run_scan(target_base.to_string(), wordlist, tx).await;
    });

    // Tunggu kedua thread selesai
    let _ = tokio::try_join!(ui_handle, scan_handle);

    println!("Scan completed!");
    process::exit(0);
}
