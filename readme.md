# 🛡️ e.-xviaa-a02 (v1.0.0-alpha)
### High-Performance Asynchronous Intelligence & Data Leakage Scanner

[![Rust Engine](https://img.shields.io/badge/Engine-Rust%202021-orange?logo=rust)](https://www.rust-lang.org/)
[![Safety](https://img.shields.io/badge/Memory-Safe-blue?logo=shield)](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
[![Concurrency](https://img.shields.io/badge/Concurrency-Tokio%20Runtime-red)](https://tokio.rs/)
[![License](https://img.shields.io/badge/License-MIT-green)](LICENSE)

**Rusty-Sentinel** adalah instrumen audit keamanan siber tingkat lanjut yang dirancang khusus untuk mendeteksi *sensitive data exposure* pada infrastruktur web skala besar. Dibangun dengan fokus pada **Zero-Cost Abstractions** dan **Extreme Parallelism**, alat ini mampu melakukan pemindaian ribuan endpoint dalam hitungan detik tanpa mengorbankan stabilitas sistem.

---

## 🏗️ Technical Architecture & Workflow

Rusty-Sentinel beroperasi menggunakan model **Producer-Consumer** yang diatur oleh sistem *Semaphore* untuk memastikan efisiensi I/O maksimal.



1. **Target Ingestion**: Memproses daftar target dan wordlist secara asinkron.
2. **Semaphore Gatekeeping**: Membatasi *concurrent requests* untuk menghindari deteksi WAF (Web Application Firewall).
3. **Response Analysis**: Melakukan inspeksi header dan body response secara *real-time*.
4. **Deep Pattern Matching**: Menggunakan mesin Regex teroptimasi untuk mengidentifikasi kebocoran kredensial.

---

## 🚀 Fitur Utama (Advanced Features)

### 1. ⚡ Extreme Concurrent Engine
Berbeda dengan scanner berbasis Python yang terhambat oleh GIL (Global Interpreter Lock), Rusty-Sentinel menggunakan **Tokio Runtime** untuk mengeksekusi ribuan *non-blocking tasks* secara benar-benar paralel.

### 2. 🔍 Heuristic Content Inspection
Bukan sekadar pengecekan status HTTP. Mesin kami melakukan pemindaian isi file untuk menemukan:
* **Cloud Infrastructure Keys**: AWS (Access/Secret), Google Cloud, Azure.
* **Database Strings**: Koneksi PostgreSQL, MongoDB, MySQL yang terekspos.
* **Environment Files**: `.env`, `.bash_history`, `docker-compose.yml`.
* **CI/CD Leaks**: `.travis.yml`, `.github/workflows/`, Jenkins configs.

### 3. 🛡️ Intelligent Rate Limiting
Sistem otomatis yang menunda request jika mendeteksi respon `429 Too Many Requests`, memastikan proses pemindaian tetap berjalan di bawah radar sistem keamanan target.

---

## 💻 Spesifikasi Teknis & Dependensi

| Kategori | Teknologi | Kegunaan |
| :--- | :--- | :--- |
| **Language** | Rust 1.70+ | Kecepatan setara C++, Keamanan memori terjamin. |
| **Async Core** | `Tokio` v1.0 | Runtime asinkron untuk performa I/O tinggi. |
| **HTTP Layer** | `Reqwest` | Mendukung TLS 1.3, HTTP/2, dan custom headers. |
| **Stream Proc** | `Futures` | Memproses aliran data besar secara efisien. |
| **Visuals** | `Colored` | Reporting terminal yang intuitif dan profesional. |

---

## 🛠️ Instalasi & Deployment

### Build dari Source
Pastikan Anda memiliki [Rust Toolchain](https://rustup.rs/) terinstal di sistem Anda.

```bash
# Clone repositori
git clone [https://github.com/182d018c619a/rusty-sentinel.git](https://github.com/182d018c619a/rusty-sentinel.git)
cd rusty-sentinel

# Build untuk performa maksimal (Mode Release)
cargo build --release

# Jalankan biner hasil build
./target/release/rusty-sentinel
