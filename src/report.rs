use std::fs::OpenOptions;
use std::io::Write;
use chrono::Local; // Kita butuh library chrono untuk timestamp

pub fn save_finding(url: &str, name: &str, severity: &str) {
    let datetime = Local::now().format("%Y-%m-%d %H:%M:%S");
    let log_entry = format!(
        "[{}] SEVERITY: {} | URL: {} | FINDING: {}\n",
        datetime, severity, url, name
    );

    // Membuka file 'results.txt', buat jika belum ada, tambahkan di baris baru (append)
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("results.txt")
        .expect("Gagal membuka file laporan");

    if let Err(e) = writeln!(file, "{}", log_entry) {
        eprintln!("Gagal menulis laporan: {}", e);
    }
}
