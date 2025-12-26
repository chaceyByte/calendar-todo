package com.taskcalendar.entity;

import com.baomidou.mybatisplus.annotation.*;
import lombok.Data;
import java.time.LocalDateTime;

@Data
@TableName("email_verification_codes")
public class EmailVerificationCode {
    
    @TableId(type = IdType.AUTO)
    private Long id;
    
    @TableField("email")
    private String email;
    
    @TableField("code")
    private String code;
    
    @TableField("type")
    private String type; // REGISTER, RESET_PASSWORD, CHANGE_EMAIL
    
    @TableField("expires_at")
    private LocalDateTime expiresAt;
    
    @TableField("used")
    private Integer used;
    
    @TableField(value = "created_at", fill = FieldFill.INSERT)
    private LocalDateTime createdAt;
}