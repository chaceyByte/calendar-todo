package com.taskcalendar.controller;

import com.taskcalendar.dto.ApiResponse;
import com.taskcalendar.dto.DailyReport;
import com.taskcalendar.dto.ManualActivityRequest;
import com.taskcalendar.dto.StartActivityRequest;
import com.taskcalendar.dto.WeeklyReport;
import com.taskcalendar.entity.ActivityRecord;
import com.taskcalendar.service.ActivityService;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.format.annotation.DateTimeFormat;
import org.springframework.web.bind.annotation.*;

import javax.validation.Valid;
import java.time.LocalDate;
import java.util.List;

@Slf4j
@RestController
@RequestMapping("/api/activities")
@RequiredArgsConstructor
public class ActivityController {
    
    private final ActivityService activityService;
    
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
}