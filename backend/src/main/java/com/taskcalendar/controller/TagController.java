package com.taskcalendar.controller;

import com.taskcalendar.config.JwtUtil;
import com.taskcalendar.dto.ApiResponse;
import com.taskcalendar.entity.Tag;
import com.taskcalendar.service.TagService;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/tags")
@RequiredArgsConstructor
public class TagController {
    
    private final TagService tagService;
    private final JwtUtil jwtUtil;
    
    @GetMapping
    public ApiResponse<List<Tag>> getTags(@RequestHeader("Authorization") String token) {
        Long userId = getUserIdFromToken(token);
        if (userId == null) {
            return ApiResponse.error("认证失败");
        }
        
        List<Tag> tags = tagService.getTagsByUserId(userId);
        return ApiResponse.success(tags);
    }
    
    @PostMapping
    public ApiResponse<Tag> createTag(
            @RequestHeader("Authorization") String token,
            @RequestBody Tag tag) {
        Long userId = getUserIdFromToken(token);
        if (userId == null) {
            return ApiResponse.error("认证失败");
        }
        
        tag.setUserId(userId);
        tagService.save(tag);
        return ApiResponse.success("标签创建成功", tag);
    }
    
    @PutMapping("/{id}")
    public ApiResponse<Tag> updateTag(
            @RequestHeader("Authorization") String token,
            @PathVariable Long id,
            @RequestBody Tag tag) {
        Long userId = getUserIdFromToken(token);
        if (userId == null) {
            return ApiResponse.error("认证失败");
        }
        
        Tag existingTag = tagService.getById(id);
        if (existingTag == null || !existingTag.getUserId().equals(userId)) {
            return ApiResponse.error("标签不存在或无权操作");
        }
        
        tag.setId(id);
        tag.setUserId(userId);
        tagService.updateById(tag);
        return ApiResponse.success("标签更新成功", tag);
    }
    
    @DeleteMapping("/{id}")
    public ApiResponse<Void> deleteTag(
            @RequestHeader("Authorization") String token,
            @PathVariable Long id) {
        Long userId = getUserIdFromToken(token);
        if (userId == null) {
            return ApiResponse.error("认证失败");
        }
        
        Tag tag = tagService.getById(id);
        if (tag == null || !tag.getUserId().equals(userId)) {
            return ApiResponse.error("标签不存在或无权操作");
        }
        
        tagService.removeById(id);
        return ApiResponse.success("标签删除成功");
    }
    
    private Long getUserIdFromToken(String token) {
        if (token != null && token.startsWith("Bearer ")) {
            token = token.substring(7);
            if (jwtUtil.validateToken(token)) {
                String username = jwtUtil.getUsernameFromToken(token);
                // 这里应该查询数据库获取用户ID，暂时返回1
                return 1L;
            }
        }
        return null;
    }
}