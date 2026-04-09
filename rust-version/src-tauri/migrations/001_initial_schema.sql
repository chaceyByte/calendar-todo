-- SQLite数据库初始化脚本 v1.0.0
-- 合并所有迁移脚本为单一初始化脚本
-- 创建日期: 2025-04-09

-- ============================================
-- 1. 用户表（桌面应用简化版本）
-- ============================================
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    nickname TEXT NOT NULL,
    avatar TEXT,
    email TEXT,
    theme_color TEXT DEFAULT '#409eff',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 插入默认用户（桌面应用默认单用户）
INSERT OR IGNORE INTO users (id, username, nickname, email) VALUES
(1, 'user', '用户', 'user@taskmanager.local');

-- ============================================
-- 2. 任务表（包含四象限分类）
-- ============================================
CREATE TABLE IF NOT EXISTS tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT,

    -- 四象限分类
    quadrant INTEGER NOT NULL DEFAULT 1, -- 1:重要紧急, 2:重要不紧急, 3:紧急不重要, 4:不紧急不重要

    -- 传统任务属性
    status INTEGER NOT NULL DEFAULT 0, -- 0:规划中, 1:进行中, 2:已暂停, 3:已完成, 4:已归档
    progress INTEGER DEFAULT 0, -- 进度百分比 0-100

    -- 四象限属性
    is_important INTEGER DEFAULT 0, -- 是否重要: 0:否, 1:是
    is_urgent INTEGER DEFAULT 0,    -- 是否紧急: 0:否, 1:是

    -- 时间相关
    start_at DATETIME,
    due_at DATETIME,
    actual_start_at DATETIME, -- 任务实际开始时间（用于计算工作时长）

    -- 工作时长缓存字段
    total_work_duration_minutes INTEGER DEFAULT 0, -- 总工作时长（分钟）

    -- 关联和管理
    user_id INTEGER NOT NULL DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,

    -- 归档相关
    archived INTEGER DEFAULT 0,
    archived_at DATETIME,

    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- 创建任务索引
CREATE INDEX IF NOT EXISTS idx_tasks_quadrant ON tasks(quadrant, status);
CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks(status);
CREATE INDEX IF NOT EXISTS idx_tasks_user ON tasks(user_id);
CREATE INDEX IF NOT EXISTS idx_tasks_due ON tasks(due_at);
CREATE INDEX IF NOT EXISTS idx_tasks_important ON tasks(is_important);
CREATE INDEX IF NOT EXISTS idx_tasks_urgent ON tasks(is_urgent);

-- ============================================
-- 3. 标签表
-- ============================================
CREATE TABLE IF NOT EXISTS tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    color TEXT DEFAULT '#409eff',
    user_id INTEGER NOT NULL DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE(user_id, name)
);

-- ============================================
-- 4. 任务标签关联表
-- ============================================
CREATE TABLE IF NOT EXISTS task_tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    task_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE,
    UNIQUE(task_id, tag_id)
);

-- ============================================
-- 5. 活动记录表
-- ============================================
CREATE TABLE IF NOT EXISTS activity_records (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    task_id INTEGER NOT NULL,
    start_time DATETIME NOT NULL,
    end_time DATETIME,

    -- 活动类型: created, started, paused, resumed, completed, work, meeting, study, other
    activity_type TEXT NOT NULL,

    description TEXT,
    duration_minutes INTEGER,
    user_id INTEGER NOT NULL DEFAULT 1,

    -- 状态变更记录
    initial_status TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- 创建活动记录索引
CREATE INDEX IF NOT EXISTS idx_activity_task ON activity_records(task_id);
CREATE INDEX IF NOT EXISTS idx_activity_start ON activity_records(start_time);
CREATE INDEX IF NOT EXISTS idx_activity_user ON activity_records(user_id);

-- ============================================
-- 6. 节假日配置表
-- ============================================
CREATE TABLE IF NOT EXISTS holiday_configs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    year TEXT NOT NULL,
    date DATE NOT NULL,
    -- 类型: 'holiday' (节假日/休假), 'makeup' (补班), 'workday' (正常工作日)
    type TEXT NOT NULL CHECK (type IN ('holiday', 'makeup', 'workday')),
    -- 名称：节假日名称或补班说明
    name TEXT,
    -- 描述：详细说明
    description TEXT,
    -- 创建和更新时间
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    -- 唯一约束：每年每个日期只能有一条记录
    UNIQUE(year, date)
);

-- 创建节假日索引
CREATE INDEX IF NOT EXISTS idx_holiday_configs_year_date ON holiday_configs(year, date);
CREATE INDEX IF NOT EXISTS idx_holiday_configs_type ON holiday_configs(type);
CREATE INDEX IF NOT EXISTS idx_holiday_configs_date ON holiday_configs(date);

-- 创建触发器：自动更新 updated_at 字段
CREATE TRIGGER IF NOT EXISTS update_holiday_configs_timestamp
AFTER UPDATE ON holiday_configs
BEGIN
    UPDATE holiday_configs SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

-- ============================================
-- 7. 任务工作记录表
-- ============================================
CREATE TABLE IF NOT EXISTS task_work_records (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    task_id INTEGER NOT NULL,
    start_time DATETIME NOT NULL,
    end_time DATETIME,
    -- 实际工作时长（分钟）
    duration_minutes INTEGER DEFAULT 0,
    -- 换算后的工作日分钟数（用于动态计算天数）
    work_days_minutes INTEGER DEFAULT 0,
    -- 记录类型：started(开始), paused(暂停), completed(完成), archived(归档)
    record_type TEXT NOT NULL DEFAULT 'started',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE
);

-- 创建工作记录表索引
CREATE INDEX IF NOT EXISTS idx_task_work_records_task_id ON task_work_records(task_id);
CREATE INDEX IF NOT EXISTS idx_task_work_records_start_time ON task_work_records(start_time);
CREATE INDEX IF NOT EXISTS idx_task_work_records_end_time ON task_work_records(end_time);

-- 创建触发器：自动更新工作记录的 updated_at
CREATE TRIGGER IF NOT EXISTS update_task_work_records_timestamp
AFTER UPDATE ON task_work_records
BEGIN
    UPDATE task_work_records SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

-- ============================================
-- 8. 工作时长配置表
-- ============================================
CREATE TABLE IF NOT EXISTS work_hours_config (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    date DATE NOT NULL UNIQUE, -- 自然日期，YYYY-MM-DD格式，唯一索引

    -- 上午工作时段
    morning_start_time TEXT DEFAULT '08:30', -- 上午开始时间，格式 HH:MM
    morning_end_time TEXT DEFAULT '12:00',   -- 上午结束时间，格式 HH:MM

    -- 下午工作时段
    afternoon_start_time TEXT DEFAULT '13:00', -- 下午开始时间，格式 HH:MM
    afternoon_end_time TEXT DEFAULT '17:30',   -- 下午结束时间，格式 HH:MM

    -- 计算字段（自动计算）
    total_work_minutes INTEGER DEFAULT 480, -- 总工作分钟数（8小时 = 480分钟）

    -- 备注
    description TEXT,

    -- 时间戳
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 创建工作时长索引
CREATE INDEX IF NOT EXISTS idx_work_hours_date ON work_hours_config(date);

-- ============================================
-- 9. 默认工作时长配置表（全局默认值）
-- ============================================
CREATE TABLE IF NOT EXISTS default_work_hours (
    id INTEGER PRIMARY KEY CHECK (id = 1), -- 只允许一条记录

    -- 上午工作时段
    morning_start_time TEXT DEFAULT '08:30',
    morning_end_time TEXT DEFAULT '12:00',

    -- 下午工作时段
    afternoon_start_time TEXT DEFAULT '13:00',
    afternoon_end_time TEXT DEFAULT '17:30',

    -- 计算字段
    total_work_minutes INTEGER DEFAULT 480,

    -- 时间戳
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 插入默认工作时长配置
INSERT OR IGNORE INTO default_work_hours (id) VALUES (1);

-- ============================================
-- 10. 报告表（日报、周报）
-- ============================================
CREATE TABLE IF NOT EXISTS reports (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    type TEXT NOT NULL, -- 'daily', 'weekly'
    title TEXT NOT NULL,
    content TEXT,
    report_date DATE NOT NULL,
    user_id INTEGER NOT NULL DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- ============================================
-- 11. 插入初始测试数据
-- ============================================

-- 插入测试标签
INSERT OR IGNORE INTO tags (id, name, color, user_id) VALUES
(1, '前端', '#409eff', 1),
(2, '后端', '#67c23a', 1),
(3, '数据库', '#e6a23c', 1),
(4, '架构', '#f56c6c', 1),
(5, '测试', '#909399', 1);

-- 插入四象限示例任务
INSERT OR IGNORE INTO tasks (id, title, description, quadrant, status, is_important, is_urgent, start_at, due_at) VALUES
(1, '修复紧急线上bug', '尽快修复生产环境的严重bug', 1, 1, 1, 1, '2025-03-30 09:00:00', '2025-03-30 12:00:00'),
(2, '系统架构规划', '规划下个季度的技术架构迭代', 2, 0, 1, 0, '2025-04-01 09:00:00', '2025-04-15 18:00:00'),
(3, '回复用户咨询邮件', '处理用户的紧急咨询邮件', 3, 0, 0, 1, '2025-03-30 14:00:00', '2025-03-30 17:00:00'),
(4, '整理文件目录', '整理电脑中的过时文件', 4, 0, 0, 0, NULL, NULL),
(5, '准备季度汇报', '准备下一次的季度工作汇报', 2, 1, 1, 0, '2025-03-25 09:00:00', '2025-04-05 18:00:00');

-- 插入测试活动记录
INSERT OR IGNORE INTO activity_records (task_id, start_time, end_time, activity_type, description, duration_minutes) VALUES
(1, '2025-03-30 09:00:00', '2025-03-30 09:30:00', 'created', '任务创建', 30),
(1, '2025-03-30 09:30:00', NULL, 'started', '开始修复bug', NULL),
(5, '2025-03-25 09:00:00', '2025-03-25 11:00:00', 'created', '任务创建', 120),
(5, '2025-03-25 11:00:00', NULL, 'started', '开始准备汇报', NULL);

-- 插入2025年中国法定节假日数据
INSERT OR IGNORE INTO holiday_configs (year, date, type, name, description) VALUES
-- 元旦
('2025', '2025-01-01', 'holiday', '元旦', 'New Year''s Day'),
-- 春节
('2025', '2025-01-28', 'holiday', '春节', 'Spring Festival'),
('2025', '2025-01-29', 'holiday', '春节', 'Spring Festival'),
('2025', '2025-01-30', 'holiday', '春节', 'Spring Festival'),
('2025', '2025-01-31', 'holiday', '春节', 'Spring Festival'),
('2025', '2025-02-01', 'holiday', '春节', 'Spring Festival'),
('2025', '2025-02-02', 'holiday', '春节', 'Spring Festival'),
('2025', '2025-02-03', 'holiday', '春节', 'Spring Festival'),
-- 春节补班
('2025', '2025-01-26', 'makeup', '春节补班', 'Spring Festival Makeup Workday'),
('2025', '2025-02-08', 'makeup', '春节补班', 'Spring Festival Makeup Workday'),
-- 清明节
('2025', '2025-04-04', 'holiday', '清明节', 'Tomb Sweeping Day'),
('2025', '2025-04-05', 'holiday', '清明节', 'Tomb Sweeping Day'),
('2025', '2025-04-06', 'holiday', '清明节', 'Tomb Sweeping Day'),
-- 劳动节
('2025', '2025-05-01', 'holiday', '劳动节', 'Labor Day'),
('2025', '2025-05-02', 'holiday', '劳动节', 'Labor Day'),
('2025', '2025-05-03', 'holiday', '劳动节', 'Labor Day'),
('2025', '2025-05-04', 'holiday', '劳动节', 'Labor Day'),
('2025', '2025-05-05', 'holiday', '劳动节', 'Labor Day'),
-- 劳动节补班
('2025', '2025-04-27', 'makeup', '劳动节补班', 'Labor Day Makeup Workday');
