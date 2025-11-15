package com.taskcalendar;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.context.annotation.ComponentScan;

@SpringBootApplication
@ComponentScan(basePackages = "com.taskcalendar")
public class TaskCalendarApplication {
    
    public static void main(String[] args) {
        SpringApplication.run(TaskCalendarApplication.class, args);
    }
}