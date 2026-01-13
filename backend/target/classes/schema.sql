-- 任务日历数据库初始化脚本

-- 创建数据库
CREATE DATABASE IF NOT EXISTS task_calendar CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

USE task_calendar;

-- 用户表
CREATE TABLE IF NOT EXISTS users (
    id BIGINT AUTO_INCREMENT PRIMARY KEY COMMENT '用户ID',
    username VARCHAR(50) NOT NULL UNIQUE COMMENT '用户名',
    password VARCHAR(255) NOT NULL COMMENT '密码',
    nickname VARCHAR(50) NOT NULL COMMENT '昵称',
    avatar VARCHAR(255) COMMENT '头像URL',
    email VARCHAR(100) COMMENT '邮箱',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    deleted TINYINT DEFAULT 0 COMMENT '逻辑删除标记'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='用户表';

-- 任务表
CREATE TABLE IF NOT EXISTS tasks (
    id BIGINT AUTO_INCREMENT PRIMARY KEY COMMENT '任务ID',
    title VARCHAR(200) NOT NULL COMMENT '任务标题',
    description TEXT COMMENT '任务描述',
    status ENUM('planning', 'in-progress', 'completed', 'paused') DEFAULT 'planning' COMMENT '任务状态',
    progress TINYINT DEFAULT 0 COMMENT '进度百分比',
    priority ENUM('low', 'medium', 'high') DEFAULT 'medium' COMMENT '优先级',
    start_date DATETIME COMMENT '开始时间',
    end_date DATETIME COMMENT '结束时间',
    user_id BIGINT NOT NULL COMMENT '用户ID',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    deleted TINYINT DEFAULT 0 COMMENT '逻辑删除标记',
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='任务表';

-- 标签表
CREATE TABLE IF NOT EXISTS tags (
    id BIGINT AUTO_INCREMENT PRIMARY KEY COMMENT '标签ID',
    name VARCHAR(50) NOT NULL COMMENT '标签名称',
    color VARCHAR(20) DEFAULT '#409eff' COMMENT '标签颜色',
    user_id BIGINT NOT NULL COMMENT '用户ID',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    deleted TINYINT DEFAULT 0 COMMENT '逻辑删除标记',
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='标签表';

-- 任务标签关联表
CREATE TABLE IF NOT EXISTS task_tags (
    id BIGINT AUTO_INCREMENT PRIMARY KEY COMMENT '关联ID',
    task_id BIGINT NOT NULL COMMENT '任务ID',
    tag_id BIGINT NOT NULL COMMENT '标签ID',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE,
    UNIQUE KEY uk_task_tag (task_id, tag_id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='任务标签关联表';

-- 报告表（日报、周报）
CREATE TABLE IF NOT EXISTS reports (
    id BIGINT AUTO_INCREMENT PRIMARY KEY COMMENT '报告ID',
    type ENUM('daily', 'weekly') NOT NULL COMMENT '报告类型',
    title VARCHAR(200) NOT NULL COMMENT '报告标题',
    content TEXT COMMENT '报告内容',
    report_date DATE NOT NULL COMMENT '报告日期',
    user_id BIGINT NOT NULL COMMENT '用户ID',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    deleted TINYINT DEFAULT 0 COMMENT '逻辑删除标记',
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='报告表';

-- 插入测试数据
INSERT IGNORE INTO users (id, username, password, nickname, avatar, email) VALUES 
(1, 'admin', '$2a$10$N.zmdr9k7uOCQb376NoUnuTJ8iAt6Z5EHsM8lE9lBOsl7iKTVwUiC', '管理员', 'https://cube.elemecdn.com/3/7c/3ea6beec64369c2642b92c6726f1epng.png', 'admin@example.com');

INSERT IGNORE INTO tags (id, name, color, user_id) VALUES 
(1, '前端', '#409eff', 1),
(2, '后端', '#67c23a', 1),
(3, '数据库', '#e6a23c', 1),
(4, '架构', '#f56c6c', 1),
(5, '测试', '#909399', 1);

INSERT IGNORE INTO tasks (id, title, description, status, progress, priority, start_date, end_date, user_id) VALUES 
(1, '项目架构设计', '设计整体项目架构和技术选型', 'completed', 100, 'high', '2024-01-10 09:00:00', '2024-01-15 18:00:00', 1),
(2, '前端页面开发', '开发前端页面和交互功能', 'in-progress', 60, 'high', '2024-01-12 09:00:00', '2024-01-22 18:00:00', 1),
(3, '后端API开发', '开发后端RESTful API接口', 'planning', 0, 'medium', '2024-01-15 09:00:00', '2024-01-25 18:00:00', 1),
(4, '数据库设计', '设计数据库表结构和关系', 'completed', 100, 'medium', '2024-01-05 09:00:00', '2024-01-12 18:00:00', 1),
(5, '测试与部署', '进行系统测试和部署上线', 'planning', 20, 'low', '2024-01-20 09:00:00', '2024-01-26 18:00:00', 1);

INSERT IGNORE INTO task_tags (task_id, tag_id) VALUES 
(1, 4), (2, 1), (3, 2), (4, 3), (5, 5);

-- 活动记录表
CREATE TABLE IF NOT EXISTS activity_records (
    id BIGINT AUTO_INCREMENT PRIMARY KEY COMMENT '活动记录ID',
    task_id BIGINT NOT NULL COMMENT '任务ID',
    start_time DATETIME NOT NULL COMMENT '开始时间',
    end_time DATETIME COMMENT '结束时间',
    user_id BIGINT NOT NULL COMMENT '用户ID',
    activity_type VARCHAR(20) NOT NULL COMMENT '活动类型',
    description TEXT COMMENT '活动描述',
    duration_minutes INT COMMENT '持续时间（分钟）',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    INDEX idx_task_id (task_id),
    INDEX idx_start_time (start_time),
    INDEX idx_task_start (task_id, start_time),
    FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='活动记录表';

-- 插入活动记录测试数据
INSERT IGNORE INTO activity_records (id, task_id, start_time, end_time, activity_type, description, duration_minutes) VALUES 
(1, 1, '2024-01-10 09:00:00', '2024-01-10 10:30:00', 'CREATED', '任务创建', 90),
(2, 1, '2024-01-10 10:30:00', '2024-01-15 18:00:00', 'STARTED', '开始任务', 6750),
(3, 1, '2024-01-15 18:00:00', '2024-01-15 18:00:00', 'COMPLETED', '任务完成', 0),
(4, 2, '2024-01-12 09:00:00', '2024-01-12 10:00:00', 'CREATED', '任务创建', 60),
(5, 2, '2024-01-12 10:00:00', '2024-01-14 16:00:00', 'STARTED', '开始任务', 3120),
(6, 2, '2024-01-14 16:00:00', '2024-01-16 09:00:00', 'PAUSED', '任务暂停', 1140),
(7, 2, '2024-01-16 09:00:00', '2024-01-18 17:00:00', 'RESUMED', '任务恢复', 1920),
(8, 3, '2024-01-15 09:00:00', '2024-01-15 10:00:00', 'CREATED', '任务创建', 60),
(9, 4, '2024-01-05 09:00:00', '2024-01-05 10:00:00', 'CREATED', '任务创建', 60),
(10, 4, '2024-01-05 10:00:00', '2024-01-12 18:00:00', 'STARTED', '开始任务', 6000),
(11, 4, '2024-01-12 18:00:00', '2024-01-12 18:00:00', 'COMPLETED', '任务完成', 0),
(12, 5, '2024-01-20 09:00:00', '2024-01-20 10:00:00', 'CREATED', '任务创建', 60),
(13, 2, '2024-01-17 14:00:00', '2024-01-17 16:00:00', 'WORK', '前端页面开发', 120),
(14, 2, '2024-01-18 09:00:00', '2024-01-18 11:30:00', 'MEETING', '前端需求评审会议', 150),
(15, 2, '2024-01-18 14:00:00', '2024-01-18 17:00:00', 'WORK', '前端页面开发', 180);

-- v3 关于节假日的数据
-- 创建休息日配置表
CREATE TABLE IF NOT EXISTS holiday_config (
    id BIGINT AUTO_INCREMENT PRIMARY KEY COMMENT '主键ID',
    year VARCHAR(4) NOT NULL COMMENT '年份，如 2025',
    date DATE NOT NULL COMMENT '具体日期',
    type ENUM('REST', 'WORK') NOT NULL COMMENT '假期类型：REST(休息), WORK(补班)',
    description VARCHAR(100) NOT NULL COMMENT '描述，如 元旦、春节等',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',

    INDEX idx_year (year) COMMENT '年份索引',
    INDEX idx_date (date) COMMENT '日期索引',
    UNIQUE KEY uk_year_date (year, date) COMMENT '年份和日期唯一索引'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='休息日配置表';

INSERT INTO holiday_config (year, date, type, description) VALUES
-- 元旦
('2025', '2025-01-01', 'REST', '元旦'),

-- 春节
('2025', '2025-01-28', 'REST', '春节'),
('2025', '2025-01-29', 'REST', '春节'),
('2025', '2025-01-30', 'REST', '春节'),
('2025', '2025-01-31', 'REST', '春节'),
('2025', '2025-02-01', 'REST', '春节'),
('2025', '2025-02-02', 'REST', '春节'),
('2025', '2025-02-03', 'REST', '春节'),

-- 清明节
('2025', '2025-04-04', 'REST', '清明节'),
('2025', '2025-04-05', 'REST', '清明节'),
('2025', '2025-04-06', 'REST', '清明节'),

-- 劳动节
('2025', '2025-05-01', 'REST', '劳动节'),
('2025', '2025-05-02', 'REST', '劳动节'),
('2025', '2025-05-03', 'REST', '劳动节'),

-- 端午节
('2025', '2025-06-10', 'REST', '端午节'),
('2025', '2025-06-11', 'REST', '端午节'),

-- 中秋节
('2025', '2025-09-17', 'REST', '中秋节'),

-- 国庆节
('2025', '2025-10-01', 'REST', '国庆节'),
('2025', '2025-10-02', 'REST', '国庆节'),
('2025', '2025-10-03', 'REST', '国庆节'),
('2025', '2025-10-04', 'REST', '国庆节'),
('2025', '2025-10-05', 'REST', '国庆节'),
('2025', '2025-10-06', 'REST', '国庆节'),
('2025', '2025-10-07', 'REST', '国庆节'),

-- 补班日
('2025', '2025-02-04', 'WORK', '春节补班'),
('2025', '2025-04-07', 'WORK', '清明节补班'),
('2025', '2025-05-06', 'WORK', '劳动节补班'),
('2025', '2025-09-16', 'WORK', '中秋节补班'),
('2025', '2025-09-29', 'WORK', '国庆节补班'),
('2025', '2025-10-12', 'WORK', '国庆节补班'),

-- 2026年
('2026', '2026-02-15', 'REST', '春节'),
('2026', '2026-02-16', 'REST', '春节'),
('2026', '2026-02-17', 'REST', '春节'),
('2026', '2026-02-18', 'REST', '春节'),
('2026', '2026-02-19', 'REST', '春节'),
('2026', '2026-02-20', 'REST', '春节'),
('2026', '2026-02-21', 'REST', '春节'),
('2026', '2026-02-22', 'REST', '春节'),
('2026', '2026-02-23', 'REST', '春节'),
-- 清明节
('2026', '2026-04-04', 'REST', '清明节'),
('2026', '2026-04-05', 'REST', '清明节'),
('2026', '2026-04-06', 'REST', '清明节'),
-- 劳动节
('2026', '2026-05-01', 'REST', '劳动节'),
('2026', '2026-05-02', 'REST', '劳动节'),
('2026', '2026-05-03', 'REST', '劳动节'),
('2026', '2026-05-04', 'REST', '劳动节'),
('2026', '2026-05-05', 'REST', '劳动节'),
-- 端午节
('2026', '2026-06-19', 'REST', '端午节'),
('2026', '2026-06-20', 'REST', '端午节'),
('2026', '2026-06-21', 'REST', '端午节'),
-- 中秋节
('2026', '2026-09-25', 'REST', '中秋节'),
('2026', '2026-09-26', 'REST', '中秋节'),
('2026', '2026-09-27', 'REST', '中秋节'),
-- 国庆节
('2026', '2026-10-01', 'REST', '国庆节'),
('2026', '2026-10-02', 'REST', '国庆节'),
('2026', '2026-10-03', 'REST', '国庆节'),
('2026', '2026-10-04', 'REST', '国庆节'),
('2026', '2026-10-05', 'REST', '国庆节'),
('2026', '2026-10-06', 'REST', '国庆节'),
('2026', '2026-10-07', 'REST', '国庆节'),
-- 补班日
('2026', '2026-02-14', 'WORK', '春节补班'),
('2026', '2026-02-28', 'WORK', '春节补班'),
('2026', '2026-05-09', 'WORK', '劳动节补班'),
('2026', '2026-09-20', 'WORK', '国庆节补班'),
('2026', '2026-10-10', 'WORK', '国庆节补班');
-- v4
-- 为任务表添加紧急程度字段
ALTER TABLE tasks ADD COLUMN urgency VARCHAR(20) NOT NULL DEFAULT '一般' COMMENT '紧急程度：非紧急、一般、紧急、加急';

-- 为紧急程度字段添加索引
CREATE INDEX idx_tasks_urgency ON tasks(urgency);

ALTER TABLE activity_records ADD initial_status varchar(20) NOT NULL COMMENT '记录状态变更初始值';

-- v5
-- 邮箱验证码表
CREATE TABLE IF NOT EXISTS email_verification_codes (
    id BIGINT AUTO_INCREMENT PRIMARY KEY COMMENT '验证码ID',
    email VARCHAR(100) NOT NULL COMMENT '邮箱地址',
    code VARCHAR(10) NOT NULL COMMENT '验证码',
    type ENUM('REGISTER', 'RESET_PASSWORD', 'CHANGE_EMAIL') NOT NULL COMMENT '验证码类型',
    expires_at DATETIME NOT NULL COMMENT '过期时间',
    used TINYINT DEFAULT 0 COMMENT '是否已使用',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    INDEX idx_email_type (email, type),
    INDEX idx_expires_at (expires_at)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='邮箱验证码表';