package com.taskcalendar.dto;

import lombok.Data;

import java.time.LocalDate;
import java.time.LocalDateTime;
import java.util.List;

@Data
public class DailyReport {
    
    private LocalDate date;
    private Integer totalTime; // 总活动时间（分钟）
    private Integer completedTasks; // 完成的任务数
    private Integer activeTasks; // 活动任务数
    private List<TaskActivityDetail> taskActivities;
    
    @Data
    public static class TaskActivityDetail {
        private Long taskId;
        private String taskTitle;
        private Integer duration; // 活动时长（分钟）
        private String status;
        private List<ActivityDetail> activities;
    }
    
    @Data
    public static class ActivityDetail {
        private Long id;
        private String activityType;
        private String description;
        private LocalDateTime startTime;
        private LocalDateTime endTime;
        private Integer duration;
    }
}