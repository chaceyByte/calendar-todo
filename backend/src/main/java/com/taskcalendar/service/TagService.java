package com.taskcalendar.service;

import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.taskcalendar.entity.Tag;
import com.taskcalendar.mapper.TagMapper;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
public class TagService extends ServiceImpl<TagMapper, Tag> {
    
    public List<Tag> getTagsByUserId(Long userId) {
        return lambdaQuery()
                .eq(Tag::getUserId, userId)
                .orderByDesc(Tag::getCreatedAt)
                .list();
    }
}