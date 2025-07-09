# Deployment Guide - Energy Trading Ledger API

## Prerequisites

- Rust 1.70 or higher
- Cargo
- Git

## Installation

1. **Clone the repository:**
   ```bash
   git clone <repository-url>
   cd ledger
   ```

2. **Build the project:**
   ```bash
   cargo build --release
   ```

3. **Run tests:**
   ```bash
   cargo test
   ```

## Running the API Server

### Development Mode

```bash
cargo run --bin api-server
```

### Production Mode

```bash
cargo build --release
./target/release/api-server
```

The server will start on `http://localhost:3000` by default.

## Environment Configuration

### Port Configuration

To change the default port, modify the `start_server` call in `src/api_server.rs`:

```rust
start_server(8080).await; // Changes port to 8080
```

### CORS Configuration

The API is configured to accept requests from any origin for development. For production, modify the CORS settings in `src/api/middleware.rs`:

```rust
pub fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin("https://yourdomain.com".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([CONTENT_TYPE])
}
```

## Health Checks

The API provides a health endpoint at `/health` that returns:

```json
{
  "success": true,
  "data": "Energy Trading Ledger API is running",
  "message": "Success",
  "timestamp": "2025-07-09T17:04:29.104603Z"
}
```

## Monitoring

### Log Files

The API logs all requests with their duration:
```
📡 GET /api/blockchain/info - Started
📡 GET /api/blockchain/info - Completed in 2.345ms
```

### Process Monitoring

Use a process manager like `systemd` or `pm2` for production deployment:

#### Systemd Service (Linux)

Create `/etc/systemd/system/energy-ledger-api.service`:

```ini
[Unit]
Description=Energy Trading Ledger API
After=network.target

[Service]
Type=simple
User=ledger
WorkingDirectory=/home/ledger/energy-ledger
ExecStart=/home/ledger/energy-ledger/target/release/api-server
Restart=always
RestartSec=10
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
```

Start the service:
```bash
sudo systemctl daemon-reload
sudo systemctl enable energy-ledger-api
sudo systemctl start energy-ledger-api
```

## Load Balancing

For high availability, place the API behind a load balancer like Nginx:

```nginx
upstream energy_api {
    server 127.0.0.1:3000;
    server 127.0.0.1:3001;
    server 127.0.0.1:3002;
}

server {
    listen 80;
    server_name api.energy-ledger.com;

    location / {
        proxy_pass http://energy_api;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

## Security Considerations

### Authentication

The current implementation doesn't include authentication. For production, implement:

1. **API Key Authentication:**
   ```rust
   // Add to middleware.rs
   pub async fn auth_middleware(
       headers: HeaderMap,
       request: Request,
       next: Next,
   ) -> Result<Response, StatusCode> {
       let api_key = headers.get("X-API-Key")
           .ok_or(StatusCode::UNAUTHORIZED)?;
       
       if !is_valid_api_key(api_key) {
           return Err(StatusCode::UNAUTHORIZED);
       }
       
       Ok(next.run(request).await)
   }
   ```

2. **JWT Authentication:**
   ```rust
   // Add JWT validation middleware
   pub async fn jwt_auth_middleware(
       headers: HeaderMap,
       request: Request,
       next: Next,
   ) -> Result<Response, StatusCode> {
       let auth_header = headers.get("Authorization")
           .ok_or(StatusCode::UNAUTHORIZED)?;
       
       let token = extract_jwt_token(auth_header)?;
       let claims = validate_jwt_token(&token)?;
       
       Ok(next.run(request).await)
   }
   ```

### Rate Limiting

Add rate limiting to prevent abuse:

```rust
use tower_governor::{governor::middleware::NoOpMiddleware, GovernorLayer};

let governor_conf = Arc::new(
    GovernorConfigBuilder::default()
        .per_second(10) // 10 requests per second
        .burst_size(20) // Allow bursts of 20 requests
        .finish()
        .unwrap()
);

let app = Router::new()
    .route("/api/*", /* routes */)
    .layer(GovernorLayer {
        config: governor_conf,
    });
```

### HTTPS

Always use HTTPS in production:

```rust
use axum_server::tls_rustls::RustlsConfig;

let config = RustlsConfig::from_pem_file("cert.pem", "key.pem").await?;
axum_server::bind_rustls("0.0.0.0:443".parse()?, config)
    .serve(app.into_make_service())
    .await?;
```

## Backup and Recovery

### Database Backup

Since the current implementation uses in-memory storage, data is lost on restart. For production:

1. **Add persistence layer:**
   ```rust
   use sled::Db;
   
   pub struct PersistentLedgerState {
       db: Db,
       // ... other fields
   }
   ```

2. **Regular backups:**
   ```bash
   # Create backup script
   #!/bin/bash
   cp -r /path/to/data /path/to/backup/$(date +%Y%m%d_%H%M%S)
   ```

## Performance Optimization

### Caching

Add Redis for caching frequently accessed data:

```rust
use redis::Client;

pub struct CachedLedgerState {
    redis_client: Client,
    // ... other fields
}
```

### Database Optimization

For production, consider using PostgreSQL or MongoDB:

```rust
use sqlx::PgPool;

pub struct DatabaseLedgerState {
    pool: PgPool,
    // ... other fields
}
```

## Docker Deployment

Create a `Dockerfile`:

```dockerfile
FROM rust:1.70 as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/api-server /usr/local/bin/api-server

EXPOSE 3000
CMD ["api-server"]
```

Build and run:
```bash
docker build -t energy-ledger-api .
docker run -p 3000:3000 energy-ledger-api
```

## Kubernetes Deployment

Create deployment manifests:

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: energy-ledger-api
spec:
  replicas: 3
  selector:
    matchLabels:
      app: energy-ledger-api
  template:
    metadata:
      labels:
        app: energy-ledger-api
    spec:
      containers:
      - name: api
        image: energy-ledger-api:latest
        ports:
        - containerPort: 3000
        env:
        - name: RUST_LOG
          value: "info"
        livenessProbe:
          httpGet:
            path: /health
            port: 3000
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /health
            port: 3000
          initialDelaySeconds: 5
          periodSeconds: 5
```

## Troubleshooting

### Common Issues

1. **Port already in use:**
   ```bash
   lsof -i :3000
   kill -9 <PID>
   ```

2. **Build failures:**
   ```bash
   cargo clean
   cargo build --release
   ```

3. **High memory usage:**
   - Monitor with `htop` or `ps aux`
   - Consider implementing proper data cleanup

### Debug Mode

Enable debug logging:
```bash
RUST_LOG=debug cargo run --bin api-server
```

### API Testing

Use the included test tools:
```bash
# Run all tests
./test_api.sh

# Test specific endpoint
curl -X GET http://localhost:3000/api/blockchain/info

# Test with data
curl -X POST http://localhost:3000/api/energy/prosumers \
  -H "Content-Type: application/json" \
  -d '{"address": "test", "name": "Test Prosumer"}'
```

## Support

For issues and questions:
- Check the API documentation: `API_DOCUMENTATION.md`
- Review the test examples: `examples/api_client_demo.rs`
- Use the web demo: `api_demo.html`
