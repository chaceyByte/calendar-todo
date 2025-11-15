-- 创建活动记录表
CREATE TABLE IF NOT EXISTS activity_records (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    task_id BIGINT NOT NULL,
    start_time DATETIME NOT NULL,
    end_time DATETIME,
    activity_type VARCHAR(20) NOT NULL,
    description TEXT,
    duration_minutes INT,
    created_at DATETIME NOT NULL,
    INDEX idx_task_id (task_id),
    INDEX idx_start_time (start_time),
    INDEX idx_task_start (task_id, start_time),
    FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE
);