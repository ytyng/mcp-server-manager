# CLAUDE.md

This file provides guidance to Claude Code when working in this repository.

## Tech Stack

- **Tauri v2** + Rust (backend)
- **SvelteKit v2** + Svelte 5 (frontend)
- **Tailwind CSS v4** (using the @tailwindcss/vite plugin)
- **TypeScript**

### Framework references

Svelte 5, SvelteKit v2, Tailwind CSS v4, and Tauri v2 are relatively new frameworks.
During development, use the **context7 MCP** to consult the latest documentation.

## Development Commands

```bash
# Start the development server
pnpm tauri dev

# Production build
pnpm tauri build

# Type check
pnpm check
```

## Architecture

### Frontend (`src/`)
- `lib/api.ts`: Tauri invoke wrapper (snake_case → camelCase conversion)
- `lib/stores.ts`: Svelte writable stores
- `lib/components/`: UI components

### Backend (`src-tauri/src/`)
- `claude_config.rs`: Reads and writes `~/.claude.json`
- `commands.rs`: Tauri commands (get_mcp_servers, set_server_enabled, etc.)
- `lib.rs`: Plugin registration and command handler registration

## JSON Structure (entries are moved between lists)

```json
{
  "mcpServers": {
    "enabled-server": { ... }
  },
  "disabledMcpServers": {
    "disabled-server": { ... }
  },
  "projects": {
    "/path/to/project": {
      "disabledMcpServers": ["server-name"]
    }
  }
}
```

## Creating the Icon

1. Generate an icon URL with `get_icon_creator_url` in ytyng-mcp-media
2. Download it with curl: `curl -s "<url>" -o /tmp/icon.png`
3. Generate the Tauri icons: `pnpm tauri icon /tmp/icon.png`

## Coding Conventions

### Rust
- Return errors as `Result<T, String>`
- Avoid `unwrap()`; use `expect("reason")` or `?`
- Comments may be written in Japanese

### Svelte/TypeScript
- Use Svelte 5 runes (`$state`, `$derived`, `$props`)
- Access stores with the `$` prefix
- Define component props with `$props()`
