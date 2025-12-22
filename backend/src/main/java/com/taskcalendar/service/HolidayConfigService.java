package com.taskcalendar.service;

import com.baomidou.mybatisplus.extension.service.IService;
import com.taskcalendar.entity.HolidayConfig;

import java.time.LocalDate;
import java.util.List;

/**
 * 休息日配置服务接口
 */
public interface HolidayConfigService extends IService<HolidayConfig> {
    
    /**
     * 根据年份获取休息日配置
     * @param year 年份
     * @return 休息日配置列表
     */
    List<HolidayConfig> getHolidaysByYear(String year);
    
    /**
     * 根据日期范围获取休息日配置
     * @param startDate 开始日期
     * @param endDate 结束日期
     * @return 休息日配置列表
     */
    List<HolidayConfig> getHolidaysByDateRange(LocalDate startDate, LocalDate endDate);
    
    /**
     * 批量更新休息日配置
     * @param year 年份
     * @param holidays 休息日配置列表
     * @return 是否成功
     */
    boolean updateHolidays(String year, List<HolidayConfig> holidays);
    
    /**
     * 判断指定日期是否为休息日
     * @param date 日期
     * @return 休息日配置信息，如果非休息日返回null
     */
    HolidayConfig getHolidayInfo(LocalDate date);
}