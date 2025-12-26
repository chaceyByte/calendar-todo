package com.taskcalendar.mapper;

import com.baomidou.mybatisplus.core.mapper.BaseMapper;
import com.taskcalendar.entity.EmailVerificationCode;
import org.apache.ibatis.annotations.Mapper;

@Mapper
public interface EmailVerificationCodeMapper extends BaseMapper<EmailVerificationCode> {
}