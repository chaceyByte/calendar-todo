## Context

当前系统存在时区处理不一致问题：前端 `<input type="datetime-local">` 产生本地时间字符串（如 `"2026-04-08T14:30"`）直接传入 Rust 后端并存入 SQLite，但后端读取时通过 `DateTime::from_naive_utc_and_offset(dt, Utc)` 强制按 UTC 解析，导致 UTC+8 用户遇到 8 小时偏移。

此外，多个后端模块混用 `NaiveDateTime`、`DateTime<Utc>`、`String` 等时间类型，`calendar/mod.rs` 中 `chrono::Local::now()` 与 `Utc::now()` 混用，存在日期边界判定风险。

## Goals / Non-Goals

**Goals:**
- 前后端时间戳语义统一：数据库统一存储 UTC 字符串（含 `Z` 后缀）
- 前端 `datetime-local` 输入在提交时转换为 UTC ISO 8601
- Rust 后端领域类型统一为 `DateTime<Utc>`（纯日期字段如 `report_date`、`holiday.date` 保留 `NaiveDate`）
- 前端显示时按用户本地时区渲染
- 向后兼容存量数据（读入时兼容无后缀格式）

**Non-Goals:**
- 不增加新依赖（dayjs 已有，无需 timezone 插件）
- 不改变数据库路径、网络请求、应用架构
- 不涉及用户时区配置选项（统一按系统本地时区）

## Decisions

### 1. 存储格式：统一为 ISO 8601 UTC 字符串
- **方案**：所有时间戳写入时带 `Z` 后缀（如 `"2026-04-08T06:30:00Z"`），存入 SQLite TEXT 列
- **理由**：SQLite 无原生时间类型，TEXT 存储带 `Z` 后缀的 ISO 8601 字符串可在不依赖数据库函数的前提下自描述时区
- **替代方案**：存储 `+08:00` 偏移 → 拒绝，因全局 UTC 更简单且无歧义；存储 Unix 时间戳 → 拒绝，降低可读性且调试不便

### 2. 前端提交：`datetime-local` → UTC ISO 8601
- **方案**：提交前用 `new Date(value).toISOString()` 将 `datetime-local` 值转为 UTC 字符串
- **理由**：`toISOString()` 始终返回 `"YYYY-MM-DDTHH:mm:ss.sssZ"` 格式，浏览器内置无需依赖
- **例外**：`date` 类型输入（如日历视图中的参数）不涉及时区，保持原样传输

### 3. Rust 后端类型统一
- **方案**：
  - 请求结构体：`start_at: Option<String>` → 保持 `String`，但约定接收 ISO 8601 UTC 字符串
  - 领域模型：`NaiveDateTime` → `DateTime<Utc>`（`tasks::Task`、`activity::ActivityRecord`、`calendar::CalendarEvent`、`work_duration::TaskWorkRecord`）
  - `work_hours::WorkHoursConfig.created_at/updated_at`：`String` → `DateTime<Utc>`
  - `calendar/mod.rs`：`chrono::Local::now()` → `Utc::now()`
  - 纯日期字段：`report_date: NaiveDate`、`holiday.date: NaiveDate` 保持不变
- **理由**：类型系统强制时区语义，编译期杜绝混淆
- **替代方案**：统一为 `DateTime<FixedOffset>` → 拒绝，全局 UTC 比任意偏移更简单

### 4. 存量数据兼容
- **方案**：读取时检测字符串是否含 `Z`/`+`/`-` 偏移后缀；无后缀的旧格式视为 UTC 处理
- **触发位置**：所有从数据库 `query_as` 读取时间戳后、构造领域类型前的转换点
- **理由**：允许渐进迁移，不要求数据迁移脚本

### 5. 前端显示：UTC → 本地时间
- **方案**：`dayjs(utcString).format(...)` 即可自动转换为本地时区（dayjs 默认将 ISO 8601 输入解析为本地时间）
- **理由**：dayjs 默认支持 ISO 8601 解析，无需额外插件或配置

### 6. SQL 触发器调整
- **方案**：`CURRENT_TIMESTAMP` 产生的字符串无后缀，改为 Rust 侧设置 `updated_at` 值，或触发器使用 `strftime('%Y-%m-%dT%H:%M:%fZ', 'now')`
- **理由**：统一所有写入路径的时间戳格式

## Risks / Trade-offs

| 风险 | 缓解措施 |
|---|---|
| 前端 `toISOString()` 在极旧浏览器中不可用 | Tauri 2.x 基于 Chromium，无兼容问题 |
| 存量数据无 `Z` 后缀导致解析歧义 | 兼容代码检测后缀，缺失则按 UTC 处理 |
| 任务 `start_at`/`due_at` 原本无时区含义（如"全天日期"） | 保持设计：用户设定时间即为本地时间的对应 UTC |
| 触发器中 `strftime` 格式与 Rust 端微秒精度不一致 | 统一使用 `%Y-%m-%dT%H:%M:%S%.fZ` 格式 |
