package com.taskcalendar.service;

import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.taskcalendar.dto.TaskDTO;
import com.taskcalendar.entity.Task;
import com.taskcalendar.mapper.TaskMapper;
import org.springframework.stereotype.Service;

import java.util.List;
import java.util.stream.Collectors;

@Service
public class TaskService extends ServiceImpl<TaskMapper, Task> {
    
    public List<TaskDTO> getTasksByUserId(Long userId) {
        List<Task> tasks = lambdaQuery()
                .eq(Task::getUserId, userId)
                .orderByDesc(Task::getCreatedAt)
                .list();
        
        return tasks.stream().map(this::convertToDTO).collect(Collectors.toList());
    }
    
    public List<TaskDTO> getAllTasks() {
        List<Task> tasks = lambdaQuery()
                .orderByDesc(Task::getCreatedAt)
                .list();
        
        return tasks.stream().map(this::convertToDTO).collect(Collectors.toList());
    }
    
    public List<TaskDTO> getTasksByStatus(Long userId, String status) {
        List<Task> tasks = lambdaQuery()
                .eq(Task::getUserId, userId)
                .eq(Task::getStatus, status)
                .orderByDesc(Task::getCreatedAt)
                .list();
        
        return tasks.stream().map(this::convertToDTO).collect(Collectors.toList());
    }
    
    public List<TaskDTO> getTasksByStatus(String status) {
        List<Task> tasks = lambdaQuery()
                .eq(Task::getStatus, status)
                .orderByDesc(Task::getCreatedAt)
                .list();
        
        return tasks.stream().map(this::convertToDTO).collect(Collectors.toList());
    }
    
    private TaskDTO convertToDTO(Task task) {
        TaskDTO dto = new TaskDTO();
        dto.setId(task.getId());
        dto.setTitle(task.getTitle());
        dto.setDescription(task.getDescription());
        dto.setStatus(task.getStatus());
        dto.setProgress(task.getProgress());
        dto.setPriority(task.getPriority());
        dto.setStartDate(task.getStartDate());
        dto.setEndDate(task.getEndDate());
        dto.setCreatedAt(task.getCreatedAt());
        dto.setUpdatedAt(task.getUpdatedAt());
        // 标签数据需要从关联表查询，这里先设为空
        return dto;
    }
}