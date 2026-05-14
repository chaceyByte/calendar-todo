---
status: STABLE
version: 1.0.0
last_updated: 2026-05-13
---

# Project: Tauri Task Manager (四象限任务管理)

## 定位
- 纯客户端桌面应用，无后端服务
- 跨平台：Windows / macOS
- 核心：四象限任务管理、日历、看板、标签、统计、本地 SQLite 存储

## 技术栈（硬约束）

| 层级 | 技术 | 版本约束 |
|------|------|---------|
| 桌面框架 | Tauri | 2.x |
| 后端 | Rust | 1.75+，async 必需 |
| 数据库 | SQLite | 单文件 `tasks.db`，sqlx 0.7 |
| 前端框架 | Vue 3 | Composition API, `&lt;script setup lang="ts"&gt;` |
| UI 库 | Element Plus | 2.x |
| CSS | Tailwind CSS | 4.x + CSS 自定义属性 |
| 状态管理 | Pinia | 2.x，Composition API 风格 |
| 构建工具 | Vite | 5.x |
| 图表 | ECharts | 5.x |
| 时间库 | dayjs / chrono | 前端 dayjs，后端 chrono |
| 异步运行时 | Tokio | 必需 |
| 序列化 | Serde | camelCase 前端交互，snake_case Rust 内部 |

## 目录结构
rust-version/
├── src/                           # Vue 3 前端
│   ├── main.ts                    # 入口：初始化 Pinia/EP/Router/Config
│   ├── App.vue                    # 根：Sidebar + router-view
│   ├── router/index.ts            # 7 路由，/ → /analytics
│   ├── stores/                    # Pinia 状态管理
│   │   ├── config.ts              # 配置（主题、db_path、备份）
│   │   └── theme.ts              # 主题（light/dark/system）
│   ├── components/                # 共享组件
│   │   ├── Sidebar.vue
│   │   ├── GlobalTaskDialog.vue
│   │   ├── ConfirmDeleteModal.vue
│   │   ├── CreateEditTagModal.vue
│   │   ├── DeleteTagModal.vue
│   │   ├── GeneralSettingsContent.vue
│   │   └── HolidaySettingsContent.vue
│   ├── views/                     # 页面视图
│   │   ├── Analytics.vue          # ECharts 统计面板
│   │   ├── Calendar.vue           # 日历视图
│   │   ├── Quadrant.vue           # 四象限矩阵
│   │   ├── Tasks.vue              # 看板任务
│   │   ├── Archive.vue            # 归档任务
│   │   ├── Tags.vue               # 标签管理
│   │   └── Settings.vue           # 应用设置
│   ├── utils/
│   │   └── eventBus.ts            # reactive ref 事件总线
│   ├── styles/
│   │   └── theme.css              # CSS 变量（light/dark）
│   └── style.css                  # 全局样式 + Tailwind 指令
├── src-tauri/                     # Rust 后端
│   ├── src/
│   │   ├── main.rs                # 入口：配置初始化、DB 初始化、~45 IPC 命令
│   │   ├── config.rs              # JSON 配置 + 版本迁移
│   │   ├── database/mod.rs        # SqlitePool + DDL 执行
│   │   ├── db_manager.rs          # 线程安全 DB 切换（状态机）
│   │   ├── tasks/mod.rs           # 任务 CRUD + 筛选
│   │   ├── tags/mod.rs            # 标签 CRUD + 任务关联
│   │   ├── calendar/mod.rs        # 节假日配置、日期类型逻辑
│   │   ├── calendar/tests.rs      # 日历单元测试
│   │   ├── work_duration/mod.rs   # 时间追踪记录
│   │   ├── work_hours/mod.rs      # 工作时间配置
│   │   ├── activity/mod.rs        # 活动记录模型
│   │   └── reports/mod.rs         # 日报/周报 CRUD
│   ├── migrations/
│   │   └── 001_initial_schema.sql
│   ├── capabilities/default.json  # Tauri 权限控制
│   └── tauri.conf.json            # 应用配置（窗口、CSP、打包）
├── scripts/                       # 构建与发布脚本
├── package.json
├── vite.config.ts
├── tsconfig.json
├── postcss.config.js
├── tailwind.config.js
└── dev.sh                         # 并发 Vite + Tauri dev

## 架构分层

| 层级 | 职责 | 约束 |
|------|------|------|
| 前端 | UI 渲染、交互、状态管理、IPC 调用 | 禁止直接操作数据库 |
| Rust 命令层 | Tauri Commands，供前端调用 | `pub async fn`，返回 `Result<T, String>` |
| Rust 数据层 | SQLite CRUD、事务、查询 | sqlx 编译时检查，参数化查询 |
| Rust 业务层 | 任务逻辑、标签关联、导出逻辑 | 复杂逻辑需单元测试覆盖 |
| 存储 | 单文件 SQLite | 路径固定，禁止修改 |

## 核心功能模块

1. **四象限视图** — 紧急/重要矩阵，拖拽排序、拖拽归档
2. **日历视图** — 按日期展示任务，导出日报/周报
3. **任务看板** — 规划中/进行中/已归档，拖拽流转
4. **归档任务** — 表格、搜索、分页
5. **标签管理** — CRUD，关联任务，删除校验
6. **统计总结** — 按标签分组，ECharts 可视化

## 编码规范

### Rust
- 模块：一目录一领域，`mod.rs` 聚合
- 命名：`snake_case` 函数/变量，`PascalCase` 类型/枚举
- DTO：`#[derive(Debug, Clone, Serialize, Deserialize)]`
- 序列化：`#[serde(rename_all = "camelCase")]` 请求/响应结构体，`#[serde(rename_all = "snake_case")]` 枚举
- 错误处理：`thiserror` 定义领域错误，命令层 `.map_err(|e| e.to_string())`
- 状态：`Arc<<Database>` 通过 `app.manage()`，`tauri::State<'_, Arc<<Database>>`
- 并发：`tokio::sync::RwLock` 内部可变性
- 注释：中文为主，`///` 文档注释

### 前端 (Vue 3 + TypeScript)
- 严格模式：`noUnusedLocals`，`noUnusedParameters`
- IPC：`invoke` 来自 `@tauri-apps/api/core`
- CSS：Tailwind 工具类 + scoped `<style>` + CSS 变量
- 主题：`theme.css` 变量，`data-theme` 属性，`matchMedia` 系统偏好
- 组件命名：`PascalCase.vue`
- 路径别名：`@/`、`@components/`、`@stores/`、`@views/`

### 数据库
- 迁移：顺序编号 `001_initial_schema.sql`
- 表名：`snake_case`，自增整数主键
- 时间戳：`created_at` / `updated_at`，`DATETIME DEFAULT CURRENT_TIMESTAMP`
- 外键：`ON DELETE CASCADE`
- 索引：`quadrant`、`status`、`due_at` 等高频查询列
- 程序迁移：Rust 代码执行 schema 变更（如 `run_migration_002`）

### 测试
- Rust：`#[cfg(test)] mod tests`，`#[tokio::test]`
- 前端：无单元测试或 E2E 测试

### Git
- 分支：`dev` 活跃开发
- 提交：`feature:` / `fix:` 前缀，中文主体

## 非功能要求（红线）

| 指标 | 约束 | 超限处理 |
|------|------|---------|
| 内存占用 | < 100MB | 必须优化，禁止放任 |
| 启动速度 | < 1s | 延迟加载非核心模块 |
| 跨平台 | Windows/macOS 一致 | 平台差异抽象到 Rust 层 |
| 安全 | 最小权限、CSP、输入校验 | 安全相关变更必须人工审核 |
| 可维护 | 清晰分层、类型安全、文档化 | 新功能必须同步更新 specs/ |