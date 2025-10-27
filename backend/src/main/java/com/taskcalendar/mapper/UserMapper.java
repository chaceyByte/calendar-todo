package com.taskcalendar.mapper;

import com.baomidou.mybatisplus.core.mapper.BaseMapper;
import com.taskcalendar.entity.User;
import org.apache.ibatis.annotations.Mapper;

@Mapper
public interface UserMapper extends BaseMapper<User> {
}