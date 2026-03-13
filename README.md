# rust-vue-admin

<div align="center">
  <h1>rust-vue-admin</h1>
  <p>基于 Rust + Vue3 的全栈后台管理框架，灵感来源于 gin-vue-admin</p>

  [![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org/)
  [![Vue](https://img.shields.io/badge/Vue-3.x-green.svg)](https://vuejs.org/)
  [![Axum](https://img.shields.io/badge/Axum-0.8-blue.svg)](https://github.com/tokio-rs/axum)
  [![SeaORM](https://img.shields.io/badge/SeaORM-1.1-purple.svg)](https://www.sea-ql.org/SeaORM/)
  [![License](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
</div>

---

## 项目简介

rust-vue-admin 是一个面向 AI 时代、追求极致效率的全栈后台管理系统框架。它以 Rust 作为高性能后端语言，以 Vue3 构建现代化前端界面。

本项目参考了 [gin-vue-admin](https://github.com/flipped-aurora/gin-vue-admin) 的成熟设计理念与架构，并以前瞻性的技术选型，使用 Rust 生态重新实现了后端部分。在 AI 应用与数据密集型服务日益成为核心的今天，开发语言本身已非瓶颈，选择性能最高、内存最安全的语言才是构建可靠、高效系统的基石。Rust 提供的卓越性能、零成本抽象和无畏并发，正是为此而生，旨在为下一代智能后台管理系统提供强大的基础设施保障。

## 技术栈

### 后端 (Rust)

| 功能 | 技术选型 |
|------|---------|
| Web 框架 | [Axum 0.8](https://github.com/tokio-rs/axum) |
| 异步运行时 | [Tokio](https://tokio.rs/) |
| ORM | [SeaORM 1.1](https://www.sea-ql.org/SeaORM/) |
| 配置管理 | [config-rs](https://github.com/mehcode/config-rs) + 热重载 ([notify](https://github.com/notify-rs/notify)) |
| 日志 | [tracing](https://github.com/tokio-rs/tracing) |
| JWT | [jsonwebtoken](https://github.com/Keats/jsonwebtoken) |
| 权限控制 | [casbin-rs](https://github.com/casbin/casbin-rs) |
| Redis | [redis-rs](https://github.com/redis-rs/redis-rs) |
| API 文档 | [utoipa](https://github.com/juhaku/utoipa) (OpenAPI 3.0) |
| 系统信息 | [sysinfo](https://github.com/GuillaumeGomez/sysinfo) |
| 参数验证 | [validator](https://github.com/Keats/validator) |
| 密码哈希 | [bcrypt](https://github.com/Keats/rust-bcrypt) / [argon2](https://github.com/RustCrypto/password-hashes) |
| 邮件 | [lettre](https://github.com/lettre/lettre) |
| 定时任务 | [tokio-cron-scheduler](https://github.com/mvniekerk/tokio-cron-scheduler) |

### 前端 (Vue3)

| 功能 | 技术选型 |
|------|---------|
| 框架 | Vue 3 |
| UI 组件库 | Element Plus |
| 状态管理 | Pinia |
| 路由 | Vue Router 4 |
| 构建工具 | Vite 6 |
| HTTP 客户端 | Axios |
| CSS 原子化 | UnoCSS |

## 项目结构

```
rust-vue-admin/
├── server/                    # Rust 后端
│   ├── Cargo.toml             # 依赖管理
│   ├── config.yaml            # 配置文件（支持热重载）
│   ├── resource/              # 资源文件（Excel 模板等）
│   └── src/
│       ├── main.rs            # 入口文件
│       ├── config/            # 配置结构体（JWT/数据库/Redis/邮件/OSS 等）
│       ├── global/            # 全局状态 (AppState)、统一响应
│       ├── core/              # 核心启动逻辑（服务器、日志）
│       ├── initialize/        # 初始化模块（数据库、Redis、Casbin、配置热重载）
│       ├── middleware/        # 中间件（JWT 认证、Casbin 权限、CORS、限流、请求日志）
│       ├── api/               # API 处理器
│       │   ├── system/        # 系统模块（用户/角色/菜单/API/字典/参数/操作记录...）
│       │   └── example/       # 示例模块
│       ├── router/            # 路由定义
│       ├── service/           # 业务逻辑层
│       ├── model/             # 数据模型
│       │   ├── common/        # 通用模型
│       │   └── system/        # 系统模型（含 request/response）
│       ├── utils/             # 工具函数（JWT、密码、验证码、文件上传）
│       ├── source/            # 初始数据
│       ├── task/              # 定时任务
│       └── migration/         # 数据库迁移（建表 + 种子数据 + Casbin 策略）
│
├── web/                       # Vue3 前端（基于 gin-vue-admin 前端适配）
│   ├── src/
│   │   ├── api/               # API 调用层
│   │   ├── components/        # 公共组件
│   │   ├── view/              # 页面视图
│   │   ├── router/            # 路由
│   │   ├── pinia/             # 状态管理
│   │   ├── core/              # 核心配置、全局注册
│   │   ├── style/             # 全局样式
│   │   ├── hooks/             # 组合式函数
│   │   ├── directive/         # 自定义指令
│   │   ├── plugin/            # 插件（公告、邮件）
│   │   └── utils/             # 工具函数
│   ├── Dockerfile             # 前端 Docker 构建
│   ├── package.json
│   └── vite.config.js
│
├── deploy/                    # 部署配置
│   ├── docker/                # Dockerfile
│   ├── docker-compose/        # Docker Compose
│   └── kubernetes/            # K8s 配置
│
├── docs/                      # 文档
├── Makefile                   # 构建脚本
└── README.md
```

## 功能特性

### 已完成功能

- [x] **项目基础架构**
  - [x] Axum Web 框架搭建
  - [x] 配置系统（config.yaml + 热重载）
  - [x] 数据库连接（MySQL / PostgreSQL / SQLite）
  - [x] 日志系统（tracing + 文件滚动）
  - [x] Redis 连接（支持集群）
  - [x] 统一响应结构
  - [x] CORS 跨域中间件
  - [x] IP 限流中间件
  - [x] 请求日志中间件（操作记录自动写入）
  - [x] 数据库初始化引导（Web 端引导式初始化）
- [x] **用户认证系统**
  - [x] 用户登录（含验证码）
  - [x] 管理员注册用户
  - [x] JWT Token 认证
  - [x] JWT 黑名单（Token 注销）
  - [x] 修改密码
  - [x] 修改个人信息 / 个人设置
- [x] **权限管理系统 (RBAC)**
  - [x] 角色管理（增删改查、拷贝、数据权限设置）
  - [x] 角色用户管理（查看/设置角色下用户）
  - [x] 菜单管理（动态路由、树形结构、菜单角色绑定）
  - [x] API 权限管理（同步路由、忽略 API、分组管理、角色绑定）
  - [x] Casbin 规则管理（完整策略覆盖、一键刷新）
  - [x] 按钮级权限控制
- [x] **系统管理**
  - [x] 用户管理（列表、启用/禁用、重置密码、分配角色）
  - [x] 字典管理（增删改查、导入导出）
  - [x] 字典详情（树形结构、路径查询）
  - [x] 操作记录（列表、单条/批量删除）
  - [x] 参数管理（增删改查、按 key 查询）
  - [x] 系统配置（获取/修改配置、服务器信息、配置重载）
  - [x] 错误日志（前端错误自动上报）
  - [x] 邮件发送测试
  - [x] 数据库迁移（SeaORM Migration）
  - [x] 种子数据初始化（含完整 Casbin 策略）
  - [x] 健康检查接口
- [x] **前端适配**
  - [x] 复用 gin-vue-admin 前端（品牌定制化）
  - [x] 后端接口全面适配
  - [x] Dashboard 页面
  - [x] 数据库初始化引导页
  - [x] Docker 部署支持（前端 Dockerfile）

### 规划中功能

- [ ] **文件管理**
  - [ ] 本地文件上传
  - [ ] MinIO/S3 对象存储
  - [ ] 阿里云 OSS / 腾讯云 COS
- [ ] **高级功能**
  - [ ] 定时任务管理
  - [ ] 代码生成器
  - [ ] 插件系统
  - [ ] 公告管理
- [ ] **部署**
  - [ ] Docker Compose 完整编排
  - [ ] Kubernetes 部署配置

## 快速开始

### 环境要求

- Rust 1.75+
- Node.js 18+
- MySQL 8.0+ / PostgreSQL 14+ / SQLite 3
- Redis 6+（可选）

### 后端启动

```bash
# 进入后端目录
cd server

# 修改配置文件，填写数据库等配置
cp config.yaml config.yaml.local  # 可选：创建本地配置副本
vim config.yaml

# 启动开发服务器（首次启动会自动运行数据库迁移）
cargo run

# 或使用热重载（需要安装 cargo-watch: cargo install cargo-watch）
make dev-server
```

> **提示**：首次启动后，访问前端页面会自动跳转到数据库初始化引导页，按提示完成初始化即可。

### 前端启动

```bash
# 进入前端目录
cd web

# 安装依赖
npm install

# 启动开发服务器
npm run dev
```

### 默认账号

| 角色 | 用户名 | 密码 |
|------|--------|------|
| 超级管理员 | admin | 123456 |

### 使用 Makefile

```bash
# 查看所有可用命令
make help

# === 后端 ===
make run-server            # 运行后端
make dev-server            # 热重载开发（需要 cargo-watch）
make build-server          # 编译后端（debug）
make build-server-release  # 编译后端（release）
make test-server           # 运行测试
make fmt                   # 代码格式化
make lint                  # 代码检查（clippy）

# === 数据库 ===
make migrate-up            # 运行数据库迁移
make migrate-down          # 回滚迁移
make migrate-fresh         # 重置数据库

# === 前端 ===
make install-web           # 安装前端依赖
make dev-web               # 启动前端开发服务器
make build-web             # 构建前端

# === 全栈 ===
make build                 # 构建全部（前端 + 后端）
make clean                 # 清理构建产物

# === Docker ===
make docker-build          # 构建 Docker 镜像
make docker-up             # Docker Compose 启动
make docker-down           # Docker Compose 停止
```

## 与 gin-vue-admin 对比

| 特性 | gin-vue-admin | rust-vue-admin |
|------|--------------|----------------|
| 后端语言 | Go | Rust |
| Web 框架 | Gin | Axum |
| ORM | GORM | SeaORM |
| 内存安全 | GC | 编译期保证 |
| 性能 | 高 | 极高 |
| 并发模型 | goroutine | async/await |
| 二进制大小 | 中等 | 较小（strip 后） |
| 编译速度 | 快 | 较慢 |
| 前端复用 | 原生 | gin-vue-admin 前端适配 |
| 配置热重载 | viper + fsnotify | config-rs + notify |

## 更新日志

### v0.1.0（开发中）

- **基础架构**：Axum + SeaORM + 配置热重载 + 日志系统
- **用户认证**：登录 / 注册 / JWT / 验证码 / 密码管理
- **RBAC 权限**：角色 / 菜单 / API / Casbin / 按钮级权限
- **系统管理**：字典 / 操作记录 / 参数 / 系统配置 / 邮件 / 错误日志
- **中间件**：JWT 认证 / Casbin 鉴权 / CORS / 限流 / 请求日志
- **前端适配**：复用 gin-vue-admin 前端，全面适配 Rust 后端
- **数据库**：完整迁移脚本 + 种子数据 + Casbin 策略

## 贡献指南

欢迎提交 Issue 和 Pull Request！

## 许可证

[MIT License](LICENSE)

## 致谢

- [gin-vue-admin](https://github.com/flipped-aurora/gin-vue-admin) — 项目灵感来源与前端基础
- [Axum](https://github.com/tokio-rs/axum) — 高性能 Rust Web 框架
- [SeaORM](https://www.sea-ql.org/SeaORM/) — Rust 异步 ORM 框架
- [Element Plus](https://element-plus.org/) — Vue3 UI 组件库
- [Casbin](https://casbin.org/) — 灵活的访问控制框架
