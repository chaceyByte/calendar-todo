package com.taskcalendar.controller;

import com.taskcalendar.context.CurrentUser;
import com.taskcalendar.dto.ApiResponse;
import com.taskcalendar.dto.TaskDTO;
import com.taskcalendar.dto.UpdateTagsRequest;
import com.taskcalendar.entity.Task;
import com.taskcalendar.service.TagService;
import com.taskcalendar.service.TaskService;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.*;

import java.time.LocalDateTime;
import java.util.List;

@RestController
@RequestMapping("/tasks")
@RequiredArgsConstructor
public class TaskController {

    private final TaskService taskService;
    private final TagService tagService;

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
        task.setCreatedAt(LocalDateTime.now());
        task.setUpdatedAt(LocalDateTime.now());
        taskService.save(task);
        return ApiResponse.success("任务创建成功", task);
    }

    @PutMapping("/{id}")
    public ApiResponse<Task> updateTask(@PathVariable Long id, @RequestBody Task task) {
        Task existingTask = taskService.getById(id);
        if (existingTask == null) {
            return ApiResponse.error("任务不存在");
        }
        // 校验,不能从进行中修改为计划中状态
        if ("in-progress".equals(existingTask.getStatus()) && "planning".equals(task.getStatus())) {
            return ApiResponse.error("不能从进行中修改为计划中状态");
        }
        // 校验, 已完成任务,不可更改状态
        if ("completed".equals(existingTask.getStatus())) {
            return ApiResponse.error("已完成任务,不可更改状态");
        }

        task.setId(id);
        task.setUserId(existingTask.getUserId());
        task.setUpdatedAt(LocalDateTime.now());
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
        try {
            boolean result = taskService.pauseTask(id);
            if (result) {
                Task task = taskService.getById(id);
                return ApiResponse.success("任务已暂停", task);
            } else {
                return ApiResponse.error("暂停任务失败");
            }
        } catch (Exception e) {
            return ApiResponse.error("暂停任务失败: " + e.getMessage());
        }
    }

    // 恢复任务
    @PostMapping("/{id}/resume")
    public ApiResponse<Task> resumeTask(@PathVariable Long id) {
        try {
            boolean result = taskService.resumeTask(id);
            if (result) {
                Task task = taskService.getById(id);
                return ApiResponse.success("任务已恢复", task);
            } else {
                return ApiResponse.error("恢复任务失败");
            }
        } catch (Exception e) {
            return ApiResponse.error("恢复任务失败: " + e.getMessage());
        }
    }

    // 更新任务标签
    @PutMapping("/{id}/tags")
    public ApiResponse<TaskDTO> updateTaskTags(@PathVariable Long id, @RequestBody UpdateTagsRequest request) {
        Long userId = CurrentUser.getUserId();

        Task task = taskService.getById(id);
        if (task == null) {
            return ApiResponse.error("任务不存在");
        }

        if (!task.getUserId().equals(userId)) {
            return ApiResponse.error("无权操作此任务");
        }

        try {
            TaskDTO updatedTask = taskService.updateTaskTags(id, request.getTagIds(), userId);
            return ApiResponse.success("任务标签更新成功", updatedTask);
        } catch (Exception e) {
            return ApiResponse.error("更新任务标签失败: " + e.getMessage());
        }
    }

    // 从任务中移除标签
    @DeleteMapping("/{id}/tags/{tagName}")
    public ApiResponse<TaskDTO> removeTagFromTask(@PathVariable Long id, @PathVariable String tagName) {
        Long userId = CurrentUser.getUserId();

        Task task = taskService.getById(id);
        if (task == null) {
            return ApiResponse.error("任务不存在");
        }

        if (!task.getUserId().equals(userId)) {
            return ApiResponse.error("无权操作此任务");
        }

        try {
            TaskDTO updatedTask = taskService.removeTagFromTask(id, tagName, userId);
            return ApiResponse.success("标签已移除", updatedTask);
        } catch (Exception e) {
            return ApiResponse.error("移除标签失败: " + e.getMessage());
        }
    }

    // 撤销任务的最近操作
    @PostMapping("/{id}/undo")
    public ApiResponse<String> undoLastActions(@PathVariable Long id) {
        try {
            // 默认撤销深度为5个最近操作
            boolean result = taskService.undoLastActivities(id, 5);
            if (result) {
                return ApiResponse.success("成功撤销最近操作");
            } else {
                return ApiResponse.error("撤销操作失败");
            }
        } catch (Exception e) {
            return ApiResponse.error("撤销操作失败: " + e.getMessage());
        }
    }

    // 撤销指定数量的最近操作
    @PostMapping("/{id}/undo/{depth}")
    public ApiResponse<String> undoLastActions(@PathVariable Long id, @PathVariable int depth) {
        try {
            if (depth <= 0) {
                return ApiResponse.error("撤销深度必须大于0");
            }

            // 限制最大撤销深度，防止性能问题
            int maxDepth = Math.min(depth, 20);

            boolean result = taskService.undoLastActivities(id, maxDepth);
            if (result) {
                return ApiResponse.success("成功撤销最近" + maxDepth + "个操作");
            } else {
                return ApiResponse.error("撤销操作失败");
            }
        } catch (Exception e) {
            return ApiResponse.error("撤销操作失败: " + e.getMessage());
        }
    }


}