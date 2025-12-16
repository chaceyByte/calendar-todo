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