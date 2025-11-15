package com.taskcalendar.interceptor;

import com.taskcalendar.config.JwtUtil;
import com.taskcalendar.context.CurrentUser;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Component;
import org.springframework.web.servlet.HandlerInterceptor;

import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;

@Component
@RequiredArgsConstructor
public class AuthInterceptor implements HandlerInterceptor {

    private final JwtUtil jwtUtil;

    @Override
    public boolean preHandle(HttpServletRequest request, HttpServletResponse response, Object handler) throws Exception {
        // 从请求头获取token
        String token = request.getHeader("Authorization");
        
        // 如果有token，验证并设置当前用户
        if (token != null && token.startsWith("Bearer ")) {
            token = token.substring(7);
            if (jwtUtil.validateToken(token)) {
                String username = jwtUtil.getUsernameFromToken(token);
                // 这里应该查询数据库获取用户ID，暂时返回1
                CurrentUser.setUserId(1L);
                CurrentUser.setUsername(username);
            }
        }
        
        // 如果没有token，设置默认用户ID（用于开发测试）
        if (CurrentUser.getUserId() == null) {
            CurrentUser.setUserId(1L);
            CurrentUser.setUsername("testuser");
        }
        
        return true;
    }

    @Override
    public void afterCompletion(HttpServletRequest request, HttpServletResponse response, Object handler, Exception ex) throws Exception {
        // 清除当前用户信息
        CurrentUser.clear();
    }
}