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

## redis-work-queue

This monorepo was made to test and demonstrate the functionality of the [redis-work-queue](https://github.com/MeVitae/redis-work-queue) library, which provides a simple interface for creating and managing background workers using Redis as a message broker.

## Verdict of redis-work-queue

the functionality is basic but works as intended. it does not natively support getting the result of a job, but this can be implemented manually by storing results in Redis. and it lacks advanced features like those found in more mature libraries.
