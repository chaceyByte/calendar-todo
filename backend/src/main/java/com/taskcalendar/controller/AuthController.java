package com.taskcalendar.controller;

import com.baomidou.mybatisplus.extension.conditions.query.LambdaQueryChainWrapper;
import com.taskcalendar.config.JwtUtil;
import com.taskcalendar.dto.*;
import com.taskcalendar.entity.EmailVerificationCode;
import com.taskcalendar.entity.User;
import com.taskcalendar.service.EmailService;
import com.taskcalendar.service.EmailVerificationCodeService;
import com.taskcalendar.service.UserService;
import lombok.RequiredArgsConstructor;
import org.springframework.security.crypto.password.PasswordEncoder;
import org.springframework.web.bind.annotation.*;

import javax.servlet.http.HttpServletResponse;
import javax.servlet.http.HttpSession;
import javax.validation.Valid;

@RestController
@RequestMapping("/api/auth")
@RequiredArgsConstructor
public class AuthController {

    private final UserService userService;
    private final PasswordEncoder passwordEncoder;
    private final JwtUtil jwtUtil;
    private final EmailVerificationCodeService emailVerificationCodeService;
    private final EmailService emailService;

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
    public ApiResponse<LoginResponse.UserInfo> getProfile(@RequestHeader("Authorization") String token,
                                                          HttpServletResponse response) {
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
            } else {
                response.setStatus(HttpServletResponse.SC_UNAUTHORIZED);
                return ApiResponse.error(401, "token无效");
            }
        }

        return ApiResponse.error("获取用户信息失败");
    }

    @GetMapping("/user/{username}")
    public ApiResponse<UserInfoResponse> getUserByUsername(@PathVariable String username) {
        User user = userService.findByUsername(username);
        if (user == null) {
            return ApiResponse.error("用户不存在");
        }

        UserInfoResponse userInfo = new UserInfoResponse();
        userInfo.setId(user.getId());
        userInfo.setUsername(user.getUsername());
        userInfo.setNickname(user.getNickname());
        userInfo.setAvatar(user.getAvatar());
        userInfo.setEmail(user.getEmail());

        return ApiResponse.success(userInfo);
    }

    @PostMapping("/send-email-code")
    public ApiResponse<String> sendEmailCode(@Valid @RequestBody SendEmailCodeRequest request) {
        // 检查是否允许发送新的验证码（考虑时间间隔）
        if (!emailVerificationCodeService.canSendNewCode(request.getEmail(), request.getType())) {
            return ApiResponse.error("验证码发送过于频繁，请1分钟后再试");
        }

        // 如果是重置密码类型，需要检查邮箱是否已注册
        if ("RESET_PASSWORD".equals(request.getType())) {
            User user = userService.findByEmail(request.getEmail());
            if (user == null) {
                return ApiResponse.error("该邮箱未注册");
            }
        }

        // 生成并发送验证码
        EmailVerificationCode verificationCode = emailVerificationCodeService.createCode(
                request.getEmail(), request.getType());

        emailService.sendVerificationCode(request.getEmail(), verificationCode.getCode(), request.getType());

        return ApiResponse.success("验证码发送成功");
    }

    @PostMapping("/change-password")
    public ApiResponse<String> changePassword(@Valid @RequestBody ChangePasswordRequest request,
                                              @RequestHeader("Authorization") String token,
                                              HttpServletResponse response) {
        // 验证token
        String username = validateToken(token, response);
        if (username == null) {
            return ApiResponse.error(401, "token无效");
        }

        // 获取当前用户
        User user = userService.findByUsername(username);
        if (user == null) {
            return ApiResponse.error("用户不存在");
        }

        // 验证原密码
        if (!passwordEncoder.matches(request.getOldPassword(), user.getPassword())) {
            return ApiResponse.error("原密码错误");
        }

        // 验证新密码和确认密码是否一致
        if (!request.getNewPassword().equals(request.getConfirmPassword())) {
            return ApiResponse.error("新密码和确认密码不一致");
        }

        // 更新密码
        user.setPassword(passwordEncoder.encode(request.getNewPassword()));
        userService.updateById(user);

        return ApiResponse.success("密码修改成功");
    }

    @PostMapping("/reset-password")
    public ApiResponse<String> resetPassword(@Valid @RequestBody ResetPasswordRequest request) {
        // 根据邮箱查找用户
        User user = userService.findByEmail(request.getEmail());
        if (user == null) {
            return ApiResponse.error("该邮箱未注册");
        }

        // 验证验证码
        if (!emailVerificationCodeService.validateCode(request.getEmail(), request.getCode(), "RESET_PASSWORD")) {
            return ApiResponse.error("验证码错误或已过期");
        }

        // 验证新密码和确认密码是否一致
        if (!request.getNewPassword().equals(request.getConfirmPassword())) {
            return ApiResponse.error("新密码和确认密码不一致");
        }

        // 更新密码
        user.setPassword(passwordEncoder.encode(request.getNewPassword()));
        userService.updateById(user);

        return ApiResponse.success("密码重置成功");
    }

    @PostMapping("/reset-password-by-username")
    public ApiResponse<ResetPasswordResponse> resetPasswordByUsername(@Valid @RequestBody ResetPasswordByUsernameRequest request) {
        // 根据用户名查找用户
        User user = userService.findByUsername(request.getUsername());
        if (user == null) {
            return ApiResponse.error("用户名不存在");
        }

        // 验证验证码
        if (!emailVerificationCodeService.validateCode(user.getEmail(), request.getCode(), "RESET_PASSWORD")) {
            return ApiResponse.error("验证码错误或已过期");
        }

        // 生成12位随机密码
        String randomPassword = generateRandomPassword(12);

        // 更新密码
        user.setPassword(passwordEncoder.encode(randomPassword));
        userService.updateById(user);

        // 返回新密码
        ResetPasswordResponse response = new ResetPasswordResponse();
        response.setNewPassword(randomPassword);
        response.setEmail(user.getEmail());

        return ApiResponse.success("密码重置成功", response);
    }

    /**
     * 生成随机密码
     */
    private String generateRandomPassword(int length) {
        String chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*";
        StringBuilder password = new StringBuilder();
        for (int i = 0; i < length; i++) {
            password.append(chars.charAt((int) (Math.random() * chars.length())));
        }
        return password.toString();
    }

    @PostMapping("/change-email")
    public ApiResponse<String> changeEmail(@Valid @RequestBody ChangeEmailRequest request,
                                           @RequestHeader("Authorization") String token,
                                           HttpServletResponse response) {
        // 验证token
        String username = validateToken(token, response);
        if (username == null) {
            return ApiResponse.error(401, "token无效");
        }

        // 获取当前用户
        User user = userService.findByUsername(username);
        if (user == null) {
            return ApiResponse.error("用户不存在");
        }

        // 验证验证码
        if (!emailVerificationCodeService.validateCode(request.getNewEmail(), request.getCode(), "CHANGE_EMAIL")) {
            return ApiResponse.error("验证码错误或已过期");
        }

        // 检查新邮箱是否已被其他用户使用
        User existingUser = lambdaQuery()
                .eq(User::getEmail, request.getNewEmail())
                .ne(User::getId, user.getId())
                .one();

        if (existingUser != null) {
            return ApiResponse.error("该邮箱已被其他用户使用");
        }

        // 更新邮箱
        user.setEmail(request.getNewEmail());
        userService.updateById(user);

        return ApiResponse.success("邮箱更换成功");
    }

    /**
     * 验证token并返回用户名
     */
    private String validateToken(String token, HttpServletResponse response) {
        if (token != null && token.startsWith("Bearer ")) {
            token = token.substring(7);
            if (jwtUtil.validateToken(token)) {
                return jwtUtil.getUsernameFromToken(token);
            } else {
                response.setStatus(HttpServletResponse.SC_UNAUTHORIZED);
            }
        }
        return null;
    }

    /**
     * 使用lambdaQuery的便捷方法
     */
    private LambdaQueryChainWrapper<com.taskcalendar.entity.User> lambdaQuery() {
        return userService.lambdaQuery();
    }
}