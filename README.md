# Habit Tracker API

A small REST API for tracking daily habits and streaks, built with Rust and Axum.

## Prerequisites

- Rust (latest stable)
- Cargo

## Run locally

```sh
cargo run
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

## Data storage

Habits are persisted to `habits.json` in the project root.

This is temporary storage for the learning/WIP phase.

## Tests

```sh
cargo test
```

## Project status

This is a work in progress / learning project, so API shape and behavior may change.

## Current limitations

- File-based JSON storage is simple but not ideal for scaling, multi-process access, or richer querying.
- If `habits.json` is manually edited into invalid JSON, the API cannot load habits.
- The completion endpoint uses habit names in the URL path (`/habits/{name}/completions`).
- Names with spaces or special characters must be URL-encoded (for example, `walk%20the%20dog`).