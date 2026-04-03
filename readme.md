# 🛡️ Rusty-Sentinel

![Rust](https://img.shields.io/badge/language-Rust-orange.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Status](https://img.shields.io/badge/status-Active--Development-green.svg)

**Rusty-Sentinel** adalah instrumen intelijen keamanan (OSINT & Scanner) tingkat tinggi yang dibangun di atas bahasa **Rust**. Dirancang untuk mendeteksi kebocoran data sensitif, file konfigurasi yang terekspos, dan kredensial kritis pada infrastruktur web dengan kecepatan asinkron yang ekstrem.

> "Lebih dari sekadar scanner; ini adalah mata yang tidak pernah tidur untuk infrastruktur digital Anda."

---

## 🚀 Fitur Unggulan (Core Capabilities)

* **⚡ Ultra-Fast Asynchronous Engine**: Menggunakan `Tokio` runtime dan `Futures` untuk menangani ribuan request secara paralel tanpa membebani memori.
* **🎯 Deep Content Inspection**: Tidak hanya mengecek status kode HTTP (200 OK), tetapi melakukan inspeksi mendalam pada isi file untuk menemukan pola data bocor.
* **🛡️ Advanced Semaphore Control**: Fitur pengontrol lalu lintas cerdas untuk mencegah deteksi WAF/DDoS saat melakukan pemindaian agresif.
* **🔍 Smart Signature Matching**: Algoritma deteksi untuk:
    * Cloud Keys (AWS, Google Cloud, Azure)
    * Database Credentials (.env, config.php, settings.py)
    * Private Keys (SSH, RSA, PGP)
    * Backup Files & Git Source Leaks

---

## 🏗️ Arsitektur Teknis

Rusty-Sentinel menggunakan paradigma **Zero-Copy** dan **Multi-threading** untuk memastikan efisiensi maksimal.



| Komponen | Teknologi | Deskripsi |
| :--- | :--- | :--- |
| **Runtime** | `Tokio` | Event-driven, non-blocking I/O. |
| **Networking** | `Reqwest` | HTTP Client dengan dukungan TLS/SSL tingkat lanjut. |
| **Concurrency** | `Semaphore` | Membatasi beban kerja (Max 100+ concurrent requests). |
| **Interface** | `Colored` | Output terminal ANSI untuk visibilitas status instan. |

---

## 🛠️ Instalasi & Penggunaan

### Prasyarat
* Rust & Cargo (Versi Terbaru)

### Build dari Source
```bash
git clone [https://github.com/182d018c619a/rusty-sentinel.git](https://github.com/182d018c619a/rusty-sentinel.git)
cd rusty-sentinel
cargo build --release
