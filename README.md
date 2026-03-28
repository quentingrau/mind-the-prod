# 🔴 mind-the-prod

**A lightweight desktop watchdog that monitors your config files and flashes a red border around your screen when dangerous production settings are detected.**

Built with Tauri v2 + Rust. Runs silently in the background, uses virtually no memory, and gets out of your way — until it doesn't.

## Why

You open your terminal. You run a migration. Everything looks fine.
Then you realize your `.env` had `POSTGRES_HOST` pointing to prod.

This app prevents that.

## How it works

mind-the-prod watches one or more config files for patterns you define as dangerous (e.g. a production database host that is uncommented). The moment it detects one, a red border flashes around your entire screen — impossible to miss.

When the danger is gone (you comment the line, switch the value), the border disappears.

## Stack

- [Tauri v2](https://tauri.app) — lightweight cross-platform desktop runtime
- Rust — file watching with `notify`, zero-cost background daemon
- TypeScript — frontend overlay

## Getting started

### Prerequisites

- [Rust](https://rustup.rs)
- Node.js + pnpm

### Install & run

```bash
git clone https://github.com/quentingrau/mind-the-prod
cd mind-the-prod
pnpm install
pnpm tauri dev
```

### Configure

Edit `config.json` at the root of the app:

```json
{
  "rules": [
    {
      "file": "/path/to/your/.env",
      "patterns": ["your-prod-db-host.example.com"]
    }
  ]
}
```

A line is considered dangerous if it contains the pattern and is **not** commented out (i.e. does not start with `#`).

## Roadmap

- [x] File watching with instant detection
- [x] Red border overlay (transparent, click-through)
- [x] Startup check (detects danger before any file change)
- [x] `config.json` support (multiple files & patterns)
- [ ] System tray icon with status indicator
- [ ] In-app config UI
- [ ] Auto-start on login
- [ ] Windows support

## License

MIT
