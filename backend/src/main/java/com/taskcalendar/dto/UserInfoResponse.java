package com.taskcalendar.dto;

import lombok.Data;

@Data
public class UserInfoResponse {
    
    private Long id;
    
    private String username;
    
    private String nickname;
    
    private String avatar;
    
    private String email;
}