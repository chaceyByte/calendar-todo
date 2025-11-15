package com.taskcalendar.dto;

import com.taskcalendar.entity.Tag;
import lombok.Data;
import lombok.EqualsAndHashCode;

@Data
@EqualsAndHashCode(callSuper = true)
public class TagWithCountDTO extends Tag {
    private Long taskCount;
}