# CEOMIS SERVER

## Tech Stack (assumed – feel free to update)
- Rust (latest stable)
- Web framework: Axum
- Database: PostgreSQL 
- Migrations: SeaORM Cli
- Containerization: Docker (recommended for infra dependencies)

### 1. Clone the repository
```bash
git clone https://github.com/zionsploit/ceomis-v2.git
cd ceomis-v2
```

### 3. Create `.env` (or `.env.local`) in the project root and fill it like this:
```bash
```env
# ────────────────────────────────────────────────
# Garage (S3-compatible storage) – main object storage
# ────────────────────────────────────────────────
# These come from: garage key create <key-name>
garage_key_id=GK6f35b404f4fba99d800b3696
garage_key_name=projects_key
garage_provider_name=                  # usually empty or "garage" – depends on your SDK/wrapper
garage_key_secret=b354f0020f2663b5bc2c0599a8507e0c0bbd62afa9768449c349140781aad20e
garage_api_url="http://[::1]:3900"     # Garage S3 API endpoint (IPv6 localhost)

# ────────────────────────────────────────────────
# Database (PostgreSQL)
# ────────────────────────────────────────────────
DATABASE_URL="postgres://perzues_postgres:perzues_postgres@[::1]:5432/ceomis"

# ────────────────────────────────────────────────
# Redis (caching / sessions / queues)
# ────────────────────────────────────────────────
REDIS_URL="redis://:redis@[::1]:6379"

# ────────────────────────────────────────────────
# MinIO (fallback / compatibility layer – often used in dev)
# ────────────────────────────────────────────────
minio_base_url="localhost:9000"           # or 127.0.0.1:9000
minio_user="ceomis"
minio_access_key="6ZrOHogtuzR9ZljiWp9rHDrz2xEyki5J8RzxXZ0ziO2Yskl2DkcSCEqfHdgSMhJ6"
minio_secret_key="wKz2RYFAWuRT3yXUsi6Yw8uOVLHSKyDg"
```
### 3. Run database migrations
```bash
# Go to the migration folder (usually contains the migration binary or embedded setup)
cd migrations

# Run the migration binary
# This applies all pending migrations to your database
cargo run
```
> If the project includes a dedicated migration README, read it for more detailed instructions:
```bash
cat migrations/README.md
# or open it in your editor
```

### 4. Run the application
Return to the project root and start the server:
Bash
```bash
cd ..                # back to project root
cargo run
```