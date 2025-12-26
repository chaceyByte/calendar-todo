package com.taskcalendar.service;

import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Service;

@Service
@Slf4j
public class EmailService {
    
    /**
     * 发送验证码邮件（模拟实现）
     * 在实际项目中，这里应该集成真实的邮件服务
     */
    public void sendVerificationCode(String email, String code, String type) {
        String subject = getEmailSubject(type);
        String content = getEmailContent(code, type);
        
        // 模拟发送邮件
        log.info("发送验证码邮件 - 收件人: {}, 主题: {}, 验证码: {}", email, subject, code);
        log.info("邮件内容: {}", content);
        
        // 在实际项目中，这里应该调用真实的邮件服务API
        // 例如：JavaMailSender, 阿里云邮件服务，腾讯云邮件服务等
    }
    
    private String getEmailSubject(String type) {
        switch (type) {
            case "REGISTER":
                return "任务日历 - 注册验证码";
            case "RESET_PASSWORD":
                return "任务日历 - 重置密码验证码";
            case "CHANGE_EMAIL":
                return "任务日历 - 更换邮箱验证码";
            default:
                return "任务日历 - 验证码";
        }
    }
    
    private String getEmailContent(String code, String type) {
        String action = "";
        switch (type) {
            case "REGISTER":
                action = "注册账号";
                break;
            case "RESET_PASSWORD":
                action = "重置密码";
                break;
            case "CHANGE_EMAIL":
                action = "更换邮箱";
                break;
            default:
                action = "验证操作";
        }
        
        return String.format("""
            <div style="font-family: Arial, sans-serif; max-width: 600px; margin: 0 auto;">
                <h2 style="color: #409eff;">任务日历</h2>
                <p>您好！</p>
                <p>您正在进行%s操作，验证码为：</p>
                <div style="text-align: center; margin: 20px 0;">
                    <span style="font-size: 24px; font-weight: bold; color: #409eff; padding: 10px 20px; border: 2px solid #409eff; border-radius: 4px;">
                        %s
                    </span>
                </div>
                <p>验证码有效期为5分钟，请及时使用。</p>
                <p>如果这不是您本人的操作，请忽略此邮件。</p>
                <hr style="border: none; border-top: 1px solid #eee; margin: 20px 0;">
                <p style="color: #999; font-size: 12px;">此邮件由系统自动发送，请勿回复。</p>
            </div>
            """, action, code);
    }
}