package com.taskcalendar.entity;

import com.baomidou.mybatisplus.annotation.*;
import com.fasterxml.jackson.annotation.JsonFormat;
import lombok.Data;

import java.time.LocalDateTime;

@Data
@TableName("activity_records")
public class ActivityRecord {
    
    @TableId(type = IdType.AUTO)
    private Long id;
    
    @TableField("task_id")
    private Long taskId;
    
    @TableField(value = "user_id", fill = FieldFill.INSERT)
    private Long userId;
    
    @TableField("start_time")
    @JsonFormat(pattern = "yyyy-MM-dd HH:mm:ss")
    private LocalDateTime startTime;
    
    @TableField("end_time")
    @JsonFormat(pattern = "yyyy-MM-dd HH:mm:ss")
    private LocalDateTime endTime;
    
    @TableField("activity_type")
    private String activityType;
    
    @TableField("description")
    private String description;
    
    @TableField("duration_minutes")
    private Integer durationMinutes;
    
    @TableField("initial_status")
    private String initialStatus;
    
    @TableField(value = "created_at", fill = FieldFill.INSERT)
    @JsonFormat(pattern = "yyyy-MM-dd HH:mm:ss")
    private LocalDateTime createdAt;
    
    // 计算持续时间的方法
    public void calculateDuration() {
        if (endTime != null && startTime != null) {
            durationMinutes = (int) java.time.Duration.between(startTime, endTime).toMinutes();
        }
    }
}