package com.taskcalendar.entity;

import com.baomidou.mybatisplus.annotation.*;
import lombok.Data;
import java.time.LocalDateTime;

@Data
@TableName("task_tags")
public class TaskTag {
    
    @TableId(type = IdType.AUTO)
    private Long id;
    
    @TableField("task_id")
    private Long taskId;
    
    @TableField("tag_id")
    private Long tagId;
    
    @TableField(value = "created_at", fill = FieldFill.INSERT)
    private LocalDateTime createdAt;
}