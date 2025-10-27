package com.taskcalendar.controller;

import com.taskcalendar.config.JwtUtil;
import com.taskcalendar.dto.ApiResponse;
import com.taskcalendar.dto.TaskDTO;
import com.taskcalendar.service.TaskService;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.*;

import java.time.LocalDate;
import java.time.YearMonth;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

@RestController
@RequestMapping("/calendar")
@RequiredArgsConstructor
public class CalendarController {
    
    private final TaskService taskService;
    private final JwtUtil jwtUtil;
    
    @GetMapping("/month/{year}/{month}")
    public ApiResponse<Map<String, Object>> getMonthData(
            @RequestHeader("Authorization") String token,
            @PathVariable int year,
            @PathVariable int month) {
        Long userId = getUserIdFromToken(token);
        if (userId == null) {
            return ApiResponse.error("认证失败");
        }
        
        // 获取该月的所有任务
        List<TaskDTO> tasks = taskService.getTasksByUserId(userId);
        
        // 按日期分组任务
        Map<String, List<TaskDTO>> tasksByDate = new HashMap<>();
        for (TaskDTO task : tasks) {
            if (task.getStartDate() != null && task.getEndDate() != null) {
                LocalDate start = task.getStartDate().toLocalDate();
                LocalDate end = task.getEndDate().toLocalDate();
                
                // 如果任务跨越该月，则包含在结果中
                YearMonth targetMonth = YearMonth.of(year, month);
                if (!start.isAfter(targetMonth.atEndOfMonth()) && !end.isBefore(targetMonth.atDay(1))) {
                    // 简化处理：将任务添加到所有相关日期
                    LocalDate current = start.isBefore(targetMonth.atDay(1)) ? 
                            targetMonth.atDay(1) : start;
                    LocalDate lastDay = end.isAfter(targetMonth.atEndOfMonth()) ? 
                            targetMonth.atEndOfMonth() : end;
                    
                    while (!current.isAfter(lastDay)) {
                        String dateKey = current.toString();
                        tasksByDate.computeIfAbsent(dateKey, k -> new java.util.ArrayList<>()).add(task);
                        current = current.plusDays(1);
                    }
                }
            }
        }
        
        Map<String, Object> result = new HashMap<>();
        result.put("year", year);
        result.put("month", month);
        result.put("tasksByDate", tasksByDate);
        
        return ApiResponse.success(result);
    }
    
    @GetMapping("/daily-report/{date}")
    public ApiResponse<Map<String, Object>> generateDailyReport(
            @RequestHeader("Authorization") String token,
            @PathVariable String date) {
        Long userId = getUserIdFromToken(token);
        if (userId == null) {
            return ApiResponse.error("认证失败");
        }
        
        LocalDate reportDate = LocalDate.parse(date);
        
        // 模拟日报数据
        Map<String, Object> report = new HashMap<>();
        report.put("date", date);
        report.put("completedTasks", 5);
        report.put("inProgressTasks", 3);
        report.put("totalTime", "8小时");
        report.put("summary", "今日工作进展顺利，完成了主要开发任务。");
        
        return ApiResponse.success(report);
    }
    
    @GetMapping("/weekly-report/{year}/{week}")
    public ApiResponse<Map<String, Object>> generateWeeklyReport(
            @RequestHeader("Authorization") String token,
            @PathVariable int year,
            @PathVariable int week) {
        Long userId = getUserIdFromToken(token);
        if (userId == null) {
            return ApiResponse.error("认证失败");
        }
        
        // 模拟周报数据
        Map<String, Object> report = new HashMap<>();
        report.put("year", year);
        report.put("week", week);
        report.put("completedTasks", 25);
        report.put("inProgressTasks", 8);
        report.put("totalTime", "40小时");
        report.put("summary", "本周项目进展良好，完成了主要功能开发。");
        
        return ApiResponse.success(report);
    }
    
    private Long getUserIdFromToken(String token) {
        if (token != null && token.startsWith("Bearer ")) {
            token = token.substring(7);
            if (jwtUtil.validateToken(token)) {
                String username = jwtUtil.getUsernameFromToken(token);
                // 这里应该查询数据库获取用户ID，暂时返回1
                return 1L;
            }
        }
        return null;
    }
}