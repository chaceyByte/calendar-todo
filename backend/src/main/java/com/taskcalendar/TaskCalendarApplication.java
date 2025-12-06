package com.taskcalendar;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.context.annotation.ComponentScan;

@SpringBootApplication
public class TaskCalendarApplication {
    
    public static void main(String[] args) {
        SpringApplication.run(TaskCalendarApplication.class, args);
    }
}