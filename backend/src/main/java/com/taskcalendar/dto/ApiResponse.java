package com.taskcalendar.dto;

import lombok.Data;

@Data
public class ApiResponse<T> {
    private boolean success;
    private String message;
    private T data;
    private long timestamp;
    
    public ApiResponse() {
        this.timestamp = System.currentTimeMillis();
    }
    
    public static <T> ApiResponse<T> success(T data) {
        ApiResponse<T> response = new ApiResponse<>();
        response.success = true;
        response.message = "操作成功";
        response.data = data;
        return response;
    }
    
    public static <T> ApiResponse<T> success(String message, T data) {
        ApiResponse<T> response = new ApiResponse<>();
        response.success = true;
        response.message = message;
        response.data = data;
        return response;
    }
    
    public static <T> ApiResponse<T> error(String message) {
        ApiResponse<T> response = new ApiResponse<>();
        response.success = false;
        response.message = message;
        return response;
    }
}