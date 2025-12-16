package com.taskcalendar.config;

import com.baomidou.mybatisplus.annotation.DbType;
import com.baomidou.mybatisplus.extension.plugins.MybatisPlusInterceptor;
import com.baomidou.mybatisplus.extension.plugins.inner.DataPermissionInterceptor;
import com.baomidou.mybatisplus.extension.plugins.inner.PaginationInnerInterceptor;
import com.taskcalendar.context.CurrentUser;
import lombok.SneakyThrows;
import net.sf.jsqlparser.expression.Expression;
import net.sf.jsqlparser.expression.LongValue;
import net.sf.jsqlparser.expression.operators.conditional.AndExpression;
import net.sf.jsqlparser.expression.operators.relational.EqualsTo;
import net.sf.jsqlparser.parser.CCJSqlParserUtil;
import net.sf.jsqlparser.schema.Column;
import net.sf.jsqlparser.statement.select.PlainSelect;
import net.sf.jsqlparser.statement.select.Select;
import org.apache.ibatis.executor.statement.StatementHandler;
import org.apache.ibatis.mapping.BoundSql;
import org.apache.ibatis.mapping.MappedStatement;
import org.apache.ibatis.plugin.*;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

import java.sql.Connection;
import java.util.Properties;

@Configuration
public class MyBatisPlusConfig {

    /**
     * 1. MyBatis-Plus 总拦截器
     */
    @Bean
    public MybatisPlusInterceptor mybatisPlusInterceptor() {
        MybatisPlusInterceptor interceptor = new MybatisPlusInterceptor();
        interceptor.addInnerInterceptor(new DataPermissionInterceptor(new DataPermissionHandler()));
        interceptor.addInnerInterceptor(new PaginationInnerInterceptor(DbType.MYSQL));
        return interceptor;
    }

    /**
     * 2. 数据权限处理器
     */
    public static class DataPermissionHandler
            implements com.baomidou.mybatisplus.extension.plugins.handler.DataPermissionHandler {

        private static final String TASK_MAPPER   = "com.taskcalendar.mapper.TaskMapper";
        private static final String TAG_MAPPER    = "com.taskcalendar.mapper.TagMapper";
        private static final String ACTIVITY_MAPPER = "com.taskcalendar.mapper.ActivityRecordMapper";

        /** ThreadLocal 保存当前 SQL，供本线程后续解析使用 */
        private static final ThreadLocal<String> SQL_CONTEXT = new ThreadLocal<>();

        /** 把 SQL 设置到 ThreadLocal（由下面的 MyBatis 拦截器调用） */
        public static void setSql(String sql) {
            SQL_CONTEXT.set(sql);
        }

        /** 用完即清，防止内存泄漏 */
        public static void clear() {
            SQL_CONTEXT.remove();
        }

        @Override
        @SneakyThrows
        public Expression getSqlSegment(Expression where, String mappedStatementId) {
            if (!needed(mappedStatementId)) {
                return where;
            }
            Long userId = CurrentUser.getUserId();
            if (userId == null) {
                return where;
            }

            /* 1. 从 ThreadLocal 拿完整 SQL */
            String originalSql = SQL_CONTEXT.get();
            if (originalSql == null) {
                // 拿不到就不处理，直接放行
                return where;
            }

            /* 2. 解析出表别名（这里演示直接写死 ar，可按需扩展） */
            PlainSelect select = (PlainSelect) ((Select) CCJSqlParserUtil.parse(originalSql)).getSelectBody();
            String tableAlias = findTableAlias(select);
            if (tableAlias == null) {
                tableAlias = "ar";      // 兜底
            }

            /* 3. 构造 user_id = ? 条件 */
            EqualsTo userCondition = new EqualsTo();
            userCondition.setLeftExpression(new Column(tableAlias + ".user_id"));
            userCondition.setRightExpression(new LongValue(userId));

            return where == null ? userCondition : new AndExpression(where, userCondition);
        }

        private boolean needed(String id) {
            return id.startsWith(TASK_MAPPER) ||
                   id.startsWith(TAG_MAPPER) ||
                   id.startsWith(ACTIVITY_MAPPER);
        }

        /* 简单示例：只认 activity_records 表 */
        private String findTableAlias(PlainSelect select) {
            if (select.getFromItem() instanceof net.sf.jsqlparser.schema.Table) {
                net.sf.jsqlparser.schema.Table table =
                        (net.sf.jsqlparser.schema.Table) select.getFromItem();
                if ("activity_records".equalsIgnoreCase(table.getName())) {
                    return table.getAlias() != null
                            ? table.getAlias().getName()
                            : "activity_records";
                }
            }
            return null;
        }
    }

    /**
     * 3. 原生 MyBatis 拦截器：负责在真正执行前把完整 SQL 塞进 ThreadLocal
     */
    @Intercepts({
        @Signature(type = StatementHandler.class,
                   method = "prepare",
                   args = {Connection.class, Integer.class})
    })
    public static class SqlExtractInterceptor implements Interceptor {

        @Override
        public Object intercept(Invocation invocation) throws Throwable {
            StatementHandler handler = (StatementHandler) invocation.getTarget();
            BoundSql boundSql = handler.getBoundSql();
            String sql = boundSql.getSql();          // 完整 SQL
            DataPermissionHandler.setSql(sql);       // 放到 ThreadLocal
            try {
                return invocation.proceed();
            } finally {
                DataPermissionHandler.clear();       // 清理
            }
        }

        @Override
        public Object plugin(Object target) {
            return Plugin.wrap(target, this);
        }

        @Override
        public void setProperties(Properties properties) {}
    }
}