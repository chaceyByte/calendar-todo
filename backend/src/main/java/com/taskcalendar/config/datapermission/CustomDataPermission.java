package com.taskcalendar.config.datapermission;

import com.baomidou.mybatisplus.extension.plugins.inner.DataPermissionInterceptor;
import com.taskcalendar.context.CurrentUser;
import net.sf.jsqlparser.parser.CCJSqlParserUtil;
import net.sf.jsqlparser.statement.select.PlainSelect;
import net.sf.jsqlparser.statement.select.Select;

import java.util.HashSet;
import java.util.List;
import java.util.Arrays;
import java.util.Set;

public class CustomDataPermission extends DataPermissionInterceptor {
    private static final List<String> allowMapper = Arrays.asList(
            "com.taskcalendar.mapper.TaskMapper",
            "com.taskcalendar.mapper.TagMapper",
            "com.taskcalendar.mapper.ActivityRecordMapper");

    public CustomDataPermission() {
        super((where, mappedStatementId) -> {
            /* 0. 黑白名单 & 登录检查 */
            if (allowMapper.stream().noneMatch(mappedStatementId::startsWith)) return where;
            Long userId = CurrentUser.getUserId();
            if (userId == null) return where;

            /* 1. 把整条 SQL 解析成 Statement（MP 已内置 JSqlParser） */
            try {

                String sql = BondSqlHolder.get();
                if (sql == null) return where;
                net.sf.jsqlparser.statement.Statement stmt = CCJSqlParserUtil.parse(sql);
                
                // 检查是否是 PlainSelect 类型
                if (!(stmt instanceof Select)) {
                    return where;
                }
                
                Select selectStmt = (Select) stmt;
                if (!(selectStmt.getSelectBody() instanceof PlainSelect)) {
                    return where;
                }
                
                PlainSelect select = (PlainSelect) selectStmt.getSelectBody();
                
                /* 2. 一次性收集所有表别名 */
                Set<String> aliases = new HashSet<>();
                // 2.1 FROM 主表
                aliases.add(select.getFromItem().getAlias() != null
                        ? select.getFromItem().getAlias().getName()
                        : select.getFromItem().toString());
                // 2.2 JOIN 表
                if (select.getJoins() != null) {
                    for (net.sf.jsqlparser.statement.select.Join j : select.getJoins()) {
                        aliases.add(j.getRightItem().getAlias() != null
                                ? j.getRightItem().getAlias().getName()
                                : j.getRightItem().toString());
                    }
                }

                /* 3. 给每张表追加 user_id = ? 条件 */
                for (String alias : aliases) {
                    net.sf.jsqlparser.expression.operators.relational.EqualsTo eq =
                            new net.sf.jsqlparser.expression.operators.relational.EqualsTo();
                    eq.setLeftExpression(new net.sf.jsqlparser.schema.Column(alias + ".user_id"));
                    eq.setRightExpression(new net.sf.jsqlparser.expression.LongValue(userId));
                    where = where == null ? eq
                            : new net.sf.jsqlparser.expression.operators.conditional.AndExpression(where, eq);
                }
                return where;
            } catch (Exception e) {
                throw new RuntimeException("MP data perm error", e);
            } finally {
                BondSqlHolder.clear();
            }
        });
    }

    @Override
    protected void processSelect(Select select, int index, String sql, Object obj) {
        BondSqlHolder.set(sql);
        super.processSelect(select, index, sql, obj);
    }
}
