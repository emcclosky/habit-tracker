# habit-tracker

A full-stack habit tracking application. Track daily habits and streaks via a REST API, with a web frontend coming soon.

## Structure

```
apps/
  api/     # Rust/Axum REST API
```

## Prerequisites

- Rust (latest stable, edition 2024)
- Node.js + pnpm

Install Node dependencies:

```sh
pnpm install
```

## Development

Run all services:

```sh
pnpm dev
```

By default the API listens on `http://127.0.0.1:8080`.

See [`apps/api/README.md`](apps/api/README.md) for API-specific commands and documentation.

## Notes

- Logging is controlled via `RUST_LOG`
- Habits are persisted to a local `habits.json` file during this phase — Postgres is planned

## Project status

In active development. Postgres storage and a Next.js frontend are planned.
