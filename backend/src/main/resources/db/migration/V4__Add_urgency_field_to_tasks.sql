-- 为任务表添加紧急程度字段
ALTER TABLE tasks ADD COLUMN urgency VARCHAR(20) NOT NULL DEFAULT '一般' COMMENT '紧急程度：非紧急、一般、紧急、加急';

-- 为紧急程度字段添加索引
CREATE INDEX idx_tasks_urgency ON tasks(urgency);

ALTER TABLE task_calendar.activity_records ADD initial_status varchar(20) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '记录状态变更初始值';
