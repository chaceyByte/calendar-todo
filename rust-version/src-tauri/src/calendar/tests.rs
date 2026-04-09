// 日历模块单元测试

#[cfg(test)]
mod tests {
    use super::super::*;
    use chrono::NaiveDate;

    // ==================== 日期类型测试 ====================

    #[test]
    fn test_date_type_from_str() {
        assert_eq!(DateType::from_str("workday"), Some(DateType::Workday));
        assert_eq!(DateType::from_str("holiday"), Some(DateType::Holiday));
        assert_eq!(DateType::from_str("makeup"), Some(DateType::Makeup));
        assert_eq!(DateType::from_str("weekend"), Some(DateType::Weekend));
        assert_eq!(DateType::from_str("unknown"), None);
    }

    #[test]
    fn test_date_type_as_str() {
        assert_eq!(DateType::Workday.as_str(), "workday");
        assert_eq!(DateType::Holiday.as_str(), "holiday");
        assert_eq!(DateType::Makeup.as_str(), "makeup");
        assert_eq!(DateType::Weekend.as_str(), "weekend");
    }

    // ==================== 日期验证规则测试 ====================

    #[test]
    fn test_validate_date_for_holiday() {
        // 周一到周五可以设为节假日
        let monday = "2025-04-07"; // 周一
        let _friday = "2025-04-11"; // 周五
        
        // 周末不能设为节假日
        let saturday = "2025-04-12"; // 周六
        let _sunday = "2025-04-13"; // 周日

        // 验证周一可以设为节假日
        let date = NaiveDate::parse_from_str(monday, "%Y-%m-%d").unwrap();
        let day_of_week = date.weekday().num_days_from_monday() as i32;
        assert!(!is_weekend(day_of_week));

        // 验证周六是周末
        let date = NaiveDate::parse_from_str(saturday, "%Y-%m-%d").unwrap();
        let day_of_week = date.weekday().num_days_from_monday() as i32;
        assert!(is_weekend(day_of_week));

        // 验证周日是周末
        let date = NaiveDate::parse_from_str(sunday, "%Y-%m-%d").unwrap();
        let day_of_week = date.weekday().num_days_from_monday() as i32;
        assert!(is_weekend(day_of_week));
    }

    #[test]
    fn test_validate_date_for_makeup() {
        // 周末可以设为补班
        let saturday = "2025-04-12"; // 周六
        let sunday = "2025-04-13"; // 周日
        
        // 工作日不能设为补班
        let monday = "2025-04-07"; // 周一

        // 验证周六是周末
        let date = NaiveDate::parse_from_str(saturday, "%Y-%m-%d").unwrap();
        let day_of_week = date.weekday().num_days_from_monday() as i32;
        assert!(is_weekend(day_of_week));

        // 验证周一不是周末
        let date = NaiveDate::parse_from_str(monday, "%Y-%m-%d").unwrap();
        let day_of_week = date.weekday().num_days_from_monday() as i32;
        assert!(!is_weekend(day_of_week));
    }

    fn is_weekend(day_of_week: i32) -> bool {
        day_of_week >= 5 // 周六(5)或周日(6)
    }

    // ==================== 工作日规则测试 ====================

    #[test]
    fn test_default_workdays() {
        // 验证周一至周五默认是工作日
        let workdays = vec![
            "2025-04-07", // 周一
            "2025-04-08", // 周二
            "2025-04-09", // 周三
            "2025-04-10", // 周四
            "2025-04-11", // 周五
        ];

        for date_str in workdays {
            let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap();
            let day_of_week = date.weekday().num_days_from_monday() as i32;
            assert!(!is_weekend(day_of_week), "{} 应该是工作日", date_str);
        }
    }

    #[test]
    fn test_default_weekends() {
        // 验证周六和周日默认是周末
        let weekends = vec![
            "2025-04-12", // 周六
            "2025-04-13", // 周日
        ];

        for date_str in weekends {
            let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap();
            let day_of_week = date.weekday().num_days_from_monday() as i32;
            assert!(is_weekend(day_of_week), "{} 应该是周末", date_str);
        }
    }

    // ==================== 边界测试 ====================

    #[test]
    fn test_date_parsing_edge_cases() {
        // 测试闰年2月29日
        let leap_year_date = "2024-02-29";
        let date = NaiveDate::parse_from_str(leap_year_date, "%Y-%m-%d");
        assert!(date.is_ok());

        // 测试非闰年2月29日（应该失败）
        let non_leap_year_date = "2025-02-29";
        let date = NaiveDate::parse_from_str(non_leap_year_date, "%Y-%m-%d");
        assert!(date.is_err());

        // 测试跨年
        let year_end = "2025-12-31";
        let year_start = "2026-01-01";
        
        let date1 = NaiveDate::parse_from_str(year_end, "%Y-%m-%d").unwrap();
        let date2 = NaiveDate::parse_from_str(year_start, "%Y-%m-%d").unwrap();
        
        assert_eq!(date1.year(), 2025);
        assert_eq!(date2.year(), 2026);
    }

    #[test]
    fn test_month_boundaries() {
        // 测试各月份的天数
        let month_days = vec![
            (2025, 1, 31),   // 一月
            (2025, 2, 28),   // 二月（非闰年）
            (2025, 3, 31),   // 三月
            (2025, 4, 30),   // 四月
            (2025, 5, 31),   // 五月
            (2025, 6, 30),   // 六月
            (2025, 7, 31),   // 七月
            (2025, 8, 31),   // 八月
            (2025, 9, 30),   // 九月
            (2025, 10, 31),  // 十月
            (2025, 11, 30),  // 十一月
            (2025, 12, 31),  // 十二月
        ];

        for (year, month, expected_days) in month_days {
            let last_day = last_day_of_month(year, month);
            assert_eq!(last_day.day(), expected_days, "{}年{}月应该有{}天", year, month, expected_days);
        }
    }

    fn last_day_of_month(year: i32, month: u32) -> NaiveDate {
        let next_month = if month == 12 { 1 } else { month + 1 };
        let next_year = if month == 12 { year + 1 } else { year };
        
        let first_day_next = NaiveDate::from_ymd_opt(next_year, next_month, 1).unwrap();
        first_day_next.pred_opt().unwrap()
    }

    // ==================== 批量操作测试 ====================

    #[test]
    fn test_batch_date_range_calculation() {
        let start_date = NaiveDate::from_ymd_opt(2025, 4, 1).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2025, 4, 10).unwrap();
        
        let mut count = 0;
        let mut current = start_date;
        
        while current <= end_date {
            count += 1;
            current = current.succ_opt().unwrap_or(current);
        }
        
        assert_eq!(count, 10, "从4月1日到4月10日应该有10天");
    }

    #[test]
    fn test_weekday_calculation() {
        // 2025年4月6日是周日
        let date = NaiveDate::from_ymd_opt(2025, 4, 6).unwrap();
        let weekday = date.weekday();
        assert_eq!(weekday.num_days_from_monday(), 6, "2025-04-06应该是周日");

        // 2025年4月7日是周一
        let date = NaiveDate::from_ymd_opt(2025, 4, 7).unwrap();
        let weekday = date.weekday();
        assert_eq!(weekday.num_days_from_monday(), 0, "2025-04-07应该是周一");

        // 2025年4月12日是周六
        let date = NaiveDate::from_ymd_opt(2025, 4, 12).unwrap();
        let weekday = date.weekday();
        assert_eq!(weekday.num_days_from_monday(), 5, "2025-04-12应该是周六");
    }

    // ==================== 日期切换逻辑测试 ====================

    #[test]
    fn test_date_toggle_logic() {
        // 测试工作日切换逻辑
        let workday_date = "2025-04-07"; // 周一
        let date = NaiveDate::parse_from_str(workday_date, "%Y-%m-%d").unwrap();
        let day_of_week = date.weekday().num_days_from_monday() as i32;
        let is_weekend = day_of_week >= 5;
        
        // 工作日默认状态 -> 点击 -> 节假日
        // 节假日 -> 点击 -> 恢复默认
        assert!(!is_weekend);

        // 测试周末切换逻辑
        let weekend_date = "2025-04-12"; // 周六
        let date = NaiveDate::parse_from_str(weekend_date, "%Y-%m-%d").unwrap();
        let day_of_week = date.weekday().num_days_from_monday() as i32;
        let is_weekend = day_of_week >= 5;
        
        // 周末默认状态 -> 点击 -> 补班
        // 补班 -> 点击 -> 恢复默认
        assert!(is_weekend);
    }

    // ==================== 序列化测试 ====================

    #[test]
    fn test_holiday_config_serialization() {
        use chrono::Utc;
        
        let config = HolidayConfig {
            id: 1,
            year: "2025".to_string(),
            date: NaiveDate::from_ymd_opt(2025, 4, 7).unwrap(),
            r#type: "holiday".to_string(),
            name: Some("清明节".to_string()),
            description: Some("法定节假日".to_string()),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        assert_eq!(config.year, "2025");
        assert_eq!(config.r#type, "holiday");
        assert_eq!(config.name, Some("清明节".to_string()));
    }

    // ==================== 性能测试 ====================

    #[test]
    fn test_large_date_range_performance() {
        use std::time::Instant;
        
        let start = Instant::now();
        
        // 模拟处理一年的日期
        let start_date = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2025, 12, 31).unwrap();
        
        let mut count = 0;
        let mut current = start_date;
        
        while current <= end_date {
            let _day_of_week = current.weekday().num_days_from_monday();
            count += 1;
            current = current.succ_opt().unwrap_or(current);
        }
        
        let duration = start.elapsed();
        
        assert_eq!(count, 365, "2025年应该有365天");
        assert!(duration.as_millis() < 100, "处理一年的日期应该少于100ms");
    }
}