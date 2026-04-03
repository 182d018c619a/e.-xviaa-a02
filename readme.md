# e.-xviaa-a02 Security Scanner

[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Arch%20Linux%20%7C%20Linux%20%7C%20macOS-lightgrey.svg)](https://www.archlinux.org/)

**e.-xviaa-a02** adalah instrumen audit keamanan asinkron berbasis Rust, dirancang untuk penemuan jalur sensitif (*sensitive path discovery*) dan analisis eksposur data tingkat lanjut. Sistem ini direkayasa untuk beroperasi dengan jejak memori (*memory footprint*) yang minimal sembari mempertahankan throughput maksimum melalui I/O non-blocking.

---

## Daftar Isi
1. [Ikhtisar Sistem](#1-ikhtisar-sistem)
2. [Arsitektur Perangkat Lunak](#2-arsitektur-perangkat-lunak)
3. [Instalasi dan Konfigurasi](#3-instalasi-dan-konfigurasi)
4. [Panduan Operasional](#4-panduan-operasional)
5. [Mesin Deteksi (Signatures)](#5-mesin-deteksi-signatures)
6. [Subsistem Pelaporan](#6-subsistem-pelaporan)
7. [Penafian Hukum](#7-penafian-hukum)

---

## 1. Ikhtisar Sistem

Sistem ini difokuskan pada pemindaian massal terhadap infrastruktur web untuk mengidentifikasi miskonfigurasi server dan kebocoran kredensial. Berbeda dengan pemindai sekuensial tradisional, aplikasi ini mendistribusikan beban kerja melalui mekanisme *task buffering* yang dikelola oleh *runtime asynchronous*.

### Karakteristik Inti
* **High-Concurrency**: Mampu menangani ribuan koneksi HTTP secara paralel.
* **Memory Safe**: Memanfaatkan sistem *ownership* Rust untuk mencegah *memory leaks* selama operasi jangka panjang.
* **Rate-Limiting Nativ**: Dilengkapi dengan `tokio::sync::Semaphore` untuk mencegah pemblokiran alamat IP host akibat *rate-limiting* agresif dari target.

---

## 2. Arsitektur Perangkat Lunak

Proyek e.-xviaa-a02 dipecah menjadi modul-modul independen untuk memfasilitasi skalabilitas:

| Modul | File Sumber | Deskripsi Fungsional |
| :--- | :--- | :--- |
| **Orchestrator** | `src/main.rs` | Mengelola *event loop*, *CLI parsing* (`clap`), dan distribusi *worker threads*. |
| **Heuristic Engine** | `src/signatures.rs` | Mesin kompilasi Regex dan definisi tingkat keparahan (*Severity*) ancaman. |
| **I/O Logger** | `src/report.rs` | Menangani operasi *file-system* secara persisten untuk pencatatan *audit trail*. |
| **Input Vector** | `wordlist.txt` | Kumpulan rute URL target eksternal, dimuat ke memori menggunakan *buffered reader*. |

---

## 3. Instalasi dan Konfigurasi

### 3.1. Prasyarat Lingkungan (Arch Linux)
Sistem membutuhkan toolchain compiler Rust dan library SSL statis. Jalankan perintah manajemen paket berikut:

```bash
sudo pacman -S --needed rustup base-devel openssl pkgconf
rustup default stable
