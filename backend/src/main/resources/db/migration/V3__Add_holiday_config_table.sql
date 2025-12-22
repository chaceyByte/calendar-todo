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

-- 插入2025年示例数据
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
('2025', '2025-10-12', 'WORK', '国庆节补班');