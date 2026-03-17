# Lanuminous

Lanuminous is an intent-driven home-lab and small-site network orchestration platform.

Stage 1 includes:

- a Rust backend workspace with a canonical domain model
- a folder-based configuration loader and normalizer
- validation and renderer scaffolding shared by CLI and API
- a minimal axum API placeholder
- a React + TypeScript + Vite frontend skeleton
- a controller container build that serves the UI and API together
- a target-host agent scaffold for future apply and rollback execution
- example site configuration and backend tests

## Workspace

- `backend/`: Rust application crate
- `frontend/`: React UI
- `examples/site/`: example folder-based site configuration

## Backend commands

```bash
cargo test -p lanuminous
cargo run -p lanuminous -- validate --config ../examples/site
cargo run -p lanuminous -- render --config ../examples/site
cargo run -p lanuminous -- serve --config examples/site --listen 127.0.0.1:9097
cargo run -p lanuminous -- agent info --state-dir /var/lib/lanuminous
```

## Frontend commands

```bash
cd frontend
npm install
npm run dev
```

## Docker controller

The controller container serves the Rust API and the built React frontend from one image.

```bash
docker compose up --build
```

That exposes the control plane on `http://127.0.0.1:9097` and mounts:

- `./examples/site` into `/config`
- a named volume into `/var/lib/lanuminous`

## Host agent model

Lanuminous should not apply live network configuration from inside the controller container.
The intended execution model is:

1. The controller loads, validates, renders, and stages artifacts.
2. A Lanuminous agent runs on the target host.
3. The agent inspects staged artifacts, manages backups, applies changed files, and reloads services locally.

Current scaffold:

- `lanuminous agent info` reports host identity and supported managed services.
- `lanuminous agent inspect-stage --stage-dir <path>` inventories a staged bundle on a target host.

Apply, backup, diff, and rollback execution still need to be wired through this agent path.

## Stage 1 status

Stage 1 focuses on the canonical model, config loading, normalization, validation, and renderer structure.
Container packaging and a host-agent scaffold are now present, but deployment execution, rollback execution, richer API endpoints, and interactive frontend management remain for the next stage.
