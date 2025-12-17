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

import java.util.HashSet;
import java.util.List;
import java.util.Set;

@Configuration
public class MyBatisPlusConfig {

    /**
     * 3.5.15 统一拦截器入口
     */
    @Bean
    public MybatisPlusInterceptor mybatisPlusInterceptor() {

        MybatisPlusInterceptor interceptor = new MybatisPlusInterceptor();
        final List<String> allowMapper = List.of(
                "com.taskcalendar.mapper.TaskMapper",
                "com.taskcalendar.mapper.TagMapper",
                "com.taskcalendar.mapper.ActivityRecordMapper");

        /* 1. 数据权限拦截器 */
        interceptor.addInnerInterceptor(new DataPermissionInterceptor(new com.baomidou.mybatisplus.extension.plugins.handler.DataPermissionHandler() {

            @Override
            public Expression getSqlSegment(Expression where, String msId) {
                if (allowMapper.stream().noneMatch(msId::startsWith)) return where;
                Long userId = CurrentUser.getUserId();
                if (userId == null) return where;

                /* 自己捞别名（3.5.15 没有 AliasUtils，就 10 行代码） */
                Set<String> aliases = new HashSet<>();
                where.accept(new net.sf.jsqlparser.expression.ExpressionVisitorAdapter() {
                    public void visit(Column c) {
                        String[] a = c.getColumnName().split("\\.");
                        if (a.length == 2) aliases.add(a[0]);
                    }
                });

                if (aliases.isEmpty()) {
                    EqualsTo eq = new EqualsTo();
                    eq.setLeftExpression(new Column("user_id"));
                    eq.setRightExpression(new LongValue(userId));
                    where = where == null ? eq : new AndExpression(where, eq);
                } else {
                    for (String a : aliases) {
                        EqualsTo eq = new EqualsTo();
                        eq.setLeftExpression(new Column(a + ".user_id"));
                        eq.setRightExpression(new LongValue(userId));
                        where = where == null ? eq : new AndExpression(where, eq);
                    }
                }
                return where;
            }
        }));

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