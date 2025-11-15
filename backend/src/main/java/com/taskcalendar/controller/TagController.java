package com.taskcalendar.controller;

import com.taskcalendar.context.CurrentUser;
import com.taskcalendar.dto.ApiResponse;
import com.taskcalendar.dto.TagWithCountDTO;
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
    
    @GetMapping
    public ApiResponse<List<TagWithCountDTO>> getTags() {
        Long userId = CurrentUser.getUserId();
        
        List<TagWithCountDTO> tags = tagService.getTagsWithTaskCount(userId);
        return ApiResponse.success(tags);
    }
    
    @PostMapping
    public ApiResponse<Tag> createTag(@RequestBody Tag tag) {
        Long userId = CurrentUser.getUserId();
        
        tag.setUserId(userId);
        tagService.save(tag);
        return ApiResponse.success("标签创建成功", tag);
    }
    
    @PutMapping("/{id}")
    public ApiResponse<Tag> updateTag(
            @PathVariable Long id,
            @RequestBody Tag tag) {
        Long userId = CurrentUser.getUserId();
        
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
    public ApiResponse<String> deleteTag(@PathVariable Long id) {
        Long userId = CurrentUser.getUserId();
        
        Tag tag = tagService.getById(id);
        if (tag == null || !tag.getUserId().equals(userId)) {
            return ApiResponse.error("标签不存在或无权操作");
        }
        
        tagService.removeById(id);
        return ApiResponse.success("标签删除成功", null);
    }
}