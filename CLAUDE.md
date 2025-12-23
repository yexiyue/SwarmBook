# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

SwarmBook is a Rust libp2p tutorial with a companion Tauri desktop app. The project teaches P2P networking concepts through documentation and interactive tools, culminating in building a P2P yjs collaborative editing backend.

## Project Structure

```
swarmbook/
├── docs/           # Astro + Starlight documentation site (Chinese + English)
├── client/         # Tauri 2.0 desktop app (React + Rust)
├── examples/       # Standalone Rust examples for each chapter
├── plans/          # Design documents
└── Cargo.toml      # Rust workspace root
```

## Common Commands

### Documentation Site (docs/)
```bash
cd docs && pnpm dev      # Start dev server
cd docs && pnpm build    # Build for production
```

### Tauri Client (client/)
```bash
cd client && pnpm tauri dev    # Start Tauri dev mode
cd client && pnpm build        # Build frontend only
```

### Rust Workspace
```bash
cargo build                    # Build all workspace members
cargo run -p peerid            # Run specific example
cargo check                    # Type check without building
```

### Route Generation (TanStack Router)
```bash
cd client && npx @tanstack/router-cli generate
```

## Architecture

### Tauri App (client/)

**Backend (src-tauri/src/):**
- `lib.rs` - Entry point, registers Tauri commands
- `commands/mod.rs` - Command module exports
- `commands/*.rs` - Individual command implementations (e.g., `peer_id.rs`)

**Frontend (src/):**
- `routes/` - TanStack Router file-based routes
- `routes/__root.tsx` - Root layout with sidebar
- `commands/` - TypeScript wrappers for Tauri invoke calls
- `components/layout/` - Shared layout components (sidebar)
- `components/ui/` - shadcn/ui components

**Adding a new Tauri command:**
1. Create `src-tauri/src/commands/new_command.rs`
2. Export in `src-tauri/src/commands/mod.rs`
3. Register in `lib.rs` invoke_handler
4. Create TypeScript wrapper in `src/commands/`

### Documentation (docs/)

- Uses Astro Starlight with mermaid diagrams
- Content in `src/content/docs/` organized by chapter (01-core-concepts through 07-yjs-collab)
- Sidebar configured in `astro.config.mjs` using autogenerate
- Supports i18n (zh-CN root, en translations)

## Key Dependencies

- **libp2p 0.56** (workspace) / **0.54** (Tauri client) - P2P networking
- **Tauri 2.0** - Desktop app framework
- **TanStack Router** - File-based routing for React
- **shadcn/ui** - UI components (uses Radix primitives)
- **Astro Starlight** - Documentation framework

## Conventions

- Tutorial chapters follow the curriculum in `plans/2025-12-21-curriculum-design.md`
- Chinese is the primary language; English translations go in `docs/src/content/docs/en/`
- Use `:::tip`, `:::caution` Starlight admonitions in docs
- Tauri commands use snake_case in Rust, camelCase in TypeScript
