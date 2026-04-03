# 🛡️ **Rusty-Sentinel**

[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Arch%20Linux%20%7C%20Linux%20%7C%20macOS-lightgrey.svg)](https://www.archlinux.org/)

**Rusty-Sentinel** adalah alat pemindai kerentanan (*vulnerability scanner*) berbasis Rust yang dirancang untuk kecepatan ekstrem dan efisiensi memori. Alat ini melakukan pemindaian asinkron terhadap path sensitif dan menggunakan analisis pola Regex untuk mendeteksi kebocoran data seperti **API Keys**, **kredensial database**, dan **file konfigurasi** yang terekspos.

Dengan fokus utama pada keamanan aplikasi web, Rusty-Sentinel memungkinkan pentester dan pengembang untuk memindai web target dengan lebih cepat dan efisien, serta menemukan potensi celah keamanan yang seringkali tersembunyi di antara ribuan file dan konfigurasi yang terabaikan.

---

## ✨ **Fitur Utama**

-   **⚡ Performa Tinggi**: Dibangun di atas runtime **Tokio**, memungkinkan ribuan request asinkron berjalan secara paralel tanpa membebani CPU.
-   **🔍 Deteksi Pintar dengan Regex**: Menggunakan **engine Regex** untuk mendeteksi pola sensitif seperti **AWS Keys**, **RSA Private Keys**, **Database Credentials**, dan **API Keys**.
-   **📂 Manajemen Wordlist yang Fleksibel**: Mendukung input **wordlist** eksternal untuk pemindaian path yang lebih spesifik, menyesuaikan dengan kebutuhan keamanan yang berbeda.
-   **📝 Pelaporan Otomatis**: Hasil scan disimpan secara otomatis dalam **`results.txt`** dengan timestamp yang akurat untuk pencatatan yang lebih rapi dan mudah dianalisis.
-   **🎨 Output Interaktif dengan Terminal UI**: Menampilkan hasil scan secara langsung dengan **UI terminal interaktif**, termasuk color-coding untuk status scan dan tingkat keparahan temuan.
-   **🔒 Keamanan dan Privasi**: Tidak menyimpan atau mengirimkan data pemindaian ke server eksternal. Semua pemindaian sepenuhnya lokal.

---

## 🏗️ **Arsitektur Proyek**

Proyek ini dibangun dengan struktur modular yang memungkinkan pengembangan yang lebih terorganisir dan dapat diperluas:

### **File dan Modul Utama:**

-   **`src/main.rs`**: 
    - Merupakan entry point dari aplikasi ini. Mengatur alur utama aplikasi dengan menjalankan pemindaian, pengolahan data, dan interaksi dengan user melalui antarmuka terminal.
    - Mengatur concurrency dengan **`tokio`**, mengelola batas request paralel dan pembatasan aliran data.
  
-   **`src/signatures.rs`**: 
    - Menyimpan **patterns Regex** yang digunakan untuk mendeteksi file sensitif dan kerentanannya.
    - Daftar signature ini bisa dikustomisasi untuk mencari pola-pola lain sesuai kebutuhan pengguna.
    - Memiliki struktur data yang memungkinkan penambahan atau pengubahan signature dengan mudah.

-   **`src/report.rs`**: 
    - Modul untuk menangani **log pemindaian** dan **pelaporan temuan**.
    - Temuan dari pemindaian akan disimpan dalam file **`results.txt`** yang disertai timestamp untuk referensi lebih lanjut.
    - Fitur **pelaporan otomatis** mempermudah pencatatan hasil scan tanpa perlu campur tangan manual.

-   **`wordlist.txt`**: 
    - Daftar path file dan URL target yang akan dipindai.
    - Bisa dikustomisasi oleh pengguna untuk menambahkan path atau file yang lebih relevan dengan struktur aplikasi yang sedang diuji.
  
---

## 🚀 **Memulai (Instalasi di Arch Linux)**

### **Prasyarat**
Sebelum memulai, pastikan sistem Anda memenuhi prasyarat berikut:
- **Rust toolchain** untuk membangun dan menjalankan aplikasi.
- **OpenSSL** untuk mendukung koneksi HTTPS yang aman.

Install dependencies di **Arch Linux** dengan perintah berikut:

```bash
sudo pacman -S rustup base-devel openssl pkgconf
rustup default stable
