package com.taskcalendar.controller;

import com.taskcalendar.dto.ApiResponse;
import com.taskcalendar.dto.HolidayRequest;
import com.taskcalendar.entity.HolidayConfig;
import com.taskcalendar.service.HolidayConfigService;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.format.annotation.DateTimeFormat;
import org.springframework.web.bind.annotation.*;

import java.time.LocalDate;
import java.util.List;

/**
 * 休息日配置控制器
 */
@Slf4j
@RestController
@RequestMapping("/api/holidays")
@RequiredArgsConstructor
public class HolidayController {

    private final HolidayConfigService holidayConfigService;

    /**
     * 获取指定年份的休息日配置
     *
     * @param year 年份
     * @return 休息日配置列表
     */
    @GetMapping("/{year}")
    public ApiResponse<List<HolidayConfig>> getHolidaysByYear(@PathVariable String year) {
        try {
            List<HolidayConfig> holidays = holidayConfigService.getHolidaysByYear(year);
            log.info("获取{}年的休息日配置，共{}条记录", year, holidays.size());
            return ApiResponse.success(holidays);
        } catch (Exception e) {
            log.error("获取休息日配置失败，年份：{}", year, e);
            return ApiResponse.error("获取休息日配置失败");
        }
    }

    /**
     * 获取日期范围内的休息日配置
     *
     * @param startDate 开始日期
     * @param endDate   结束日期
     * @return 休息日配置列表
     */
    @GetMapping("/range")
    public ApiResponse<List<HolidayConfig>> getHolidaysByRange(
            @RequestParam @DateTimeFormat(pattern = "yyyy-MM-dd") LocalDate startDate,
            @RequestParam @DateTimeFormat(pattern = "yyyy-MM-dd") LocalDate endDate) {
        try {
            List<HolidayConfig> holidays = holidayConfigService.getHolidaysByDateRange(startDate, endDate);
            log.info("获取{}到{}的休息日配置，共{}条记录", startDate, endDate, holidays.size());
            return ApiResponse.success(holidays);
        } catch (Exception e) {
            log.error("获取休息日配置失败，日期范围：{} - {}", startDate, endDate, e);
            return ApiResponse.error("获取休息日配置失败");
        }
    }

    /**
     * 批量更新休息日配置
     *
     * @param request 休息日请求
     * @return 操作结果
     */
    @PostMapping("/batch")
    public ApiResponse<Boolean> updateHolidays(@RequestBody HolidayRequest request) {
        try {
            boolean result = holidayConfigService.updateHolidays(request.getYear(), request.getHolidays());
            if (result) {
                log.info("成功更新{}年的休息日配置", request.getYear());
                return ApiResponse.success(true);
            } else {
                return ApiResponse.error("休息日配置更新失败");
            }
        } catch (Exception e) {
            log.error("更新休息日配置失败，年份：{}", request.getYear(), e);
            return ApiResponse.error("更新休息日配置失败：" + e.getMessage());
        }
    }

    /**
     * 获取指定日期的休息日信息
     *
     * @param date 日期
     * @return 休息日信息
     */
    @GetMapping("/date/{date}")
    public ApiResponse<HolidayConfig> getHolidayInfo(
            @PathVariable @DateTimeFormat(pattern = "yyyy-MM-dd") LocalDate date) {
        try {
            HolidayConfig holidayInfo = holidayConfigService.getHolidayInfo(date);
            log.info("获取{}的休息日信息：{}", date, holidayInfo != null ? holidayInfo.getDescription() : "非休息日");
            return ApiResponse.success(holidayInfo);
        } catch (Exception e) {
            log.error("获取休息日信息失败，日期：{}", date, e);
            return ApiResponse.error("获取休息日信息失败");
        }
    }
}