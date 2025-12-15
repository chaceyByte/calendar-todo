package com.taskcalendar.service;

import com.baomidou.mybatisplus.core.toolkit.Wrappers;
import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.taskcalendar.dto.TagWithCountDTO;
import com.taskcalendar.entity.Tag;
import com.taskcalendar.entity.TaskTag;
import com.taskcalendar.mapper.TagMapper;
import com.taskcalendar.mapper.TaskTagMapper;
import io.jsonwebtoken.lang.Collections;
import org.springframework.stereotype.Service;

import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;

@Service
public class TagService extends ServiceImpl<TagMapper, Tag> {

    private final TaskTagMapper taskTagMapper;

    public TagService(TaskTagMapper taskTagMapper) {
        this.taskTagMapper = taskTagMapper;
    }

    public List<Tag> getTagsByUserId() {
        return lambdaQuery().orderByDesc(Tag::getCreatedAt).list();
    }

    public List<TagWithCountDTO> getTagsWithTaskCount() {
        // 获取用户的所有标签
        List<Tag> tags = getTagsByUserId();
        if (Collections.isEmpty(tags)) {
            return new ArrayList<>(0);
        }
        // 获取每个标签的任务数量
        List<TaskTag> taskTags = taskTagMapper.selectList(
                Wrappers.<TaskTag>lambdaQuery()
                        .in(TaskTag::getTagId, tags.stream().map(Tag::getId).collect(Collectors.toList()))
        );

        // 统计每个标签的任务数量
        Map<Long, Long> tagTaskCountMap = taskTags.stream()
                .collect(Collectors.groupingBy(TaskTag::getTagId, Collectors.counting()));

        // 转换为DTO并设置任务数量
        return tags.stream().map(tag -> {
            TagWithCountDTO dto = new TagWithCountDTO();
            dto.setId(tag.getId());
            dto.setName(tag.getName());
            dto.setColor(tag.getColor());
            dto.setUserId(tag.getUserId());
            dto.setCreatedAt(tag.getCreatedAt());
            dto.setUpdatedAt(tag.getUpdatedAt());
            dto.setTaskCount(tagTaskCountMap.getOrDefault(tag.getId(), 0L));
            return dto;
        }).collect(Collectors.toList());
    }
}