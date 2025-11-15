package com.taskcalendar.mapper;

import com.baomidou.mybatisplus.core.mapper.BaseMapper;
import com.taskcalendar.entity.ActivityRecord;
import org.apache.ibatis.annotations.Mapper;
import org.apache.ibatis.annotations.Param;
import org.springframework.stereotype.Repository;

import java.time.LocalDate;
import java.util.List;

@Mapper
@Repository
public interface ActivityRecordMapper extends BaseMapper<ActivityRecord> {
    
    /**
     * 查询指定日期的任务活动统计
     */
    List<TaskActivitySummary> getDailyTaskActivitySummary(@Param("date") LocalDate date);
    
    /**
     * 查询指定周的任务活动统计
     */
    List<TaskActivitySummary> getWeeklyTaskActivitySummary(@Param("weekStart") LocalDate weekStart);
    
    /**
     * 任务活动统计内部类
     */
    class TaskActivitySummary {
        private Long taskId;
        private String taskTitle;
        private Integer totalMinutes;
        private String status;
        
        // Getters and Setters
        public Long getTaskId() { return taskId; }
        public void setTaskId(Long taskId) { this.taskId = taskId; }
        
        public String getTaskTitle() { return taskTitle; }
        public void setTaskTitle(String taskTitle) { this.taskTitle = taskTitle; }
        
        public Integer getTotalMinutes() { return totalMinutes; }
        public void setTotalMinutes(Integer totalMinutes) { this.totalMinutes = totalMinutes; }
        
        public String getStatus() { return status; }
        public void setStatus(String status) { this.status = status; }
    }
}