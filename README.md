# Docker Polyglot Monorepo

A polyglot monorepo demonstrating Docker Compose Watch with hot reloading across multiple languages.

## Services

- **worker-rust** (Port 8080): Rust worker using Warp and cargo-watch
- **web-node** (Port 5173): Vite development server with HMR
- **worker-py**: Python worker with uv and watchdog auto-restart
- **worker-ts**: TypeScript worker with Bun watch mode
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
- **Vite/Bun**: Changes to `src/` and `index.html` sync with HMR
- **Python/uv**: Changes to `main.py` trigger auto-restart via watchdog
- **TypeScript/Bun**: Changes to `src/` trigger rebuild via Bun watch

## Accessing Services

- Vite Web: http://localhost:5173
- Rust Worker: http://localhost:8080
- Redis: localhost:6379

## Testing Connectivity

```bash
# Check Redis connection from Python worker
docker compose logs worker-py

# Check Redis connection from TypeScript worker
docker compose logs worker-ts

# Test Rust worker
curl http://localhost:8080
curl http://localhost:8080/health
```

## Structure

```
/apps
  /worker-rust   - Rust worker with multi-stage Dockerfile
  /web-node      - Vite app with Bun and HMR
  /worker-py     - Python worker with uv and watchdog
  /worker-ts     - TypeScript worker with Bun
docker-compose.yml
```
