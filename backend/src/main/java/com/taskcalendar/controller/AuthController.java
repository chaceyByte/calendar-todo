package com.taskcalendar.controller;

import com.taskcalendar.config.JwtUtil;
import com.taskcalendar.dto.ApiResponse;
import com.taskcalendar.dto.LoginRequest;
import com.taskcalendar.dto.LoginResponse;
import com.taskcalendar.entity.User;
import com.taskcalendar.service.UserService;
import lombok.RequiredArgsConstructor;
import org.springframework.security.crypto.password.PasswordEncoder;
import org.springframework.web.bind.annotation.*;

import javax.validation.Valid;

@RestController
@RequestMapping("/auth")
@RequiredArgsConstructor
public class AuthController {
    
    private final UserService userService;
    private final PasswordEncoder passwordEncoder;
    private final JwtUtil jwtUtil;
    
    @PostMapping("/login")
    public ApiResponse<LoginResponse> login(@Valid @RequestBody LoginRequest request) {
        User user = userService.findByUsername(request.getUsername());
        
        if (user == null || !passwordEncoder.matches(request.getPassword(), user.getPassword())) {
            return ApiResponse.error("用户名或密码错误");
        }
        
        String token = jwtUtil.generateToken(user.getUsername());
        
        LoginResponse.UserInfo userInfo = new LoginResponse.UserInfo();
        userInfo.setId(user.getId());
        userInfo.setUsername(user.getUsername());
        userInfo.setNickname(user.getNickname());
        userInfo.setAvatar(user.getAvatar());
        userInfo.setEmail(user.getEmail());
        
        LoginResponse response = new LoginResponse();
        response.setToken(token);
        response.setUser(userInfo);
        
        return ApiResponse.success("登录成功", response);
    }
    
    @GetMapping("/profile")
    public ApiResponse<LoginResponse.UserInfo> getProfile(@RequestHeader("Authorization") String token) {
        if (token != null && token.startsWith("Bearer ")) {
            token = token.substring(7);
            if (jwtUtil.validateToken(token)) {
                String username = jwtUtil.getUsernameFromToken(token);
                User user = userService.findByUsername(username);
                
                if (user != null) {
                    LoginResponse.UserInfo userInfo = new LoginResponse.UserInfo();
                    userInfo.setId(user.getId());
                    userInfo.setUsername(user.getUsername());
                    userInfo.setNickname(user.getNickname());
                    userInfo.setAvatar(user.getAvatar());
                    userInfo.setEmail(user.getEmail());
                    
                    return ApiResponse.success(userInfo);
                }
            }
        }
        
        return ApiResponse.error("获取用户信息失败");
    }
}