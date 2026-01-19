package com.taskcalendar.controller;

import com.baomidou.mybatisplus.core.metadata.IPage;
import com.taskcalendar.context.CurrentUser;
import com.taskcalendar.dto.ApiResponse;
import com.taskcalendar.dto.CreateTaskRequest;
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
@RequestMapping("/api/tasks")
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
    public ApiResponse<Task> createTask(@RequestBody CreateTaskRequest task) {
        // userId、createdAt、updatedAt字段将由MetaObjectHandler自动填充
        Task taskResult = taskService.saveNewTask(task);
        return ApiResponse.success("任务创建成功", taskResult);
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
        // updatedAt字段将由MetaObjectHandler自动填充
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
        if ("completed".equals(task.getStatus())) {
            return ApiResponse.error("已完成任务,不可暂存");
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

        // 如果任务状态是paused，则恢复任务
        if ("paused".equals(task.getStatus())) {
            boolean result = taskService.resumeTask(id);
            if (result) {
                return ApiResponse.success("任务已从暂存队列移除并恢复", null);
            } else {
                return ApiResponse.error("从暂存队列移除失败");
            }
        }

        return ApiResponse.success("任务已从暂存队列移除", null);
    }

    @GetMapping("/staging")
    public ApiResponse<List<TaskDTO>> getStagingTasks() {
        // 获取所有暂存状态的任务
        List<TaskDTO> tasks = taskService.getTasksByStatus("paused");
        return ApiResponse.success(tasks);
    }

    // 归档任务相关接口
    @GetMapping("/archived")
    public ApiResponse<IPage<TaskDTO>> getArchivedTasks(
            @RequestParam(defaultValue = "1") int page,
            @RequestParam(defaultValue = "20") int size,
            @RequestParam(required = false) String keyword) {

        IPage<TaskDTO> tasks = taskService.getArchivedTasks(page, size, keyword);
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
    public ApiResponse<String> undoLastActions(@PathVariable Long id, Integer depth) {
        try {
            // 默认撤销深度为5个最近操作
            boolean result = taskService.undoLastActivities(id, depth == null ? 1 : depth);
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

    // 四象限任务管理接口
    @GetMapping("/quadrant")
    public ApiResponse<List<TaskDTO>> getQuadrantTasks() {
        try {
            List<TaskDTO> tasks = taskService.getQuadrantTasks();
            return ApiResponse.success(tasks);
        } catch (Exception e) {
            return ApiResponse.error("获取四象限任务失败: " + e.getMessage());
        }
    }

    // 更新任务重要紧急程度
    @PutMapping("/{id}/priority-urgency")
    public ApiResponse<Task> updatePriorityUrgency(@PathVariable Long id, 
                                                   @RequestParam String priority, 
                                                   @RequestParam String urgency) {
        try {
            // 验证优先级和紧急程度的有效性
            if (!isValidPriority(priority) || !isValidUrgency(urgency)) {
                return ApiResponse.error("无效的重要或紧急程度值");
            }

            Task task = taskService.getById(id);
            if (task == null) {
                return ApiResponse.error("任务不存在");
            }

            task.setPriority(priority);
            task.setUrgency(urgency);
            task.setUpdatedAt(LocalDateTime.now());
            
            taskService.updateById(task);
            return ApiResponse.success("任务重要紧急程度更新成功", task);
        } catch (Exception e) {
            return ApiResponse.error("更新任务重要紧急程度失败: " + e.getMessage());
        }
    }

    // 归档任务
    @PostMapping("/{id}/archive")
    public ApiResponse<Task> archiveTask(@PathVariable Long id) {
        try {
            Task task = taskService.getById(id);
            if (task == null) {
                return ApiResponse.error("任务不存在");
            }

            // 只有已完成的任务可以归档
            if (!"completed".equals(task.getStatus())) {
                return ApiResponse.error("只有已完成的任务可以归档");
            }

            task.setArchivedAt(LocalDateTime.now());
            task.setUpdatedAt(LocalDateTime.now());
            
            taskService.updateById(task);
            return ApiResponse.success("任务归档成功", task);
        } catch (Exception e) {
            return ApiResponse.error("任务归档失败: " + e.getMessage());
        }
    }

    // 取消归档任务
    @PostMapping("/{id}/unarchive")
    public ApiResponse<Task> unarchiveTask(@PathVariable Long id) {
        try {
            Task task = taskService.getById(id);
            if (task == null) {
                return ApiResponse.error("任务不存在");
            }

            task.setArchivedAt(null);
            task.setUpdatedAt(LocalDateTime.now());
            
            taskService.updateById(task);
            return ApiResponse.success("任务取消归档成功", task);
        } catch (Exception e) {
            return ApiResponse.error("任务取消归档失败: " + e.getMessage());
        }
    }

    // 验证优先级有效性
    private boolean isValidPriority(String priority) {
        return priority != null && (priority.equals("-high") || priority.equals("-middle") || 
               priority.equals("-low") || priority.equals("low") || 
               priority.equals("middle") || priority.equals("high"));
    }

    // 验证紧急程度有效性
    private boolean isValidUrgency(String urgency) {
        return urgency != null && (urgency.equals("-high") || urgency.equals("-middle") || 
               urgency.equals("-low") || urgency.equals("low") || 
               urgency.equals("middle") || urgency.equals("high"));
    }

}