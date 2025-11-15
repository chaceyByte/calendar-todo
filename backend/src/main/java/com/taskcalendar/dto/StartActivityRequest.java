package com.taskcalendar.dto;

import com.taskcalendar.enums.ActivityType;
import lombok.Data;

import javax.validation.constraints.NotNull;
import java.time.LocalDateTime;

@Data
public class StartActivityRequest {
    
    @NotNull(message = "任务ID不能为空")
    private Long taskId;
    
    @NotNull(message = "活动类型不能为空")
    private ActivityType activityType;
    
    private LocalDateTime startTime;
    
    private String description;
}