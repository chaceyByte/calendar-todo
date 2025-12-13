package com.taskcalendar.controller;

import com.taskcalendar.dto.ApiResponse;
import com.taskcalendar.dto.DailyReport;
import com.taskcalendar.dto.ManualActivityRequest;
import com.taskcalendar.dto.StartActivityRequest;
import com.taskcalendar.dto.WeeklyReport;
import com.taskcalendar.entity.ActivityRecord;
import com.taskcalendar.entity.Task;
import com.taskcalendar.service.ActivityService;
import com.taskcalendar.service.TaskService;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.format.annotation.DateTimeFormat;
import org.springframework.web.bind.annotation.*;

import javax.validation.Valid;
import java.time.LocalDate;
import java.util.List;
import java.util.Map;

@Slf4j
@RestController
@RequestMapping("/activities")
@RequiredArgsConstructor
public class ActivityController {
    
    private final ActivityService activityService;
    private final TaskService taskService;
    
    /**
     * 开始活动记录
     */
    @PostMapping("/start")
    public ApiResponse<ActivityRecord> startActivity(@Valid @RequestBody StartActivityRequest request) {
        try {
            ActivityRecord activity = activityService.startActivity(request);
            return ApiResponse.success("开始活动记录成功", activity);
        } catch (Exception e) {
            log.error("开始活动记录失败", e);
            return ApiResponse.error("开始活动记录失败: " + e.getMessage());
        }
    }
    
    /**
     * 结束任务当前活动
     */
    @PostMapping("/end/{taskId}")
    public ApiResponse<ActivityRecord> endActivity(@PathVariable Long taskId) {
        try {
            ActivityRecord activity = activityService.endActivity(taskId);
            return ApiResponse.success("结束活动记录成功", activity);
        } catch (Exception e) {
            log.error("结束活动记录失败", e);
            return ApiResponse.error("结束活动记录失败: " + e.getMessage());
        }
    }
    
    /**
     * 添加手动活动记录
     */
    @PostMapping("/manual")
    public ApiResponse<ActivityRecord> addManualActivity(@Valid @RequestBody ManualActivityRequest request) {
        try {
            ActivityRecord activity = activityService.addManualActivity(request);
            return ApiResponse.success("添加手动活动记录成功", activity);
        } catch (Exception e) {
            log.error("添加手动活动记录失败", e);
            return ApiResponse.error("添加手动活动记录失败: " + e.getMessage());
        }
    }
    
    /**
     * 获取任务活动记录
     */
    @GetMapping("/task/{taskId}")
    public ApiResponse<List<ActivityRecord>> getTaskActivities(@PathVariable Long taskId) {
        try {
            List<ActivityRecord> activities = activityService.getTaskActivities(taskId);
            return ApiResponse.success("获取任务活动记录成功", activities);
        } catch (Exception e) {
            log.error("获取任务活动记录失败", e);
            return ApiResponse.error("获取任务活动记录失败: " + e.getMessage());
        }
    }
    
    /**
     * 批量获取所有任务的活动记录
     */
    @GetMapping("/all")
    public ApiResponse<List<ActivityRecord>> getAllActivities() {
        try {
            List<ActivityRecord> activities = activityService.getAllActivities();
            return ApiResponse.success("获取所有活动记录成功", activities);
        } catch (Exception e) {
            log.error("获取所有活动记录失败", e);
            return ApiResponse.error("获取所有活动记录失败: " + e.getMessage());
        }
    }
    
    /**
     * 获取任务当前活动
     */
    @GetMapping("/current/{taskId}")
    public ApiResponse<ActivityRecord> getCurrentActivity(@PathVariable Long taskId) {
        try {
            ActivityRecord activity = activityService.getCurrentActivity(taskId);
            if (activity != null) {
                return ApiResponse.success("获取任务当前活动成功", activity);
            } else {
                return ApiResponse.error("任务没有进行中的活动");
            }
        } catch (Exception e) {
            log.error("获取任务当前活动失败", e);
            return ApiResponse.error("获取任务当前活动失败: " + e.getMessage());
        }
    }
    
    /**
     * 获取日报数据
     */
    @GetMapping("/report/daily")
    public ApiResponse<DailyReport> getDailyReport(
            @RequestParam @DateTimeFormat(iso = DateTimeFormat.ISO.DATE) LocalDate date) {
        try {
            DailyReport report = activityService.getDailyReport(date);
            return ApiResponse.success("获取日报数据成功", report);
        } catch (Exception e) {
            log.error("获取日报数据失败", e);
            return ApiResponse.error("获取日报数据失败: " + e.getMessage());
        }
    }
    
    /**
     * 获取周报数据
     */
    @GetMapping("/report/weekly")
    public ApiResponse<WeeklyReport> getWeeklyReport(
            @RequestParam @DateTimeFormat(iso = DateTimeFormat.ISO.DATE) LocalDate weekStart) {
        try {
            WeeklyReport report = activityService.getWeeklyReport(weekStart);
            return ApiResponse.success("获取周报数据成功", report);
        } catch (Exception e) {
            log.error("获取周报数据失败", e);
            return ApiResponse.error("获取周报数据失败: " + e.getMessage());
        }
    }
    
    // ========== 首页统计相关接口 ==========
    
    /**
     * 获取时间占用最长的前5个任务
     */
    @GetMapping("/stats/top-time-consuming")
    public ApiResponse<List<Map<String, Object>>> getTopTimeConsumingTasks() {
        try {
            List<Map<String, Object>> tasks = activityService.getTopTimeConsumingTasks(5);
            return ApiResponse.success("获取时间占用最长的任务成功", tasks);
        } catch (Exception e) {
            log.error("获取时间占用最长的任务失败", e);
            return ApiResponse.error("获取时间占用最长的任务失败: " + e.getMessage());
        }
    }
    
    /**
     * 获取最近14天每日处理的任务数量
     */
    @GetMapping("/stats/daily-processed")
    public ApiResponse<Map<String, Object>> getDailyProcessedTasks() {
        try {
            Map<String, Object> data = activityService.getDailyProcessedTasks(14);
            return ApiResponse.success("获取最近14天每日处理的任务数量成功", data);
        } catch (Exception e) {
            log.error("获取最近14天每日处理的任务数量失败", e);
            return ApiResponse.error("获取最近14天每日处理的任务数量失败: " + e.getMessage());
        }
    }
    
    /**
     * 获取最近14天每日创建的任务数量
     */
    @GetMapping("/stats/daily-created")
    public ApiResponse<Map<String, Object>> getDailyCreatedTasks() {
        try {
            Map<String, Object> data = activityService.getDailyCreatedTasks(14);
            return ApiResponse.success("获取最近14天每日创建的任务数量成功", data);
        } catch (Exception e) {
            log.error("获取最近14天每日创建的任务数量失败", e);
            return ApiResponse.error("获取最近14天每日创建的任务数量失败: " + e.getMessage());
        }
    }
    
    /**
     * 获取按标签分类的任务甘特图数据
     */
    @GetMapping("/stats/gantt-by-tags")
    public ApiResponse<Map<String, Object>> getGanttChartByTags() {
        try {
            Map<String, Object> data = taskService.getGanttChartByTags();
            return ApiResponse.success("获取按标签分类的甘特图数据成功", data);
        } catch (Exception e) {
            log.error("获取按标签分类的甘特图数据失败", e);
            return ApiResponse.error("获取按标签分类的甘特图数据失败: " + e.getMessage());
        }
    }
}