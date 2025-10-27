package com.taskcalendar.dto;

import lombok.Data;
import java.time.LocalDateTime;
import java.util.List;

@Data
public class TaskDTO {
    private Long id;
    private String title;
    private String description;
    private String status;
    private Integer progress;
    private String priority;
    private LocalDateTime startDate;
    private LocalDateTime endDate;
    private List<String> tags;
    private LocalDateTime createdAt;
    private LocalDateTime updatedAt;
}