# APIPup

ネイティブデスクトップのパフォーマンスを持つ高性能なクロスプラットフォームAPIテストツール。

**Tauri** + **Rust** + **SvelteKit** で構築されており、高速、軽量、そして応答性の高いエクスペリエンスを提供します。

**macOS** と **Windows** をサポートしています。

## デザイン哲学

- **サーバーなし、クラウドストレージなし** — すべてのリクエストはRustレイヤーから直接送信され、セキュリティ上の問題を回避します
- **ローカルデータストレージ** — データはディスク上に保存されます。バージョン管理とチームコラボレーションにはGitを使用してください
- **AIフレンドリー＆CLIフレンドリー** — 自動化とAIツールとの統合のために設計されています

## パフォーマンス

| 指標 | APIPup | 一般的なElectronアプリ |
|--------|--------|----------------------|
| バンドルサイズ | ~10MB | ~450MB |
| 起動速度 | 超高速 | 遅い |
| 1000リクエスト時間 | 高速（Rust） | 遅い |
| ピークメモリ（100同時接続、1000合計） | 20–60MB | ~100–200MB |

## 機能

- HTTPリクエストの送信（GET、POST、PUT、DELETEなど）
- レスポンスヘッダーと本文の表示
- 英語、中国語、日本語、フランス語、ドイツ語の内蔵サポート
- 高速で軽量

## 開発者向け

### アーキテクチャ

```
ui/ (SvelteKit)  ──invoke()──>  tauri/ (Rust)
                              └── HTTPリクエスト
                              └── レスポンスを返す
```

- **フロントエンド** (`ui/`): Tailwind CSSを使用したSvelteKit
- **バックエンド** (`tauri/`): Tauri + Rust HTTPエンジン
- **通信**: Tauri `invoke()` — CORS問題なし、余分なポートなし

### 前提条件

- [Node.js](https://nodejs.org/) >= 18
- [pnpm](https://pnpm.io/) >= 8
- [Rust](https://rustup.rs/) >= 1.77
- プラットフォーム固有の依存関係:
  - **macOS**: Xcode Command Line Tools (`xcode-select --install`)
  - **Windows**: [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)、WebView2（Windows 10/11にプリインストール）

### 開始方法

```bash
# リポジトリをクローン
git clone <repo-url>
cd APIPup

# 依存関係をインストール
pnpm install

# 開発を開始（フロントエンド開発サーバーとTauriウィンドウを起動）
pnpm dev
```

`pnpm dev` は以下を実行します:
1. SvelteKit Vite開発サーバーを `http://127.0.0.1:1420` で起動
2. Rustバックエンドをコンパイル
3. APIPupデスクトップウィンドウを開く

### 本番ビルド

```bash
# .appをビルド
pnpm build

# .dmgをビルド（必要: brew install create-dmg）
pnpm build:dmg
```

出力は `tauri/target/release/bundle/macos/` にあります。

## ライセンス

MIT
