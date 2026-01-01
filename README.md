# Axum Rust Starter – Production-Ready API Boilerplate

![Rust](https://img.shields.io/badge/Rust-1.83-000000?style=flat&logo=rust&logoColor=white)
![Axum](https://img.shields.io/badge/Axum-0.8-000000?style=flat&logo=rust&logoColor=white)
![PostgreSQL](https://img.shields.io/badge/PostgreSQL-17-4169E1?style=flat&logo=postgresql&logoColor=white)
![SQLx](https://img.shields.io/badge/SQLx-0.8-orange?style=flat)
![Docker](https://img.shields.io/badge/Docker-Ready-2496ED?style=flat&logo=docker&logoColor=white)

A modern **Rust backend boilerplate** powered by **Axum** framework. This project is designed for performance, type safety, and production readiness, with database migrations, authentication, health checks, structured logging, and comprehensive testing preconfigured.

---

## ✨ Features

- ⚡ **Blazing fast** Axum web framework
- 🔒 **Type-safe** database queries with SQLx
- 🐘 **PostgreSQL** with async connection pooling
- 🗄️ **MongoDB** integration for NoSQL (optional)
- 🔴 **Redis** for caching, sessions, and pub/sub (optional)
- 📤 **File upload** with multipart form support
- ☁️ **AWS S3** integration for cloud storage
- 🌐 **Third-party APIs** - payment gateways, WhatsApp, email services
- 🔐 **JWT authentication** ready
- 🛡️ **Security** - rate limiting, security headers, input validation
- 📖 **OpenAPI/Swagger** auto-generation with utoipa
- 🔄 **Background tasks** with Tokio (optional)
- 🔌 **WebSocket** support for real-time features (optional)
- 🚀 **CI/CD** - GitHub Actions workflows included
- 🩺 **Health check** endpoint with database connectivity
- 📝 **Structured logging** with tracing & tracing-subscriber
- 🗃️ **Database migrations** with SQLx CLI
- 🧪 **Testing setup** with testcontainers for integration tests
- 🐳 **Docker & Docker Compose** ready
- 🔧 **Environment-based configuration**
- 📊 **Request ID tracking** for distributed tracing
- ⏱️ **Timezone support** with chrono-tz

---

## 🧰 Tech Stack

| Category | Technology |
|--------|------------|
| Language | Rust `1.83` |
| Web Framework | Axum `0.8` |
| Database | PostgreSQL `17` |
| ORM/Query | SQLx `0.8` (compile-time checked) |
| NoSQL | MongoDB `2.8` (optional) |
| Cache/Session | Redis `0.24` (optional) |
| Authentication | JWT with jsonwebtoken |
| File Storage | AWS S3 SDK |
| HTTP Client | reqwest `0.11` |
| API Docs | utoipa (OpenAPI/Swagger) |
| Security | tower-governor (rate limiting) |
| WebSocket | Axum WebSocket (optional) |
| Background Jobs | Tokio tasks (optional) |
| Logging | tracing, tracing-subscriber |
| Testing | cargo test, testcontainers |
| Serialization | serde, serde_json |
| Environment | dotenvy |
| Date/Time | chrono, chrono-tz |
| CI/CD | GitHub Actions |

---

## 🚀 Getting Started

### Prerequisites

- Rust 1.83 or later
- PostgreSQL 17 (or use Docker Compose)
- SQLx CLI: `cargo install sqlx-cli --no-default-features --features postgres`

### Install dependencies

```bash
cargo build
```

### Setup environment variables

Copy the example environment file:

```bash
cp .env.example .env
```

Edit `.env` with your configuration:

```env
# Server
HOST=127.0.0.1
PORT=8000
RUST_LOG=debug

# Database
DATABASE_URL=postgresql://postgres:password@localhost:5432/app_db

# JWT
JWT_SECRET=your-super-secret-jwt-key-change-in-production
JWT_EXPIRATION=86400

# Timezone
TZ=Asia/Jakarta
```

### Start PostgreSQL with Docker

```bash
docker-compose up -d postgres
```

### Run database migrations

```bash
sqlx migrate run
```

### Start development server

```bash
cargo run
```

Or with auto-reload using `cargo-watch`:

```bash
cargo install cargo-watch
cargo watch -x run
```

The application will be available at:

```
http://localhost:8000
```

---

## 🏗 Build & Production

### Build for release

```bash
cargo build --release
```

### Run production binary

```bash
./target/release/axum-rust-starter
```

### Docker deployment

Build and run with Docker:

```bash
docker-compose up --build
```

---

## 🧪 Testing

This project includes unit tests and integration tests with testcontainers for database testing.

### Run all tests

```bash
cargo test
```

### Run tests with output

```bash
cargo test -- --nocapture
```

### Run specific test

```bash
cargo test test_name
```

### Integration tests with database

Integration tests automatically spin up PostgreSQL containers using testcontainers:

```bash
cargo test --test integration_tests
```

---

## 🩺 Health Check

A health check endpoint is included to verify application readiness and database connectivity.

**Access the endpoint**:

```bash
GET /api/health
```

**Response example**:

```json
{
  "status": "healthy",
  "timestamp": "2026-01-01T10:30:00Z",
  "version": "1.0.0",
  "database": "connected",
  "uptime_seconds": 3600
}
```

Useful for:
- Kubernetes liveness/readiness probes
- Load balancer health checks
- Monitoring systems

---

## 📝 Logger

This project uses **tracing** for structured, async-aware logging.

**Features**:

- **Environment-driven log level** via `RUST_LOG`
- **Request ID tracking** across middleware and handlers
- **Structured fields** for better log filtering
- **JSON formatting** option for production

**Usage example**:

```rust
use tracing::{info, warn, error};

info!("Server started on port {}", port);
warn!(user_id = %user.id, "Failed login attempt");
error!(error = ?e, "Database connection failed");
```

**Environment setup**:

```bash
# Development (verbose)
RUST_LOG=debug cargo run

# Production (minimal)
RUST_LOG=info cargo run

# Specific modules
RUST_LOG=axum_rust_starter=debug,sqlx=warn cargo run
```

---

## 🗃️ Database Migrations

### Create a new migration

```bash
sqlx migrate add create_users_table
```

This creates timestamped migration files in `migrations/`:
- `{timestamp}_create_users_table.up.sql`
- `{timestamp}_create_users_table.down.sql`

### Run migrations

```bash
sqlx migrate run
```

### Revert last migration

```bash
sqlx migrate revert
```

### Check migration status

```bash
sqlx migrate info
```

---

## 🔐 Authentication

JWT-based authentication is preconfigured with middleware.

**Protected route example**:

```rust
use crate::middleware::auth::AuthUser;

async fn protected_handler(
    AuthUser(user): AuthUser,
) -> Json<Value> {
    Json(json!({
        "message": "Access granted",
        "user_id": user.id
    }))
}
```

**Generate JWT token**:

```rust
use crate::utils::jwt::create_token;

let token = create_token(user.id)?;
```

---

## 📤 File Upload & S3 Integration

### Multipart Form Handling

Handle file uploads with `axum::extract::Multipart`:

```rust
use axum::extract::Multipart;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

async fn upload_file(
    mut multipart: Multipart,
) -> Result<Json<Value>, AppError> {
    while let Some(field) = multipart.next_field().await? {
        let name = field.name().unwrap_or("").to_string();
        let filename = field.file_name().unwrap_or("").to_string();
        let data = field.bytes().await?;
        
        // Save to disk
        let path = format!("./uploads/{}", filename);
        let mut file = File::create(&path).await?;
        file.write_all(&data).await?;
        
        info!("Uploaded file: {}", filename);
    }
    
    Ok(Json(json!({"message": "File uploaded"})))
}
```

### AWS S3 Integration

Upload files directly to S3 using `aws-sdk-s3`:

**Add to Cargo.toml**:
```toml
aws-config = "1.1"
aws-sdk-s3 = "1.13"
```

**Implementation**:

```rust
use aws_sdk_s3::{Client, primitives::ByteStream};
use aws_config::BehaviorVersion;

// Initialize S3 client
pub async fn create_s3_client() -> Client {
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region("ap-southeast-1")
        .load()
        .await;
    Client::new(&config)
}

// Upload to S3
pub async fn upload_to_s3(
    client: &Client,
    bucket: &str,
    key: &str,
    data: Vec<u8>,
) -> Result<String, AppError> {
    let body = ByteStream::from(data);
    
    client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(body)
        .content_type("image/jpeg")
        .send()
        .await?;
    
    let url = format!("https://{}.s3.amazonaws.com/{}", bucket, key);
    Ok(url)
}

// Usage in handler
async fn upload_image(
    State(s3_client): State<Client>,
    mut multipart: Multipart,
) -> Result<Json<Value>, AppError> {
    while let Some(field) = multipart.next_field().await? {
        let filename = field.file_name().unwrap_or("").to_string();
        let data = field.bytes().await?.to_vec();
        
        let key = format!("uploads/{}", filename);
        let url = upload_to_s3(&s3_client, "my-bucket", &key, data).await?;
        
        return Ok(Json(json!({ "url": url })));
    }
    
    Err(AppError::BadRequest("No file provided".into()))
}
```

**Environment variables**:
```env
AWS_ACCESS_KEY_ID=your_access_key
AWS_SECRET_ACCESS_KEY=your_secret_key
AWS_REGION=ap-southeast-1
S3_BUCKET=your-bucket-name
```

---

## 🗄️ MongoDB Integration

For NoSQL database support alongside PostgreSQL:

**Add to Cargo.toml**:
```toml
mongodb = "2.8"
bson = { version = "2.9", features = ["chrono-0_4"] }
```

**Setup MongoDB client**:

```rust
use mongodb::{Client, Database, Collection};
use mongodb::options::ClientOptions;

pub async fn create_mongo_client(uri: &str) -> Result<Client, mongodb::error::Error> {
    let client_options = ClientOptions::parse(uri).await?;
    let client = Client::with_options(client_options)?;
    
    // Ping to verify connection
    client
        .database("admin")
        .run_command(doc! { "ping": 1 }, None)
        .await?;
    
    info!("Connected to MongoDB");
    Ok(client)
}

pub fn get_database(client: &Client) -> Database {
    client.database("myapp_db")
}
```

**Model example**:

```rust
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub content: String,
    pub author_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
```

**CRUD operations**:

```rust
use mongodb::bson::doc;

// Create
async fn create_article(
    State(db): State<Database>,
    Json(article): Json<Article>,
) -> Result<Json<Value>, AppError> {
    let collection: Collection<Article> = db.collection("articles");
    let result = collection.insert_one(article, None).await?;
    
    Ok(Json(json!({
        "id": result.inserted_id.as_object_id().unwrap().to_hex()
    })))
}

// Read
async fn get_article(
    State(db): State<Database>,
    Path(id): Path<String>,
) -> Result<Json<Article>, AppError> {
    let collection: Collection<Article> = db.collection("articles");
    let oid = ObjectId::parse_str(&id)?;
    
    let article = collection
        .find_one(doc! { "_id": oid }, None)
        .await?
        .ok_or(AppError::NotFound)?;
    
    Ok(Json(article))
}

// Update
async fn update_article(
    State(db): State<Database>,
    Path(id): Path<String>,
    Json(update): Json<ArticleUpdate>,
) -> Result<Json<Value>, AppError> {
    let collection: Collection<Article> = db.collection("articles");
    let oid = ObjectId::parse_str(&id)?;
    
    let result = collection
        .update_one(
            doc! { "_id": oid },
            doc! { "$set": bson::to_document(&update)? },
            None,
        )
        .await?;
    
    Ok(Json(json!({ "modified": result.modified_count })))
}

// Delete
async fn delete_article(
    State(db): State<Database>,
    Path(id): Path<String>,
) -> Result<Json<Value>, AppError> {
    let collection: Collection<Article> = db.collection("articles");
    let oid = ObjectId::parse_str(&id)?;
    
    collection.delete_one(doc! { "_id": oid }, None).await?;
    Ok(Json(json!({ "message": "Deleted" })))
}
```

**Environment variables**:
```env
MONGODB_URI=mongodb://localhost:27017
MONGODB_DATABASE=myapp_db
```

---

## 🔴 Redis Integration

For caching, session storage, or pub/sub:

**Add to Cargo.toml**:
```toml
redis = { version = "0.24", features = ["tokio-comp", "connection-manager"] }
```

**Setup Redis client**:

```rust
use redis::{Client, AsyncCommands};
use redis::aio::ConnectionManager;

pub async fn create_redis_client(url: &str) -> Result<ConnectionManager, redis::RedisError> {
    let client = Client::open(url)?;
    let manager = ConnectionManager::new(client).await?;
    
    info!("Connected to Redis");
    Ok(manager)
}
```

**Caching example**:

```rust
use redis::AsyncCommands;

async fn get_user_cached(
    State(mut redis): State<ConnectionManager>,
    State(db): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<User>, AppError> {
    let cache_key = format!("user:{}", id);
    
    // Try cache first
    if let Ok(Some(cached)) = redis.get::<_, Option<String>>(&cache_key).await {
        let user: User = serde_json::from_str(&cached)?;
        info!("Cache hit for user {}", id);
        return Ok(Json(user));
    }
    
    // Cache miss - fetch from database
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(&db)
        .await?;
    
    // Store in cache (expire in 1 hour)
    let user_json = serde_json::to_string(&user)?;
    redis.set_ex(&cache_key, user_json, 3600).await?;
    
    info!("Cache miss for user {}, stored in cache", id);
    Ok(Json(user))
}
```

**Session storage**:

```rust
// Store session
async fn login(
    State(mut redis): State<ConnectionManager>,
    Json(credentials): Json<LoginRequest>,
) -> Result<Json<Value>, AppError> {
    // Verify credentials...
    let user = verify_user(credentials)?;
    
    let session_id = uuid::Uuid::new_v4().to_string();
    let session_key = format!("session:{}", session_id);
    
    // Store session data (expire in 24 hours)
    let session_data = serde_json::to_string(&user)?;
    redis.set_ex(&session_key, session_data, 86400).await?;
    
    Ok(Json(json!({ "session_id": session_id })))
}

// Verify session
async fn verify_session(
    State(mut redis): State<ConnectionManager>,
    session_id: &str,
) -> Result<User, AppError> {
    let session_key = format!("session:{}", session_id);
    
    let session_data: Option<String> = redis.get(&session_key).await?;
    let user: User = serde_json::from_str(
        &session_data.ok_or(AppError::Unauthorized)?
    )?;
    
    Ok(user)
}
```

**Pub/Sub example**:

```rust
use redis::AsyncCommands;

// Publisher
async fn publish_event(
    State(mut redis): State<ConnectionManager>,
    Json(event): Json<Event>,
) -> Result<Json<Value>, AppError> {
    let channel = "notifications";
    let message = serde_json::to_string(&event)?;
    
    redis.publish::<_, _, ()>(channel, message).await?;
    
    Ok(Json(json!({ "published": true })))
}

// Subscriber (in separate task)
pub async fn subscribe_to_events(mut redis: ConnectionManager) {
    let mut pubsub = redis.into_pubsub();
    pubsub.subscribe("notifications").await.unwrap();
    
    loop {
        let msg = pubsub.on_message().next().await.unwrap();
        let payload: String = msg.get_payload().unwrap();
        
        if let Ok(event) = serde_json::from_str::<Event>(&payload) {
            info!("Received event: {:?}", event);
            // Process event...
        }
    }
}
```

**Environment variables**:
```env
REDIS_URL=redis://localhost:6379
```

---

## 🌐 Third-Party API Integration

### HTTP Client Setup

Use `reqwest` for external API calls:

**Add to Cargo.toml**:
```toml
reqwest = { version = "0.11", features = ["json"] }
```

**Create API client**:

```rust
use reqwest::{Client, header};

pub fn create_http_client() -> Client {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_static("MyApp/1.0")
    );
    
    Client::builder()
        .timeout(Duration::from_secs(30))
        .default_headers(headers)
        .build()
        .unwrap()
}
```

### Payment Gateway Example (Midtrans/Xendit)

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct PaymentRequest {
    order_id: String,
    amount: i64,
    customer_email: String,
}

#[derive(Deserialize)]
struct PaymentResponse {
    token: String,
    redirect_url: String,
}

async fn create_payment(
    State(client): State<Client>,
    Json(order): Json<PaymentRequest>,
) -> Result<Json<PaymentResponse>, AppError> {
    let api_key = std::env::var("MIDTRANS_SERVER_KEY")?;
    let base_url = std::env::var("MIDTRANS_API_URL")?;
    
    let response = client
        .post(format!("{}/charge", base_url))
        .basic_auth(&api_key, Some(""))
        .json(&order)
        .send()
        .await?;
    
    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(AppError::ExternalAPI(error_text));
    }
    
    let payment: PaymentResponse = response.json().await?;
    Ok(Json(payment))
}
```

### WhatsApp API Example (Fonnte/Wablas)

```rust
#[derive(Serialize)]
struct WhatsAppMessage {
    target: String,
    message: String,
}

async fn send_whatsapp(
    State(client): State<Client>,
    phone: &str,
    message: &str,
) -> Result<(), AppError> {
    let api_key = std::env::var("WHATSAPP_API_KEY")?;
    let api_url = std::env::var("WHATSAPP_API_URL")?;
    
    let payload = WhatsAppMessage {
        target: phone.to_string(),
        message: message.to_string(),
    };
    
    client
        .post(&api_url)
        .header("Authorization", api_key)
        .json(&payload)
        .send()
        .await?;
    
    info!("WhatsApp sent to {}", phone);
    Ok(())
}
```

### Email Service Example (Resend/SendGrid)

```rust
#[derive(Serialize)]
struct EmailRequest {
    from: String,
    to: Vec<String>,
    subject: String,
    html: String,
}

async fn send_email(
    State(client): State<Client>,
    email: EmailRequest,
) -> Result<(), AppError> {
    let api_key = std::env::var("RESEND_API_KEY")?;
    
    let response = client
        .post("https://api.resend.com/emails")
        .bearer_auth(&api_key)
        .json(&email)
        .send()
        .await?;
    
    if !response.status().is_success() {
        let error = response.text().await?;
        return Err(AppError::ExternalAPI(error));
    }
    
    info!("Email sent to {:?}", email.to);
    Ok(())
}
```

### General Third-Party API Pattern

```rust
// Create a reusable API client struct
pub struct ExternalAPI {
    client: Client,
    base_url: String,
    api_key: String,
}

impl ExternalAPI {
    pub fn new(base_url: String, api_key: String) -> Self {
        Self {
            client: create_http_client(),
            base_url,
            api_key,
        }
    }
    
    pub async fn get<T: DeserializeOwned>(
        &self,
        endpoint: &str,
    ) -> Result<T, AppError> {
        let url = format!("{}/{}", self.base_url, endpoint);
        
        let response = self.client
            .get(&url)
            .bearer_auth(&self.api_key)
            .send()
            .await?;
        
        let data = response.json::<T>().await?;
        Ok(data)
    }
    
    pub async fn post<T: Serialize, R: DeserializeOwned>(
        &self,
        endpoint: &str,
        payload: &T,
    ) -> Result<R, AppError> {
        let url = format!("{}/{}", self.base_url, endpoint);
        
        let response = self.client
            .post(&url)
            .bearer_auth(&self.api_key)
            .json(payload)
            .send()
            .await?;
        
        let data = response.json::<R>().await?;
        Ok(data)
    }
}

// Usage
let api = ExternalAPI::new(
    env::var("API_BASE_URL")?,
    env::var("API_KEY")?,
);

let result: ApiResponse = api.get("users/123").await?;
```

**Environment variables for third-party APIs**:
```env
# Payment
MIDTRANS_SERVER_KEY=your_server_key
MIDTRANS_API_URL=https://api.sandbox.midtrans.com/v2

# WhatsApp
WHATSAPP_API_KEY=your_api_key
WHATSAPP_API_URL=https://api.fonnte.com/send

# Email
RESEND_API_KEY=re_your_api_key

# Other APIs
EXTERNAL_API_KEY=your_key
EXTERNAL_API_URL=https://api.example.com
```

---

## 📖 OpenAPI / Swagger Documentation

Auto-generate API documentation with **utoipa**.

**Add to Cargo.toml**:
```toml
utoipa = { version = "4.2", features = ["axum_extras"] }
utoipa-swagger-ui = { version = "6.0", features = ["axum"] }
```

**Setup OpenAPI**:

```rust
// src/main.rs
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        routes::health::health_check,
        routes::users::get_user,
        routes::users::create_user,
        routes::users::list_users,
        routes::auth::login,
        routes::auth::register,
    ),
    components(
        schemas(User, CreateUserRequest, LoginRequest, LoginResponse)
    ),
    tags(
        (name = "users", description = "User management endpoints"),
        (name = "auth", description = "Authentication endpoints"),
    ),
    info(
        title = "Axum Rust Starter API",
        version = "1.0.0",
        description = "Production-ready REST API built with Axum",
        contact(
            name = "API Support",
            email = "support@example.com"
        )
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    // ... setup
    
    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui")
            .url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/api/users", get(list_users).post(create_user))
        .route("/api/users/:id", get(get_user))
        // ... other routes
        .with_state(state);
    
    // ... start server
}
```

**Document route handlers**:

```rust
// src/routes/users.rs
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub name: String,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateUserRequest {
    pub email: String,
    pub name: String,
    pub password: String,
}

/// Get user by ID
#[utoipa::path(
    get,
    path = "/api/users/{id}",
    params(
        ("id" = i32, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User found successfully", body = User),
        (status = 404, description = "User not found"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "users",
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_user(
    Path(id): Path<i32>,
    State(db): State<PgPool>,
) -> Result<Json<User>, AppError> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(&db)
        .await?;
    
    Ok(Json(user))
}

/// Create new user
#[utoipa::path(
    post,
    path = "/api/users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created successfully", body = User),
        (status = 400, description = "Invalid input"),
        (status = 409, description = "User already exists")
    ),
    tag = "users"
)]
pub async fn create_user(
    State(db): State<PgPool>,
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<User>, AppError> {
    // ... create user logic
}
```

**Access Swagger UI**:
```
http://localhost:8000/swagger-ui
```

**Features**:
- 📄 Auto-generated OpenAPI 3.0 spec
- 🎨 Interactive Swagger UI
- ✅ Type-safe schemas
- 🔐 Security definitions (JWT)
- 📝 Request/response examples

---

## 🛡️ Security

### Rate Limiting

Use **tower-governor** for in-memory rate limiting:

**Add to Cargo.toml**:
```toml
tower-governor = "0.4"
```

**Implementation**:

```rust
// src/middleware/rate_limit.rs
use tower_governor::{
    governor::GovernorConfigBuilder,
    GovernorLayer,
    key_extractor::SmartIpKeyExtractor,
};
use std::time::Duration;

pub fn create_rate_limiter() -> GovernorLayer<SmartIpKeyExtractor> {
    let governor_conf = Box::new(
        GovernorConfigBuilder::default()
            .per_second(10)      // 10 requests per second
            .burst_size(20)      // Allow burst of 20 requests
            .finish()
            .unwrap(),
    );
    
    GovernorLayer {
        config: Box::leak(governor_conf),
    }
}

// Apply to routes
let app = Router::new()
    .route("/api/users", get(list_users))
    .layer(create_rate_limiter());
```

**Per-endpoint rate limiting**:

```rust
// Strict limit for auth endpoints
let auth_limiter = GovernorConfigBuilder::default()
    .per_minute(5)
    .burst_size(10)
    .finish()
    .unwrap();

// Generous limit for public endpoints
let public_limiter = GovernorConfigBuilder::default()
    .per_second(50)
    .burst_size(100)
    .finish()
    .unwrap();

let app = Router::new()
    .route("/api/auth/login", post(login))
    .layer(GovernorLayer { config: Box::leak(Box::new(auth_limiter)) })
    .route("/api/public", get(public_handler))
    .layer(GovernorLayer { config: Box::leak(Box::new(public_limiter)) });
```

### Security Headers

```rust
// src/middleware/security_headers.rs
use axum::{
    http::{Request, header},
    middleware::Next,
    response::Response,
};

pub async fn security_headers<B>(
    request: Request<B>,
    next: Next<B>,
) -> Response {
    let mut response = next.run(request).await;
    
    let headers = response.headers_mut();
    
    // Prevent clickjacking
    headers.insert(
        header::HeaderName::from_static("x-frame-options"),
        header::HeaderValue::from_static("DENY"),
    );
    
    // Prevent MIME sniffing
    headers.insert(
        header::HeaderName::from_static("x-content-type-options"),
        header::HeaderValue::from_static("nosniff"),
    );
    
    // XSS protection
    headers.insert(
        header::HeaderName::from_static("x-xss-protection"),
        header::HeaderValue::from_static("1; mode=block"),
    );
    
    // Referrer policy
    headers.insert(
        header::HeaderName::from_static("referrer-policy"),
        header::HeaderValue::from_static("strict-origin-when-cross-origin"),
    );
    
    // Content Security Policy (adjust as needed)
    headers.insert(
        header::HeaderName::from_static("content-security-policy"),
        header::HeaderValue::from_static("default-src 'self'"),
    );
    
    // Strict Transport Security (HTTPS only)
    headers.insert(
        header::HeaderName::from_static("strict-transport-security"),
        header::HeaderValue::from_static("max-age=31536000; includeSubDomains"),
    );
    
    response
}

// Apply globally
let app = Router::new()
    .route("/api/users", get(list_users))
    .layer(middleware::from_fn(security_headers));
```

### Validator Organization

This project uses a **hybrid approach** for organizing validators:

**Shared validators** → `src/validators/` directory:
- `common.rs` - Reusable validators (password, username, slug)
- `indonesian.rs` - Indonesian-specific (NIK, phone, postal code)
- `file.rs` - File validation utilities

**DTO-specific validators** → In the DTO file itself (e.g., `src/dto/auth.rs`)

**Structure**:

```rust
// src/validators/mod.rs
pub mod common;
pub mod indonesian;
pub mod file;

// Re-export commonly used validators
pub use common::*;
pub use indonesian::*;
pub use file::*;

// src/validators/common.rs
use validator::ValidationError;
use std::borrow::Cow;

pub fn validate_password_strength(password: &str) -> Result<(), ValidationError> {
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_numeric());
    let has_special = password.chars().any(|c| !c.is_alphanumeric());
    
    if has_uppercase && has_lowercase && has_digit && has_special {
        Ok(())
    } else {
        Err(ValidationError::new("password_too_weak")
            .with_message(Cow::from(
                "Password must contain uppercase, lowercase, digit, and special character"
            )))
    }
}

pub fn validate_username(username: &str) -> Result<(), ValidationError> {
    let is_valid = username.chars().all(|c| c.is_alphanumeric() || c == '_')
        && username.len() >= 3;
    
    if is_valid {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_username"))
    }
}

// src/validators/indonesian.rs
use validator::ValidationError;
use regex::Regex;
use std::borrow::Cow;

lazy_static::lazy_static! {
    static ref INDO_PHONE_REGEX: Regex = 
        Regex::new(r"^(\+62|62|0)8[1-9][0-9]{6,10}$").unwrap();
    static ref NIK_REGEX: Regex = Regex::new(r"^\d{16}$").unwrap();
    static ref POSTAL_CODE_REGEX: Regex = Regex::new(r"^\d{5}$").unwrap();
}

pub fn validate_indonesian_phone(phone: &str) -> Result<(), ValidationError> {
    if INDO_PHONE_REGEX.is_match(phone) {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_phone")
            .with_message(Cow::from(
                "Phone must be Indonesian format (08xx, +628xx, or 628xx)"
            )))
    }
}

pub fn validate_nik(nik: &str) -> Result<(), ValidationError> {
    if NIK_REGEX.is_match(nik) {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_nik")
            .with_message(Cow::from("NIK must be exactly 16 digits")))
    }
}

pub fn validate_postal_code(code: &str) -> Result<(), ValidationError> {
    if POSTAL_CODE_REGEX.is_match(code) {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_postal_code"))
    }
}

// src/validators/file.rs
#[derive(Debug)]
pub struct FileValidationError {
    pub message: String,
}

const MAX_FILE_SIZE: usize = 5 * 1024 * 1024; // 5MB
const ALLOWED_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "pdf"];
const ALLOWED_MIME_TYPES: &[&str] = &["image/jpeg", "image/png", "application/pdf"];

pub fn validate_file_extension(filename: &str) -> Result<(), FileValidationError> {
    let extension = std::path::Path::new(filename)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase());
    
    match extension {
        Some(ext) if ALLOWED_EXTENSIONS.contains(&ext.as_str()) => Ok(()),
        _ => Err(FileValidationError {
            message: format!("Extension not allowed. Allowed: {}", 
                ALLOWED_EXTENSIONS.join(", ")),
        }),
    }
}

pub fn validate_file_size(size: usize) -> Result<(), FileValidationError> {
    if size > MAX_FILE_SIZE {
        Err(FileValidationError {
            message: format!("File too large. Max: {} MB", MAX_FILE_SIZE / 1024 / 1024),
        })
    } else if size == 0 {
        Err(FileValidationError { message: "File is empty".into() })
    } else {
        Ok(())
    }
}

pub fn validate_mime_type(content_type: &str) -> Result<(), FileValidationError> {
    if ALLOWED_MIME_TYPES.contains(&content_type) {
        Ok(())
    } else {
        Err(FileValidationError {
            message: format!("Invalid type. Allowed: {}", ALLOWED_MIME_TYPES.join(", ")),
        })
    }
}

pub fn validate_image_content(data: &[u8]) -> Result<(), FileValidationError> {
    if data.len() < 12 {
        return Err(FileValidationError {
            message: "File too small to be valid image".into(),
        });
    }
    
    let is_jpeg = data.starts_with(&[0xFF, 0xD8, 0xFF]);
    let is_png = data.starts_with(&[0x89, 0x50, 0x4E, 0x47]);
    let is_gif = data.starts_with(b"GIF87a") || data.starts_with(b"GIF89a");
    let is_webp = &data[0..4] == b"RIFF" && &data[8..12] == b"WEBP";
    
    if is_jpeg || is_png || is_gif || is_webp {
        Ok(())
    } else {
        Err(FileValidationError {
            message: "Not a valid image format".into(),
        })
    }
}

pub fn sanitize_filename(filename: &str) -> String {
    filename
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '.' || c == '-' || c == '_' { 
            c 
        } else { 
            '_' 
        })
        .collect::<String>()
        .replace("..", "_")
}
```

**Usage in DTOs**:

```rust
// src/dto/auth.rs
use crate::validators::{validate_password_strength, validate_indonesian_phone};
use validator::Validate;

// DTO-specific validator (only used by this DTO)
fn validate_referral_code(code: &str) -> Result<(), validator::ValidationError> {
    if code.len() == 6 && code.chars().all(|c| c.is_alphanumeric()) {
        Ok(())
    } else {
        Err(validator::ValidationError::new("invalid_referral_code"))
    }
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct RegisterRequest {
    #[validate(email)]
    pub email: String,
    
    // Use shared validator from validators/
    #[validate(custom = "crate::validators::validate_indonesian_phone")]
    pub phone: String,
    
    // Use shared validator from validators/
    #[validate(custom = "crate::validators::validate_password_strength")]
    pub password: String,
    
    // Use DTO-specific validator
    #[validate(custom = "validate_referral_code")]
    pub referral_code: Option<String>,
}
```

**Benefits of this approach**:
- ✅ Shared validators are reusable across DTOs
- ✅ DTO-specific validators stay close to their usage
- ✅ Easy to find and maintain
- ✅ No circular dependencies
- ✅ Easy to unit test validators independently

---

### Input Validation

**Add to Cargo.toml**:
```toml
validator = { version = "0.18", features = ["derive"] }
regex = "1.10"
```

#### JSON Body Validation

**Basic validation**:

```rust
use validator::{Validate, ValidationError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateUserRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    
    #[validate(length(min = 3, max = 50, message = "Name must be between 3-50 characters"))]
    pub name: String,
    
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    #[validate(custom = "validate_password_strength")]
    pub password: String,
    
    #[validate(range(min = 13, max = 120, message = "Age must be between 13-120"))]
    pub age: Option<i32>,
    
    #[validate(phone(message = "Invalid phone number"))]
    pub phone: Option<String>,
    
    #[validate(url(message = "Invalid URL format"))]
    pub website: Option<String>,
}

// Custom password validation
fn validate_password_strength(password: &str) -> Result<(), ValidationError> {
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_numeric());
    let has_special = password.chars().any(|c| !c.is_alphanumeric());
    
    if has_uppercase && has_lowercase && has_digit && has_special {
        Ok(())
    } else {
        Err(ValidationError::new("password_too_weak")
            .with_message(std::borrow::Cow::from(
                "Password must contain uppercase, lowercase, digit, and special character"
            )))
    }
}

// In handler
#[utoipa::path(
    post,
    path = "/api/users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created", body = User),
        (status = 400, description = "Validation error")
    )
)]
pub async fn create_user(
    State(db): State<PgPool>,
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<User>, AppError> {
    // Validate request
    req.validate()
        .map_err(|e| AppError::ValidationError(e))?;
    
    // ... create user logic
}
```

**Nested validation**:

```rust
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateOrderRequest {
    #[validate]
    pub customer: CustomerInfo,
    
    #[validate]
    pub items: Vec<OrderItem>,
    
    #[validate(length(min = 1, message = "At least one item required"))]
    items_check: Option<String>, // Dummy field for collection validation
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CustomerInfo {
    #[validate(email)]
    pub email: String,
    
    #[validate(length(min = 3, max = 100))]
    pub name: String,
    
    #[validate]
    pub address: Address,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct Address {
    #[validate(length(min = 5, max = 200))]
    pub street: String,
    
    #[validate(length(min = 2, max = 50))]
    pub city: String,
    
    #[validate(regex = "POSTAL_CODE_REGEX")]
    pub postal_code: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct OrderItem {
    pub product_id: i32,
    
    #[validate(range(min = 1, max = 1000))]
    pub quantity: i32,
    
    #[validate(range(min = 0.01))]
    pub price: f64,
}

// Regex validators
lazy_static::lazy_static! {
    static ref POSTAL_CODE_REGEX: regex::Regex = 
        regex::Regex::new(r"^\d{5}$").unwrap();
}
```

**Custom validators for Indonesian context**:

```rust
use regex::Regex;

lazy_static::lazy_static! {
    // Indonesian phone number: 08xx or +628xx or 628xx
    static ref INDO_PHONE_REGEX: Regex = 
        regex::Regex::new(r"^(\+62|62|0)8[1-9][0-9]{6,10}$").unwrap();
    
    // Indonesian NIK (16 digits)
    static ref NIK_REGEX: Regex = 
        regex::Regex::new(r"^\d{16}$").unwrap();
    
    // Indonesian postal code (5 digits)
    static ref POSTAL_CODE_REGEX: Regex = 
        regex::Regex::new(r"^\d{5}$").unwrap();
}

fn validate_indonesian_phone(phone: &str) -> Result<(), ValidationError> {
    if INDO_PHONE_REGEX.is_match(phone) {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_phone")
            .with_message(std::borrow::Cow::from(
                "Phone number must be valid Indonesian format (08xx, +628xx, or 628xx)"
            )))
    }
}

fn validate_nik(nik: &str) -> Result<(), ValidationError> {
    if NIK_REGEX.is_match(nik) {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_nik")
            .with_message(std::borrow::Cow::from(
                "NIK must be exactly 16 digits"
            )))
    }
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RegisterIndonesianUser {
    #[validate(email)]
    pub email: String,
    
    #[validate(custom = "validate_indonesian_phone")]
    pub phone: String,
    
    #[validate(custom = "validate_nik")]
    pub nik: String,
    
    #[validate(regex = "POSTAL_CODE_REGEX")]
    pub postal_code: String,
}
```

#### Multipart Form Validation

**File upload with validation**:

```rust
use axum::extract::Multipart;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

const MAX_FILE_SIZE: usize = 5 * 1024 * 1024; // 5MB
const ALLOWED_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "pdf"];
const ALLOWED_MIME_TYPES: &[&str] = &[
    "image/jpeg",
    "image/png",
    "application/pdf",
];

#[derive(Debug)]
pub struct FileValidationError {
    pub message: String,
}

impl From<FileValidationError> for AppError {
    fn from(err: FileValidationError) -> Self {
        AppError::BadRequest(err.message)
    }
}

// Validate file extension
fn validate_file_extension(filename: &str) -> Result<(), FileValidationError> {
    let extension = std::path::Path::new(filename)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase());
    
    match extension {
        Some(ext) if ALLOWED_EXTENSIONS.contains(&ext.as_str()) => Ok(()),
        _ => Err(FileValidationError {
            message: format!(
                "File extension not allowed. Allowed: {}",
                ALLOWED_EXTENSIONS.join(", ")
            ),
        }),
    }
}

// Validate file size
fn validate_file_size(size: usize) -> Result<(), FileValidationError> {
    if size > MAX_FILE_SIZE {
        Err(FileValidationError {
            message: format!(
                "File too large. Max size: {} MB",
                MAX_FILE_SIZE / 1024 / 1024
            ),
        })
    } else if size == 0 {
        Err(FileValidationError {
            message: "File is empty".to_string(),
        })
    } else {
        Ok(())
    }
}

// Validate MIME type
fn validate_mime_type(content_type: &str) -> Result<(), FileValidationError> {
    if ALLOWED_MIME_TYPES.contains(&content_type) {
        Ok(())
    } else {
        Err(FileValidationError {
            message: format!(
                "Invalid file type. Allowed: {}",
                ALLOWED_MIME_TYPES.join(", ")
            ),
        })
    }
}

// Sanitize filename (prevent path traversal)
fn sanitize_filename(filename: &str) -> String {
    filename
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '.' || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect::<String>()
        .replace("..", "_") // Prevent directory traversal
}

/// Upload file with validation
#[utoipa::path(
    post,
    path = "/api/upload",
    request_body(content = inline(UploadRequest), content_type = "multipart/form-data"),
    responses(
        (status = 200, description = "File uploaded successfully"),
        (status = 400, description = "Validation error"),
        (status = 413, description = "File too large")
    )
)]
pub async fn upload_file(
    mut multipart: Multipart,
) -> Result<Json<serde_json::Value>, AppError> {
    let mut uploaded_files = Vec::new();
    
    while let Some(field) = multipart.next_field().await? {
        let field_name = field.name().unwrap_or("").to_string();
        
        // Handle file field
        if field_name == "file" {
            let filename = field
                .file_name()
                .ok_or_else(|| AppError::BadRequest("Filename is required".into()))?
                .to_string();
            
            let content_type = field
                .content_type()
                .ok_or_else(|| AppError::BadRequest("Content-Type is required".into()))?
                .to_string();
            
            // Validate extension
            validate_file_extension(&filename)?;
            
            // Validate MIME type
            validate_mime_type(&content_type)?;
            
            // Read file data
            let data = field.bytes().await?;
            
            // Validate size
            validate_file_size(data.len())?;
            
            // Sanitize filename
            let safe_filename = sanitize_filename(&filename);
            let unique_filename = format!(
                "{}_{}", 
                uuid::Uuid::new_v4(), 
                safe_filename
            );
            
            // Save file
            let path = format!("./uploads/{}", unique_filename);
            let mut file = File::create(&path).await?;
            file.write_all(&data).await?;
            
            uploaded_files.push(serde_json::json!({
                "original_name": filename,
                "saved_name": unique_filename,
                "size": data.len(),
                "content_type": content_type,
            }));
            
            info!("File uploaded: {} ({} bytes)", unique_filename, data.len());
        }
    }
    
    if uploaded_files.is_empty() {
        return Err(AppError::BadRequest("No files uploaded".into()));
    }
    
    Ok(Json(serde_json::json!({
        "message": "Files uploaded successfully",
        "files": uploaded_files,
    })))
}
```

**Multipart form with mixed fields (file + JSON data)**:

```rust
#[derive(Debug, Deserialize, Validate)]
pub struct ProfileUpdateData {
    #[validate(length(min = 3, max = 50))]
    pub name: String,
    
    #[validate(length(max = 500))]
    pub bio: Option<String>,
}

pub async fn update_profile_with_avatar(
    State(db): State<PgPool>,
    State(s3): State<S3Client>,
    mut multipart: Multipart,
) -> Result<Json<serde_json::Value>, AppError> {
    let mut profile_data: Option<ProfileUpdateData> = None;
    let mut avatar_url: Option<String> = None;
    
    while let Some(field) = multipart.next_field().await? {
        let field_name = field.name().unwrap_or("").to_string();
        
        match field_name.as_str() {
            "data" => {
                // Parse JSON data from form field
                let text = field.text().await?;
                let data: ProfileUpdateData = serde_json::from_str(&text)
                    .map_err(|_| AppError::BadRequest("Invalid JSON data".into()))?;
                
                // Validate data
                data.validate()
                    .map_err(|e| AppError::ValidationError(e))?;
                
                profile_data = Some(data);
            }
            "avatar" => {
                let filename = field.file_name().unwrap_or("avatar.jpg").to_string();
                let content_type = field.content_type().unwrap_or("image/jpeg").to_string();
                
                // Validate image file
                validate_file_extension(&filename)?;
                validate_mime_type(&content_type)?;
                
                let data = field.bytes().await?;
                validate_file_size(data.len())?;
                
                // Validate it's actually an image (optional but recommended)
                validate_image_content(&data)?;
                
                // Upload to S3
                let key = format!("avatars/{}/{}", uuid::Uuid::new_v4(), filename);
                let url = upload_to_s3(&s3, "my-bucket", &key, data.to_vec()).await?;
                
                avatar_url = Some(url);
            }
            _ => {
                // Ignore unknown fields
            }
        }
    }
    
    let data = profile_data
        .ok_or_else(|| AppError::BadRequest("Profile data is required".into()))?;
    
    // Update database
    sqlx::query(
        "UPDATE users SET name = $1, bio = $2, avatar_url = COALESCE($3, avatar_url) 
         WHERE id = $4"
    )
    .bind(&data.name)
    .bind(&data.bio)
    .bind(&avatar_url)
    .bind(1) // user_id from auth
    .execute(&db)
    .await?;
    
    Ok(Json(serde_json::json!({
        "message": "Profile updated successfully",
        "avatar_url": avatar_url,
    })))
}

// Validate image content (detect fake extensions)
fn validate_image_content(data: &[u8]) -> Result<(), FileValidationError> {
    // Check image magic numbers (file signatures)
    if data.len() < 12 {
        return Err(FileValidationError {
            message: "File too small to be a valid image".into(),
        });
    }
    
    let is_jpeg = data.starts_with(&[0xFF, 0xD8, 0xFF]);
    let is_png = data.starts_with(&[0x89, 0x50, 0x4E, 0x47]);
    let is_gif = data.starts_with(b"GIF87a") || data.starts_with(b"GIF89a");
    let is_webp = &data[0..4] == b"RIFF" && &data[8..12] == b"WEBP";
    
    if is_jpeg || is_png || is_gif || is_webp {
        Ok(())
    } else {
        Err(FileValidationError {
            message: "File is not a valid image format".into(),
        })
    }
}
```

**Multiple files validation**:

```rust
pub async fn upload_multiple_images(
    mut multipart: Multipart,
) -> Result<Json<serde_json::Value>, AppError> {
    let mut uploaded_files = Vec::new();
    const MAX_FILES: usize = 10;
    
    while let Some(field) = multipart.next_field().await? {
        if field.name() != Some("images") {
            continue;
        }
        
        // Check max files limit
        if uploaded_files.len() >= MAX_FILES {
            return Err(AppError::BadRequest(
                format!("Maximum {} files allowed", MAX_FILES)
            ));
        }
        
        let filename = field
            .file_name()
            .ok_or_else(|| AppError::BadRequest("Filename required".into()))?
            .to_string();
        
        // Validate
        validate_file_extension(&filename)?;
        
        let data = field.bytes().await?;
        validate_file_size(data.len())?;
        validate_image_content(&data)?;
        
        // Process file...
        uploaded_files.push(filename);
    }
    
    if uploaded_files.is_empty() {
        return Err(AppError::BadRequest("At least one image required".into()));
    }
    
    Ok(Json(serde_json::json!({
        "uploaded": uploaded_files.len(),
        "files": uploaded_files,
    })))
}
```

#### Error Handling for Validation

**Custom error type**:

```rust
// src/errors.rs
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use validator::ValidationErrors;

#[derive(Debug)]
pub enum AppError {
    ValidationError(ValidationErrors),
    BadRequest(String),
    // ... other errors
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::ValidationError(errors) => {
                let error_map = errors
                    .field_errors()
                    .iter()
                    .map(|(field, errors)| {
                        let messages: Vec<String> = errors
                            .iter()
                            .filter_map(|e| e.message.as_ref().map(|m| m.to_string()))
                            .collect();
                        (field.to_string(), messages)
                    })
                    .collect::<std::collections::HashMap<_, _>>();
                
                (
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({
                        "error": "Validation failed",
                        "fields": error_map,
                    })),
                )
                    .into_response()
            }
            AppError::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": msg,
                })),
            )
                .into_response(),
            // ... handle other errors
        }
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(err: validator::ValidationErrors) -> Self {
        AppError::ValidationError(err)
    }
}
```

**Validation middleware** (optional):

```rust
// Auto-validate all JSON requests
use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};

pub async fn validate_json_middleware(
    request: Request,
    next: Next,
) -> Response {
    // This is handled by the Validate trait in handlers
    // But you can add global JSON schema validation here if needed
    next.run(request).await
}
```

---

### CORS Configuration

```rust
use tower_http::cors::{CorsLayer, Any};

let cors = CorsLayer::new()
    .allow_origin(["https://myapp.com".parse().unwrap()])
    .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
    .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
    .allow_credentials(true)
    .max_age(Duration::from_secs(3600));

let app = Router::new()
    .route("/api/users", get(list_users))
    .layer(cors);
```

---

## 🔄 Background Tasks (Optional)

Simple background task processing with **Tokio tasks**.

### Queue-based Email Worker

```rust
// src/jobs/mod.rs
use tokio::task;
use tokio::time::{sleep, Duration};

pub async fn spawn_background_workers(state: AppState) {
    // Email queue processor
    let email_state = state.clone();
    task::spawn(async move {
        info!("Email worker started");
        process_email_queue(email_state).await;
    });
    
    // Image processing worker
    let image_state = state.clone();
    task::spawn(async move {
        info!("Image processor started");
        process_image_queue(image_state).await;
    });
}

// src/jobs/email_worker.rs
use crate::models::EmailJob;

async fn process_email_queue(state: AppState) {
    loop {
        // Fetch pending emails from database
        let emails = sqlx::query_as::<_, EmailJob>(
            "SELECT * FROM email_queue 
             WHERE status = 'pending' 
             ORDER BY created_at ASC 
             LIMIT 10"
        )
        .fetch_all(&state.db)
        .await
        .unwrap_or_default();
        
        for email in emails {
            info!("Processing email job: {}", email.id);
            
            match send_email(&state.email_client, &email).await {
                Ok(_) => {
                    // Mark as sent
                    sqlx::query(
                        "UPDATE email_queue 
                         SET status = 'sent', sent_at = NOW() 
                         WHERE id = $1"
                    )
                    .bind(email.id)
                    .execute(&state.db)
                    .await
                    .ok();
                    
                    info!("Email sent successfully: {}", email.id);
                }
                Err(e) => {
                    // Mark as failed
                    sqlx::query(
                        "UPDATE email_queue 
                         SET status = 'failed', error = $1 
                         WHERE id = $2"
                    )
                    .bind(e.to_string())
                    .bind(email.id)
                    .execute(&state.db)
                    .await
                    .ok();
                    
                    error!("Email failed: {} - {}", email.id, e);
                }
            }
        }
        
        // Poll every 5 seconds
        sleep(Duration::from_secs(5)).await;
    }
}

async fn send_email(
    client: &EmailClient,
    job: &EmailJob,
) -> Result<(), AppError> {
    // Actual email sending logic
    client.send(&job.to, &job.subject, &job.body).await?;
    Ok(())
}
```

### Scheduled Tasks

```rust
// src/jobs/scheduled.rs
use tokio::time::{interval, Duration};

pub async fn spawn_scheduled_tasks(state: AppState) {
    // Cleanup old sessions every hour
    let cleanup_state = state.clone();
    task::spawn(async move {
        let mut interval = interval(Duration::from_secs(3600));
        
        loop {
            interval.tick().await;
            info!("Running session cleanup task");
            
            if let Err(e) = cleanup_old_sessions(&cleanup_state.db).await {
                error!("Cleanup task failed: {}", e);
            }
        }
    });
    
    // Generate daily reports at 2 AM (using chrono for scheduling)
    let report_state = state.clone();
    task::spawn(async move {
        loop {
            let now = chrono::Utc::now();
            let target = (now + chrono::Duration::days(1))
                .date_naive()
                .and_hms_opt(2, 0, 0)
                .unwrap()
                .and_utc();
            
            let duration_until = (target - now).to_std().unwrap();
            sleep(duration_until).await;
            
            info!("Running daily report generation");
            if let Err(e) = generate_daily_reports(&report_state.db).await {
                error!("Report generation failed: {}", e);
            }
        }
    });
}

async fn cleanup_old_sessions(db: &PgPool) -> Result<(), AppError> {
    sqlx::query("DELETE FROM sessions WHERE expires_at < NOW()")
        .execute(db)
        .await?;
    Ok(())
}

async fn generate_daily_reports(db: &PgPool) -> Result<(), AppError> {
    // Generate and send reports
    Ok(())
}
```

### Database Schema for Job Queue

```sql
-- migrations/XXXXXX_create_email_queue.up.sql
CREATE TABLE email_queue (
    id SERIAL PRIMARY KEY,
    to_email VARCHAR(255) NOT NULL,
    subject VARCHAR(500) NOT NULL,
    body TEXT NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    error TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    sent_at TIMESTAMP,
    CHECK (status IN ('pending', 'sent', 'failed'))
);

CREATE INDEX idx_email_queue_status ON email_queue(status, created_at);
```

### Usage: Enqueue Job

```rust
// In your API handler
pub async fn register_user(
    State(db): State<PgPool>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<User>, AppError> {
    // Create user...
    let user = create_user(&db, &req).await?;
    
    // Enqueue welcome email
    sqlx::query(
        "INSERT INTO email_queue (to_email, subject, body) 
         VALUES ($1, $2, $3)"
    )
    .bind(&user.email)
    .bind("Welcome to Our Platform!")
    .bind("Thank you for signing up...")
    .execute(&db)
    .await?;
    
    Ok(Json(user))
}
```

---

## 🔌 WebSocket Support (Optional)

Basic WebSocket implementation for real-time features.

**Setup**:

```rust
// src/websocket/mod.rs
use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade, Message},
        State,
    },
    response::IntoResponse,
};
use futures::{sink::SinkExt, stream::StreamExt};

// WebSocket handler
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

// Handle individual connection
async fn handle_socket(socket: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = socket.split();
    
    // Send welcome message
    if sender
        .send(Message::Text("Connected to WebSocket!".into()))
        .await
        .is_err()
    {
        return;
    }
    
    // Listen for messages
    while let Some(Ok(msg)) = receiver.next().await {
        match msg {
            Message::Text(text) => {
                info!("Received WebSocket message: {}", text);
                
                // Echo back (or process message)
                if sender
                    .send(Message::Text(format!("Echo: {}", text)))
                    .await
                    .is_err()
                {
                    break;
                }
            }
            Message::Close(_) => {
                info!("WebSocket connection closed");
                break;
            }
            _ => {}
        }
    }
}
```

**Broadcast System**:

```rust
use tokio::sync::broadcast;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    db: PgPool,
    // Broadcast channel for WebSocket messages
    ws_tx: broadcast::Sender<String>,
}

async fn handle_socket_with_broadcast(
    socket: WebSocket,
    state: AppState,
) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.ws_tx.subscribe();
    
    // Task to receive broadcasts and send to this client
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });
    
    // Task to receive messages from this client and broadcast
    let tx = state.ws_tx.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            // Broadcast to all connected clients
            let _ = tx.send(text);
        }
    });
    
    // Wait for either task to finish
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }
}
```

**Add route**:

```rust
// In main.rs
let (ws_tx, _) = broadcast::channel(100);

let state = AppState {
    db: db_pool,
    ws_tx,
};

let app = Router::new()
    .route("/ws", get(ws_handler))
    .route("/api/users", get(list_users))
    .with_state(state);
```

**Client-side example (JavaScript)**:

```javascript
const ws = new WebSocket('ws://localhost:8000/ws');

ws.onopen = () => {
    console.log('WebSocket connected');
    ws.send('Hello from client!');
};

ws.onmessage = (event) => {
    console.log('Received:', event.data);
};

ws.onclose = () => {
    console.log('WebSocket disconnected');
};
```

---

## 🚀 CI/CD

### GitHub Actions Workflow

Create `.github/workflows/ci.yml`:

```yaml
name: CI/CD

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

env:
  CARGO_TERM_COLOR: always

jobs:
  test:
    name: Test
    runs-on: ubuntu-latest
    
    services:
      postgres:
        image: postgres:17
        env:
          POSTGRES_PASSWORD: postgres
          POSTGRES_DB: test_db
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
        ports:
          - 5432:5432
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
      
      - name: Cache dependencies
        uses: actions/cache@v3
        with:
          path: |
            ~/.cargo/bin/
            ~/.cargo/registry/index/
            ~/.cargo/registry/cache/
            ~/.cargo/git/db/
            target/
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
      
      - name: Install SQLx CLI
        run: cargo install sqlx-cli --no-default-features --features postgres
      
      - name: Run migrations
        env:
          DATABASE_URL: postgresql://postgres:postgres@localhost:5432/test_db
        run: sqlx migrate run
      
      - name: Run tests
        env:
          DATABASE_URL: postgresql://postgres:postgres@localhost:5432/test_db
        run: cargo test --verbose
      
      - name: Check formatting
        run: cargo fmt -- --check
      
      - name: Run clippy
        run: cargo clippy -- -D warnings

  build:
    name: Build
    runs-on: ubuntu-latest
    needs: test
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
      
      - name: Build
        run: cargo build --release --verbose
      
      - name: Upload artifact
        uses: actions/upload-artifact@v3
        with:
          name: axum-rust-starter
          path: target/release/axum-rust-starter

  docker:
    name: Build Docker Image
    runs-on: ubuntu-latest
    needs: test
    if: github.ref == 'refs/heads/main'
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Set up Docker Buildx
        uses: docker/setup-buildx-action@v3
      
      - name: Login to Docker Hub
        uses: docker/login-action@v3
        with:
          username: ${{ secrets.DOCKER_USERNAME }}
          password: ${{ secrets.DOCKER_PASSWORD }}
      
      - name: Build and push
        uses: docker/build-push-action@v5
        with:
          context: .
          push: true
          tags: |
            ${{ secrets.DOCKER_USERNAME }}/axum-rust-starter:latest
            ${{ secrets.DOCKER_USERNAME }}/axum-rust-starter:${{ github.sha }}
          cache-from: type=gha
          cache-to: type=gha,mode=max
```

### Deployment Workflow (Example: Fly.io)

Create `.github/workflows/deploy.yml`:

```yaml
name: Deploy to Fly.io

on:
  push:
    branches: [ main ]

jobs:
  deploy:
    name: Deploy
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Flyctl
        uses: superfly/flyctl-actions/setup-flyctl@master
      
      - name: Deploy to Fly.io
        run: flyctl deploy --remote-only
        env:
          FLY_API_TOKEN: ${{ secrets.FLY_API_TOKEN }}
```

### Alternative: Railway Deployment

Create `railway.json`:

```json
{
  "$schema": "https://railway.app/railway.schema.json",
  "build": {
    "builder": "NIXPACKS"
  },
  "deploy": {
    "numReplicas": 1,
    "restartPolicyType": "ON_FAILURE",
    "restartPolicyMaxRetries": 10
  }
}
```

### Alternative: VPS Deployment Script

Create `scripts/deploy.sh`:

```bash
#!/bin/bash
set -e

SERVER_USER="deploy"
SERVER_HOST="your-server.com"
APP_NAME="axum-rust-starter"
DEPLOY_PATH="/opt/$APP_NAME"

echo "Building release binary..."
cargo build --release

echo "Uploading binary to server..."
scp target/release/$APP_NAME $SERVER_USER@$SERVER_HOST:$DEPLOY_PATH/

echo "Restarting service..."
ssh $SERVER_USER@$SERVER_HOST "sudo systemctl restart $APP_NAME"

echo "Deployment complete!"
```

### Systemd Service (for VPS)

Create `/etc/systemd/system/axum-rust-starter.service`:

```ini
[Unit]
Description=Axum Rust Starter API
After=network.target postgresql.service

[Service]
Type=simple
User=deploy
WorkingDirectory=/opt/axum-rust-starter
Environment="DATABASE_URL=postgresql://user:pass@localhost/db"
Environment="JWT_SECRET=your-secret-key"
Environment="RUST_LOG=info"
ExecStart=/opt/axum-rust-starter/axum-rust-starter
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

**Enable and start**:

```bash
sudo systemctl enable axum-rust-starter
sudo systemctl start axum-rust-starter
sudo systemctl status axum-rust-starter
```

---

## 📁 Project Structure

```
.
├── src/
│   ├── main.rs                  # Application entry point & OpenAPI setup
│   ├── config.rs                # Configuration management (env vars)
│   │
│   ├── routes/                  # API route handlers
│   │   ├── mod.rs
│   │   ├── health.rs            # Health check endpoint
│   │   ├── auth.rs              # Login, register, refresh token
│   │   ├── users.rs             # User CRUD operations
│   │   ├── upload.rs            # File upload & S3 integration
│   │   └── articles.rs          # Article management (MongoDB example)
│   │
│   ├── models/                  # Database models & schemas
│   │   ├── mod.rs
│   │   ├── user.rs              # User model (PostgreSQL)
│   │   ├── article.rs           # Article model (MongoDB)
│   │   └── session.rs           # Session model (Redis)
│   │
│   ├── middleware/              # Custom middleware
│   │   ├── mod.rs
│   │   ├── auth.rs              # JWT authentication middleware
│   │   ├── request_id.rs        # Request ID tracking
│   │   ├── cors.rs              # CORS configuration
│   │   ├── rate_limit.rs        # Rate limiting (tower-governor)
│   │   ├── security_headers.rs  # Security headers
│   │   └── logger.rs            # Request/response logging
│   │
│   ├── services/                # Business logic layer
│   │   ├── mod.rs
│   │   ├── user_service.rs      # User business logic
│   │   ├── auth_service.rs      # Authentication logic
│   │   ├── upload_service.rs    # File upload & S3 logic
│   │   ├── cache_service.rs     # Redis caching logic
│   │   └── email_service.rs     # Email sending logic
│   │
│   ├── clients/                 # External API clients
│   │   ├── mod.rs
│   │   ├── s3.rs                # AWS S3 client
│   │   ├── payment.rs           # Payment gateway (Midtrans/Xendit)
│   │   ├── whatsapp.rs          # WhatsApp API client
│   │   ├── email.rs             # Email service (Resend/SendGrid)
│   │   └── storage.rs           # Generic storage client
│   │
│   ├── database/                # Database connections
│   │   ├── mod.rs
│   │   ├── postgres.rs          # PostgreSQL connection pool
│   │   ├── mongodb.rs           # MongoDB client setup
│   │   └── redis.rs             # Redis connection manager
│   │
│   ├── validators/              # Validation functions
│   │   ├── mod.rs               # Re-export all validators
│   │   ├── common.rs            # Common validators (password, username)
│   │   ├── indonesian.rs        # Indonesian-specific (NIK, phone, postal)
│   │   └── file.rs              # File validation (extension, size, MIME)
│   │
│   ├── jobs/                    # Background tasks (OPTIONAL)
│   │   ├── mod.rs               # Job spawner & manager
│   │   ├── email_worker.rs      # Email queue processor
│   │   ├── image_worker.rs      # Image processing worker
│   │   └── scheduled.rs         # Scheduled/cron tasks
│   │
│   ├── websocket/               # WebSocket handlers (OPTIONAL)
│   │   ├── mod.rs               # WebSocket setup
│   │   ├── handler.rs           # Connection handler
│   │   └── broadcast.rs         # Broadcast system
│   │
│   ├── utils/                   # Utility functions
│   │   ├── mod.rs
│   │   ├── jwt.rs               # JWT token creation & validation
│   │   ├── password.rs          # Password hashing (bcrypt/argon2)
│   │   ├── time.rs              # Timezone & datetime utilities
│   │   └── file.rs              # File handling utilities
│   │
│   ├── dto/                     # Data Transfer Objects
│   │   ├── mod.rs
│   │   ├── auth.rs              # Login, register DTOs (+ specific validators)
│   │   ├── user.rs              # User request/response DTOs
│   │   └── upload.rs            # Upload request/response DTOs
│   │
│   └── errors.rs                # Custom error types & handlers
│
├── migrations/                  # SQLx database migrations
│   ├── 20240101000001_create_users_table.up.sql
│   ├── 20240101000001_create_users_table.down.sql
│   ├── 20240101000002_create_sessions_table.up.sql
│   ├── 20240101000002_create_sessions_table.down.sql
│   ├── 20240101000003_create_email_queue.up.sql
│   └── 20240101000003_create_email_queue.down.sql
│
├── tests/                       # Integration & unit tests
│   ├── common/                  # Shared test utilities
│   │   ├── mod.rs
│   │   └── fixtures.rs          # Test fixtures & helpers
│   ├── integration/             # Integration tests
│   │   ├── auth_tests.rs
│   │   ├── user_tests.rs
│   │   └── upload_tests.rs
│   └── unit/                    # Unit tests
│       ├── jwt_tests.rs
│       ├── validation_tests.rs  # Validator unit tests
│       └── file_tests.rs
│
├── scripts/                     # Utility scripts
│   ├── seed.rs                  # Database seeding
│   ├── migrate.sh               # Migration runner script
│   └── deploy.sh                # Deployment script
│
├── .github/                     # GitHub Actions workflows
│   └── workflows/
│       ├── ci.yml               # CI pipeline
│       └── deploy.yml           # CD pipeline
│
├── .env.example                 # Example environment file
├── .env                         # Local environment (gitignored)
├── .gitignore                   # Git ignore rules
├── Cargo.toml                   # Rust dependencies
├── Cargo.lock                   # Locked dependencies
├── Dockerfile                   # Production container
├── docker-compose.yml           # Development setup
├── fly.toml                     # Fly.io configuration (optional)
├── railway.json                 # Railway configuration (optional)
├── README.md                    # This file
└── rust-toolchain.toml          # Rust version specification
```

### 📂 Directory Explanations

#### **routes/** - API Endpoints
Handles HTTP requests and responses. Each file represents a resource (users, auth, etc.).
- Thin layer - delegates business logic to services
- Request validation and response formatting
- Error handling
- **Annotated with utoipa macros** for OpenAPI generation

#### **models/** - Data Models
Database schemas and data structures.
- PostgreSQL models with SQLx
- MongoDB models with Serde
- Shared types across the application
- **Implements utoipa::ToSchema** for API docs

#### **middleware/** - Request Processing
Functions that process requests before reaching handlers.
- Authentication checks
- Request/response logging
- Rate limiting (tower-governor)
- Security headers
- CORS handling

#### **services/** - Business Logic
Core business logic separated from HTTP layer.
- Reusable across different routes
- Database operations
- External API calls
- Complex computations

#### **clients/** - External Integrations
Wrappers for third-party services.
- AWS S3 operations
- Payment gateways
- Email/WhatsApp services
- Consistent error handling

#### **database/** - Database Connections
Connection setup and management.
- Connection pooling
- Health checks
- Configuration

#### **validators/** - Validation Functions
Centralized validation logic (hybrid approach).
- **common.rs**: Reusable validators (password strength, username, slug)
- **indonesian.rs**: Indonesian-specific (NIK, phone, postal code)
- **file.rs**: File validation (extension, size, MIME type, content)
- DTO-specific validators remain in their respective DTO files
- Easy to test independently

#### **jobs/** - Background Tasks (OPTIONAL)
Asynchronous task processing with Tokio.
- Email queue worker
- Image processing
- Scheduled cleanup tasks
- Cron-like jobs

#### **websocket/** - Real-time Communication (OPTIONAL)
WebSocket support for real-time features.
- Connection management
- Broadcast system
- Message handling

#### **utils/** - Helper Functions
Reusable utility functions.
- JWT operations
- Password hashing
- Date/time utilities
- File handling helpers

#### **dto/** - Data Transfer Objects
Request/response structures.
- Input validation (uses validators/)
- API contracts
- Serialization/deserialization
- **Implements utoipa::ToSchema**
- May contain DTO-specific validators

#### **errors.rs** - Error Handling
Centralized error types and handlers.
- Custom error types
- Error conversions
- HTTP error responses
- Validation error formatting

#### **.github/workflows/** - CI/CD
Automated workflows for testing and deployment.
- Continuous Integration
- Automated testing
- Docker builds
- Deployment automation

---

## 🛠 Development Tools

### Format code

```bash
cargo fmt
```

### Lint code

```bash
cargo clippy
```

### Check for errors without building

```bash
cargo check
```

### Generate documentation

```bash
cargo doc --open
```

---

## 🐳 Docker Support

### Development with Docker Compose

```bash
docker-compose up
```

Services included:
- **app**: Rust application
- **postgres**: PostgreSQL database
- **adminer**: Database admin UI at `http://localhost:8080`

### Production Dockerfile

Multi-stage build for optimal image size:

```bash
docker build -t axum-rust-starter .
docker run -p 8000:8000 axum-rust-starter
```

---

## 🔧 Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `HOST` | Server host | `127.0.0.1` |
| `PORT` | Server port | `8000` |
| `DATABASE_URL` | PostgreSQL connection string | Required |
| `MONGODB_URI` | MongoDB connection string | Optional |
| `MONGODB_DATABASE` | MongoDB database name | Optional |
| `REDIS_URL` | Redis connection string | Optional |
| `JWT_SECRET` | Secret for JWT signing | Required |
| `JWT_EXPIRATION` | Token expiration (seconds) | `86400` |
| `AWS_ACCESS_KEY_ID` | AWS access key for S3 | Optional |
| `AWS_SECRET_ACCESS_KEY` | AWS secret key for S3 | Optional |
| `AWS_REGION` | AWS region | `ap-southeast-1` |
| `S3_BUCKET` | S3 bucket name | Optional |
| `MIDTRANS_SERVER_KEY` | Midtrans payment API key | Optional |
| `WHATSAPP_API_KEY` | WhatsApp API key | Optional |
| `RESEND_API_KEY` | Email service API key | Optional |
| `RUST_LOG` | Log level | `info` |
| `TZ` | Application timezone | `UTC` |

---

## 🚢 Deployment

### Fly.io

```bash
fly launch
fly deploy
```

### Railway

```bash
railway up
```

### VPS/Dedicated Server

1. Build release binary
2. Copy binary to server
3. Setup systemd service
4. Configure reverse proxy (nginx/caddy)

---

## 📊 Performance

Rust + Axum provides exceptional performance:

- **Low latency**: Sub-millisecond response times
- **High throughput**: Handle 100k+ requests/second
- **Memory efficiency**: Minimal overhead
- **Compile-time guarantees**: No runtime errors

---

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

---

## 📚 Additional Resources

- [Axum Documentation](https://docs.rs/axum)
- [SQLx Documentation](https://docs.rs/sqlx)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Tracing Guide](https://docs.rs/tracing)

---

## 🔐 Security

- Never commit `.env` file
- Rotate JWT secrets regularly
- Use environment variables for sensitive data
- Enable HTTPS in production
- Keep dependencies updated: `cargo update`

---

## 📝 License

This project is open source and available under the MIT License.
