-- SQLite Database Initialization Script v1.0.0
-- Core schema for Task Manager Application

-- ============================================
-- 1. Users Table
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

INSERT OR IGNORE INTO users (id, username, nickname, email) VALUES
(1, 'user', 'User', 'user@taskmanager.local');

-- ============================================
-- 2. Tasks Table
-- ============================================
CREATE TABLE IF NOT EXISTS tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT,
    quadrant INTEGER NOT NULL DEFAULT 1,
    status INTEGER NOT NULL DEFAULT 0,
    progress INTEGER DEFAULT 0,
    is_important INTEGER DEFAULT 0,
    is_urgent INTEGER DEFAULT 0,
    start_at DATETIME,
    due_at DATETIME,
    actual_start_at DATETIME,
    total_work_duration_minutes INTEGER DEFAULT 0,
    user_id INTEGER NOT NULL DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    archived INTEGER DEFAULT 0,
    archived_at DATETIME,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_tasks_quadrant ON tasks(quadrant, status);
CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks(status);
CREATE INDEX IF NOT EXISTS idx_tasks_user ON tasks(user_id);
CREATE INDEX IF NOT EXISTS idx_tasks_due ON tasks(due_at);
CREATE INDEX IF NOT EXISTS idx_tasks_important ON tasks(is_important);
CREATE INDEX IF NOT EXISTS idx_tasks_urgent ON tasks(is_urgent);

-- ============================================
-- 3. Tags Table
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
-- 4. Task Tags Table
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
-- 5. Activity Records Table
-- ============================================
CREATE TABLE IF NOT EXISTS activity_records (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    task_id INTEGER NOT NULL,
    start_time DATETIME NOT NULL,
    end_time DATETIME,
    activity_type TEXT NOT NULL,
    description TEXT,
    duration_minutes INTEGER,
    user_id INTEGER NOT NULL DEFAULT 1,
    initial_status TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_activity_task ON activity_records(task_id);
CREATE INDEX IF NOT EXISTS idx_activity_start ON activity_records(start_time);
CREATE INDEX IF NOT EXISTS idx_activity_user ON activity_records(user_id);

-- ============================================
-- 6. Holiday Configs Table
-- ============================================
CREATE TABLE IF NOT EXISTS holiday_configs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    year TEXT NOT NULL,
    date DATE NOT NULL,
    type TEXT NOT NULL CHECK (type IN ('holiday', 'makeup', 'workday')),
    name TEXT,
    description TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(year, date)
);

CREATE INDEX IF NOT EXISTS idx_holiday_configs_year_date ON holiday_configs(year, date);
CREATE INDEX IF NOT EXISTS idx_holiday_configs_type ON holiday_configs(type);
CREATE INDEX IF NOT EXISTS idx_holiday_configs_date ON holiday_configs(date);

CREATE TRIGGER IF NOT EXISTS update_holiday_configs_timestamp
AFTER UPDATE ON holiday_configs
BEGIN
    UPDATE holiday_configs SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

-- ============================================
-- 7. Task Work Records Table
-- ============================================
CREATE TABLE IF NOT EXISTS task_work_records (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    task_id INTEGER NOT NULL,
    start_time DATETIME NOT NULL,
    end_time DATETIME,
    duration_minutes INTEGER DEFAULT 0,
    work_days_minutes INTEGER DEFAULT 0,
    record_type TEXT NOT NULL DEFAULT 'started',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_task_work_records_task_id ON task_work_records(task_id);
CREATE INDEX IF NOT EXISTS idx_task_work_records_start_time ON task_work_records(start_time);
CREATE INDEX IF NOT EXISTS idx_task_work_records_end_time ON task_work_records(end_time);

CREATE TRIGGER IF NOT EXISTS update_task_work_records_timestamp
AFTER UPDATE ON task_work_records
BEGIN
    UPDATE task_work_records SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

-- ============================================
-- 8. Work Hours Config Table
-- ============================================
CREATE TABLE IF NOT EXISTS work_hours_config (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    date DATE NOT NULL UNIQUE,
    morning_start_time TEXT DEFAULT '08:30',
    morning_end_time TEXT DEFAULT '12:00',
    afternoon_start_time TEXT DEFAULT '13:00',
    afternoon_end_time TEXT DEFAULT '17:30',
    total_work_minutes INTEGER DEFAULT 480,
    description TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_work_hours_date ON work_hours_config(date);

-- ============================================
-- 9. Default Work Hours Table
-- ============================================
CREATE TABLE IF NOT EXISTS default_work_hours (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    morning_start_time TEXT DEFAULT '08:30',
    morning_end_time TEXT DEFAULT '12:00',
    afternoon_start_time TEXT DEFAULT '13:00',
    afternoon_end_time TEXT DEFAULT '17:30',
    total_work_minutes INTEGER DEFAULT 480,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

INSERT OR IGNORE INTO default_work_hours (id) VALUES (1);

-- ============================================
-- 10. Reports Table
-- ============================================
CREATE TABLE IF NOT EXISTS reports (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    type TEXT NOT NULL,
    title TEXT NOT NULL,
    content TEXT,
    report_date DATE NOT NULL,
    user_id INTEGER NOT NULL DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
