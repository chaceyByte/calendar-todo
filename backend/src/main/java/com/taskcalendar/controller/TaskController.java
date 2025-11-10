package com.taskcalendar.controller;

import com.taskcalendar.config.JwtUtil;
import com.taskcalendar.dto.ApiResponse;
import com.taskcalendar.dto.TaskDTO;
import com.taskcalendar.entity.Task;
import com.taskcalendar.service.TaskService;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/tasks")
@RequiredArgsConstructor
public class TaskController {
    
    private final TaskService taskService;
    private final JwtUtil jwtUtil;
    
    @GetMapping
    public ApiResponse<List<TaskDTO>> getTasks(@RequestHeader("Authorization") String token) {
        Long userId = getUserIdFromToken(token);
        if (userId == null) {
            return ApiResponse.error("认证失败");
        }
        
        List<TaskDTO> tasks = taskService.getTasksByUserId(userId);
        return ApiResponse.success(tasks);
    }
    
    @GetMapping("/status/{status}")
    public ApiResponse<List<TaskDTO>> getTasksByStatus(
            @RequestHeader("Authorization") String token,
            @PathVariable String status) {
        Long userId = getUserIdFromToken(token);
        if (userId == null) {
            return ApiResponse.error("认证失败");
        }
        
        List<TaskDTO> tasks = taskService.getTasksByStatus(userId, status);
        return ApiResponse.success(tasks);
    }
    
    @PostMapping
    public ApiResponse<Task> createTask(
            @RequestHeader("Authorization") String token,
            @RequestBody Task task) {
        Long userId = getUserIdFromToken(token);
        if (userId == null) {
            return ApiResponse.error("认证失败");
        }
        
        task.setUserId(userId);
        taskService.save(task);
        return ApiResponse.success("任务创建成功", task);
    }
    
    @PutMapping("/{id}")
    public ApiResponse<Task> updateTask(
            @RequestHeader("Authorization") String token,
            @PathVariable Long id,
            @RequestBody Task task) {
        Long userId = getUserIdFromToken(token);
        if (userId == null) {
            return ApiResponse.error("认证失败");
        }
        
        Task existingTask = taskService.getById(id);
        if (existingTask == null || !existingTask.getUserId().equals(userId)) {
            return ApiResponse.error("任务不存在或无权操作");
        }
        
        task.setId(id);
        task.setUserId(userId);
        taskService.updateById(task);
        return ApiResponse.success("任务更新成功", task);
    }
    
    @DeleteMapping("/{id}")
    public ApiResponse<String> deleteTask(
            @RequestHeader("Authorization") String token,
            @PathVariable Long id) {
        Long userId = getUserIdFromToken(token);
        if (userId == null) {
            return ApiResponse.error("认证失败");
        }
        
        Task task = taskService.getById(id);
        if (task == null || !task.getUserId().equals(userId)) {
            return ApiResponse.error("任务不存在或无权操作");
        }
        
        taskService.removeById(id);
        return ApiResponse.success("任务删除成功", null);
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