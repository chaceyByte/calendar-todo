package com.taskcalendar.config;

import com.baomidou.mybatisplus.annotation.DbType;
import com.baomidou.mybatisplus.extension.plugins.MybatisPlusInterceptor;
import com.baomidou.mybatisplus.extension.plugins.inner.DataPermissionInterceptor;
import com.baomidou.mybatisplus.extension.plugins.inner.PaginationInnerInterceptor;
import com.taskcalendar.context.CurrentUser;
import net.sf.jsqlparser.expression.Expression;
import net.sf.jsqlparser.expression.LongValue;
import net.sf.jsqlparser.expression.operators.conditional.AndExpression;
import net.sf.jsqlparser.expression.operators.relational.EqualsTo;
import net.sf.jsqlparser.schema.Column;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

@Configuration
public class MyBatisPlusConfig {

    @Bean
    public MybatisPlusInterceptor mybatisPlusInterceptor() {
        MybatisPlusInterceptor interceptor = new MybatisPlusInterceptor();

        // 添加数据权限拦截器
        interceptor.addInnerInterceptor(new DataPermissionInterceptor(new DataPermissionHandler()));

        // 添加分页拦截器
        interceptor.addInnerInterceptor(new PaginationInnerInterceptor(DbType.MYSQL));

        return interceptor;
    }

    /**
     * 数据权限处理器
     */
    public static class DataPermissionHandler implements com.baomidou.mybatisplus.extension.plugins.handler.DataPermissionHandler {
        /**
         * 只对 TaskMapper 里的 SQL 生效
         */
        private static final String TASK_MAPPER = "com.taskcalendar.mapper.TaskMapper";
        private static final String TAG_MAPPER = "com.taskcalendar.mapper.TagMapper";

        @Override
        public Expression getSqlSegment(Expression where, String mappedStatementId) {
            /* 1. 非 TaskMapper 直接放行 */
            if (!mappedStatementId.startsWith(TASK_MAPPER) && !mappedStatementId.startsWith(TAG_MAPPER)) {
                return where;
            }

            /* 2. 取当前用户 */
            Long userId = CurrentUser.getUserId();
            if (userId == null) {
                // 拿不到用户就不过滤，也可以 throw new RuntimeException("请先登录");
                return where;
            }

            /* 3. 构造 user_id = ? */
            EqualsTo userCondition = new EqualsTo();
            userCondition.setLeftExpression(new Column("user_id"));
            userCondition.setRightExpression(new LongValue(userId));

            /* 4. 原 where 为空则直接返回条件；否则用 AND 拼接 */
            if (where == null) {
                return userCondition;
            }
            return new AndExpression(where, userCondition);
        }

        /**
         * 获取当前登录用户ID
         * 从ThreadLocal中获取当前用户ID
         */
        private Long getCurrentUserId() {
            try {
                return CurrentUser.getUserId();
            } catch (Exception e) {
                // 如果无法获取用户ID，返回null表示不进行权限过滤
                return null;
            }
        }
    }
}