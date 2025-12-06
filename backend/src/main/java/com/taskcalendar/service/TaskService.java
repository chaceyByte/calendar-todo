package com.taskcalendar.service;

import com.baomidou.mybatisplus.core.toolkit.Wrappers;
import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.baomidou.mybatisplus.extension.toolkit.Db;
import com.taskcalendar.dto.TaskDTO;
import com.taskcalendar.entity.ActivityRecord;
import com.taskcalendar.entity.Tag;
import com.taskcalendar.entity.Task;
import com.taskcalendar.entity.TaskTag;
import com.taskcalendar.enums.ActivityType;
import com.taskcalendar.mapper.ActivityRecordMapper;
import com.taskcalendar.mapper.TagMapper;
import com.taskcalendar.mapper.TaskMapper;
import com.taskcalendar.mapper.TaskTagMapper;
import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import java.time.LocalDateTime;
import java.util.*;
import java.util.stream.Collectors;

@Slf4j
@Service
public class TaskService extends ServiceImpl<TaskMapper, Task> {

    private final TaskTagMapper taskTagMapper;
    private final TagMapper tagMapper;
    private final ActivityRecordMapper activityRecordMapper;

    public TaskService(TaskTagMapper taskTagMapper, TagMapper tagMapper, ActivityRecordMapper activityRecordMapper) {
        this.taskTagMapper = taskTagMapper;
        this.tagMapper = tagMapper;
        this.activityRecordMapper = activityRecordMapper;
    }

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

        // 查询任务关联的标签
        List<TaskTag> taskTags = taskTagMapper.selectList(
                Wrappers.<TaskTag>lambdaQuery().eq(TaskTag::getTaskId, task.getId())
        );

        if (!taskTags.isEmpty()) {
            List<Long> tagIds = taskTags.stream().map(TaskTag::getTagId).collect(Collectors.toList());
            List<Tag> tags = tagMapper.selectBatchIds(tagIds);
            List<String> tagNames = tags.stream().map(Tag::getName).collect(Collectors.toList());
            dto.setTags(tagNames);
        }

        return dto;
    }

    // 更新任务标签
    public TaskDTO updateTaskTags(Long taskId, List<Long> tagIds, Long userId) {
        // 验证任务是否存在且属于当前用户
        Task task = getById(taskId);
        if (task == null || !task.getUserId().equals(userId)) {
            throw new RuntimeException("任务不存在或无权操作");
        }

        // 删除现有的任务标签关联
        taskTagMapper.delete(
                Wrappers.<TaskTag>lambdaQuery().eq(TaskTag::getTaskId, taskId)
        );

        // 如果有新标签，创建关联
        if (tagIds != null && !tagIds.isEmpty()) {
            for (Long tagId : tagIds) {
                // 验证标签是否存在且属于当前用户
                Tag tag = tagMapper.selectById(tagId);
                if (tag == null || !tag.getUserId().equals(userId)) {
                    throw new RuntimeException("标签不存在或无权操作");
                }

                // 创建任务标签关联
                TaskTag taskTag = new TaskTag();
                taskTag.setTaskId(taskId);
                taskTag.setTagId(tagId);
                taskTagMapper.insert(taskTag);
            }
        }

        // 返回更新后的任务DTO
        return convertToDTO(getById(taskId));
    }

    // 从任务中移除标签
    public TaskDTO removeTagFromTask(Long taskId, String tagName, Long userId) {
        // 验证任务是否存在且属于当前用户
        Task task = getById(taskId);
        if (task == null || !task.getUserId().equals(userId)) {
            throw new RuntimeException("任务不存在或无权操作");
        }

        // 查找标签
        Tag tag = tagMapper.selectOne(
                Wrappers.<Tag>lambdaQuery()
                        .eq(Tag::getName, tagName)
                        .eq(Tag::getUserId, userId)
        );

        if (tag == null) {
            throw new RuntimeException("标签不存在");
        }

        // 删除任务标签关联
        taskTagMapper.delete(
                Wrappers.<TaskTag>lambdaQuery()
                        .eq(TaskTag::getTaskId, taskId)
                        .eq(TaskTag::getTagId, tag.getId())
        );

        // 返回更新后的任务DTO
        return convertToDTO(getById(taskId));
    }

    /**
     * 创建任务时记录活动
     */
    @Transactional
    @Override
    public boolean save(Task entity) {
        boolean result = super.save(entity);
        if (result) {
            // 记录创建活动
            recordActivity(entity.getId(), ActivityType.CREATED, "任务创建");
        }
        return result;
    }

    /**
     * 更新任务状态时记录活动
     */
    @Transactional
    @Override
    public boolean updateById(Task entity) {
        // 获取更新前的任务状态
        Task oldTask = getById(entity.getId());
        boolean result = super.updateById(entity);

        if (result && oldTask != null && !oldTask.getStatus().equals(entity.getStatus())) {
            // 状态发生变化，记录活动
            ActivityType activityType = getActivityTypeFromStatus(entity.getStatus());
            String description = String.format("任务状态从 %s 变更为 %s",
                    oldTask.getStatus(), entity.getStatus());

            // 结束当前活动
            endCurrentActivity(entity.getId());

            // 开始新活动
            recordActivity(entity.getId(), activityType, description);
        }

        return result;
    }

    /**
     * 暂停任务
     */
    @Transactional
    public boolean pauseTask(Long taskId) {
        Task task = getById(taskId);
        if (task == null) {
            throw new RuntimeException("任务不存在");
        }

        // 更新任务状态为暂停
        task.setStatus("paused");
        boolean result = updateById(task);

        if (result) {
            // 结束当前活动
            endCurrentActivity(taskId);
            // 记录暂停活动
            recordActivity(taskId, ActivityType.PAUSED, "任务暂停");
        }

        return result;
    }

    /**
     * 恢复任务
     */
    @Transactional
    public boolean resumeTask(Long taskId) {
        Task task = getById(taskId);
        if (task == null) {
            throw new RuntimeException("任务不存在");
        }

        // 更新任务状态为进行中
        task.setStatus("in-progress");
        boolean result = updateById(task);

        if (result) {
            // 记录恢复活动
            recordActivity(taskId, ActivityType.RESUMED, "任务恢复");
        }

        return result;
    }

    /**
     * 记录活动
     */
    private void recordActivity(Long taskId, ActivityType activityType, String description) {
        ActivityRecord activity = new ActivityRecord();
        activity.setTaskId(taskId);
        activity.setActivityType(activityType.name());
        activity.setStartTime(LocalDateTime.now());
        activity.setDescription(description);
        activity.setCreatedAt(LocalDateTime.now());

        activityRecordMapper.insert(activity);
        log.info("记录任务活动: taskId={}, type={}, description={}",
                taskId, activityType, description);
    }

    /**
     * 结束当前活动
     */
    private void endCurrentActivity(Long taskId) {
        ActivityRecord currentActivity = Db.lambdaQuery(ActivityRecord.class)
                .eq(ActivityRecord::getTaskId, taskId)
                .isNull(ActivityRecord::getEndTime)
                .orderByDesc(ActivityRecord::getStartTime)
                .last("LIMIT 1")
                .one();

        if (currentActivity != null) {
            currentActivity.setEndTime(LocalDateTime.now());
            currentActivity.calculateDuration();
            activityRecordMapper.updateById(currentActivity);
            log.info("结束任务当前活动: taskId={}", taskId);
        }
    }

    /**
     * 根据状态获取活动类型
     */
    private ActivityType getActivityTypeFromStatus(String status) {
        switch (status) {
            case "planning":
                return ActivityType.STARTED;
            case "in-progress":
                return ActivityType.STARTED;
            case "completed":
                return ActivityType.COMPLETED;
            case "paused":
                return ActivityType.PAUSED;
            default:
                return ActivityType.OTHER;
        }
    }
    
    /**
     * 获取按标签分类的任务甘特图数据
     */
    public Map<String, Object> getGanttChartByTags() {
        // 获取所有标签及其关联的任务
        List<Tag> allTags = tagMapper.selectList(null);
        
        Map<String, Object> result = new HashMap<>();
        List<Map<String, Object>> tagGroups = new ArrayList<>();
        
        for (Tag tag : allTags) {
            // 获取该标签关联的所有任务
            List<TaskTag> taskTags = taskTagMapper.selectList(
                    Wrappers.<TaskTag>lambdaQuery().eq(TaskTag::getTagId, tag.getId())
            );
            
            if (taskTags.isEmpty()) {
                continue;
            }
            
            List<Long> taskIds = taskTags.stream()
                    .map(TaskTag::getTaskId)
                    .collect(Collectors.toList());
            
            List<Task> tasks = listByIds(taskIds);
            
            // 为每个任务构建甘特图数据
            List<Map<String, Object>> taskData = new ArrayList<>();
            for (Task task : tasks) {
                Map<String, Object> taskInfo = new HashMap<>();
                taskInfo.put("id", task.getId());
                taskInfo.put("title", task.getTitle());
                taskInfo.put("status", task.getStatus());
                taskInfo.put("progress", task.getProgress());
                taskInfo.put("priority", task.getPriority());
                
                // 获取任务的活动记录
                List<ActivityRecord> activities = activityRecordMapper.selectList(
                        Wrappers.<ActivityRecord>lambdaQuery()
                                .eq(ActivityRecord::getTaskId, task.getId())
                                .orderByAsc(ActivityRecord::getStartTime)
                );
                
                // 处理活动记录为甘特图数据段
                List<Map<String, Object>> segments = new ArrayList<>();
                for (ActivityRecord activity : activities) {
                    Map<String, Object> segment = new HashMap<>();
                    segment.put("type", activity.getActivityType());
                    segment.put("typeDescription", getActivityTypeDescription(activity.getActivityType()));
                    segment.put("startTime", activity.getStartTime());
                    segment.put("endTime", activity.getEndTime());
                    segment.put("durationMinutes", activity.getDurationMinutes());
                    segment.put("description", activity.getDescription());
                    segments.add(segment);
                }
                
                taskInfo.put("segments", segments);
                taskData.add(taskInfo);
            }
            
            // 按开始日期排序任务
            taskData.sort((a, b) -> {
                LocalDateTime aTime = getTaskStartTime(a);
                LocalDateTime bTime = getTaskStartTime(b);
                if (aTime == null) return 1;
                if (bTime == null) return -1;
                return aTime.compareTo(bTime);
            });
            
            Map<String, Object> tagGroup = new HashMap<>();
            tagGroup.put("tagId", tag.getId());
            tagGroup.put("tagName", tag.getName());
            tagGroup.put("tagColor", tag.getColor());
            tagGroup.put("tasks", taskData);
            
            tagGroups.add(tagGroup);
        }
        
        result.put("tagGroups", tagGroups);
        
        // 添加没有标签的任务
        List<Long> tasksWithTags = taskTagMapper.selectList(null).stream()
                .map(TaskTag::getTaskId)
                .distinct()
                .collect(Collectors.toList());
        
        List<Task> tasksWithoutTags = lambdaQuery()
                .notIn(Task::getId, tasksWithTags)
                .list();
        
        if (!tasksWithoutTags.isEmpty()) {
            List<Map<String, Object>> untaggedTaskData = new ArrayList<>();
            for (Task task : tasksWithoutTags) {
                Map<String, Object> taskInfo = new HashMap<>();
                taskInfo.put("id", task.getId());
                taskInfo.put("title", task.getTitle());
                taskInfo.put("status", task.getStatus());
                taskInfo.put("progress", task.getProgress());
                taskInfo.put("priority", task.getPriority());
                
                List<ActivityRecord> activities = activityRecordMapper.selectList(
                        Wrappers.<ActivityRecord>lambdaQuery()
                                .eq(ActivityRecord::getTaskId, task.getId())
                                .orderByAsc(ActivityRecord::getStartTime)
                );
                
                List<Map<String, Object>> segments = new ArrayList<>();
                for (ActivityRecord activity : activities) {
                    Map<String, Object> segment = new HashMap<>();
                    segment.put("type", activity.getActivityType());
                    segment.put("typeDescription", getActivityTypeDescription(activity.getActivityType()));
                    segment.put("startTime", activity.getStartTime());
                    segment.put("endTime", activity.getEndTime());
                    segment.put("durationMinutes", activity.getDurationMinutes());
                    segment.put("description", activity.getDescription());
                    segments.add(segment);
                }
                
                taskInfo.put("segments", segments);
                untaggedTaskData.add(taskInfo);
            }
            
            untaggedTaskData.sort((a, b) -> {
                LocalDateTime aTime = getTaskStartTime(a);
                LocalDateTime bTime = getTaskStartTime(b);
                if (aTime == null) return 1;
                if (bTime == null) return -1;
                return aTime.compareTo(bTime);
            });
            
            Map<String, Object> untaggedGroup = new HashMap<>();
            untaggedGroup.put("tagId", null);
            untaggedGroup.put("tagName", "未分类");
            untaggedGroup.put("tagColor", "#9E9E9E");
            untaggedGroup.put("tasks", untaggedTaskData);
            
            tagGroups.add(untaggedGroup);
        }
        
        return result;
    }
    
    /**
     * 获取任务开始时间（最早的活动记录或任务创建时间）
     */
    private LocalDateTime getTaskStartTime(Map<String, Object> taskInfo) {
        @SuppressWarnings("unchecked")
        List<Map<String, Object>> segments = (List<Map<String, Object>>) taskInfo.get("segments");
        
        if (segments != null && !segments.isEmpty()) {
            for (Map<String, Object> segment : segments) {
                LocalDateTime startTime = (LocalDateTime) segment.get("startTime");
                if (startTime != null) {
                    return startTime;
                }
            }
        }
        
        return null;
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
}