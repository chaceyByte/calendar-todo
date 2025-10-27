package com.taskcalendar.mapper;

import com.baomidou.mybatisplus.core.mapper.BaseMapper;
import com.taskcalendar.entity.Task;
import org.apache.ibatis.annotations.Mapper;

@Mapper
public interface TaskMapper extends BaseMapper<Task> {
}