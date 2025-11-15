package com.taskcalendar.service;

import com.taskcalendar.dto.DailyReport;
import com.taskcalendar.dto.ManualActivityRequest;
import com.taskcalendar.dto.StartActivityRequest;
import com.taskcalendar.entity.ActivityRecord;
import com.taskcalendar.enums.ActivityType;
import com.taskcalendar.mapper.ActivityRecordMapper;
import com.taskcalendar.mapper.TaskMapper;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;
import org.mockito.InjectMocks;
import org.mockito.Mock;
import org.mockito.junit.jupiter.MockitoExtension;

import java.time.LocalDate;
import java.time.LocalDateTime;
import java.util.Arrays;
import java.util.List;

import static org.junit.jupiter.api.Assertions.*;
import static org.mockito.ArgumentMatchers.any;
import static org.mockito.Mockito.*;

@ExtendWith(MockitoExtension.class)
class ActivityServiceTest {

    @Mock
    private ActivityRecordMapper activityRecordMapper;
    
    @Mock
    private TaskMapper taskMapper;
    
    @InjectMocks
    private ActivityService activityService;
    
    private ActivityRecord testActivity;
    
    @BeforeEach
    void setUp() {
        testActivity = new ActivityRecord();
        testActivity.setId(1L);
        testActivity.setTaskId(1L);
        testActivity.setActivityType("WORK");
        testActivity.setStartTime(LocalDateTime.now().minusHours(2));
        testActivity.setEndTime(LocalDateTime.now().minusHours(1));
        testActivity.setDurationMinutes(60);
        testActivity.setCreatedAt(LocalDateTime.now().minusHours(2));
    }
    
    @Test
    void testGetTaskActivities() {
        // Given
        Long taskId = 1L;
        List<ActivityRecord> expectedActivities = Arrays.asList(testActivity);
        when(activityRecordMapper.selectList(any())).thenReturn(expectedActivities);
        
        // When
        List<ActivityRecord> actualActivities = activityService.getTaskActivities(taskId);
        
        // Then
        assertEquals(expectedActivities, actualActivities);
        verify(activityRecordMapper).selectList(any());
    }
    
    @Test
    void testStartActivity() {
        // Given
        Long taskId = 1L;
        StartActivityRequest request = new StartActivityRequest();
        request.setTaskId(taskId);
        request.setActivityType(ActivityType.WORK);
        request.setDescription("测试活动");
        
        when(taskMapper.selectById(taskId)).thenReturn(new com.taskcalendar.entity.Task());
        when(activityRecordMapper.selectOne(any())).thenReturn(null);
        when(activityRecordMapper.insert(any())).thenReturn(1);
        
        // When
        ActivityRecord result = activityService.startActivity(request);
        
        // Then
        assertNotNull(result);
        assertEquals(taskId, result.getTaskId());
        assertEquals("WORK", result.getActivityType());
        assertEquals("测试活动", result.getDescription());
        verify(taskMapper).selectById(taskId);
        verify(activityRecordMapper).insert(any());
    }
    
    @Test
    void testAddManualActivity() {
        // Given
        Long taskId = 1L;
        ManualActivityRequest request = new ManualActivityRequest();
        request.setTaskId(taskId);
        request.setActivityType(ActivityType.MEETING);
        request.setStartTime(LocalDateTime.now().minusHours(2));
        request.setEndTime(LocalDateTime.now().minusHours(1));
        request.setDescription("测试会议");
        
        when(taskMapper.selectById(taskId)).thenReturn(new com.taskcalendar.entity.Task());
        when(activityRecordMapper.insert(any())).thenReturn(1);
        
        // When
        ActivityRecord result = activityService.addManualActivity(request);
        
        // Then
        assertNotNull(result);
        assertEquals(taskId, result.getTaskId());
        assertEquals("MEETING", result.getActivityType());
        assertEquals("测试会议", result.getDescription());
        assertEquals(60, result.getDurationMinutes());
        verify(taskMapper).selectById(taskId);
        verify(activityRecordMapper).insert(any());
    }
    
    @Test
    void testGetDailyReport() {
        // Given
        LocalDate date = LocalDate.now();
        List<ActivityRecord> activities = Arrays.asList(testActivity);
        when(activityRecordMapper.selectList(any())).thenReturn(activities);
        
        // When
        DailyReport report = activityService.getDailyReport(date);
        // Then
        assertNotNull(report);
        assertEquals(date, report.getDate());
        assertEquals(60, report.getTotalTime());
        verify(activityRecordMapper).selectList(any());
    }
}