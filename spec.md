# Project Spec: Tauri Task Manager (四象限任务管理)
## 1. 项目定位
- 纯客户端桌面应用（无后端服务）
- 跨平台：Windows / macOS
- 技术栈：Tauri 2.x + Rust + SQLite + Web 前端（HTML/CSS/JS/TS）
- 核心：四象限任务管理、日历、看板、标签、统计、本地 SQLite 存储

## 2. 技术栈（硬约束）
- 桌面框架：Tauri 2.x（多窗口、IPC、系统集成）
- 后端：Rust 1.75+（async、内存安全、SQLite 操作）
- 数据库：SQLite（单文件存储，Rust 用 `sqlx` / `rusqlite`）
- 前端：HTML + CSS + TypeScript（可搭配 React/Vue/Svelte，优先原生或轻量框架）
- 构建：Vite + Tauri CLI
- 通信：Tauri IPC Commands（前端 invoke ↔ Rust 命令）
- 权限：Tauri Capabilities / Permissions 严格控制

## 3. 核心功能模块
1. 四象限视图（紧急/重要四象限，拖拽排序、拖拽归档）
2. 日历视图（按日期展示任务，导出日报/周报）
3. 任务看板（规划中/进行中/已归档，拖拽流转）
4. 归档任务（表格、搜索、分页）
5. 标签管理（CRUD，关联任务，删除校验）
6. 统计总结（按标签分组，可视化）

## 4. 架构分层
- 前端：UI 渲染、交互、状态管理、IPC 调用
- Rust 后端：
  - 命令层：Tauri Commands（供前端调用）
  - 数据层：SQLite CRUD、事务、查询
  - 业务层：任务逻辑、标签关联、导出逻辑
- 存储：单文件 SQLite DB（`tasks.db`）
- IPC：严格类型化、异步、错误处理

## 5. 非功能要求
- 内存占用：< 100MB
- 启动速度：< 1s
- 跨平台一致体验
- 安全：最小权限、CSP、输入校验
- 可维护：清晰分层、类型安全、文档化