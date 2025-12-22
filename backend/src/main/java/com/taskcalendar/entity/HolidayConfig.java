package com.taskcalendar.entity;

import com.baomidou.mybatisplus.annotation.*;
import com.fasterxml.jackson.annotation.JsonFormat;
import lombok.Data;
import lombok.EqualsAndHashCode;

import java.time.LocalDate;

/**
 * 休息日配置实体类
 */
@Data
@EqualsAndHashCode(callSuper = false)
@TableName("holiday_config")
public class HolidayConfig {
    
    @TableId(value = "id", type = IdType.AUTO)
    private Long id;
    
    /**
     * 年份，如 "2025"
     */
    @TableField("year")
    private String year;
    
    /**
     * 具体日期
     */
    @TableField("date")
    @JsonFormat(pattern = "yyyy-MM-dd")
    private LocalDate date;
    
    /**
     * 假期类型：REST(休息), WORK(补班)
     */
    @TableField("type")
    private HolidayType type;
    
    /**
     * 描述，如 "元旦"、"春节"等
     */
    @TableField("description")
    private String description;
    
    /**
     * 创建时间
     */
    @TableField(value = "created_at", fill = FieldFill.INSERT)
    @JsonFormat(pattern = "yyyy-MM-dd HH:mm:ss")
    private java.time.LocalDateTime createdAt;
    
    /**
     * 更新时间
     */
    @TableField(value = "updated_at", fill = FieldFill.INSERT_UPDATE)
    @JsonFormat(pattern = "yyyy-MM-dd HH:mm:ss")
    private java.time.LocalDateTime updatedAt;
    
    /**
     * 假期类型枚举
     */
    public enum HolidayType {
        REST, // 休息日
        WORK  // 补班日
    }
}