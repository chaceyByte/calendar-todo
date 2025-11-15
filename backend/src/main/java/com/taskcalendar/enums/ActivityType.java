package com.taskcalendar.enums;

public enum ActivityType {
    CREATED("创建"),
    STARTED("开始"),
    PAUSED("暂停"),
    RESUMED("恢复"),
    COMPLETED("完成"),
    WORK("工作"),
    MEETING("会议"),
    STUDY("学习"),
    OTHER("其他");
    
    private final String description;
    
    ActivityType(String description) {
        this.description = description;
    }
    
    public String getDescription() {
        return description;
    }
}