package com.taskcalendar.service;

import com.baomidou.mybatisplus.core.conditions.query.LambdaQueryWrapper;
import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.taskcalendar.dto.DailyReport;
import com.taskcalendar.dto.ManualActivityRequest;
import com.taskcalendar.dto.StartActivityRequest;
import com.taskcalendar.dto.WeeklyReport;
import com.taskcalendar.entity.ActivityRecord;
import com.taskcalendar.entity.Task;
import com.taskcalendar.mapper.ActivityRecordMapper;
import com.taskcalendar.mapper.TaskMapper;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import java.time.LocalDate;
import java.time.LocalDateTime;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;

@Slf4j
@Service
@RequiredArgsConstructor
public class ActivityService extends ServiceImpl<ActivityRecordMapper, ActivityRecord> {
    
    private final ActivityRecordMapper activityRecordMapper;
    private final TaskMapper taskMapper;
    
    /**
     * 开始活动记录
     */
    @Transactional
    public ActivityRecord startActivity(StartActivityRequest request) {
        // 检查任务是否存在
        if (taskMapper.selectById(request.getTaskId()) == null) {
            throw new IllegalArgumentException("任务不存在");
        }
        
        // 检查是否已有未结束的活动
        ActivityRecord currentActivity = findCurrentActivityByTaskId(request.getTaskId());
        if (currentActivity != null) {
            log.warn("任务 {} 已有未结束的活动，将先结束该活动", request.getTaskId());
            endActivity(request.getTaskId());
        }
        
        // 创建新的活动记录
        ActivityRecord activity = new ActivityRecord();
        activity.setTaskId(request.getTaskId());
        activity.setActivityType(request.getActivityType().name());
        activity.setStartTime(request.getStartTime() != null ? request.getStartTime() : LocalDateTime.now());
        activity.setDescription(request.getDescription());
        // createdAt字段将由MetaObjectHandler自动填充
        
        save(activity);
        log.info("开始任务 {} 的活动: {}", request.getTaskId(), request.getActivityType());
        
        return activity;
    }
    
    /**
     * 结束任务当前活动
     */
    @Transactional
    public ActivityRecord endActivity(Long taskId) {
        ActivityRecord currentActivity = findCurrentActivityByTaskId(taskId);
        if (currentActivity == null) {
            throw new IllegalStateException("任务没有进行中的活动");
        }
        
        LocalDateTime endTime = LocalDateTime.now();
        currentActivity.setEndTime(endTime);
        currentActivity.calculateDuration();
        
        updateById(currentActivity);
        log.info("结束任务 {} 的活动，持续时间: {} 分钟", taskId, currentActivity.getDurationMinutes());
        
        return currentActivity;
    }
    
    /**
     * 添加手动活动记录
     */
    @Transactional
    public ActivityRecord addManualActivity(ManualActivityRequest request) {
        // 检查任务是否存在
        if (taskMapper.selectById(request.getTaskId()) == null) {
            throw new IllegalArgumentException("任务不存在");
        }
        
        // 验证时间范围
        if (request.getStartTime().isAfter(request.getEndTime())) {
            throw new IllegalArgumentException("开始时间不能晚于结束时间");
        }
        
        // 创建手动活动记录
        ActivityRecord activity = new ActivityRecord();
        activity.setTaskId(request.getTaskId());
        activity.setActivityType(request.getActivityType().name());
        activity.setStartTime(request.getStartTime());
        activity.setEndTime(request.getEndTime());
        activity.setDescription(request.getDescription());
        // createdAt字段将由MetaObjectHandler自动填充
        activity.calculateDuration();
        
        save(activity);
        log.info("添加任务 {} 的手动活动: {} - {}", 
                request.getTaskId(), request.getStartTime(), request.getEndTime());
        
        return activity;
    }
    
    /**
     * 获取任务活动记录
     */
    public List<ActivityRecord> getTaskActivities(Long taskId) {
        return lambdaQuery()
                .eq(ActivityRecord::getTaskId, taskId)
                .orderByDesc(ActivityRecord::getStartTime)
                .list();
    }
    
    /**
     * 获取任务当前活动
     */
    public ActivityRecord getCurrentActivity(Long taskId) {
        return findCurrentActivityByTaskId(taskId);
    }
    
    /**
     * 查找任务当前活动（未结束的）
     */
    private ActivityRecord findCurrentActivityByTaskId(Long taskId) {
        return lambdaQuery()
                .eq(ActivityRecord::getTaskId, taskId)
                .isNull(ActivityRecord::getEndTime)
                .orderByDesc(ActivityRecord::getStartTime)
                .one();
    }
    
    /**
     * 获取日报数据
     */
    public DailyReport getDailyReport(LocalDate date) {
        // 查询指定日期的活动记录
        LocalDateTime startOfDay = date.atStartOfDay();
        LocalDateTime endOfDay = date.plusDays(1).atStartOfDay();
        
        List<ActivityRecord> activities = lambdaQuery()
                .ge(ActivityRecord::getStartTime, startOfDay)
                .lt(ActivityRecord::getStartTime, endOfDay)
                .orderByDesc(ActivityRecord::getStartTime)
                .list();
        
        // 获取任务活动摘要
        List<ActivityRecordMapper.TaskActivitySummary> summaries = 
                activityRecordMapper.getDailyTaskActivitySummary(date);
        
        DailyReport report = new DailyReport();
        report.setDate(date);
        
        // 计算总活动时间
        int totalTime = activities.stream()
                .filter(a -> a.getDurationMinutes() != null)
                .mapToInt(ActivityRecord::getDurationMinutes)
                .sum();
        report.setTotalTime(totalTime);
        
        // 计算完成任务数
        int completedTasks = (int) activities.stream()
                .filter(a -> "COMPLETED".equals(a.getActivityType()))
                .count();
        report.setCompletedTasks(completedTasks);
        
        // 计算活动任务数
        int activeTasks = (int) summaries.stream().count();
        report.setActiveTasks(activeTasks);
        
        // 构建任务活动详情
        List<DailyReport.TaskActivityDetail> taskDetails = summaries.stream()
                .map(this::convertToTaskActivityDetail)
                .collect(java.util.stream.Collectors.toList());
        report.setTaskActivities(taskDetails);
        
        return report;
    }
    
    /**
     * 获取周报数据
     */
    public WeeklyReport getWeeklyReport(LocalDate weekStart) {
        LocalDate weekEnd = weekStart.plusDays(6);
        
        // 查询指定周的活动记录
        LocalDateTime startOfWeek = weekStart.atStartOfDay();
        LocalDateTime endOfWeek = weekEnd.plusDays(1).atStartOfDay();
        
        List<ActivityRecord> activities = lambdaQuery()
                .ge(ActivityRecord::getStartTime, startOfWeek)
                .lt(ActivityRecord::getStartTime, endOfWeek)
                .orderByDesc(ActivityRecord::getStartTime)
                .list();
        
        List<ActivityRecordMapper.TaskActivitySummary> summaries = 
                activityRecordMapper.getWeeklyTaskActivitySummary(weekStart);
        
        WeeklyReport report = new WeeklyReport();
        report.setWeekStart(weekStart);
        report.setWeekEnd(weekEnd);
        
        // 计算总活动时间
        int totalTime = activities.stream()
                .filter(a -> a.getDurationMinutes() != null)
                .mapToInt(ActivityRecord::getDurationMinutes)
                .sum();
        report.setTotalTime(totalTime);
        
        // 计算完成任务数
        int completedTasks = (int) activities.stream()
                .filter(a -> "COMPLETED".equals(a.getActivityType()))
                .count();
        report.setCompletedTasks(completedTasks);
        
        // 构建每日摘要
        java.util.Map<LocalDate, WeeklyReport.DaySummary> dailySummaries = new java.util.HashMap<>();
        for (LocalDate date = weekStart; !date.isAfter(weekEnd); date = date.plusDays(1)) {
            LocalDate currentDate = date;
            List<ActivityRecord> dayActivities = activities.stream()
                    .filter(a -> a.getStartTime().toLocalDate().isEqual(currentDate))
                    .collect(java.util.stream.Collectors.toList());
            
            WeeklyReport.DaySummary daySummary = new WeeklyReport.DaySummary();
            daySummary.setDate(currentDate);
            
            int dayTotalTime = dayActivities.stream()
                    .filter(a -> a.getDurationMinutes() != null)
                    .mapToInt(ActivityRecord::getDurationMinutes)
                    .sum();
            daySummary.setTotalTime(dayTotalTime);
            
            int dayCompletedTasks = (int) dayActivities.stream()
                    .filter(a -> "COMPLETED".equals(a.getActivityType()))
                    .count();
            daySummary.setCompletedTasks(dayCompletedTasks);
            
            int dayActiveTasks = (int) dayActivities.stream()
                    .map(ActivityRecord::getTaskId)
                    .distinct()
                    .count();
            daySummary.setActiveTasks(dayActiveTasks);
            
            dailySummaries.put(currentDate, daySummary);
        }
        report.setDailySummaries(dailySummaries);
        
        // 构建任务活动详情
        List<WeeklyReport.TaskActivityDetail> taskDetails = summaries.stream()
                .map(this::convertToWeeklyTaskActivityDetail)
                .collect(java.util.stream.Collectors.toList());
        report.setTaskActivities(taskDetails);
        
        return report;
    }
    
    /**
     * 转换为日报任务活动详情
     */
    private DailyReport.TaskActivityDetail convertToTaskActivityDetail(ActivityRecordMapper.TaskActivitySummary summary) {
        DailyReport.TaskActivityDetail detail = new DailyReport.TaskActivityDetail();
        detail.setTaskId(summary.getTaskId());
        detail.setTaskTitle(summary.getTaskTitle());
        detail.setDuration(summary.getTotalMinutes());
        detail.setStatus(summary.getStatus());
        
        // 获取该任务的详细活动记录
        List<ActivityRecord> activities = getTaskActivities(summary.getTaskId());
        List<DailyReport.ActivityDetail> activityDetails = activities.stream()
                .map(this::convertToActivityDetail)
                .collect(java.util.stream.Collectors.toList());
        detail.setActivities(activityDetails);
        
        return detail;
    }
    
    /**
     * 转换为周报任务活动详情
     */
    private WeeklyReport.TaskActivityDetail convertToWeeklyTaskActivityDetail(ActivityRecordMapper.TaskActivitySummary summary) {
        WeeklyReport.TaskActivityDetail detail = new WeeklyReport.TaskActivityDetail();
        detail.setTaskId(summary.getTaskId());
        detail.setTaskTitle(summary.getTaskTitle());
        detail.setTotalDuration(summary.getTotalMinutes());
        detail.setStatus(summary.getStatus());
        
        // 构建每日活动时长
        java.util.Map<LocalDate, Integer> dailyDurations = new java.util.HashMap<>();
        List<ActivityRecord> activities = getTaskActivities(summary.getTaskId());
        
        for (ActivityRecord activity : activities) {
            if (activity.getDurationMinutes() != null) {
                LocalDate date = activity.getStartTime().toLocalDate();
                dailyDurations.merge(date, activity.getDurationMinutes(), Integer::sum);
            }
        }
        detail.setDailyDurations(dailyDurations);
        
        return detail;
    }
    
    /**
     * 转换为活动详情
     */
    private DailyReport.ActivityDetail convertToActivityDetail(ActivityRecord activity) {
        DailyReport.ActivityDetail detail = new DailyReport.ActivityDetail();
        detail.setId(activity.getId());
        detail.setActivityType(activity.getActivityType());
        detail.setDescription(activity.getDescription());
        detail.setStartTime(activity.getStartTime());
        detail.setEndTime(activity.getEndTime());
        detail.setDuration(activity.getDurationMinutes());
        return detail;
    }
    
    /**
     * 获取活动类型描述
     */
    private String getActivityTypeDescription(String activityType) {
        switch (activityType) {
            case "CREATED":
                return "创建";
            case "STARTED":
                return "开始";
            case "PAUSED":
                return "暂停";
            case "RESUMED":
                return "恢复";
            case "COMPLETED":
                return "完成";
            case "WORK":
                return "工作";
            case "MEETING":
                return "会议";
            case "STUDY":
                return "学习";
            case "OTHER":
                return "其他";
            default:
                return activityType;
        }
    }
    
    // ========== 首页统计相关方法 ==========
    
    /**
     * 获取时间占用最长的前N个任务
     */
    public List<Map<String, Object>> getTopTimeConsumingTasks(int limit) {
        // 查询所有有活动记录的任务，计算总时长
        List<ActivityRecord> allActivities = lambdaQuery()
                .isNotNull(ActivityRecord::getDurationMinutes)
                .orderByDesc(ActivityRecord::getDurationMinutes)
                .list();
        
        // 按任务ID分组并计算总时长
        Map<Long, Integer> taskTotalDuration = allActivities.stream()
                .collect(Collectors.groupingBy(
                        ActivityRecord::getTaskId,
                        Collectors.summingInt(ActivityRecord::getDurationMinutes)
                ));
        
        // 获取任务信息并按时长排序
        return taskTotalDuration.entrySet().stream()
                .sorted(Map.Entry.<Long, Integer>comparingByValue().reversed())
                .limit(limit)
                .map(entry -> {
                    Long taskId = entry.getKey();
                    Integer totalMinutes = entry.getValue();
                    Task task = taskMapper.selectById(taskId);
                    
                    Map<String, Object> taskData = new HashMap<>();
                    taskData.put("taskId", taskId);
                    taskData.put("title", task != null ? task.getTitle() : "未知任务");
                    taskData.put("totalMinutes", totalMinutes);
                    taskData.put("totalHours", String.format("%.1f", totalMinutes / 60.0));
                    taskData.put("status", task != null ? task.getStatus() : "未知");
                    
                    // 获取该任务的所有活动类型及各自时长
                    List<ActivityRecord> taskActivities = allActivities.stream()
                            .filter(a -> a.getTaskId().equals(taskId))
                            .collect(Collectors.toList());
                    
                    Map<String, Integer> activityTypeDurations = taskActivities.stream()
                            .collect(Collectors.groupingBy(
                                    ActivityRecord::getActivityType,
                                    Collectors.summingInt(ActivityRecord::getDurationMinutes)
                            ));
                    
                    Map<String, String> formattedDurations = new HashMap<>();
                    activityTypeDurations.forEach((type, minutes) -> {
                        formattedDurations.put(getActivityTypeDescription(type), 
                                String.format("%.1f 小时", minutes / 60.0));
                    });
                    
                    taskData.put("activitiesByType", formattedDurations);
                    return taskData;
                })
                .collect(Collectors.toList());
    }
    
    /**
     * 获取最近N天每日处理的任务数量
     */
    public Map<String, Object> getDailyProcessedTasks(int days) {
        LocalDate endDate = LocalDate.now();
        LocalDate startDate = endDate.minusDays(days - 1);
        
        List<LocalDate> dates = new ArrayList<>();
        for (LocalDate date = startDate; !date.isAfter(endDate); date = date.plusDays(1)) {
            dates.add(date);
        }
        
        // 查询这段时间内的活动记录
        List<ActivityRecord> activities = lambdaQuery()
                .ge(ActivityRecord::getStartTime, startDate.atStartOfDay())
                .lt(ActivityRecord::getStartTime, endDate.plusDays(1).atStartOfDay())
                .list();
        
        // 按日期分组统计处理任务数
        Map<LocalDate, Long> dailyProcessedCount = dates.stream()
                .collect(Collectors.toMap(
                        date -> date,
                        date -> activities.stream()
                                .filter(a -> a.getStartTime().toLocalDate().equals(date))
                                .map(ActivityRecord::getTaskId)
                                .distinct()
                                .count()
                ));
        
        Map<String, Object> result = new HashMap<>();
        result.put("dates", dates.stream()
                .map(date -> date.toString())
                .collect(Collectors.toList()));
        result.put("processedCounts", dailyProcessedCount.entrySet().stream()
                .map(entry -> entry.getValue())
                .collect(Collectors.toList()));
        result.put("totalProcessed", dailyProcessedCount.values().stream()
                .mapToLong(count -> count).sum());
        
        return result;
    }
    
    /**
     * 获取最近N天每日创建的任务数量
     */
    public Map<String, Object> getDailyCreatedTasks(int days) {
        LocalDate endDate = LocalDate.now();
        LocalDate startDate = endDate.minusDays(days - 1);
        
        List<LocalDate> dates = new ArrayList<>();
        for (LocalDate date = startDate; !date.isAfter(endDate); date = date.plusDays(1)) {
            dates.add(date);
        }
        
        // 查询这段时间内创建的任务
        List<Task> tasks = taskMapper.selectList(
                new com.baomidou.mybatisplus.core.conditions.query.LambdaQueryWrapper<Task>()
                        .ge(Task::getCreatedAt, startDate.atStartOfDay())
                        .lt(Task::getCreatedAt, endDate.plusDays(1).atStartOfDay())
        );
        
        // 按日期分组统计创建任务数
        Map<LocalDate, Long> dailyCreatedCount = dates.stream()
                .collect(Collectors.toMap(
                        date -> date,
                        date -> tasks.stream()
                                .filter(t -> t.getCreatedAt().toLocalDate().equals(date))
                                .count()
                ));
        
        Map<String, Object> result = new HashMap<>();
        result.put("dates", dates.stream()
                .map(date -> date.toString())
                .collect(Collectors.toList()));
        result.put("createdCounts", dailyCreatedCount.entrySet().stream()
                .map(entry -> entry.getValue())
                .collect(Collectors.toList()));
        result.put("totalCreated", dailyCreatedCount.values().stream()
                .mapToLong(count -> count).sum());
        
        return result;
    }
    
    /**
     * 获取所有活动记录
     */
    public List<ActivityRecord> getAllActivities() {
        return lambdaQuery()
                .orderByDesc(ActivityRecord::getStartTime)
                .list();
    }
}