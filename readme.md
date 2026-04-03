# **/e.-xviaa-a02/ - RustySentinel: Web Pentest Tool**

**/e.-xviaa-a02/** adalah sebuah alat pemindaian web berbasis **Rust** yang dirancang untuk mendeteksi kebocoran file sensitif dan kerentanannya di website target. Alat ini mengutamakan **keamanan dan kecepatan**, serta menyediakan **antarmuka pengguna berbasis terminal (TUI)** yang modern dan responsif. Proyek ini sangat cocok bagi para pentester dan profesional keamanan siber yang ingin memastikan bahwa tidak ada data sensitif yang terekspos di web.

---

## **Fitur Utama**

- **Pemindaian File Sensitif**: Menemukan file sensitif seperti `.env`, `wp-config.php`, dan `id_rsa`.
- **Deteksi Konfigurasi yang Terbuka**: Memeriksa kebocoran informasi dari konfigurasi web dan environment variables.
- **Pemindaian HTTP Headers**: Mendeteksi header HTTP yang mungkin memberi informasi lebih tentang server atau aplikasi web.
- **Tampilan Terminal Modern (TUI)**: Menampilkan hasil pemindaian dalam antarmuka pengguna berbasis terminal yang dinamis dan interaktif menggunakan pustaka `ratatui`.
- **Pemindaian Paralel**: Mendukung pemindaian yang cepat dengan menggunakan pemrograman asinkron `tokio` untuk melakukan pemindaian paralel.
- **Real-time Findings**: Menampilkan temuan secara langsung dengan color-coding yang membedakan tingkat keparahan temuan.

---

## **Tampilan Aplikasi**

![UI Screenshot](https://via.placeholder.com/800x400.png)  
*Tampilan antarmuka pengguna terminal RustySentinel yang sedang memindai website target.*

---

## **Instalasi dan Pengaturan**

### 1. **Persyaratan Sistem**

Sebelum memulai, pastikan **Rust** sudah terpasang di sistemmu. Kamu bisa mengunduh dan menginstalnya dari situs resmi [Rust](https://www.rust-lang.org/).

Selain itu, alat ini membutuhkan beberapa dependensi tambahan yang tercatat di **Cargo.toml**:

- `reqwest`: Untuk mengirimkan permintaan HTTP.
- `tokio`: Untuk pemrograman asinkron.
- `ratatui`: Untuk antarmuka pengguna berbasis terminal.
- `crossterm`: Untuk menangani input pengguna di terminal.

### 2. **Clone Repository**

Untuk memulai, clone repositori ini ke direktori lokal kamu dengan perintah berikut:

```bash
git clone https://github.com/username/e.-xviaa-a02.git
cd e.-xviaa-a02
