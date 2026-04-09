# APIPup

一个高性能、跨平台的 API 测试工具，具有原生桌面性能。

使用 **Tauri** + **Rust** + **SvelteKit** 构建，提供快速、轻量和响应式的体验。

支持 **macOS** 和 **Windows**。

## 设计理念

- **无服务器、无云存储** — 所有请求直接从 Rust 层发送，避免安全问题
- **本地数据存储** — 数据保存在您的磁盘上。使用 Git 进行版本控制和团队协作
- **AI 友好和 CLI 友好** — 专为自动化和与 AI 工具集成而设计

## 性能

| 指标 | APIPup | 典型 Electron 应用 |
|--------|--------|-------------------|
| 包大小 | ~10MB | ~450MB |
| 启动速度 | 超快 | 较慢 |
| 1000 请求时间 | 快（Rust） | 较慢 |
| 峰值内存（100 并发，1000 总计） | 20–60MB | ~100–200MB |

## 功能

- 发送 HTTP 请求（GET、POST、PUT、DELETE 等）
- 查看响应头和响应体
- 内置英语、中文、日语、法语和德语支持
- 快速轻量

## 开发者指南

### 架构

```
ui/ (SvelteKit)  ──invoke()──>  tauri/ (Rust)
                              └── HTTP 请求
                              └── 返回响应
```

- **前端** (`ui/`): SvelteKit + Tailwind CSS
- **后端** (`tauri/`): Tauri + Rust HTTP 引擎
- **通信**: Tauri `invoke()` — 无 CORS 问题，无额外端口

### 前置要求

- [Node.js](https://nodejs.org/) >= 18
- [pnpm](https://pnpm.io/) >= 8
- [Rust](https://rustup.rs/) >= 1.77
- 平台特定依赖:
  - **macOS**: Xcode Command Line Tools (`xcode-select --install`)
  - **Windows**: [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)，WebView2（Windows 10/11 预装）

### 开始开发

```bash
# 克隆仓库
git clone <repo-url>
cd APIPup

# 安装依赖
pnpm install

# 启动开发环境（启动前端开发服务器和 Tauri 窗口）
pnpm dev
```

`pnpm dev` 将会：
1. 在 `http://127.0.0.1:1420` 启动 SvelteKit Vite 开发服务器
2. 编译 Rust 后端
3. 打开 APIPup 桌面窗口

### 生产构建

```bash
# 构建 .app
pnpm build

# 构建 .dmg（需要: brew install create-dmg）
pnpm build:dmg
```

输出文件位于 `tauri/target/release/bundle/macos/`。

## 许可证

MIT
