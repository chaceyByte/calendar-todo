package com.taskcalendar.config;

import com.baomidou.mybatisplus.core.handlers.MetaObjectHandler;
import com.taskcalendar.context.CurrentUser;
import lombok.extern.slf4j.Slf4j;
import org.apache.ibatis.reflection.MetaObject;
import org.springframework.stereotype.Component;

import java.time.LocalDateTime;

/**
 * MyBatis Plus 自动填充处理器
 * 用于自动填充创建时间、更新时间、创建人、更新人等字段
 */
@Slf4j
@Component
public class MyMetaObjectHandler implements MetaObjectHandler {

    /**
     * 插入时自动填充
     */
    @Override
    public void insertFill(MetaObject metaObject) {
        log.debug("开始插入自动填充...");
        
        // 自动填充创建时间
        this.strictInsertFill(metaObject, "createdAt", LocalDateTime.class, LocalDateTime.now());
        
        // 自动填充更新时间
        this.strictInsertFill(metaObject, "updatedAt", LocalDateTime.class, LocalDateTime.now());
        
        // 自动填充用户ID
        Long currentUserId = getCurrentUserId();
        if (currentUserId != null) {
            this.strictInsertFill(metaObject, "userId", Long.class, currentUserId);
            log.debug("插入时自动填充用户ID: {}", currentUserId);
        }
        
        log.debug("插入自动填充完成");
    }

    /**
     * 更新时自动填充
     */
    @Override
    public void updateFill(MetaObject metaObject) {
        log.debug("开始更新自动填充...");
        
        // 自动填充更新时间
        this.strictUpdateFill(metaObject, "updatedAt", LocalDateTime.class, LocalDateTime.now());
        
        // 更新时也可以填充更新人ID（如果需要）
        // this.strictUpdateFill(metaObject, "updatedBy", Long.class, getCurrentUserId());
        
        log.debug("更新自动填充完成");
    }

    /**
     * 获取当前用户ID
     * 从ThreadLocal中获取当前登录用户ID
     */
    private Long getCurrentUserId() {
        try {
            return CurrentUser.getUserId();
        } catch (Exception e) {
            log.warn("无法获取当前用户ID: {}", e.getMessage());
            // 如果无法获取用户ID，返回null，让数据库使用默认值或由业务逻辑处理
            return null;
        }
    }
}