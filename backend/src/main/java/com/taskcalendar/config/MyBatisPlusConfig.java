package com.taskcalendar.config;

import com.baomidou.mybatisplus.annotation.DbType;
import com.baomidou.mybatisplus.extension.plugins.MybatisPlusInterceptor;
import com.baomidou.mybatisplus.extension.plugins.inner.PaginationInnerInterceptor;
import com.taskcalendar.config.datapermission.CustomDataPermission;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

@Configuration
public class MyBatisPlusConfig {


    /**
     * 3.5.15 统一拦截器入口
     */
    @Bean
    public MybatisPlusInterceptor mybatisPlusInterceptor() {

        MybatisPlusInterceptor interceptor = new MybatisPlusInterceptor();

        /* 1. 数据权限拦截器 */
        interceptor.addInnerInterceptor(new CustomDataPermission());

        /* 2. 分页插件 */
        PaginationInnerInterceptor page = new PaginationInnerInterceptor(DbType.MYSQL);
        // page.setMaxLimit(500L);   // 需要时再开
        interceptor.addInnerInterceptor(page);

        /* 3. 其他插件（按需打开） */
        // interceptor.addInnerInterceptor(new OptimisticLockerInnerInterceptor());
        // interceptor.addInnerInterceptor(new BlockAttackInnerInterceptor());
        return interceptor;
    }
}