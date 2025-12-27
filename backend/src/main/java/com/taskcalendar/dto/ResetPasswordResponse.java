package com.taskcalendar.dto;

import lombok.Data;

@Data
public class ResetPasswordResponse {
    
    private String newPassword;
    
    private String email;
}