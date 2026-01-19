-- 更新任务表的重要程度和紧急程度字段，添加归档时间字段

-- 修改重要程度字段，支持新的枚举值
ALTER TABLE tasks
    MODIFY COLUMN priority ENUM ('-high', '-middle', '-low', 'low', 'middle', 'high')
        DEFAULT 'middle'
        COMMENT '重要程度：-high(极不重要), -middle(较不重要), -low(不重要), low(不是很重要), middle(一般), high(重要)';

-- 修改紧急程度字段，支持新的枚举值
ALTER TABLE tasks
    MODIFY COLUMN urgency ENUM ('-high', '-middle', '-low', 'low', 'middle', 'high')
        DEFAULT 'middle'
        COMMENT '紧急程度：-high(极不紧急), -middle(较不紧急), -low(不紧急), low(不是很紧急), middle(一般), high(紧急)';

-- 添加归档时间字段
ALTER TABLE tasks
    ADD COLUMN archived_at DATETIME
        COMMENT '归档时间，记录任务完成时的归档时间';

-- 为归档时间字段添加索引
CREATE INDEX idx_tasks_archived_at ON tasks (archived_at);

-- 为重要程度和紧急程度字段重新添加索引
DROP INDEX IF EXISTS idx_tasks_priority ON tasks;
DROP INDEX IF EXISTS idx_tasks_urgency ON tasks;

CREATE INDEX idx_tasks_priority ON tasks (priority);
CREATE INDEX idx_tasks_urgency ON tasks (urgency);

-- 更新现有数据，将原有值映射到新枚举值
UPDATE tasks
SET priority = CASE
                   WHEN priority = 'low' THEN '-low'
                   WHEN priority = 'medium' THEN 'middle'
                   WHEN priority = 'high' THEN 'high'
                   ELSE 'middle'
    END,
    urgency  = CASE
                   WHEN urgency = '非紧急' THEN '-low'
                   WHEN urgency = '一般' THEN 'middle'
                   WHEN urgency = '紧急' THEN 'high'
                   WHEN urgency = '加急' THEN 'high'
                   ELSE 'middle'
        END
WHERE priority IN ('low', 'medium', 'high')
   OR urgency IN ('非紧急', '一般', '紧急', '加急');