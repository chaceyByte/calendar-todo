package com.taskcalendar.entity;

import com.baomidou.mybatisplus.annotation.*;
import lombok.Data;
import java.time.LocalDateTime;

@Data
@TableName("tasks")
public class Task {
    
    @TableId(type = IdType.AUTO)
    private Long id;
    
    @TableField("title")
    private String title;
    
    @TableField("description")
    private String description;
    
    @TableField("status")
    private String status; // planning, in-progress, completed, paused
    
    @TableField("progress")
    private Integer progress;
    
    @TableField("priority")
    private String priority; // low, medium, high
    
    @TableField("urgency")
    private String urgency; // 非紧急, 一般, 紧急, 加急
    
    @TableField("start_date")
    private LocalDateTime startDate;
    
    @TableField("end_date")
    private LocalDateTime endDate;
    
    @TableField(value = "user_id", fill = FieldFill.INSERT)
    private Long userId;
    
    @TableField(value = "created_at", fill = FieldFill.INSERT)
    private LocalDateTime createdAt;
    
    @TableField(value = "updated_at", fill = FieldFill.INSERT_UPDATE)
    private LocalDateTime updatedAt;
    
    @TableField("deleted")
    @TableLogic
    private Integer deleted;
}