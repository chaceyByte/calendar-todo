package com.taskcalendar.controller;

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
    
    @GetMapping
    public ApiResponse<List<TaskDTO>> getTasks() {
        List<TaskDTO> tasks = taskService.getAllTasks();
        return ApiResponse.success(tasks);
    }
    
    @GetMapping("/status/{status}")
    public ApiResponse<List<TaskDTO>> getTasksByStatus(@PathVariable String status) {
        List<TaskDTO> tasks = taskService.getTasksByStatus(status);
        return ApiResponse.success(tasks);
    }
    
    @PostMapping
    public ApiResponse<Task> createTask(@RequestBody Task task) {
        task.setUserId(1L); // 暂时使用默认用户ID
        taskService.save(task);
        return ApiResponse.success("任务创建成功", task);
    }
    
    @PutMapping("/{id}")
    public ApiResponse<Task> updateTask(@PathVariable Long id, @RequestBody Task task) {
        Task existingTask = taskService.getById(id);
        if (existingTask == null) {
            return ApiResponse.error("任务不存在");
        }
        
        task.setId(id);
        task.setUserId(existingTask.getUserId());
        taskService.updateById(task);
        return ApiResponse.success("任务更新成功", task);
    }
    
    @DeleteMapping("/{id}")
    public ApiResponse<String> deleteTask(@PathVariable Long id) {
        Task task = taskService.getById(id);
        if (task == null) {
            return ApiResponse.error("任务不存在");
        }
        
        taskService.removeById(id);
        return ApiResponse.success("任务删除成功", null);
    }
    
    // 暂存任务相关接口
    @PostMapping("/{id}/staging")
    public ApiResponse<String> addToStaging(@PathVariable Long id) {
        Task task = taskService.getById(id);
        if (task == null) {
            return ApiResponse.error("任务不存在");
        }
        
        // 这里可以添加暂存逻辑，比如更新状态为staging
        // 目前先简单返回成功
        return ApiResponse.success("任务已添加到暂存队列", null);
    }
    
    @DeleteMapping("/{id}/staging")
    public ApiResponse<String> removeFromStaging(@PathVariable Long id) {
        Task task = taskService.getById(id);
        if (task == null) {
            return ApiResponse.error("任务不存在");
        }
        
        // 这里可以添加从暂存队列移除的逻辑
        return ApiResponse.success("任务已从暂存队列移除", null);
    }
    
    @GetMapping("/staging")
    public ApiResponse<List<TaskDTO>> getStagingTasks() {
        // 获取所有暂存状态的任务
        List<TaskDTO> tasks = taskService.getTasksByStatus("paused");
        return ApiResponse.success(tasks);
    }
    
    // 暂停任务
    @PostMapping("/{id}/pause")
    public ApiResponse<Task> pauseTask(@PathVariable Long id) {
        Task task = taskService.getById(id);
        if (task == null) {
            return ApiResponse.error("任务不存在");
        }
        
        // 更新任务状态为暂停
        task.setStatus("paused");
        taskService.updateById(task);
        return ApiResponse.success("任务已暂停", task);
    }
    

}