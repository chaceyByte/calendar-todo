## ADDED Requirements

### Requirement: 数据库时间戳统一存储为 UTC ISO 8601 格式
所有写入 SQLite DATETIME/TEXT 列的时间戳值必须以 ISO 8601 UTC 格式存储，即以 `Z` 后缀结尾。

#### Scenario: 新任务创建时 start_at 存储格式正确
- **WHEN** 用户设置任务开始时间并通过 `create_task` IPC 提交
- **THEN** 数据库中 `tasks.start_at` 列存储的字符串以 `Z` 结尾的 ISO 8601 格式

#### Scenario: 新任务创建时 due_at 存储格式正确
- **WHEN** 用户设置任务截止时间并通过 `create_task` IPC 提交
- **THEN** 数据库中 `tasks.due_at` 列存储的字符串以 `Z` 结尾的 ISO 8601 格式

#### Scenario: 触发器更新的 updated_at 带 Z 后缀
- **WHEN** 任意数据行被更新且触发器更新 `updated_at`
- **THEN** `updated_at` 列的值以 `Z` 结尾

### Requirement: 前端 datetime-local 输入提交时转换为 UTC
前端 `<input type="datetime-local">` 的值在通过 `invoke` 发送到 Rust 后端之前，必须使用 `new Date(value).toISOString()` 转换为 UTC ISO 8601 字符串。

#### Scenario: 创建任务时 startAt 为 UTC 格式
- **WHEN** 用户在 `GlobalTaskDialog` 中填写开始时间并点击保存
- **THEN** `invoke('create_task', ...)` 调用中 `startAt` 参数以 `Z` 结尾

#### Scenario: 创建任务时 dueAt 为 UTC 格式
- **WHEN** 用户在 `GlobalTaskDialog` 中填写截止时间并点击保存
- **THEN** `invoke('create_task', ...)` 调用中 `dueAt` 参数以 `Z` 结尾

#### Scenario: 更新任务时时间字段为 UTC 格式
- **WHEN** 用户编辑现有任务的开始时间或截止时间
- **THEN** `invoke('update_task', ...)` 调用中 `startAt` 或 `dueAt` 参数以 `Z` 结尾

### Requirement: Rust 后端统一使用 DateTime<Utc>
Rust 后端所有领域模型中的时间戳字段必须使用 `chrono::DateTime<Utc>` 类型，禁止直接使用 `NaiveDateTime` 表示时间戳（纯日期字段如 `report_date`、`holiday.date` 可保留 `NaiveDate`）。

#### Scenario: Task 模型的 start_at 类型正确
- **WHEN** Rust 后端读取 `tasks` 表数据
- **THEN** `Task.start_at` 字段类型为 `Option<DateTime<Utc>>` 而非 `Option<NaiveDateTime>`

#### Scenario: 活动记录的 start_time 类型正确
- **WHEN** Rust 后端读取 `activity_records` 表数据
- **THEN** `ActivityRecord.start_time` 字段类型为 `DateTime<Utc>`

#### Scenario: 工作记录的 start_time 类型正确
- **WHEN** Rust 后端读取 `task_work_records` 表数据
- **THEN** `TaskWorkRecord.start_time` 字段类型为 `DateTime<Utc>`

#### Scenario: 日历模块不使用 Local::now()
- **WHEN** `calendar/mod.rs` 中需要获取当前时间
- **THEN** 使用 `Utc::now()` 而非 `chrono::Local::now()`

### Requirement: 存量数据兼容读取
从数据库读取的时间戳字符串如果缺少 `Z` 后缀或时区偏移，系统必须兼容处理并按 UTC 解析。

#### Scenario: 读取无后缀旧格式数据
- **WHEN** 数据库中 `tasks.start_at` 值为 `"2026-04-08 14:30:00"`（旧格式，无后缀）
- **THEN** Rust 后端成功解析并按 UTC 处理，即等同于 `2026-04-08T14:30:00Z`

### Requirement: 前端按本地时区显示时间
前端从后端接收的时间（ISO 8601 UTC 格式）在显示给用户时必须转换为用户本地时区。

#### Scenario: UTC+8 用户查看任务截止时间
- **WHEN** 后端返回 `dueAt: "2026-04-08T06:30:00Z"`
- **THEN** 前端 `dayjs("2026-04-08T06:30:00Z").format(...)` 显示为 UTC+8 的 `"2026-04-08 14:30"`

#### Scenario: 日历视图工作记录时间正确
- **WHEN** 日历视图中显示工作记录的 `start_time` 和 `end_time`
- **THEN** 时间按用户本地时区渲染，而非直接显示 UTC 时间

### Requirement: WorkHoursConfig 时间戳类型安全
`WorkHoursConfig` 和 `DefaultWorkHours` 结构体中的 `created_at` 和 `updated_at` 字段必须使用 `DateTime<Utc>` 而非 `String`。

#### Scenario: WorkHoursConfig.created_at 类型正确
- **WHEN** Rust 后端读取 `work_hours_config` 表数据
- **THEN** `WorkHoursConfig.created_at` 类型为 `DateTime<Utc>`

#### Scenario: DefaultWorkHours.updated_at 类型正确
- **WHEN** Rust 后端读取 `default_work_hours` 表数据
- **THEN** `DefaultWorkHours.updated_at` 类型为 `DateTime<Utc>`
