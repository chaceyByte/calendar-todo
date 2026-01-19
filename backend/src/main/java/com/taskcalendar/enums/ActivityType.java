package com.taskcalendar.enums;

public enum ActivityType {
    CREATED("创建"),
    DOING("开始行动"),
    STARTED("开始"),
    PAUSED("暂停"),
    RESUMED("恢复"),
    COMPLETED("完成"),
    OTHER("其他");

    private final String description;
    
    ActivityType(String description) {
        this.description = description;
    }
    
    public String getDescription() {
        return description;
    }
}