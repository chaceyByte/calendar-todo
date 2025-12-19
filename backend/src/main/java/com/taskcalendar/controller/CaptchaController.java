package com.taskcalendar.controller;

import com.taskcalendar.dto.ApiResponse;
import com.taskcalendar.util.CaptchaUtil;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import javax.servlet.http.HttpSession;
import java.util.HashMap;
import java.util.Map;

@RestController
@RequestMapping("/api/captcha")
public class CaptchaController {

    @GetMapping("/generate")
    public ApiResponse<Map<String, String>> generate(HttpSession session) {
        String[] captchaData = CaptchaUtil.generateCaptcha();
        String code = captchaData[0];
        String image = captchaData[1];
        
        // 将验证码存入session
        session.setAttribute("captcha", code.toLowerCase());
        
        Map<String, String> result = new HashMap<>();
        result.put("image", "data:image/png;base64," + image);
        
        return ApiResponse.success("验证码生成成功", result);
    }
}