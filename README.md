# Docker Polyglot Monorepo

A polyglot monorepo demonstrating Docker Compose Watch with hot reloading across multiple languages.

## Services

- **api-rust** (Port 8080): Rust API using Warp and cargo-watch
- **web-node** (Port 5173): Vite development server with HMR
- **worker-py**: Python worker with watchdog auto-restart
- **worker-ts**: TypeScript worker with tsx watch mode
- **redis** (Port 6379): Redis for inter-service communication

## Quick Start

```bash
# Start all services with hot reloading
docker compose watch

# Or start services normally
docker compose up --build
```

## Development

All services use Docker Compose Watch for instant code synchronization:

- **Rust**: Changes to `src/` sync instantly, cargo-watch rebuilds
- **Node/Vite**: Changes to `src/` and `index.html` sync with HMR
- **Python**: Changes to `main.py` trigger auto-restart via watchdog
- **TypeScript**: Changes to `src/` trigger rebuild via tsx watch

## Accessing Services

- Vite Web: http://localhost:5173
- Rust API: http://localhost:8080
- Redis: localhost:6379

## Testing Connectivity

```bash
# Check Redis connection from Python worker
docker compose logs worker-py

# Check Redis connection from TypeScript worker
docker compose logs worker-ts

# Test Rust API
curl http://localhost:8080
curl http://localhost:8080/health
```

## Structure

```
/apps
  /api-rust      - Rust API with multi-stage Dockerfile
  /web-node      - Vite app with hot module replacement
  /worker-py     - Python worker with watchdog
  /worker-ts     - TypeScript worker with tsx
docker-compose.yml
```
