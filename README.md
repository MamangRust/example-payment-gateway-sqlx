# Example Payment Gateway System (Modular Monolith)


This project is an example implementation of a payment gateway system built with a **modular monolith** architecture. The backend is developed in **Rust** using the `tokio` runtime, `tonic` for gRPC, and `sqlx` for database interaction. The frontend dashboard is built with **React**, **TypeScript**, **Vite**, and **Tauri** for the desktop version.

## Application Preview

Here are a few screenshots of the application:

**Main Dashboard:**
![Example Dashboard](./images/example-dashboard.png)

**API Documentation (Swagger UI):**
![Swagger UI](./images/swagger-ui.png)

## Architecture Overview

This system is designed as a modular monolith. While it resides in a single repository, it is composed of multiple services (crates) that communicate with each other via gRPC. This design provides clear separation of concerns while simplifying deployment and development compared to a full microservices setup.

The `APIGateway` is the single entry point for all external HTTP requests. It authenticates requests and routes them to the appropriate internal service over gRPC. All services share a single PostgreSQL database.

```mermaid
graph TD
    subgraph "Client (Web/Desktop)"
        A[Dashboard / API Client]
    end

    subgraph "Payment Gateway System"
        B(API Gateway <br> HTTP/REST)

        subgraph "gRPC Services (Internal Modules)"
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

### Services

- **`apigateway`**: The main gateway that accepts HTTP requests from clients and forwards them to the appropriate service via gRPC.
- **`auth`**: Manages authentication (login, register) and authorization (token management).
- **`user`**: Manages user data.
- **`card`**: Manages payment cards linked to users.
- **`saldo`**: Manages user digital wallets or balances.
- **`merchant`**: Manages merchant/store data.
- **`role`**: Manages user roles and permissions.
- **`topup`**: Manages the balance top-up process.
- **`transaction`**: Records and manages transaction history.
- **`transfer`**: Manages balance transfers between users.
- **`withdraw`**: Manages the balance withdrawal process.
- **`genproto`**: A dedicated crate for compiling `.proto` files into Rust code for gRPC.
- **`dashboard`**: A frontend application (React + Tauri) for administration and data visualization.

## Key Features

- User Management & JWT-based Authentication.
- Role-Based Access Control (RBAC).
- Digital Wallet Management.
- Financial Transactions: Top-up, Transfer, and Withdraw.
- Payment Card Management.
- Administrative Dashboard.
- Efficient, type-safe inter-service communication with gRPC.

## Tech Stack

**Backend:**
- **Language**: Rust (Stable)
- **Async Runtime**: `tokio`
- **Communication**: `tonic` (gRPC) & `prost`
- **Database ORM**: `sqlx` (with PostgreSQL)
- **Web Framework (API Gateway)**: `axum`
- **Logging**: `tracing`
- **Validation**: `validator`

**Frontend (Dashboard):**
- **Framework**: React.js
- **Language**: TypeScript
- **Build Tool**: Vite
- **Styling**: Tailwind CSS
- **Desktop App**: Tauri

**Other:**
- **Database**: PostgreSQL
- **API Definition**: Protocol Buffers (gRPC)
- **Containerization**: Docker (via `docker-compose.yml`)

## Prerequisites

Before you begin, ensure you have the following software installed:
- [Rust & Cargo](https://www.rust-lang.org/tools/install)
- [Node.js & npm](https://nodejs.org/) (or `bun`)
- [`sqlx-cli`](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli) (`cargo install sqlx-cli`)
- [`protoc`](https://grpc.io/docs/protoc-installation/) (Protocol Buffers Compiler)

## Installation & Setup

1.  **Clone the Repository**
    ```bash
    git clone https://github.com/MamangRust/example-payment-gateway-sqlx.git
    cd example-payment-gateway-sqlx
    ```

2.  **Configure Environment Variables**
    Copy the `.env.example` file (if it exists) or create a new `.env` file in the project root and fill in your local configuration details.

    ```env
    # Example .env content
    DATABASE_URL="postgres://user:password@localhost:5432/payment_gateway"
    APP_PORT=8000
    # URLs for each service...
    AUTH_SERVICE_URL="http://127.0.0.1:50051"
    USER_SERVICE_URL="http://127.0.0.1:50052"
    # ...and so on
    ```



3.  **Run Database Migrations**
    Ensure the `DATABASE_URL` in your `.env` file is correct, then run the migrations.
    ```bash
    sqlx migrate run
    ```

4.  **Build Protobuf Definitions**
    Compile the `.proto` files to be used by all services.
    ```bash
    cargo build -p genproto
    ```

5.  **Build All Backend Services**
    ```bash
    cargo build --workspace
    ```

## Running the Application

1.  **Run Backend Services**
    Open multiple terminal tabs and run each service individually.

    ```bash
    # Terminal 1: API Gateway
    cargo run -p apigateway

    # Terminal 2: Auth Service
    cargo run -p auth

    # Terminal 3: User Service
    cargo run -p user

    # ...run other services as needed
    ```

2.  **Run the Frontend Dashboard**
    Open a new terminal in the dashboard directory.

    ```bash
    cd crates/dashboard
    npm install
    npm run dev
    ```
    The frontend application will be available at `http://localhost:1420`.

3.  **Run the Dashboard as a Desktop App (Tauri)**
    ```bash
    cd crates/dashboard
    npm install
    npm run tauri dev
    ```

## API Documentation

Once the `apigateway` is running, the API documentation (generated with Swagger UI) can be accessed at:
[http://127.0.0.1:5000/swagger-ui/](http://127.0.0.1:5000/swagger-ui/)

