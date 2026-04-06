## Prerequisites

- Rust (latest stable, edition 2024)
- Node.js + pnpm (for Nx — install from the monorepo root with `pnpm install`)

## Commands

Run from the monorepo root:

```sh
pnpm nx run api:serve    # start the development server
pnpm nx run api:build    # compile a release build
pnpm nx run api:test     # run the test suite
pnpm nx run api:clippy   # lint with Clippy
```

You can also run Cargo commands directly from `apps/api/`:

```sh
cargo run
cargo test
```

Server URL: `http://127.0.0.1:8080`

## API endpoints

### List habits

```sh
curl http://127.0.0.1:8080/habits
```

### Add habit

```sh
curl -X POST http://127.0.0.1:8080/habits \
  -H "Content-Type: application/json" \
  -d '{"name": "exercise"}'
```

### Complete habit

```sh
curl -X POST http://127.0.0.1:8080/habits/exercise/completions
```

Habit names with spaces or special characters must be URL-encoded:

```sh
curl -X POST http://127.0.0.1:8080/habits/walk%20the%20dog/completions
```

## Data storage

Habits are persisted to `habits.json` in `apps/api/`. File-based storage is intentional for this phase — Postgres is planned.

## Tests

Integration tests use isolated temporary files and run against the full Axum router without a live server.
