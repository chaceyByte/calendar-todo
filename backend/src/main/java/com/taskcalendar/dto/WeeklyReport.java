package com.taskcalendar.dto;

import lombok.Data;

import java.time.LocalDate;
import java.util.List;
import java.util.Map;

@Data
public class WeeklyReport {
    
    private LocalDate weekStart;
    private LocalDate weekEnd;
    private Integer totalTime; // 总活动时间（分钟）
    private Integer completedTasks; // 完成的任务数
    private Map<LocalDate, DaySummary> dailySummaries;
    private List<TaskActivityDetail> taskActivities;
    
    @Data
    public static class DaySummary {
        private LocalDate date;
        private Integer totalTime; // 当天活动时间（分钟）
        private Integer completedTasks; // 当天完成的任务数
        private Integer activeTasks; // 当天活动任务数
    }
    
    @Data
    public static class TaskActivityDetail {
        private Long taskId;
        private String taskTitle;
        private Integer totalDuration; // 总活动时长（分钟）
        private Map<LocalDate, Integer> dailyDurations; // 每日活动时长
        private String status;
    }
}