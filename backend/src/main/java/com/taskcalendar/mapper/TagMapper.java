package com.taskcalendar.mapper;

import com.baomidou.mybatisplus.core.mapper.BaseMapper;
import com.taskcalendar.entity.Tag;
import org.apache.ibatis.annotations.Mapper;

@Mapper
public interface TagMapper extends BaseMapper<Tag> {
}