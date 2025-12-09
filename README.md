# Example Payment Gateway (Modular Monolith)

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white)
![Kubernetes](https://img.shields.io/badge/kubernetes-%23326ce5.svg?style=for-the-badge&logo=kubernetes&logoColor=white)
![Postgres](https://img.shields.io/badge/postgres-%23316192.svg?style=for-the-badge&logo=postgresql&logoColor=white)
![gRPC](https://img.shields.io/badge/gRPC-00ADD8?style=for-the-badge&logo=grpc&logoColor=white)
![Prometheus](https://img.shields.io/badge/Prometheus-E6522C?style=for-the-badge&logo=Prometheus&logoColor=white)
![Grafana](https://img.shields.io/badge/grafana-%23F46800.svg?style=for-the-badge&logo=grafana&logoColor=white)

This project is an example implementation of a **payment gateway system** built with Rust, featuring a **modular monolith** architecture. It simulates a complete digital financial transaction process, including user management, digital wallets, and various transaction types (top-up, transfer, withdrawal).

The primary goal is to provide a comprehensive, real-world reference for building robust, scalable, and observable systems using Rust's modern ecosystem.

## Features

-   ✅ **JWT Authentication** with refresh tokens
-   ✅ **Role-Based Access Control (RBAC)**
-   ✅ **Digital Wallet Management**
-   ✅ **Complete Transaction Lifecycle** (Top-up, Transfer, Withdraw, Payment)
-   ✅ **Payment Card Management**
-   ✅ **Merchant API Key Management**
-   ✅ **Comprehensive Observability** with metrics, logging, and distributed tracing
-   ✅ **Containerized Deployment** with Docker and Kubernetes
-   ✅ **API Documentation** with Swagger UI

## Architecture

The system uses a modular monolith architecture where an **API Gateway** serves as the single entry point. Each business domain is separated into an independent Rust crate (module), and all inter-module communication is handled via gRPC for high performance and type safety.

```mermaid
graph TD
    subgraph "Clients (Web/Mobile/CLI)"
        A[End User or API Client]
    end

    subgraph "Payment Gateway System"
        B(API Gateway <br> HTTP/REST)

        subgraph "Internal gRPC Services (Modules)"
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
        N[(Cache <br> Redis)]
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

    C --> M & N
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

## Technology Stack

| Category              | Technology                                                                                              |
| --------------------- | ------------------------------------------------------------------------------------------------------- |
| **Language**          | Rust (Stable)                                                                                           |
| **Async Runtime**     | `tokio`                                                                                                 |
| **Web Framework**     | `axum` (for API Gateway)                                                                                |
| **Inter-service**     | `tonic` (gRPC), `prost` (Protobuf)                                                                      |
| **Database**          | PostgreSQL                                                                                              |
| **ORM / DB Driver**   | `sqlx`                                                                                                  |
| **Cache**             | Redis                                                                                                   |
| **Containerization**  | Docker, Docker Compose                                                                                  |
| **Orchestration**     | Kubernetes (Minikube for local setup)                                                                   |
| **Observability**     | **OpenTelemetry**, **Prometheus** (metrics), **Grafana** (dashboards), **Jaeger** (tracing), **Loki** (logs) |                                                                        

## Getting Started

### Prerequisites

-   [Docker](https://docs.docker.com/get-docker/) and [Docker Compose](https://docs.docker.com/compose/install/)
-   [`sqlx-cli`](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli) for database migrations.
-   (Optional, for K8s) [Minikube](https://minikube.sigs.k8s.io/docs/start/) and [kubectl](https://kubernetes.io/docs/tasks/tools/).

---

### Option 1: Run with Docker Compose (Recommended)

This is the fastest way to get the entire system running on your local machine.

1.  **Clone the Repository**

    ```bash
    git clone https://github.com/MamangRust/example-payment-gateway-sqlx.git
    cd example-payment-gateway-sqlx/backend
    ```

2.  **Configure Environment**

    Review the `.env` file and ensure the settings (especially `DATABASE_URL`) match the configuration in `docker-compose.yml`. The defaults should work out of the box.

3.  **Run the Database Migration**

    Before starting the services, you need to set up the database schema.
    
    First, start the database container:
    ```bash
    docker-compose up -d db
    ```
    
    Wait a few seconds for it to initialize, then run the migrations:
    ```bash
    # Ensure your .env file is present in the current directory
    sqlx migrate run
    ```

4.  **Start All Services**

    Now, bring up the entire stack, including all application services and the observability suite.

    ```bash
    docker-compose up -d
    ```

    The application services use pre-built images from `ghcr.io`. If you want to use your local code changes, you must first build the images using the `./build-docker-images.sh` script.

5.  **Access the System**
    -   **API Gateway / Swagger UI**: `http://localhost:5000/swagger-ui/`
    -   See the **Observability** section below for more URLs.

---

### Option 2: Deploy to Kubernetes (Minikube)

This method simulates a production-like deployment on a local Kubernetes cluster.

1.  **Clone the Repository**

    ```bash
    git clone https://github.com/MamangRust/example-payment-gateway-sqlx.git
    cd example-payment-gateway-sqlx/backend
    ```

2.  **Build Local Docker Images**

    The Kubernetes manifests are configured to use local images. Run the build script to create them.

    ```bash
    ./build-docker-images.sh
    ```

3.  **Run the Minikube Setup Script**

    This script will:
    -   Start Minikube (if not already running).
    -   Load the necessary Docker images into Minikube's context.
    -   Apply all Kubernetes manifests for databases, observability, and application services.

    ```bash
    ./k8s/scripts/setup-minikube.sh
    ```

4.  **Access the System**

    The script will output all the access URLs. The application will be available via a NodePort on your Minikube IP. Example:

    -   **Main Application**: `http://<MINIKUBE_IP>:30080`
    -   **Grafana**: `http://<MINIKUBE_IP>:30030`
    -   **Jaeger**: `http://<MINIKUBE_IP>:31686`

## Observability Stack

The `docker-compose` and `minikube` setups include a full observability stack. Here’s how to access the different tools when running with **Docker Compose**:

| Service        | URL                             | Description                                            |
| -------------- | ------------------------------- | ------------------------------------------------------ |
| **Grafana**    | `http://localhost:3000`         | Dashboards for metrics and logs. (Login: admin/admin)  |
| **Prometheus** | `http://localhost:9090`         | Time-series database for metrics.                      |
| **Jaeger**     | `http://localhost:16686`        | Distributed tracing UI.                                |
| **Loki**       | `http://localhost:3100`         | Log aggregation system.                                |
| **Alertmanager**| `http://localhost:9093`        | Manages alerts sent by Prometheus.                     |

![Example Dashboard](./backend/images/example-dashboard.png)

## API Documentation

The API Gateway provides OpenAPI documentation via Swagger UI. Once the system is running, you can access it at:

-   `http://localhost:5000/swagger-ui/`

![Swagger UI](./backend/images/swagger-ui.png)

## Project Structure

-   `crates/`: Contains all the independent Rust modules (services).
    -   `apigateway`: The public-facing REST API gateway.
    -   `auth`, `user`, `card`, etc.: Internal services, each representing a business domain.
    -   `genproto`: Crate for compiling `.proto` files into Rust code for gRPC.
-   `proto/`: Protobuf definition files.
-   `migrations/`: SQLx database migrations.
-   `docker-compose.yml`: Defines the local development environment.
-   `k8s/`: Contains all Kubernetes manifests for deployment.
-   `observability/`: Configuration files for Prometheus, Grafana, Loki, etc.

<details>
<summary><b>Manual Installation (Without Containers)</b></summary>

### Prerequisites

-   [Rust & Cargo](https://www.rust-lang.org/tools/install)
-   [`sqlx-cli`](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli)
-   [`protoc`](https://grpc.io/docs/protoc-installation/)
-   A running PostgreSQL instance.

### Installation Steps

1.  **Clone Repository**

    ```bash
    git clone https://github.com/MamangRust/example-payment-gateway-sqlx.git
    cd example-payment-gateway-sqlx/backend
    ```

2.  **Setup Environment**
    Create and edit an `.env` file with your database configuration.

3.  **Database Migration**

    ```bash
    sqlx migrate run
    ```

4.  **Build Protobuf**

    ```bash
    cargo build -p genproto
    ```

5.  **Build All Services**
    ```bash
    cargo build --workspace
    ```

### Running the Application

You need to run each service in a separate terminal.

```bash
# Terminal 1: API Gateway
cargo run -p apigateway

# Terminal 2: Auth Service
cargo run -p auth

# ... and so on for every other service in the `crates` directory.
```

</details>


## Preview

**Jaeger UI**
![Jaeger UI](./backend/images/jaeger.png)

**Node Exporter**
![Node Exporter](./backend/images/node-exporter.png)

**Monitoring Memory**
![Monitoring Memotry](./backend/images/memory_allocation.png)

**Monitoring Card Service**
![Card-Service](./backend/images/CardService.png)

**Monitoring Merchant Service**
![Merchant-Service](./backend/images/MerchantServic.png)

**Monitoring User Service**
![User-Service](./backend/images/UserService.png)

**Monitoring Role Service**
![Role-Service](./backend/images/RoleService.png)

**Monitoring Saldo Service**
![Saldo-Service](./backend/images/SaldoService.png)

**Monitoring Topup Service**
![Topup-Service](./backend/images/TopupService.png)

**Monitoring Transaction Service**
![Transaction-Service](./backend/images/TransactionService.png)

**Monitoring Transfer Service**
![Transfer-Service](./backend/images/TransferService.png)

**Monitoring Withdraw Service**
![Withdraw-Service](./backend/images/TransferService.png)
