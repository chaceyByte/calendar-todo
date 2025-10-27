package com.taskcalendar.controller;

import com.taskcalendar.config.JwtUtil;
import com.taskcalendar.dto.ApiResponse;
import lombok.RequiredArgsConstructor;
import org.springframework.ai.chat.client.ChatClient;
import org.springframework.web.bind.annotation.*;

import java.util.HashMap;
import java.util.Map;

@RestController
@RequestMapping("/ai")
@RequiredArgsConstructor
public class AIController {
    
    private final ChatClient chatClient;
    private final JwtUtil jwtUtil;
    
    @PostMapping("/polish-report")
    public ApiResponse<Map<String, String>> polishReport(
            @RequestHeader("Authorization") String token,
            @RequestBody Map<String, String> request) {
        Long userId = getUserIdFromToken(token);
        if (userId == null) {
            return ApiResponse.error("认证失败");
        }
        
        String originalContent = request.get("content");
        String reportType = request.get("type");
        
        if (originalContent == null || originalContent.trim().isEmpty()) {
            return ApiResponse.error("报告内容不能为空");
        }
        
        try {
            String prompt = String.format("请润色以下%s报告，使其更加专业、流畅，同时保持原意不变：\n\n%s\n\n请直接返回润色后的内容，不要添加其他说明。", 
                reportType != null ? reportType : "", originalContent);
            
            String polishedContent = chatClient.prompt()
                    .user(prompt)
                    .call()
                    .content();
            
            Map<String, String> result = new HashMap<>();
            result.put("original", originalContent);
            result.put("polished", polishedContent);
            
            return ApiResponse.success(result);
        } catch (Exception e) {
            return ApiResponse.error("AI润色失败：" + e.getMessage());
        }
    }
    
    @PostMapping("/generate-suggestions")
    public ApiResponse<Map<String, Object>> generateSuggestions(
            @RequestHeader("Authorization") String token,
            @RequestBody Map<String, Object> request) {
        Long userId = getUserIdFromToken(token);
        if (userId == null) {
            return ApiResponse.error("认证失败");
        }
        
        String taskDescription = (String) request.get("taskDescription");
        String context = (String) request.get("context");
        
        if (taskDescription == null || taskDescription.trim().isEmpty()) {
            return ApiResponse.error("任务描述不能为空");
        }
        
        try {
            String prompt = String.format("基于以下任务描述和上下文，提供任务处理建议：\n\n任务：%s\n上下文：%s\n\n请提供：1. 优先级建议 2. 时间安排建议 3. 可能的风险和解决方案", 
                taskDescription, context != null ? context : "");
            
            String suggestions = chatClient.prompt()
                    .user(prompt)
                    .call()
                    .content();
            
            Map<String, Object> result = new HashMap<>();
            result.put("taskDescription", taskDescription);
            result.put("suggestions", suggestions);
            
            return ApiResponse.success(result);
        } catch (Exception e) {
            return ApiResponse.error("AI建议生成失败：" + e.getMessage());
        }
    }
    
    @PostMapping("/summarize-weekly")
    public ApiResponse<Map<String, String>> summarizeWeekly(
            @RequestHeader("Authorization") String token,
            @RequestBody Map<String, String> request) {
        Long userId = getUserIdFromToken(token);
        if (userId == null) {
            return ApiResponse.error("认证失败");
        }
        
        String weeklyData = request.get("weeklyData");
        
        if (weeklyData == null || weeklyData.trim().isEmpty()) {
            return ApiResponse.error("周数据不能为空");
        }
        
        try {
            String prompt = String.format("请将以下周工作数据整理成专业的周报总结：\n\n%s\n\n要求：1. 简洁明了 2. 突出重点 3. 包含本周完成情况和下周计划", weeklyData);
            
            String summary = chatClient.prompt()
                    .user(prompt)
                    .call()
                    .content();
            
            Map<String, String> result = new HashMap<>();
            result.put("original", weeklyData);
            result.put("summary", summary);
            
            return ApiResponse.success(result);
        } catch (Exception e) {
            return ApiResponse.error("周报总结生成失败：" + e.getMessage());
        }
    }
    
    private Long getUserIdFromToken(String token) {
        try {
            if (token != null && token.startsWith("Bearer ")) {
                token = token.substring(7);
            }
            return jwtUtil.getUserIdFromToken(token);
        } catch (Exception e) {
            return null;
        }
    }
}