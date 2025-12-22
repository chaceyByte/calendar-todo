package com.taskcalendar.service.impl;

import com.baomidou.mybatisplus.core.conditions.query.QueryWrapper;
import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.taskcalendar.entity.HolidayConfig;
import com.taskcalendar.mapper.HolidayConfigMapper;
import com.taskcalendar.service.HolidayConfigService;
import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import java.time.LocalDate;
import java.util.List;

/**
 * 休息日配置服务实现类
 */
@Slf4j
@Service
public class HolidayConfigServiceImpl extends ServiceImpl<HolidayConfigMapper, HolidayConfig> implements HolidayConfigService {

    @Override
    public List<HolidayConfig> getHolidaysByYear(String year) {
        QueryWrapper<HolidayConfig> wrapper = new QueryWrapper<>();
        wrapper.eq("year", year)
               .orderByAsc("date");
        return baseMapper.selectList(wrapper);
    }

    @Override
    public List<HolidayConfig> getHolidaysByDateRange(LocalDate startDate, LocalDate endDate) {
        QueryWrapper<HolidayConfig> wrapper = new QueryWrapper<>();
        wrapper.between("date", startDate, endDate)
               .orderByAsc("date");
        return baseMapper.selectList(wrapper);
    }

    @Override
    @Transactional
    public boolean updateHolidays(String year, List<HolidayConfig> holidays) {
        try {
            // 删除该年份的现有配置
            QueryWrapper<HolidayConfig> wrapper = new QueryWrapper<>();
            wrapper.eq("year", year);
            baseMapper.delete(wrapper);
            
            // 批量插入新的配置
            if (holidays != null && !holidays.isEmpty()) {
                for (HolidayConfig holiday : holidays) {
                    holiday.setYear(year);
                }
                baseMapper.batchInsert(holidays);
            }
            
            log.info("成功更新{}年的休息日配置，共{}条记录", year, holidays != null ? holidays.size() : 0);
            return true;
        } catch (Exception e) {
            log.error("更新休息日配置失败，年份：{}", year, e);
            throw new RuntimeException("更新休息日配置失败");
        }
    }

    @Override
    public HolidayConfig getHolidayInfo(LocalDate date) {
        QueryWrapper<HolidayConfig> wrapper = new QueryWrapper<>();
        wrapper.eq("date", date);
        return baseMapper.selectOne(wrapper);
    }
}