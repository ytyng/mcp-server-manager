# MCP Server Manager

`~/.claude.json` を編集して MCP サーバーの ON/OFF を管理する Tauri デスクトップアプリ

## 機能

### Global タブ
- `mcpServers` と `disabledMcpServers` 間でサーバー設定を移動
- チェック ON: `disabledMcpServers` → `mcpServers`
- チェック OFF: `mcpServers` → `disabledMcpServers`

### Projects タブ
- プロジェクト一覧をフィルタリング表示
- プロジェクト別の `disabledMcpServers` 配列を編集

## 技術スタック

### フロントエンド
- SvelteKit v2
- Svelte 5
- TypeScript
- Tailwind CSS v4
- svelte-sonner (トースト通知)

### バックエンド
- Tauri v2
- Rust
- serde_json (JSON操作)
- dirs (ホームディレクトリ取得)

## 開発

```bash
# 依存パッケージのインストール
pnpm install

# 開発サーバー起動
pnpm tauri dev
```

## ビルド

```bash
# プロダクションビルド
pnpm tauri build
```

成果物:
- `src-tauri/target/release/bundle/macos/mcp-server-manager.app`
- `src-tauri/target/release/bundle/dmg/mcp-server-manager_x.x.x_aarch64.dmg`

## CLI 引数

```bash
# プロジェクトタブをデフォルト表示 + フィルター適用
mcp-server-manager --project=<filter>
mcp-server-manager -p <filter>
```

## プロジェクト構造

```
src/
├── app.css                     # Tailwind CSS エントリ
├── lib/
│   ├── api.ts                  # Tauri コマンドラッパー
│   ├── stores.ts               # Svelte stores
│   ├── types.ts                # TypeScript 型定義
│   └── components/
│       ├── GlobalSettings.svelte
│       ├── ProjectSettings.svelte
│       ├── ProjectCard.svelte
│       ├── ServerCheckbox.svelte
│       └── TabContainer.svelte
└── routes/
    ├── +layout.svelte
    └── +page.svelte

src-tauri/
├── src/
│   ├── lib.rs                  # Tauri エントリ
│   ├── claude_config.rs        # JSON 読み書きユーティリティ
│   └── commands.rs             # Tauri コマンド
├── icons/                      # アプリアイコン
├── Cargo.toml
└── tauri.conf.json
```
