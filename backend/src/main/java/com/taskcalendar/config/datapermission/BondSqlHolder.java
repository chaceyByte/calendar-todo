package com.taskcalendar.config.datapermission;

public class BondSqlHolder {
    private static final ThreadLocal<String> SQL = new ThreadLocal<>();

    public static void set(String bs) {
        SQL.set(bs);
    }

    public static String get() {
        return SQL.get();
    }

    public static void clear() {
        SQL.remove();
    }
}
