# APIPup 框架搭建设计

## 概述

APIPup 是一个类似 Postman/Apifox 的 API 测试工具，采用 Tauri 2 + Rust 核心 + SvelteKit (SPA) 前端架构，支持 macOS 和 Windows。

本次目标：搭建基础框架并实现一个完整的 HTTP 请求功能。

## 技术选型

| 层级 | 技术 | 说明 |
|------|------|------|
| 桌面壳 | Tauri 2 | 跨平台桌面容器 |
| 后端核心 | Rust + reqwest | HTTP 请求引擎，绕过浏览器 CORS 限制 |
| 前端框架 | SvelteKit (SPA adapter) | 单页应用模式 |
| 样式 | Tailwind CSS | utility-first CSS |
| 前后端通信 | Tauri invoke() | 直接调用 Rust command，无需额外端口 |

## 架构

```
UI (SvelteKit SPA) ──invoke()──> Tauri Command (Rust)
                                     └── reqwest 发 HTTP 请求
                                     └── 返回响应给前端
```

选择 Tauri Command 直连方案，最简洁，后续可按需演进。

## 目录结构

```
APIPup/
├── tauri/                  # Tauri + Rust 核心
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── src/
│   │   ├── main.rs         # Tauri 入口
│   │   ├── lib.rs          # 模块注册
│   │   └── commands/
│   │       ├── mod.rs
│   │       └── http.rs     # send_request command
│   └── icons/
├── ui/                     # SvelteKit 前端 (SPA)
│   ├── src/
│   │   ├── routes/
│   │   │   └── +page.svelte
│   │   ├── lib/
│   │   │   ├── components/
│   │   │   └── tauri.ts
│   │   └── app.html
│   ├── static/
│   ├── svelte.config.js
│   ├── vite.config.ts
│   ├── tailwind.config.js
│   ├── package.json
│   └── tsconfig.json
└── README.md
```

## Rust 侧数据契约

```rust
#[derive(Debug, Serialize, Deserialize)]
struct HttpRequest {
    method: String,           // GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS
    url: String,
    headers: HashMap<String, String>,
    body: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct HttpResponse {
    status: u16,
    status_text: String,
    headers: HashMap<String, String>,
    body: String,
    elapsed_ms: u64,
}
```

Tauri command `send_request` 接收 `HttpRequest`，用 reqwest 发出请求，返回 `HttpResponse`。

## 前端布局

参考 Apifox 布局：

```
┌──────────────────────────────────────────────────┐
│  APIPup                                    ─ □ × │
├────────────┬─────────────────────────────────────┤
│            │  [GET ▾] [https://api.example.com ] │
│  请求列表   │  [Send]                             │
│            ├──────────────────────────────────────┤
│  + 新请求   │  Params │ Headers │ Body            │
│            │  ┌────────────┬───────────────┐     │
│  GET /users│  │ Key        │ Value         │     │
│  POST /api │  ├────────────┼───────────────┤     │
│  PUT /item │  │            │               │     │
│            │  └────────────┴───────────────┘     │
│            ├──────────────────────────────────────┤
│            │  Response  200 OK  128ms             │
│            │  Body │ Headers                      │
│            │  ┌──────────────────────────────┐   │
│            │  │ { "data": [...] }            │   │
│            │  │                              │   │
│            │  └──────────────────────────────┘   │
└────────────┴─────────────────────────────────────┘
```

- **左侧面板**：请求列表，可新增/选择请求，内存中保存（框架阶段不做持久化）
- **右侧上半**：请求编辑区 — 方法选择、URL 输入、Params/Headers/Body tab 切换
- **右侧下半**：响应展示区 — 状态码、耗时、Body/Headers tab 切换

## 前端状态管理

使用 Svelte store 管理：

- `requestList`: 请求列表数组
- `activeRequest`: 当前选中的请求
- `response`: 当前响应结果
- `loading`: 请求进行中状态

框架阶段全部内存保存，不做持久化。

## 框架阶段范围

**包含：**
- Tauri 2 + SvelteKit SPA + Tailwind CSS 项目初始化
- Rust reqwest command 实现
- Apifox 风格三栏布局
- 完整的 GET/POST 等请求发送和响应展示
- 请求列表的增删和切换
- macOS + Windows 双平台支持

**不包含：**
- 数据持久化（请求保存到文件/数据库）
- 环境变量管理
- 请求集合/文件夹
- 导入导出
- 认证管理
- 代码生成
