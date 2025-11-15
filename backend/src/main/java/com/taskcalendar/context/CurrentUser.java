package com.taskcalendar.context;

public class CurrentUser {
    private static final ThreadLocal<Long> userId = new ThreadLocal<>();
    private static final ThreadLocal<String> username = new ThreadLocal<>();

    public static Long getUserId() {
        return userId.get();
    }

    public static void setUserId(Long id) {
        userId.set(id);
    }

    public static String getUsername() {
        return username.get();
    }

    public static void setUsername(String name) {
        username.set(name);
    }

    public static void clear() {
        userId.remove();
        username.remove();
    }
}