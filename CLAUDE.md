# CLAUDE.md

このファイルは Claude Code がこのリポジトリで作業する際のガイダンスを提供する。

## 技術スタック

- **Tauri v2** + Rust (バックエンド)
- **SvelteKit v2** + Svelte 5 (フロントエンド)
- **Tailwind CSS v4** (@tailwindcss/vite プラグイン使用)
- **TypeScript**

### フレームワーク参照

Svelte 5、SvelteKit v2、Tailwind CSS v4、Tauri v2 は比較的新しいフレームワーク。
開発時は **context7 MCP** を使用して最新のドキュメントを参照すること。

## 開発コマンド

```bash
# 開発サーバー起動
pnpm tauri dev

# プロダクションビルド
pnpm tauri build

# 型チェック
pnpm check
```

## アーキテクチャ

### フロントエンド (`src/`)
- `lib/api.ts`: Tauri invoke ラッパー (snake_case → camelCase 変換)
- `lib/stores.ts`: Svelte writable stores
- `lib/components/`: UI コンポーネント

### バックエンド (`src-tauri/src/`)
- `claude_config.rs`: `~/.claude.json` の読み書き
- `commands.rs`: Tauri コマンド (get_mcp_servers, set_server_enabled, etc.)
- `lib.rs`: プラグイン登録・コマンドハンドラ登録

## JSON 構造 (配列移動方式)

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

## アイコン作成手順

1. ytyng-mcp-media の `get_icon_creator_url` でアイコン URL を生成
2. curl でダウンロード: `curl -s "<url>" -o /tmp/icon.png`
3. Tauri アイコン生成: `pnpm tauri icon /tmp/icon.png`

## コーディング規約

### Rust
- エラーは `Result<T, String>` で返す
- `unwrap()` は避け、`expect("理由")` か `?` を使用
- コメントは日本語可

### Svelte/TypeScript
- Svelte 5 の runes (`$state`, `$derived`, `$props`) を使用
- stores は `$` プレフィックスでアクセス
- コンポーネント props は `$props()` で定義
