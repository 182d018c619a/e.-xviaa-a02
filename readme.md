# e.-xviaa-a02: High-Performance Asynchronous Security Scanner

[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Arch%20Linux%20%7C%20Linux%20%7C%20macOS-lightgrey.svg)](https://www.archlinux.org/)

**e.-xviaa-a02** adalah instrumen audit keamanan berbasis Rust yang dioptimalkan untuk pemindaian jalur sensitif (*sensitive path discovery*) dan deteksi kebocoran informasi (*information exposure*). Menggunakan paradigma pemrograman asinkron, alat ini mampu melakukan inspeksi massal terhadap infrastruktur web dengan footprint memori yang minimal namun dengan throughput yang maksimal.

---

## 1. Spesifikasi Teknis dan Core Engine

### 1.1 Asynchronous Runtime
Dibangun di atas runtime **Tokio**, e.-xviaa-a02 mengimplementasikan model I/O non-blocking yang memungkinkan penanganan ribuan koneksi konkuren secara simultan tanpa overhead thread sistem operasi yang besar.

### 1.2 Concurrency Control
Sistem menggunakan modul `std::sync::Arc` untuk berbagi state antar thread secara aman dan `tokio::sync::Semaphore` sebagai mekanisme *rate-limiting*. Hal ini memastikan beban request tetap terkendali dan mencegah *socket exhaustion* pada sistem host (Arch Linux).

### 1.3 Heuristic Pattern Matching
Engine deteksi menggunakan library `Regex` yang dikompilasi satu kali pada fase inisialisasi untuk melakukan pencarian heuristik pada payload HTTP response. Sistem mendeteksi pola spesifik seperti:
* **Kriptografi**: RSA Private Keys, SSH Keys.
* **Cloud Credentials**: AWS Access Keys (AKIA), Firebase Instance URLs.
* **Database**: Connection Strings (MySQL, MongoDB, PostgreSQL).
* **Environment**: Kebocoran variabel `.env` (DB_PASSWORD, SECRET_KEY).

---

## 2. Dekonstruksi Arsitektur Modular

Proyek ini mengadopsi desain modular guna memisahkan logika bisnis dari manajemen I/O:

### 2.1 Orchestration Layer (`src/main.rs`)
Bertanggung jawab atas:
* Inisialisasi runtime dan pemrosesan argumen via `Clap`.
* Manajemen siklus hidup HTTP request menggunakan `Reqwest`.
* Implementasi `buffer_unordered` untuk memproses stream tugas secara paralel.

### 2.2 Detection Logic (`src/signatures.rs`)
Komponen ini merupakan basis pengetahuan scanner. Setiap signature didefinisikan dengan:
* **Pattern**: Ekspresi reguler untuk identifikasi data sensitif.
* **Severity**: Klasifikasi dampak temuan (Critical, High, Medium, Low).

### 2.3 Reporting Subsystem (`src/report.rs`)
Menangani abstraksi I/O file hasil temuan. Implementasi menggunakan `OpenOptions` dengan mode *append* untuk memastikan persistensi data tanpa risiko *data overwriting* pada sesi pemindaian berikutnya.

---

## 3. Prosedur Instalasi (Optimasi Arch Linux)

### 3.1 Prasyarat Sistem
Pastikan toolchain Rust dan library TLS telah terkonfigurasi pada sistem Anda:

```bash
sudo pacman -S --needed rustup base-devel openssl pkgconf
rustup default stable
