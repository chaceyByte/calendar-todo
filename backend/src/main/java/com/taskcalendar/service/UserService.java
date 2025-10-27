package com.taskcalendar.service;

import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.taskcalendar.entity.User;
import com.taskcalendar.mapper.UserMapper;
import org.springframework.stereotype.Service;

@Service
public class UserService extends ServiceImpl<UserMapper, User> {
    
    public User findByUsername(String username) {
        return lambdaQuery()
                .eq(User::getUsername, username)
                .one();
    }
}