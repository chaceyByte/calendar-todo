package com.taskcalendar.interceptor;

import com.taskcalendar.config.JwtUtil;
import com.taskcalendar.context.CurrentUser;
import com.taskcalendar.entity.User;
import com.taskcalendar.service.UserService;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Component;
import org.springframework.web.servlet.HandlerInterceptor;

import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;

/**
 * 认证拦截器，用于设置当前用户信息
 */
@Slf4j
@Component
@RequiredArgsConstructor
public class AuthInterceptor implements HandlerInterceptor {

    private final JwtUtil jwtUtil;
    private final UserService userService;

    @Override
    public boolean preHandle(HttpServletRequest request, HttpServletResponse response, Object handler) throws Exception {
        // 从请求中获取用户信息
        Long userId = getUserIdFromRequest(request);
        
        if (userId != null) {
            // 根据用户ID查询用户信息
            User user = userService.getById(userId);
            if (user != null) {
                CurrentUser.setUserId(userId);
                CurrentUser.setUsername(user.getUsername());
                log.debug("设置当前用户: userId={}, username={}", userId, user.getUsername());
            } else {
                log.warn("用户不存在: userId={}", userId);
            }
        }
        
        return true;
    }

    @Override
    public void afterCompletion(HttpServletRequest request, HttpServletResponse response, Object handler, Exception ex) throws Exception {
        // 清理ThreadLocal，避免内存泄漏
        CurrentUser.clear();
    }

    /**
     * 从请求中获取用户ID
     */
    private Long getUserIdFromRequest(HttpServletRequest request) {
        // 1. 从请求头中获取用户ID（兼容性）
        String userIdHeader = request.getHeader("X-User-Id");
        if (userIdHeader != null && !userIdHeader.trim().isEmpty()) {
            try {
                return Long.parseLong(userIdHeader);
            } catch (NumberFormatException e) {
                log.warn("无效的用户ID格式: {}", userIdHeader);
            }
        }
        
        // 2. 从JWT token中解析用户ID（主要方式）
        String token = extractTokenFromRequest(request);
        if (token != null && jwtUtil.validateToken(token)) {
            try {
                String username = jwtUtil.getUsernameFromToken(token);
                User user = userService.findByUsername(username);
                if (user != null) {
                    return user.getId();
                } else {
                    log.warn("JWT token中的用户不存在: username={}", username);
                }
            } catch (Exception e) {
                log.warn("解析JWT token失败: {}", e.getMessage());
            }
        }
        
        // 3. 如果无法获取用户信息，使用默认用户ID（开发环境使用）
        // 在生产环境中，应该返回null并让请求被拒绝
        log.debug("未找到有效的用户认证信息，使用默认用户ID");
        return 1L; // 默认用户ID
    }

    /**
     * 从请求中提取JWT token
     */
    private String extractTokenFromRequest(HttpServletRequest request) {
        String authHeader = request.getHeader("Authorization");
        if (authHeader != null && authHeader.startsWith("Bearer ")) {
            return authHeader.substring(7);
        }
        
        // 也可以从其他位置获取token，比如查询参数
        String tokenParam = request.getParameter("token");
        if (tokenParam != null && !tokenParam.trim().isEmpty()) {
            return tokenParam;
        }
        
        return null;
    }
}