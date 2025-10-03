# Example Payment Gateway System (Modular Monolith)

Proyek ini adalah contoh implementasi sistem payment gateway yang dibangun dengan arsitektur **modular monolith**. Backend dikembangkan menggunakan **Rust** dengan runtime `tokio`, `tonic` untuk gRPC, dan `sqlx` untuk interaksi database. Dashboard frontend dibangun dengan **React**, **TypeScript**, **Vite**, dan **Tauri** untuk versi desktop.


## Tentang Project Ini

Proyek ini merupakan sebuah contoh sistem *payment gateway* yang dirancang untuk mensimulasikan proses transaksi keuangan digital. Dibangun dengan arsitektur monolit modular, sistem ini memisahkan setiap domain bisnis (seperti pengguna, saldo, transaksi) ke dalam modul-modul (crate) yang independen namun tetap berada dalam satu basis kode. Komunikasi antar modul dilakukan secara efisien menggunakan gRPC.

Tujuan utama dari proyek ini adalah untuk menyediakan contoh nyata penerapan arsitektur modern di ekosistem Rust, lengkap dengan backend, frontend, dan versi aplikasi desktop.

## Fitur Utama

- **Manajemen Pengguna & Autentikasi berbasis JWT**: Proses registrasi, login, dan manajemen sesi pengguna yang aman.
- **Kontrol Akses Berbasis Peran (RBAC)**: Pembatasan akses fitur berdasarkan peran pengguna (misalnya, admin vs. pengguna biasa).
- **Manajemen Dompet Digital (Saldo)**: Setiap pengguna memiliki saldo digital yang dapat diisi ulang dan digunakan untuk transaksi.
- **Transaksi Keuangan**:
  - **Top-up**: Menambah saldo dari sumber eksternal.
  - **Transfer**: Mengirim saldo antar pengguna di dalam sistem.
  - **Withdraw**: Menarik saldo ke rekening eksternal.
- **Manajemen Kartu Pembayaran**: Pengguna dapat menautkan dan mengelola kartu pembayaran mereka.
- **Dashboard Administratif**: Antarmuka untuk memantau dan mengelola aktivitas sistem.
- **Komunikasi Antar-Layanan**: Komunikasi yang efisien dan *type-safe* dengan gRPC.

## Arsitektur

Sistem ini dirancang sebagai monolit modular. Meskipun berada dalam satu repositori, sistem ini terdiri dari beberapa layanan (crate) yang berkomunikasi satu sama lain melalui gRPC. Desain ini memberikan pemisahan tanggung jawab yang jelas sambil menyederhanakan proses deployment dan pengembangan dibandingkan dengan arsitektur microservices penuh.

`APIGateway` adalah satu-satunya titik masuk untuk semua permintaan HTTP eksternal. Gateway ini melakukan autentikasi permintaan dan meneruskannya ke layanan internal yang sesuai melalui gRPC. Semua layanan berbagi satu database PostgreSQL.

```mermaid
graph TD
    subgraph "Klien (Web/Desktop)"
        A[Dashboard / API Client]
    end

    subgraph "Sistem Payment Gateway"
        B(API Gateway <br> HTTP/REST)

        subgraph "Layanan gRPC (Modul Internal)"
            C[Auth Service]
            D[User Service]
            E[Card Service]
            F[Saldo Service]
            G[Merchant Service]
            H[Role Service]
            I[Topup Service]
            J[Transaction Service]
            K[Transfer Service]
            L[Withdraw Service]
        end

        M[(Database <br> PostgreSQL)]
    end

    A --> B

    B -- gRPC --> C
    B -- gRPC --> D
    B -- gRPC --> E
    B -- gRPC --> F
    B -- gRPC --> G
    B -- gRPC --> H
    B -- gRPC --> I
    B -- gRPC --> J
    B -- gRPC --> K
    B -- gRPC --> L

    C --> M
    D --> M
    E --> M
    F --> M
    G --> M
    H --> M
    I --> M
    J --> M
    K --> M
    L --> M
```

## Entity Relationship Diagram (ERD)

Diagram berikut menggambarkan hubungan antar tabel dalam database.

```mermaid
erDiagram
    users {
        INT user_id PK
        VARCHAR firstname
        VARCHAR lastname
        VARCHAR email
        VARCHAR password
    }
    roles {
        INT role_id PK
        VARCHAR role_name
    }
    user_roles {
        INT user_role_id PK
        INT user_id FK
        INT role_id FK
    }
    refresh_tokens {
        INT refresh_token_id PK
        INT user_id FK
        VARCHAR token
        TIMESTAMP expiration
    }
    cards {
        INT card_id PK
        INT user_id FK
        VARCHAR card_number
        VARCHAR card_type
        DATE expire_date
    }
    saldos {
        INT saldo_id PK
        VARCHAR card_number FK
        INT total_balance
    }
    merchants {
        INT merchant_id PK
        UUID merchant_no
        VARCHAR name
        VARCHAR api_key
        INT user_id FK
    }
    topups {
        INT topup_id PK
        UUID topup_no
        VARCHAR card_number FK
        INT topup_amount
        VARCHAR topup_method
    }
    transactions {
        INT transaction_id PK
        UUID transaction_no
        VARCHAR card_number FK
        INT amount
        INT merchant_id FK
    }
    transfers {
        INT transfer_id PK
        UUID transfer_no
        VARCHAR transfer_from FK
        VARCHAR transfer_to FK
        INT transfer_amount
    }
    withdraws {
        INT withdraw_id PK
        UUID withdraw_no
        VARCHAR card_number FK
        INT withdraw_amount
    }

    users ||--o{ user_roles : "has"
    roles ||--o{ user_roles : "has"
    users ||--o{ refresh_tokens : "has"
    users ||--o{ cards : "has"
    users ||--o{ merchants : "owns"
    cards ||--o{ saldos : "has"
    cards ||--o{ topups : "has"
    cards ||--o{ transactions : "has"
    merchants ||--o{ transactions : "receives"
    cards ||--o{ transfers : "sends"
    cards ||--o{ transfers : "receives"
    cards ||--o{ withdraws : "has"
```

## Teknologi yang Digunakan

**Backend:**
- **Bahasa**: Rust (Stable)
- **Async Runtime**: `tokio`
- **Komunikasi**: `tonic` (gRPC) & `prost`
- **Database ORM**: `sqlx` (dengan PostgreSQL)
- **Web Framework (API Gateway)**: `axum`
- **Logging**: `tracing`
- **Validasi**: `validator`

**Frontend (Dashboard):**
- **Framework**: React.js
- **Bahasa**: TypeScript
- **Build Tool**: Vite
- **Styling**: Tailwind CSS
- **Aplikasi Desktop**: Tauri

**Lainnya:**
- **Database**: PostgreSQL
- **Definisi API**: Protocol Buffers (gRPC)

## Prasyarat

Sebelum memulai, pastikan perangkat lunak berikut sudah terinstal:
- [Rust & Cargo](https://www.rust-lang.org/tools/install)
- [Node.js & npm](https://nodejs.org/) (atau `bun`)
- [`sqlx-cli`](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli) (`cargo install sqlx-cli`)
- [`protoc`](https://grpc.io/docs/protoc-installation/) (Protocol Buffers Compiler)

## Instalasi & Setup (Lokal)

1.  **Clone Repositori**
    ```bash
    git clone https://github.com/MamangRust/example-payment-gateway-sqlx.git
    cd example-payment-gateway-sqlx
    ```

2.  **Konfigurasi Variabel Lingkungan**
    Salin file `.env.example` (jika ada) atau buat file `.env` baru di root proyek dan isi detail konfigurasi lokal Anda.

    ```env
    # Contoh isi file .env
    DATABASE_URL="postgres://user:password@localhost:5432/payment_gateway"
    APP_PORT=8000
    # URL untuk setiap layanan...
    AUTH_SERVICE_URL="http://127.0.0.1:50051"
    USER_SERVICE_URL="http://127.0.0.1:50052"
    # ...dan seterusnya
    ```

3.  **Jalankan Migrasi Database**
    Pastikan `DATABASE_URL` di file `.env` Anda sudah benar, lalu jalankan migrasi.
    ```bash
    sqlx migrate run
    ```

4.  **Build Definisi Protobuf**
    Kompilasi file `.proto` untuk digunakan oleh semua layanan.
    ```bash
    cargo build -p genproto
    ```

5.  **Build Semua Layanan Backend**
    ```bash
    cargo build --workspace
    ```

## Menjalankan Aplikasi

1.  **Jalankan Layanan Backend**
    Buka beberapa tab terminal dan jalankan setiap layanan secara terpisah.

    ```bash
    # Terminal 1: API Gateway
    cargo run -p apigateway

    # Terminal 2: Auth Service
    cargo run -p auth

    # Terminal 3: User Service
    cargo run -p user

    # ...jalankan layanan lain sesuai kebutuhan
    ```

2.  **Jalankan Dashboard Frontend**
    Buka terminal baru di direktori `crates/dashboard`.

    ```bash
    cd crates/dashboard
    npm install
    npm run dev
    ```
    Aplikasi frontend akan tersedia di `http://localhost:1420`.

3.  **Jalankan Dashboard sebagai Aplikasi Desktop (Tauri)**
    ```bash
    cd crates/dashboard
    npm install
    npm run tauri dev
    ```

## Dokumentasi API

Setelah `apigateway` berjalan, dokumentasi API (dihasilkan dengan Swagger UI) dapat diakses di:
[http://127.0.0.1:5000/swagger-ui/](http://127.0.0.1:5000/swagger-ui/)


## Tampilan Aplikasi

Berikut adalah beberapa tangkapan layar dari aplikasi:

**Dashboard Utama:**
![Example Dashboard](./images/example-dashboard.png)

**Dokumentasi API (Swagger UI):**
![Swagger UI](./images/swagger-ui.png)
