# Lantricate

Lantricate is an intent-driven home-lab and small-site network orchestration platform.

Stage 1 includes:

- a Rust backend workspace with a canonical domain model
- a folder-based configuration loader and normalizer
- validation and renderer scaffolding shared by CLI and API
- a minimal axum API placeholder
- a React + TypeScript + Vite frontend skeleton
- example site configuration and backend tests

## Workspace

- `backend/`: Rust application crate
- `frontend/`: React UI
- `examples/site/`: example folder-based site configuration

## Backend commands

```bash
cargo test -p lantricate
cargo run -p lantricate -- validate --config ../examples/site
cargo run -p lantricate -- render --config ../examples/site
```

## Frontend commands

```bash
cd frontend
npm install
npm run dev
```

## Stage 1 status

Stage 1 focuses on the canonical model, config loading, normalization, validation, and renderer structure.
Deployment, rollback execution, richer API endpoints, and interactive frontend management remain for Stage 2.

