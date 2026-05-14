## Why

数据库中的时间戳存在时区处理不一致问题：前端 `datetime-local` 输入（本地时间）未经转换直接存入 SQLite，但 Rust 后端读取时统一按 UTC 解析，导致 UTC+8 等非零时区用户的日期偏差 8 小时。同时，后端混用 `NaiveDateTime` 和 `DateTime<Utc>` 类型、混用 `chrono::Local::now()` 与 `Utc::now()`，存在潜在的数据一致性问题。

## What Changes

- **前端**：`datetime-local` 输入提交前转换为 UTC ISO 8601 字符串（含 `Z` 后缀），使前后端时区语义一致
- **Rust 后端**：
  - 统一使用 `DateTime<Utc>` 替代 `NaiveDateTime`（仅纯日期字段如 `report_date` 保留 `NaiveDate`）
  - `calendar/mod.rs` 中 `chrono::Local::now()` 替换为 `Utc::now()`
  - `work_hours/mod.rs` 中 `String` 类型时间戳改为 `DateTime<Utc>`
- **数据库**：确保所有写入的时间戳带时区标识（统一 UTC），SQL 触发器改用明确转换
- **前端显示**：`dayjs` 输出时按本地时区转换显示

## Capabilities

### New Capabilities
- `date-timezone`: 日期时间时区处理规范 — 定义数据库时间戳存储格式、前后端时区转换规则、API 契约

### Modified Capabilities
- 无（暂无已有 specs）

## Impact

- **IPC 接口变动**：`start_at` / `due_at` 等时间字段的输入输出格式从裸本地时间字符串改为 ISO 8601 UTC 字符串
- **前端兼容**：提交和显示两端需要适配新格式
- **数据库向后兼容**：存量数据格式不变，读入时需兼容旧格式
- **无网络请求变更**，无数据库路径变更，无新依赖引入
