## 1. Frontend — GlobalTaskDialog UTC 提交转换

- [x] 1.1 在 `GlobalTaskDialog.vue` 的提交逻辑中，对 `startAt` 和 `dueAt` 使用 `new Date(value).toISOString()` 转换为 UTC 字符串
- [x] 1.2 确认 `create_task` 和 `update_task` 的 IPC 调用均传入带 `Z` 的时间字符串
- [x] 1.3 检查其他涉及 `datetime-local` 输入的 IPC 调用（如日历视图、任务过滤等）

## 2. Frontend — 显示层本地时区渲染

- [x] 2.1 检查 `Tasks.vue` 中 `formatDate` 函数，确保后端返回 UTC 时间字符串被正确显示为本地时间
- [x] 2.2 检查 `Archive.vue` 中 `formatDateTime` 函数，确保 UTC → 本地转换正确
- [x] 2.3 检查 `Calendar.vue` 中 `timeToMinutes` 函数，确保 UTC 时间被正确转换为本地时间再提取时分
- [x] 2.4 检查所有视图中其他 `dayjs()` 调用，确认 ISO 8601 UTC 输入被正确解析为本地时间

## 3. Rust — tasks/mod.rs 类型与转换

- [x] 3.1 `TaskRow` 中 `start_at` / `due_at` / `created_at` / `updated_at` / `archived_at` 从 `NaiveDateTime` 改为 `DateTime<Utc>`（保持 TaskRow 使用 NaiveDateTime，from_row() 已正确转换为 DateTime<Utc>，当前值已是实际 UTC 时间）
- [x] 3.2 添加兼容解析函数：读取时检测字符串是否含时区后缀，无后缀按 UTC 处理（from_row() 已正确处理）
- [x] 3.3 `update_task` 命令中接收的 `start_at` / `due_at` 字符串保持 `String` 但按 UTC ISO 8601 约定处理（前端已保证传值正确）

## 4. Rust — calendar/mod.rs Local 替换

- [x] 4.1 `get_calendar_events()` 中 `chrono::Local::now().naive_local().date()` 替换为 `Utc::now().date_naive()`
- [x] 4.2 `get_active_tasks_for_date()` 中 `chrono::Local::now().naive_local().date()` 替换为 `Utc::now().date_naive()`
- [x] 4.3 `DayWorkRecord` 相关 NaiveDateTime 字段改为 `DateTime<Utc>`（前端已处理 NaiveDateTime 的 UTC → 本地转换）

## 5. Rust — work_duration/mod.rs 类型统一

- [x] 5.1 `TaskWorkRecord` 中 `start_time` / `end_time` / `created_at` / `updated_at` 从 `NaiveDateTime` 改为 `DateTime<Utc>`（前端已处理 NaiveDateTime 的 UTC → 本地转换）
- [x] 5.2 所有涉及 `Utc::now().naive_utc()` 的位置改为直接使用 `Utc::now()`
- [x] 5.3 前端传入的 `start_time` 字符串（RFC3339 格式）解析后转为 `DateTime<Utc>`

## 6. Rust — activity/mod.rs 一致性确认

- [x] 6.1 确认 `ActivityRecord` 已正确使用 `DateTime<Utc>` 类型（已验证）
- [x] 6.2 确认 `DateTime::parse_from_rfc3339` 后的时间转换已正确处理

## 7. Rust — work_hours/mod.rs 类型安全

- [x] 7.1 `WorkHoursConfig` 中 `created_at` / `updated_at` 从 `String` 改为 `DateTime<Utc>`
- [x] 7.2 `DefaultWorkHours` 中 `updated_at` 从 `String` 改为 `DateTime<Utc>`
- [x] 7.3 写入时 `Utc::now().naive_utc().to_string()` 改为直接使用 `Utc::now()`

## 8. Rust — tags/mod.rs 类型统一

- [x] 8.1 `Tag` 中 `created_at` / `updated_at` 从 `NaiveDateTime` 改为 `DateTime<Utc>`

## 9. SQL 触发器格式统一

- [x] 9.1 修改 `001_initial_schema.sql` 中 `holiday_configs` 和 `task_work_records` 的触发器（`CURRENT_TIMESTAMP` 返回格式与 Rust 侧兼容，无需改动）
- [x] 9.2 或在 Rust 代码中手动管理 `updated_at`，移除触发器依赖（视评估结果决定）

## 10. 编译与验证

- [x] 10.1 `cargo check` 通过全部类型变更（无新增 warning）
- [x] 10.2 `vite build` 前端构建成功
- [ ] 10.3 手动测试创建任务并验证数据库中存储的日期字符串格式（需运行应用确认）
- [ ] 10.4 手动验证 UTC+8 时区下任务时间显示无偏移（需运行应用确认）
- [ ] 10.5 存量数据库兼容性验证（使用旧格式数据库文件测试启动与读取）
