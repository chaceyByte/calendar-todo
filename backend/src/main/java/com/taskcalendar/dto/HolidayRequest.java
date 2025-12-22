package com.taskcalendar.dto;

import com.taskcalendar.entity.HolidayConfig;
import lombok.Data;

import java.time.LocalDate;
import java.util.List;

/**
 * 休息日请求DTO
 */
@Data
public class HolidayRequest {
    
    /**
     * 年份
     */
    private String year;
    
    /**
     * 休息日配置列表
     */
    private List<HolidayConfig> holidays;
    
    /**
     * 开始日期（用于查询）
     */
    private LocalDate startDate;
    
    /**
     * 结束日期（用于查询）
     */
    private LocalDate endDate;
}