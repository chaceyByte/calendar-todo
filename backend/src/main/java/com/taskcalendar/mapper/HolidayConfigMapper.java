package com.taskcalendar.mapper;

import com.baomidou.mybatisplus.core.mapper.BaseMapper;
import com.taskcalendar.entity.HolidayConfig;
import org.apache.ibatis.annotations.Mapper;
import org.apache.ibatis.annotations.Param;
import org.apache.ibatis.annotations.Select;

import java.util.List;

/**
 * 休息日配置Mapper接口
 */
@Mapper
public interface HolidayConfigMapper extends BaseMapper<HolidayConfig> {
    
    /**
     * 根据年份查询休息日配置
     * @param year 年份
     * @return 休息日配置列表
     */
    @Select("SELECT * FROM holiday_config WHERE year = #{year} ORDER BY date")
    List<HolidayConfig> selectByYear(@Param("year") String year);
    
    /**
     * 根据日期范围查询休息日配置
     * @param startDate 开始日期
     * @param endDate 结束日期
     * @return 休息日配置列表
     */
    @Select("SELECT * FROM holiday_config WHERE date BETWEEN #{startDate} AND #{endDate} ORDER BY date")
    List<HolidayConfig> selectByDateRange(@Param("startDate") String startDate, 
                                         @Param("endDate") String endDate);
    
    /**
     * 批量插入休息日配置
     * @param holidayConfigs 休息日配置列表
     * @return 影响行数
     */
    int batchInsert(@Param("list") List<HolidayConfig> holidayConfigs);
    
    /**
     * 根据年份删除休息日配置
     * @param year 年份
     * @return 影响行数
     */
    @Select("DELETE FROM holiday_config WHERE year = #{year}")
    int deleteByYear(@Param("year") String year);
}