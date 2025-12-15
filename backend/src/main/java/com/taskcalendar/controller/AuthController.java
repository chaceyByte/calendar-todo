package com.taskcalendar.controller;

import com.taskcalendar.config.JwtUtil;
import com.taskcalendar.dto.ApiResponse;
import com.taskcalendar.dto.LoginRequest;
import com.taskcalendar.dto.LoginResponse;
import com.taskcalendar.dto.RegisterRequest;
import com.taskcalendar.entity.User;
import com.taskcalendar.service.UserService;
import lombok.RequiredArgsConstructor;
import org.springframework.security.crypto.password.PasswordEncoder;
import org.springframework.web.bind.annotation.*;

import javax.servlet.http.HttpSession;
import javax.validation.Valid;

@RestController
@RequestMapping("/auth")
@RequiredArgsConstructor
public class AuthController {

    private final UserService userService;
    private final PasswordEncoder passwordEncoder;
    private final JwtUtil jwtUtil;

    @PostMapping("/register")
    public ApiResponse<LoginResponse> register(@Valid @RequestBody RegisterRequest request, HttpSession session) {
        // 验证码验证
        String captcha = (String) session.getAttribute("captcha");
        if (captcha == null || !captcha.equalsIgnoreCase(request.getCaptcha())) {
            return ApiResponse.error("验证码错误");
        }
        
        // 使用后立即清除验证码
        session.removeAttribute("captcha");
        
        // 检查用户名是否已存在
        User existingUser = userService.findByUsername(request.getUsername());
        if (existingUser != null) {
            return ApiResponse.error("用户名已存在");
        }

        // 创建新用户
        User newUser = new User();
        newUser.setUsername(request.getUsername());
        newUser.setPassword(passwordEncoder.encode(request.getPassword()));
        newUser.setNickname(request.getNickname());
        newUser.setEmail(request.getEmail());
        newUser.setAvatar("https://cube.elemecdn.com/3/7c/3ea6beec64369c2642b92c6726f1epng.png");

        userService.save(newUser);

        // 生成token
        String token = jwtUtil.generateToken(newUser.getUsername());

        LoginResponse.UserInfo userInfo = new LoginResponse.UserInfo();
        userInfo.setId(newUser.getId());
        userInfo.setUsername(newUser.getUsername());
        userInfo.setNickname(newUser.getNickname());
        userInfo.setAvatar(newUser.getAvatar());
        userInfo.setEmail(newUser.getEmail());

        LoginResponse response = new LoginResponse();
        response.setToken(token);
        response.setUser(userInfo);

        return ApiResponse.success("注册成功", response);
    }

    @PostMapping("/login")
    public ApiResponse<LoginResponse> login(@Valid @RequestBody LoginRequest request, HttpSession session) {
        // 验证码验证
        String captcha = (String) session.getAttribute("captcha");
        if (captcha == null || !captcha.equalsIgnoreCase(request.getCaptcha())) {
            return ApiResponse.error("验证码错误");
        }

        // 使用后立即清除验证码
        session.removeAttribute("captcha");

        // 特殊处理演示账号
        if ("admin".equals(request.getUsername()) && "123456".equals(request.getPassword())) {
            User user = userService.findByUsername("admin");
            if (user == null) {
                // 如果admin用户不存在，创建一个默认的admin用户
                user = new User();
                user.setUsername("admin");
                user.setPassword(passwordEncoder.encode("123456"));
                user.setNickname("管理员");
                user.setEmail("admin@example.com");
                user.setAvatar("https://cube.elemecdn.com/3/7c/3ea6beec64369c2642b92c6726f1epng.png");
                userService.save(user);
            }

            String token = jwtUtil.generateToken(request.getUsername());
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

        // 正常用户登录逻辑
        User user = userService.findByUsername(request.getUsername());
        if (user == null || !passwordEncoder.matches(request.getPassword(), user.getPassword())) {
            return ApiResponse.error("用户名或密码错误");
        }

        String token = jwtUtil.generateToken(request.getUsername());
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