package com.taskcalendar.service;

import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.taskcalendar.entity.EmailVerificationCode;
import com.taskcalendar.mapper.EmailVerificationCodeMapper;
import org.springframework.stereotype.Service;

import java.time.LocalDateTime;
import java.util.Random;

@Service
public class EmailVerificationCodeService extends ServiceImpl<EmailVerificationCodeMapper, EmailVerificationCode> {
    
    private static final int CODE_LENGTH = 6;
    private static final int EXPIRE_MINUTES = 5;
    
    /**
     * 生成随机验证码
     */
    public String generateCode() {
        Random random = new Random();
        StringBuilder code = new StringBuilder();
        for (int i = 0; i < CODE_LENGTH; i++) {
            code.append(random.nextInt(10));
        }
        return code.toString();
    }
    
    /**
     * 创建验证码记录
     */
    public EmailVerificationCode createCode(String email, String type) {
        // 使之前的验证码失效
        lambdaUpdate()
                .set(EmailVerificationCode::getUsed, 1)
                .eq(EmailVerificationCode::getEmail, email)
                .eq(EmailVerificationCode::getType, type)
                .eq(EmailVerificationCode::getUsed, 0)
                .update();
        
        // 创建新的验证码
        EmailVerificationCode verificationCode = new EmailVerificationCode();
        verificationCode.setEmail(email);
        verificationCode.setCode(generateCode());
        verificationCode.setType(type);
        verificationCode.setExpiresAt(LocalDateTime.now().plusMinutes(EXPIRE_MINUTES));
        verificationCode.setUsed(0);
        
        save(verificationCode);
        return verificationCode;
    }
    
    /**
     * 验证验证码
     */
    public boolean validateCode(String email, String code, String type) {
        EmailVerificationCode verificationCode = lambdaQuery()
                .eq(EmailVerificationCode::getEmail, email)
                .eq(EmailVerificationCode::getCode, code)
                .eq(EmailVerificationCode::getType, type)
                .eq(EmailVerificationCode::getUsed, 0)
                .gt(EmailVerificationCode::getExpiresAt, LocalDateTime.now())
                .one();
        
        if (verificationCode != null) {
            // 标记为已使用
            verificationCode.setUsed(1);
            updateById(verificationCode);
            return true;
        }
        
        return false;
    }
    
    /**
     * 检查邮箱是否已存在验证码（未过期且未使用）
     */
    public boolean hasActiveCode(String email, String type) {
        return lambdaQuery()
                .eq(EmailVerificationCode::getEmail, email)
                .eq(EmailVerificationCode::getType, type)
                .eq(EmailVerificationCode::getUsed, 0)
                .gt(EmailVerificationCode::getExpiresAt, LocalDateTime.now())
                .exists();
    }
    
    /**
     * 检查是否允许发送新的验证码（考虑时间间隔）
     */
    public boolean canSendNewCode(String email, String type) {
        // 检查最近1分钟内是否已发送过验证码
        LocalDateTime oneMinuteAgo = LocalDateTime.now().minusMinutes(1);
        
        return !lambdaQuery()
                .eq(EmailVerificationCode::getEmail, email)
                .eq(EmailVerificationCode::getType, type)
                .gt(EmailVerificationCode::getCreatedAt, oneMinuteAgo)
                .exists();
    }
}