<template>
  <div class="calendar-page">
    <!-- Header -->
    <header class="page-header">
      <div class="header-title">
        <h1 class="title">Day View</h1>
      </div>
      <div class="header-nav-center">
        <div class="view-switcher">
          <button class="view-btn" :class="{ active: currentView === 'day' }" @click="setView('day')">Day</button>
          <button class="view-btn" :class="{ active: currentView === 'week' }" @click="setView('week')">Week</button>
          <button class="view-btn" :class="{ active: currentView === 'month' }" @click="setView('month')">Month</button>
        </div>
      </div>
      <div class="header-actions-right">
        <button class="icon-btn">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9"/>
            <path d="M13.73 21a2 2 0 0 1-3.46 0"/>
          </svg>
        </button>
        <button class="icon-btn">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="3"/>
            <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/>
          </svg>
        </button>
        <div class="user-avatar">
          <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=user" alt="User" />
        </div>
      </div>
    </header>

    <!-- Calendar Grid -->
    <div class="calendar-section">
      <!-- Day View -->
      <div v-if="currentView === 'day'" class="day-view-container">
        <!-- Left: Timeline -->
        <div class="day-timeline-section">
          <div class="day-header-row">
            <div class="day-title-block">
              <h2 class="day-date-title">{{ currentDate.format('dddd, MMMM D, YYYY') }}</h2>
              <p class="day-task-count">You have {{ dayTasks.length }} tasks scheduled for today.</p>
            </div>
            <div class="day-nav">
              <button class="nav-arrow" @click="prev">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <polyline points="15 18 9 12 15 6"/>
                </svg>
              </button>
              <button class="today-nav-btn" @click="goToToday">Today</button>
              <button class="nav-arrow" @click="next">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <polyline points="9 18 15 12 9 6"/>
                </svg>
              </button>
            </div>
          </div>

          <!-- Timeline -->
          <div class="timeline-container">
            <div class="timeline">
              <!-- 上午工作时间背景区域 -->
              <div class="work-hours-bg morning-bg" :style="workHoursBgStyle"></div>
              <!-- 下午工作时间背景区域 -->
              <div class="work-hours-bg afternoon-bg" :style="afternoonWorkHoursBgStyle"></div>
              
              <!-- 时间刻度线 -->
              <div
                v-for="hour in displayHours"
                :key="hour"
                class="timeline-hour"
              >
                <span class="hour-label">{{ formatHour(hour) }}</span>
                <div class="hour-line"></div>
              </div>
              
              <!-- 当前时间线 -->
              <div v-if="isToday" class="current-time-line" :style="currentTimeLineStyle">
                <span class="current-time-dot"></span>
                <div class="current-time-line-body"></div>
              </div>
              
              <!-- Task Cards positioned on timeline -->
              <div
                v-for="task in positionedTasksWithLayout"
                :key="task.id"
                class="timeline-task-card"
                :class="[
                  getTaskColorClass(task.taskQuadrant),
                  {
                    'has-conflict': task.hasConflict,
                    [`conflict-index-${task.conflictIndex}`]: task.hasConflict,
                    'is-archived': task.taskStatus === 'archived'
                  }
                ]"
                :style="getTaskCardStyle(task)"
              >
                <!-- 任务时长徽章 -->
                <div class="task-duration-badge" v-if="task.durationMinutes && task.durationMinutes >= 30">
                  {{ formatDurationShort(task.durationMinutes) }}
                </div>
                <!-- 状态指示器 -->
                <div class="task-status-indicator" :class="task.taskStatus"></div>
                <!-- 任务标题 -->
                <span class="task-card-title">{{ task.title }}</span>
                <!-- 任务时间 -->
                <span class="task-card-time">{{ formatTaskTime(task) }}</span>
                <!-- 视觉化标签 -->
                <div v-if="task.tags && task.tags.length > 0" class="task-tags">
                  <span
                    v-for="(tag, idx) in task.tags.slice(0, 2)"
                    :key="idx"
                    class="task-tag-item"
                    :class="getTagColorClass(tag)"
                  >{{ tag.replace('#', '') }}</span>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- Right: Sidebar -->
        <div class="day-sidebar">
          <!-- Daily Focus Card -->
          <div class="daily-focus-card">
            <span class="focus-label">DAILY FOCUS</span>
            <h3 class="focus-title">Revitalize the Q3 Growth Roadmap</h3>
            <div class="focus-progress">
              <div class="progress-bar">
                <div class="progress-fill" style="width: 65%"></div>
              </div>
              <span class="progress-text">65%</span>
            </div>
          </div>

          <!-- Stats Section -->
          <div class="stats-section">
            <!-- Efficiency -->
            <div class="stat-item">
              <div class="stat-header">
                <div class="stat-icon efficiency">
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <circle cx="12" cy="12" r="10"/>
                    <polyline points="12 6 12 12 16 14"/>
                  </svg>
                </div>
                <span class="stat-label-small">EFFICIENCY</span>
              </div>
              <div class="stat-value-large">4.2h</div>
              <div class="stat-desc">Deep work time today</div>
            </div>

            <!-- Intensity -->
            <div class="stat-item">
              <div class="stat-header">
                <div class="stat-icon intensity">
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2"/>
                  </svg>
                </div>
                <span class="stat-label-small">INTENSITY</span>
              </div>
              <div class="stat-value-large">High</div>
              <div class="stat-desc">Cognitive load peak at 10 AM</div>
            </div>
          </div>

          <!-- Mini Calendar -->
          <div class="mini-calendar">
            <div class="mini-calendar-header">
              <span class="mini-calendar-title">{{ currentDate.format('MMMM YYYY') }}</span>
              <div class="mini-calendar-nav">
                <button @click="prevMonth">
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <polyline points="15 18 9 12 15 6"/>
                  </svg>
                </button>
                <button @click="nextMonth">
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <polyline points="9 18 15 12 9 6"/>
                  </svg>
                </button>
              </div>
            </div>
            <div class="mini-calendar-grid">
              <div v-for="day in miniCalendarDays" :key="day.date" 
                   class="mini-calendar-day"
                   :class="{ 
                     'is-today': day.isToday, 
                     'is-current': day.isCurrentMonth,
                     'is-selected': day.isSelected 
                   }">
                {{ day.day }}
              </div>
            </div>
          </div>

          <!-- Upcoming Next -->
          <div class="upcoming-section">
            <span class="upcoming-label">UPCOMING NEXT</span>
            <div class="upcoming-item">
              <div class="upcoming-indicator"></div>
              <div class="upcoming-content">
                <div class="upcoming-title">User Interview: Sarah J.</div>
                <div class="upcoming-time">Tomorrow, 09:30 AM</div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Week View -->
      <div v-else-if="currentView === 'week'" class="week-view">
        <div class="week-grid">
          <div class="weekday-headers">
            <div v-for="day in weekDaysWithNames" :key="day.key" class="weekday" :class="{ 'is-weekend': day.isWeekend }">
              {{ day.weekday }}
            </div>
          </div>
          <div class="week-days">
            <div
              v-for="day in weekDays"
              :key="day.date"
              class="week-day"
              :class="{
                today: day.isToday,
                weekend: day.isWeekend,
                holiday: day.isHoliday,
                makeup: day.isMakeup
              }"
            >
              <div class="day-header-row">
                <span class="day-number" :class="{ 'is-today': day.isToday }">{{ day.dayNumber }}</span>
                <span v-if="day.typeLabel" class="day-type-indicator" :class="day.typeClass">{{ day.typeLabel }}</span>
              </div>
              <div class="day-tasks">
                <div
                  v-for="task in day.tasks.slice(0, 4)"
                  :key="task.id"
                  class="task-tag"
                  :class="getTaskColorClass(task.taskQuadrant)"
                >
                  <span class="task-dot" :class="getTaskColorClass(task.taskQuadrant)"></span>
                  {{ task.title }}
                </div>
                <div v-if="day.tasks.length > 4" class="more-tasks">
                  +{{ day.tasks.length - 4 }} 更多
                </div>
              </div>
              <div v-if="day.isToday" class="today-badge">今天</div>
            </div>
          </div>
        </div>
      </div>

      <!-- Month View -->
      <div v-else class="month-view">
        <!-- Weekday Headers -->
        <div class="weekday-headers">
          <div v-for="day in weekDaysWithNames" :key="day.key" class="weekday" :class="{ 'is-weekend': day.isWeekend }">
            {{ day.weekday }}
          </div>
        </div>

        <!-- Calendar Days -->
        <div class="calendar-days">
          <div
            v-for="day in calendarData?.days || []"
            :key="day.date"
            class="day-cell"
            :class="{
              'other-month': !day.is_current_month,
              'today': day.is_today,
              'weekend': day.is_weekend && day.date_type !== 'holiday' && day.date_type !== 'makeup',
              'holiday': day.date_type === 'holiday',
              'makeup': day.date_type === 'makeup'
            }"
          >
            <div class="day-header-row">
              <span class="day-number" :class="{ 'is-today': day.is_today }">{{ day.day }}</span>
              <span v-if="getTypeLabel(day.date_type)" class="day-type-indicator" :class="day.date_type">{{ getTypeLabel(day.date_type) }}</span>
            </div>
            <div class="day-tasks">
              <div
                v-for="task in day.tasks.slice(0, 3)"
                :key="task.id"
                class="task-tag"
                :class="getTaskColorClass(task.task_quadrant)"
              >
                <span class="task-dot" :class="getTaskColorClass(task.task_quadrant)"></span>
                {{ task.title }}
              </div>
              <div v-if="day.tasks.length > 3" class="more-tasks">
                +{{ day.tasks.length - 3 }}
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Floating Add Button -->
    <button class="fab-add-btn" @click="showAddTaskModal">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <line x1="12" y1="5" x2="12" y2="19"/>
        <line x1="5" y1="12" x2="19" y2="12"/>
      </svg>
    </button>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import dayjs from 'dayjs'
import 'dayjs/locale/zh-cn'

dayjs.locale('zh-cn')

type ViewType = 'day' | 'week' | 'month'

interface CalendarEvent {
  id: number
  task_id: number
  title: string
  description?: string
  start_time: string
  end_time?: string
  color: string
  task_status: string
  task_quadrant: number
}

interface DayWorkRecord {
  id: number
  task_id: number
  start_time: string
  end_time?: string
  duration_minutes?: number
  record_type: string
  task_title: string
  task_description?: string
  task_status: number
  task_quadrant: number
}

interface DayData {
  date: string
  year: number
  month: number
  day: number
  day_of_week: number
  day_of_week_name: string
  is_current_month: boolean
  is_today: boolean
  is_weekend: boolean
  date_type: string
  name?: string
  description?: string
  tasks: CalendarEvent[]
}

interface MonthCalendarData {
  year: number
  month: number
  days: DayData[]
}

interface Task {
  id: number
  title: string
  startTime: string
  endTime?: string
  taskQuadrant: number
  taskStatus: string
  participants?: { name: string; avatar: string }[]
  tags?: string[]
  durationMinutes?: number
  hasConflict?: boolean
  conflictIndex?: number
  conflictTotal?: number
}

// 工作时段配置接口
interface WorkSession {
  start_time: string
  end_time: string
  duration_minutes: number
}

interface WorkHoursDetail {
  date: string
  morning_session: WorkSession
  afternoon_session: WorkSession
  total_hours: number
  total_minutes: number
  is_custom: boolean
  description?: string
}

const currentView = ref<ViewType>('day')
const currentDate = ref(dayjs())
const miniCalendarDate = ref(dayjs())
const calendarData = ref<MonthCalendarData | null>(null)
const dayWorkRecords = ref<DayWorkRecord[]>([])
const inProgressTasks = ref<any[]>([]) // 进行中的任务（跨天）
const archivedTasks = ref<any[]>([]) // 当天归档的任务
const loading = ref(false)

// 当天工作时长配置
const dayWorkHours = ref<WorkHoursDetail | null>(null)

// 解析时间字符串为小时和分钟
const parseTime = (timeStr: string): { hour: number; minute: number } => {
  const [hour, minute] = timeStr.split(':').map(Number)
  return { hour, minute }
}

// 计算工作开始时间（从当天工作时长配置）
const workDayStart = computed(() => {
  if (!dayWorkHours.value) return 8.5 // 默认 8:30
  const { hour, minute } = parseTime(dayWorkHours.value.morning_session.start_time)
  return hour + minute / 60
})

// 计算工作结束时间（从当天工作时长配置）
const workDayEnd = computed(() => {
  if (!dayWorkHours.value) return 17.5 // 默认 17:30
  const { hour, minute } = parseTime(dayWorkHours.value.afternoon_session.end_time)
  return hour + minute / 60
})

// 上午工作时段
const morningSession = computed(() => {
  if (!dayWorkHours.value) {
    return { start: 8.5, end: 12, startTime: '08:30', endTime: '12:00' }
  }
  const start = parseTime(dayWorkHours.value.morning_session.start_time)
  const end = parseTime(dayWorkHours.value.morning_session.end_time)
  return {
    start: start.hour + start.minute / 60,
    end: end.hour + end.minute / 60,
    startTime: dayWorkHours.value.morning_session.start_time,
    endTime: dayWorkHours.value.morning_session.end_time
  }
})

// 下午工作时段
const afternoonSession = computed(() => {
  if (!dayWorkHours.value) {
    return { start: 13.5, end: 17.5, startTime: '13:30', endTime: '17:30' }
  }
  const start = parseTime(dayWorkHours.value.afternoon_session.start_time)
  const end = parseTime(dayWorkHours.value.afternoon_session.end_time)
  return {
    start: start.hour + start.minute / 60,
    end: end.hour + end.minute / 60,
    startTime: dayWorkHours.value.afternoon_session.start_time,
    endTime: dayWorkHours.value.afternoon_session.end_time
  }
})

// 计算包含所有任务的时间范围
const timelineStartHour = computed(() => {
  let minHour = Math.floor(workDayStart.value)
  
  // 检查所有任务，找到最早的开始时间
  dayTasks.value.forEach(task => {
    if (task.startTime) {
      const hour = timeToMinutes(task.startTime) / 60
      minHour = Math.min(minHour, Math.floor(hour))
    }
  })
  
  return minHour
})

const timelineEndHour = computed(() => {
  let maxHour = Math.ceil(workDayEnd.value)
  
  // 检查所有任务，找到最晚的结束时间
  dayTasks.value.forEach(task => {
    if (task.endTime) {
      const hour = timeToMinutes(task.endTime) / 60
      maxHour = Math.max(maxHour, Math.ceil(hour))
    } else if (task.startTime) {
      const hour = timeToMinutes(task.startTime) / 60 + 1
      maxHour = Math.max(maxHour, Math.ceil(hour))
    }
  })
  
  return maxHour
})

// 时间线显示的小时范围（只显示整点，包含所有任务）
const displayHours = computed(() => {
  const startHour = timelineStartHour.value
  const endHour = timelineEndHour.value
  return Array.from({ length: endHour - startHour + 1 }, (_, i) => startHour + i)
})

const weekdays = ['周一', '周二', '周三', '周四', '周五', '周六', '周日']
const miniWeekdays = ['S', 'M', 'T', 'W', 'T', 'F', 'S']

const weekDaysWithNames = computed(() => {
  return weekdays.map((weekday, index) => ({
    key: index,
    weekday,
    isWeekend: index >= 5
  }))
})

const setView = (view: ViewType) => {
  currentView.value = view
}

const loadCalendarData = async () => {
  loading.value = true
  try {
    const year = currentDate.value.year()
    const month = currentDate.value.month() + 1
    const result = await invoke<MonthCalendarData>('get_calendar_events', { year, month })
    calendarData.value = result
    console.log('Calendar data loaded:', result)

    // Day视图下需要额外加载工作记录、工作时长配置、进行中任务和归档任务
    if (currentView.value === 'day') {
      await loadDayWorkRecords()
      await loadDayWorkHours()
      await loadInProgressTasks()
      await loadArchivedTasks()
    }
  } catch (error) {
    console.error('加载日历数据失败:', error)
  } finally {
    loading.value = false
  }
}

const loadDayWorkRecords = async () => {
  try {
    const date = currentDate.value.format('YYYY-MM-DD')
    const result = await invoke<DayWorkRecord[]>('get_day_work_records', { date })
    dayWorkRecords.value = result
    console.log('Day work records loaded:', result)
  } catch (error) {
    console.error('加载工作记录失败:', error)
    dayWorkRecords.value = []
  }
}

// 加载当天工作时长配置
const loadDayWorkHours = async () => {
  try {
    const date = currentDate.value.format('YYYY-MM-DD')
    const result = await invoke<WorkHoursDetail>('get_work_hours_by_date', { date })
    dayWorkHours.value = result
    console.log('Day work hours loaded:', result)
  } catch (error) {
    console.error('加载工作时长配置失败:', error)
    dayWorkHours.value = null
  }
}

// 加载进行中的任务（跨天任务）
const loadInProgressTasks = async () => {
  try {
    // 获取所有进行中的任务（status = 1）
    const result = await invoke<any[]>('get_tasks_by_status', { status: 1, includeArchived: false })
    inProgressTasks.value = result
    console.log('In progress tasks loaded:', result)
  } catch (error) {
    console.error('加载进行中任务失败:', error)
    inProgressTasks.value = []
  }
}

// 加载当天归档的任务
const loadArchivedTasks = async () => {
  try {
    const date = currentDate.value.format('YYYY-MM-DD')
    const result = await invoke<any[]>('get_archived_tasks_by_date', { date })
    archivedTasks.value = result
    console.log('Archived tasks loaded:', result)
  } catch (error) {
    console.error('加载归档任务失败:', error)
    archivedTasks.value = []
  }
}

const prev = async () => {
  if (currentView.value === 'day') {
    currentDate.value = currentDate.value.subtract(1, 'day')
    await loadDayWorkRecords()
    await loadDayWorkHours()
    await loadInProgressTasks()
    await loadArchivedTasks()
  } else if (currentView.value === 'week') {
    currentDate.value = currentDate.value.subtract(1, 'week')
  } else {
    currentDate.value = currentDate.value.subtract(1, 'month')
  }
}

const next = async () => {
  if (currentView.value === 'day') {
    currentDate.value = currentDate.value.add(1, 'day')
    await loadDayWorkRecords()
    await loadDayWorkHours()
    await loadInProgressTasks()
    await loadArchivedTasks()
  } else if (currentView.value === 'week') {
    currentDate.value = currentDate.value.add(1, 'week')
  } else {
    currentDate.value = currentDate.value.add(1, 'month')
  }
}

const goToToday = async () => {
  currentDate.value = dayjs()
  miniCalendarDate.value = dayjs()
  if (currentView.value === 'day') {
    await loadDayWorkRecords()
    await loadDayWorkHours()
    await loadInProgressTasks()
    await loadArchivedTasks()
  }
}

const prevMonth = () => {
  miniCalendarDate.value = miniCalendarDate.value.subtract(1, 'month')
}

const nextMonth = () => {
  miniCalendarDate.value = miniCalendarDate.value.add(1, 'month')
}

const getTaskColorClass = (quadrant: number) => {
  const colors = ['blue', 'green', 'orange', 'purple']
  return colors[quadrant - 1] || 'blue'
}

const getStatusLabel = (status: string) => {
  const labels: Record<string, string> = {
    'planning': '规划中',
    'in_progress': '进行中',
    'paused': '已暂停',
    'completed': '已完成',
    'archived': '已归档'
  }
  return labels[status] || status
}

const getTypeLabel = (dateType: string) => {
  const labels: Record<string, string> = {
    'holiday': '休假',
    'makeup': '补班',
    'weekend': '周末',
    'workday': ''
  }
  return labels[dateType] || ''
}

const weekDays = computed(() => {
  const currentDayOfWeek = currentDate.value.day()
  const daysFromMonday = currentDayOfWeek === 0 ? 6 : currentDayOfWeek - 1
  const mondayOfWeek = currentDate.value.subtract(daysFromMonday, 'day')
  const days = []

  for (let i = 0; i < 7; i++) {
    const day = mondayOfWeek.add(i, 'day')
    const dateStr = day.format('YYYY-MM-DD')
    const dayData = calendarData.value?.days.find(d => d.date === dateStr)

    if (dayData) {
      days.push({
        dayNumber: dayData.day,
        date: dateStr,
        isToday: dayData.is_today,
        isWeekend: dayData.is_weekend,
        isHoliday: dayData.date_type === 'holiday',
        isMakeup: dayData.date_type === 'makeup',
        typeLabel: getTypeLabel(dayData.date_type),
        typeClass: dayData.date_type,
        tasks: dayData.tasks.map(t => ({
          id: t.id,
          taskId: t.task_id,
          title: t.title,
          description: t.description,
          startTime: t.start_time,
          endTime: t.end_time,
          color: t.color,
          taskStatus: t.task_status,
          taskQuadrant: t.task_quadrant
        }))
      })
    } else {
      days.push({
        dayNumber: day.date(),
        date: dateStr,
        isToday: day.isSame(dayjs(), 'day'),
        isWeekend: i >= 5,
        isHoliday: false,
        isMakeup: false,
        typeLabel: i >= 5 ? '周末' : '',
        typeClass: i >= 5 ? 'weekend' : 'workday',
        tasks: []
      })
    }
  }

  return days
})

// Day view tasks - 合并工作记录、日历任务和进行中任务
const dayTasks = computed((): Task[] => {
  const dateStr = currentDate.value.format('YYYY-MM-DD')
  const tasks: Task[] = []
  const taskIds = new Set<number>()

  // 1. 首先添加当天有工作记录的任务
  // 只添加非规划中状态的任务（status != 0）
  dayWorkRecords.value.forEach(record => {
    if (!taskIds.has(record.task_id) && record.task_status !== 0) {
      taskIds.add(record.task_id)
      tasks.push({
        id: record.task_id,
        title: record.task_title,
        startTime: record.start_time,
        endTime: record.end_time,
        taskQuadrant: record.task_quadrant,
        taskStatus: getStatusText(record.task_status),
        participants: [
          { name: 'User', avatar: `https://api.dicebear.com/7.x/avataaars/svg?seed=${record.task_id}` }
        ],
        tags: record.task_quadrant === 1 ? ['#URGENT', '#CORE'] :
              record.task_quadrant === 2 ? ['#STRATEGY', '#DEEPWORK'] :
              record.task_quadrant === 3 ? ['#EXTERNAL'] : ['#EMAIL']
      })
    }
  })

  // 2. 从日历数据中获取当天的任务
  const dayData = calendarData.value?.days.find(d => d.date === dateStr)
  if (dayData?.tasks) {
    dayData.tasks.forEach(t => {
      if (!taskIds.has(t.task_id)) {
        taskIds.add(t.task_id)
        tasks.push({
          id: t.task_id,
          title: t.title,
          startTime: t.start_time,
          endTime: t.end_time,
          taskQuadrant: t.task_quadrant,
          taskStatus: t.task_status,
          participants: [
            { name: 'User', avatar: `https://api.dicebear.com/7.x/avataaars/svg?seed=${t.task_id}` }
          ],
          tags: t.task_quadrant === 1 ? ['#URGENT', '#CORE'] :
                t.task_quadrant === 2 ? ['#STRATEGY', '#DEEPWORK'] :
                t.task_quadrant === 3 ? ['#EXTERNAL'] : ['#EMAIL']
        })
      }
    })
  }

  // 3. 添加进行中的任务（跨天任务）
  // 只添加有今天工作记录的进行中的任务
  inProgressTasks.value.forEach(task => {
    if (!taskIds.has(task.id)) {
      // 查找该任务在今天的工作记录
      const todayWorkRecords = dayWorkRecords.value.filter(
        record => record.task_id === task.id
      )

      // 如果有今天的工作记录，则显示
      if (todayWorkRecords.length > 0) {
        taskIds.add(task.id)

        // 使用工作记录的开始和结束时间
        todayWorkRecords.forEach(record => {
          tasks.push({
            id: task.id,
            title: task.title,
            startTime: record.start_time,
            endTime: record.end_time,
            taskQuadrant: task.quadrant ? (typeof task.quadrant === 'string' ? ['', 'ImportantUrgent', 'ImportantNotUrgent', 'NotImportantUrgent', 'NotImportantNotUrgent'].indexOf(task.quadrant) : task.quadrant) : 1,
            taskStatus: 'in_progress',
            participants: [
              { name: 'User', avatar: `https://api.dicebear.com/7.x/avataaars/svg?seed=${task.id}` }
            ],
            tags: task.quadrant === 'ImportantUrgent' || task.quadrant === 1 ? ['#URGENT', '#CORE'] :
                  task.quadrant === 'ImportantNotUrgent' || task.quadrant === 2 ? ['#STRATEGY', '#DEEPWORK'] :
                  task.quadrant === 'NotImportantUrgent' || task.quadrant === 3 ? ['#EXTERNAL'] : ['#EMAIL']
          })
        })
      }
    }
  })

  // 4. 添加当天归档的任务
  // 只添加有今天工作记录的归档任务
  archivedTasks.value.forEach(task => {
    if (!taskIds.has(task.id)) {
      // 查找该任务在今天的工作记录
      const todayWorkRecords = dayWorkRecords.value.filter(
        record => record.task_id === task.id
      )

      // 如果有今天的工作记录，则显示
      if (todayWorkRecords.length > 0) {
        taskIds.add(task.id)

        // 使用工作记录的开始和结束时间
        todayWorkRecords.forEach(record => {
          tasks.push({
            id: task.id,
            title: task.title,
            startTime: record.start_time,
            endTime: record.end_time,
            taskQuadrant: task.quadrant || 1,
            taskStatus: 'archived',
            participants: [
              { name: 'User', avatar: `https://api.dicebear.com/7.x/avataaars/svg?seed=${task.id}` }
            ],
            tags: task.quadrant === 1 ? ['#URGENT', '#CORE'] :
                  task.quadrant === 2 ? ['#STRATEGY', '#DEEPWORK'] :
                  task.quadrant === 3 ? ['#EXTERNAL'] : ['#EMAIL']
          })
        })
      }
    }
  })

  return tasks
})

// 检查是否是今天
const isToday = computed(() => {
  return currentDate.value.isSame(dayjs(), 'day')
})

// 当前时间线位置（相对于时间线开始）
const currentTimeLineStyle = computed(() => {
  const now = dayjs()
  const hour = now.hour()
  const minute = now.minute()
  const offsetMinutes = hour * 60 + minute
  const timelineStartMinutes = timelineStartHour.value * 60
  const timelineEndMinutes = timelineEndHour.value * 60
  
  // 如果当前时间在时间线范围内才显示
  if (offsetMinutes < timelineStartMinutes || offsetMinutes > timelineEndMinutes) {
    return { display: 'none' }
  }
  
  const relativeMinutes = offsetMinutes - timelineStartMinutes
  const top = relativeMinutes * 1.5 // 1.5px per minute
  return {
    top: `${top}px`
  }
})

// 工作时间背景样式（显示上午和下午两个时段，中间跳过午休）
const workHoursBgStyle = computed(() => {
  const minuteHeight = 1.5
  const timelineStartMinutes = timelineStartHour.value * 60
  
  // 上午时段
  const morningStartMinutes = morningSession.value.start * 60 - timelineStartMinutes
  const morningEndMinutes = morningSession.value.end * 60 - timelineStartMinutes
  const morningHeight = (morningEndMinutes - morningStartMinutes) * minuteHeight
  
  // 返回上午时段的样式
  return {
    top: `${morningStartMinutes * minuteHeight}px`,
    height: `${morningHeight}px`
  }
})

// 下午工作时段背景样式
const afternoonWorkHoursBgStyle = computed(() => {
  const minuteHeight = 1.5
  const timelineStartMinutes = timelineStartHour.value * 60
  
  // 下午时段
  const afternoonStartMinutes = afternoonSession.value.start * 60 - timelineStartMinutes
  const afternoonEndMinutes = afternoonSession.value.end * 60 - timelineStartMinutes
  const afternoonHeight = (afternoonEndMinutes - afternoonStartMinutes) * minuteHeight
  
  return {
    top: `${afternoonStartMinutes * minuteHeight}px`,
    height: `${afternoonHeight}px`
  }
})

// 将时间转换为分钟数（从0点开始）
const timeToMinutes = (timeStr: string): number => {
  // 处理 ISO 格式时间字符串，如 "2026-04-08T08:30:00" 或 "08:30:00"
  let timePart = timeStr
  
  // 如果是完整的 ISO 日期时间格式，提取时间部分
  if (timeStr.includes('T')) {
    timePart = timeStr.split('T')[1]
  }
  
  // 解析时间部分
  const parts = timePart.split(':')
  const hour = parseInt(parts[0] || '0')
  const minute = parseInt(parts[1] || '0')
  
  return hour * 60 + minute
}

// 将时间转换为相对于时间线开始的分钟数（用于定位）
const timeToRelativeMinutes = (timeStr: string): number => {
  const totalMinutes = timeToMinutes(timeStr)
  const timelineStartMinutes = timelineStartHour.value * 60
  return totalMinutes - timelineStartMinutes
}

// 检测任务时间是否重叠
const checkTaskOverlap = (task1: Task, task2: Task): boolean => {
  const start1 = timeToMinutes(task1.startTime)
  const end1 = task1.endTime ? timeToMinutes(task1.endTime) : start1 + 60
  const start2 = timeToMinutes(task2.startTime)
  const end2 = task2.endTime ? timeToMinutes(task2.endTime) : start2 + 60
  
  return start1 < end2 && start2 < end1
}

// 计算任务布局（处理重叠）
const positionedTasksWithLayout = computed((): Task[] => {
  console.log('dayTasks:', dayTasks.value)
  console.log('dayWorkRecords:', dayWorkRecords.value)
  console.log('calendarData:', calendarData.value)
  
  const tasks = dayTasks.value.filter(task => task.startTime)
  console.log('Filtered tasks with startTime:', tasks)
  
  // 计算每个任务的持续时间
  const tasksWithMeta = tasks.map(task => {
    const startMinutes = timeToMinutes(task.startTime)
    const endMinutes = task.endTime ? timeToMinutes(task.endTime) : startMinutes + 60
    const durationMinutes = endMinutes - startMinutes
    
    return {
      ...task,
      durationMinutes,
      _startMinutes: startMinutes,
      _endMinutes: endMinutes
    }
  })
  
  // 按开始时间排序
  tasksWithMeta.sort((a, b) => a._startMinutes - b._startMinutes)
  
  // 检测冲突并分组
  const conflictGroups: typeof tasksWithMeta[] = []
  
  tasksWithMeta.forEach(task => {
    let added = false
    for (const group of conflictGroups) {
      // 检查是否与组内任何任务冲突
      const hasConflict = group.some(t => checkTaskOverlap(t, task))
      if (hasConflict) {
        group.push(task)
        added = true
        break
      }
    }
    if (!added) {
      conflictGroups.push([task])
    }
  })
  
  // 为每个任务分配冲突索引
  const result: Task[] = []
  conflictGroups.forEach(group => {
    const total = group.length
    group.forEach((task, index) => {
      result.push({
        ...task,
        hasConflict: total > 1,
        conflictIndex: index,
        conflictTotal: total
      })
    })
  })
  
  return result
})

const getTaskCardStyle = (task: Task) => {
  const startMinutes = timeToMinutes(task.startTime)
  const endMinutes = task.endTime ? timeToMinutes(task.endTime) : startMinutes + 60
  const duration = endMinutes - startMinutes

  // 计算相对于工作时间开始的偏移量
  const relativeStartMinutes = timeToRelativeMinutes(task.startTime)

  // 每个小时 = 90px, 所以 1分钟 = 1.5px
  const minuteHeight = 1.5
  const top = relativeStartMinutes * minuteHeight
  // 高度与持续时间成比例，最小24px确保能显示内容
  const height = Math.max(duration * minuteHeight - 2, 24)

  // 处理冲突任务的偏移
  let left = '60px'
  const cardWidth = 100 // 固定宽度100px

  if (task.hasConflict && task.conflictTotal && task.conflictIndex !== undefined) {
    // 冲突任务并排显示，每个任务偏移 cardWidth + 10px 间隙
    const offset = task.conflictIndex * (cardWidth + 10)
    left = `${60 + offset}px`
  }

  return {
    top: `${top}px`,
    height: `${height}px`,
    left
  }
}

// 格式化短时长（用于徽章）
const formatDurationShort = (minutes: number): string => {
  const hours = Math.floor(minutes / 60)
  const mins = minutes % 60
  if (hours > 0) {
    return `${hours}h${mins > 0 ? mins + 'm' : ''}`
  }
  return `${mins}m`
}

const formatHour = (hour: number) => {
  return `${String(hour).padStart(2, '0')}:00`
}

const formatTaskTime = (task: Task) => {
  // 提取时间部分（支持 ISO 格式和纯时间格式）
  const extractTime = (timeStr: string): string => {
    if (timeStr.includes('T')) {
      return timeStr.split('T')[1].substring(0, 5)
    }
    return timeStr.substring(0, 5)
  }
  
  const start = extractTime(task.startTime)
  if (!task.endTime) return start
  const end = extractTime(task.endTime)
  return `${start} - ${end}`
}

const getStatusText = (status: number): string => {
  const statusMap: Record<number, string> = {
    0: 'planning',
    1: 'in_progress',
    2: 'paused',
    3: 'completed',
    4: 'archived'
  }
  return statusMap[status] || 'planning'
}

// 获取标签颜色类
const getTagColorClass = (tag: string): string => {
  const tagLower = tag.toLowerCase()
  if (tagLower.includes('urgent') || tagLower.includes('紧急')) return 'tag-urgent'
  if (tagLower.includes('core') || tagLower.includes('核心')) return 'tag-core'
  if (tagLower.includes('strategy') || tagLower.includes('战略')) return 'tag-strategy'
  if (tagLower.includes('deepwork') || tagLower.includes('深度')) return 'tag-deepwork'
  if (tagLower.includes('external') || tagLower.includes('外部')) return 'tag-external'
  if (tagLower.includes('email') || tagLower.includes('邮件')) return 'tag-email'
  return 'tag-default'
}

// Mini calendar days
const miniCalendarDays = computed(() => {
  const year = miniCalendarDate.value.year()
  const month = miniCalendarDate.value.month()
  const firstDay = dayjs(new Date(year, month, 1))
  const lastDay = dayjs(new Date(year, month + 1, 0))
  const startOfWeek = firstDay.day()
  const daysInMonth = lastDay.date()
  
  const days = []
  
  // Previous month days
  const prevMonthDays = startOfWeek === 0 ? 6 : startOfWeek - 1
  const prevMonth = firstDay.subtract(1, 'month')
  const daysInPrevMonth = prevMonth.daysInMonth()
  
  for (let i = prevMonthDays - 1; i >= 0; i--) {
    days.push({
      day: daysInPrevMonth - i,
      date: prevMonth.date(daysInPrevMonth - i).format('YYYY-MM-DD'),
      isCurrentMonth: false,
      isToday: false,
      isSelected: false
    })
  }
  
  // Current month days
  const today = dayjs()
  for (let i = 1; i <= daysInMonth; i++) {
    const date = dayjs(new Date(year, month, i))
    days.push({
      day: i,
      date: date.format('YYYY-MM-DD'),
      isCurrentMonth: true,
      isToday: date.isSame(today, 'day'),
      isSelected: date.isSame(currentDate.value, 'day')
    })
  }
  
  // Next month days
  const remainingDays = 42 - days.length
  const nextMonth = firstDay.add(1, 'month')
  for (let i = 1; i <= remainingDays; i++) {
    days.push({
      day: i,
      date: nextMonth.date(i).format('YYYY-MM-DD'),
      isCurrentMonth: false,
      isToday: false,
      isSelected: false
    })
  }
  
  return days
})

const showAddTaskModal = () => {
  console.log('Add task clicked')
}

watch(currentDate, async (newDate, oldDate) => {
  // 只有在Day视图下且日期真正变化时才加载工作记录
  if (currentView.value === 'day' && newDate.format('YYYY-MM-DD') !== oldDate?.format('YYYY-MM-DD')) {
    await loadDayWorkRecords()
    await loadDayWorkHours()
    await loadInProgressTasks()
    await loadArchivedTasks()
  }
  loadCalendarData()
}, { immediate: false })

watch(currentView, (newView) => {
  if (newView === 'day') {
    loadDayWorkRecords()
    loadDayWorkHours()
    loadInProgressTasks()
    loadArchivedTasks()
  }
  loadCalendarData()
})

watch(() => currentDate.value.format('YYYY-MM'), (newMonth) => {
  miniCalendarDate.value = currentDate.value
})

onMounted(() => {
  loadCalendarData()
  if (currentView.value === 'day') {
    loadDayWorkRecords()
    loadDayWorkHours()
    loadInProgressTasks()
    loadArchivedTasks()
  }
})
</script>

<style scoped>
.calendar-page {
  padding: 24px 32px;
  min-height: 100vh;
  background: var(--bg-page);
  position: relative;
}

/* Header */
.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
}

.header-title .title {
  font-size: 20px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
}

.header-nav-center {
  flex: 1;
  display: flex;
  justify-content: center;
}

.header-actions-right {
  display: flex;
  align-items: center;
  gap: 12px;
}

.icon-btn {
  width: 36px;
  height: 36px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  background: transparent;
  border-radius: 8px;
  cursor: pointer;
  color: var(--text-secondary);
  transition: all 0.2s ease;
}

.icon-btn:hover {
  background: var(--bg-input);
}

.icon-btn svg {
  width: 18px;
  height: 18px;
}

.user-avatar {
  width: 36px;
  height: 36px;
  border-radius: 50%;
  overflow: hidden;
  cursor: pointer;
}

.user-avatar img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.view-switcher {
  display: flex;
  background: var(--bg-input);
  border-radius: 10px;
  padding: 4px;
}

.view-btn {
  padding: 8px 20px;
  background: transparent;
  border: none;
  border-radius: 8px;
  font-size: 13px;
  font-weight: 500;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.2s ease;
}

.view-btn.active {
  background: var(--bg-card);
  color: var(--color-primary);
  box-shadow: 0 1px 3px var(--border-color);
}

/* Calendar Section */
.calendar-section {
  background: var(--bg-card);
  border-radius: 20px;
  padding: 24px;
  box-shadow: 0 1px 3px var(--bg-hover);
}

.weekday-headers {
  display: grid;
  grid-template-columns: repeat(7, 1fr);
  gap: 8px;
  margin-bottom: 16px;
  padding-bottom: 16px;
  border-bottom: 1px solid var(--bg-input);
}

.weekday {
  text-align: center;
  font-size: 12px;
  font-weight: 600;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.weekday.is-weekend {
  color: var(--text-tertiary);
}

/* Day View Container */
.day-view-container {
  display: grid;
  grid-template-columns: 1fr 320px;
  gap: 24px;
  min-height: 600px;
}

/* Day Timeline Section */
.day-timeline-section {
  display: flex;
  flex-direction: column;
}

.day-header-row {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 24px;
}

.day-title-block {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.day-date-title {
  font-size: 24px;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0;
}

.day-task-count {
  font-size: 14px;
  color: var(--text-secondary);
  margin: 0;
}

.day-nav {
  display: flex;
  align-items: center;
  gap: 8px;
}

.nav-arrow {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: 1px solid var(--bg-input);
  background: var(--bg-card);
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s ease;
  color: var(--text-secondary);
}

.nav-arrow:hover {
  background: var(--bg-page);
  border-color: #d0d7de;
}

.nav-arrow svg {
  width: 14px;
  height: 14px;
}

.today-nav-btn {
  padding: 6px 14px;
  background: var(--bg-card);
  border: 1px solid var(--bg-input);
  border-radius: 8px;
  font-size: 13px;
  font-weight: 500;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.2s ease;
}

.today-nav-btn:hover {
  background: var(--bg-page);
  border-color: #d0d7de;
}

/* Timeline */
.timeline-container {
  flex: 1;
  overflow-y: auto;
  max-height: 600px;
}

.timeline {
  position: relative;
  padding-right: 16px;
  /* 动态计算高度：根据时间线范围 */
  min-height: calc((v-bind('timelineEndHour') - v-bind('timelineStartHour')) * 90px);
}

/* 工作时间背景 */
.work-hours-bg {
  position: absolute;
  left: 60px;
  right: 0;
  background: rgba(59, 130, 246, 0.03);
  border-left: 2px solid var(--color-primary-light);
  border-right: 2px solid var(--color-primary-light);
  pointer-events: none;
  z-index: 1;
}

.work-hours-bg.morning-bg {
  background: rgba(59, 130, 246, 0.03);
}

.work-hours-bg.afternoon-bg {
  background: rgba(59, 130, 246, 0.03);
}

.timeline-hour {
  display: flex;
  align-items: center;
  height: 90px;
  position: relative;
}

.hour-label {
  width: 50px;
  font-size: 12px;
  font-weight: 500;
  color: var(--text-tertiary);
  flex-shrink: 0;
}

.hour-line {
  flex: 1;
  height: 1px;
  background: var(--bg-input);
  margin-left: 12px;
}

/* 当前时间线 */
.current-time-line {
  position: absolute;
  left: 60px;
  right: 0;
  display: flex;
  align-items: center;
  z-index: 20;
  pointer-events: none;
}

.current-time-dot {
  width: 10px;
  height: 10px;
  background: var(--color-danger);
  border-radius: 50%;
  margin-left: -5px;
  box-shadow: 0 0 0 3px rgba(239, 68, 68, 0.3);
  animation: pulse 2s infinite;
}

@keyframes pulse {
  0%, 100% { box-shadow: 0 0 0 3px rgba(239, 68, 68, 0.3); }
  50% { box-shadow: 0 0 0 6px rgba(239, 68, 68, 0.1); }
}

.current-time-line-body {
  flex: 1;
  height: 2px;
  background: linear-gradient(90deg, var(--color-danger) 0%, rgba(239, 68, 68, 0.3) 100%);
}

/* Timeline Task Cards */
.timeline-task-card {
  position: absolute;
  left: 60px;
  width: 100px;
  border-radius: 8px;
  padding: 6px 8px;
  display: flex;
  flex-direction: column;
  gap: 2px;
  z-index: 10;
  cursor: pointer;
  transition: all 0.2s ease;
  overflow: hidden;
  box-sizing: border-box;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
}

.timeline-task-card:hover {
  transform: translateX(4px);
  box-shadow: 0 6px 20px rgba(0, 0, 0, 0.15);
  z-index: 15;
}

.timeline-task-card.blue {
  background: linear-gradient(135deg, var(--color-primary-light) 0%, rgba(59, 130, 246, 0.05) 100%);
  border-left: 4px solid var(--color-primary);
}

.timeline-task-card.green {
  background: linear-gradient(135deg, var(--color-success-bg) 0%, rgba(16, 185, 129, 0.05) 100%);
  border-left: 4px solid var(--color-success);
}

.timeline-task-card.orange {
  background: linear-gradient(135deg, var(--color-warning-bg) 0%, rgba(245, 158, 11, 0.05) 100%);
  border-left: 4px solid var(--color-warning);
}

.timeline-task-card.purple {
  background: linear-gradient(135deg, rgba(139, 92, 246, 0.15) 0%, rgba(139, 92, 246, 0.05) 100%);
  border-left: 4px solid #8b5cf6;
}

/* 任务时长徽章 */
.task-duration-badge {
  position: absolute;
  top: 4px;
  right: 4px;
  font-size: 8px;
  font-weight: 700;
  padding: 1px 4px;
  background: rgba(0, 0, 0, 0.6);
  color: white;
  border-radius: 3px;
  z-index: 5;
}

/* 任务状态指示器 */
.task-status-indicator {
  position: absolute;
  top: 6px;
  left: 4px;
  width: 3px;
  height: 3px;
  border-radius: 50%;
  background: var(--text-tertiary);
}

.task-status-indicator.planning {
  background: var(--text-tertiary);
}

.task-status-indicator.in_progress {
  background: var(--color-primary);
  animation: blink 1.5s infinite;
}

.task-status-indicator.paused {
  background: var(--color-warning);
}

.task-status-indicator.completed {
  background: var(--color-success);
}

.task-status-indicator.archived {
  background: var(--text-tertiary);
  border: 1px solid var(--text-tertiary);
}

/* 归档任务特殊样式 */
.timeline-task-card.is-archived {
  opacity: 0.7;
  background: linear-gradient(135deg, rgba(107, 114, 128, 0.1) 0%, rgba(107, 114, 128, 0.05) 100%);
  border-left: 4px solid #6b7280;
}

.timeline-task-card.is-archived .task-card-title {
  color: #6b7280;
  text-decoration: line-through;
}

.timeline-task-card.is-archived .task-card-time {
  color: #9ca3af;
}

@keyframes blink {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.4; }
}

.task-card-title {
  font-size: 11px;
  font-weight: 600;
  color: #1e293b;
  line-height: 1.2;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  max-width: 100%;
}

.timeline-task-card.blue .task-card-title {
  color: #1e40af;
}

.timeline-task-card.green .task-card-title {
  color: #065f46;
}

.timeline-task-card.orange .task-card-title {
  color: #92400e;
}

.timeline-task-card.purple .task-card-title {
  color: #5b21b6;
}

.task-card-time {
  font-size: 10px;
  font-weight: 500;
  color: var(--text-secondary);
  line-height: 1.2;
}

.task-participants {
  display: flex;
  align-items: center;
}

.participant-avatar {
  width: 18px;
  height: 18px;
  border-radius: 50%;
  border: 2px solid var(--bg-card);
  margin-left: -6px;
}

.participant-avatar:first-child {
  margin-left: 0;
}

.participant-more {
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background: var(--bg-input);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 9px;
  font-weight: 600;
  color: var(--text-secondary);
  border: 2px solid var(--bg-card);
  margin-left: -6px;
}

.task-tags {
  display: flex;
  gap: 3px;
  flex-wrap: wrap;
  margin-top: auto;
}

.task-tag-item {
  font-size: 8px;
  font-weight: 600;
  padding: 2px 5px;
  border-radius: 3px;
  line-height: 1.2;
  text-transform: uppercase;
  letter-spacing: 0.3px;
}

/* 标签颜色样式 */
.task-tag-item.tag-urgent {
  background: rgba(239, 68, 68, 0.15);
  color: #dc2626;
}

.task-tag-item.tag-core {
  background: rgba(59, 130, 246, 0.15);
  color: #2563eb;
}

.task-tag-item.tag-strategy {
  background: rgba(139, 92, 246, 0.15);
  color: #7c3aed;
}

.task-tag-item.tag-deepwork {
  background: rgba(16, 185, 129, 0.15);
  color: #059669;
}

.task-tag-item.tag-external {
  background: rgba(245, 158, 11, 0.15);
  color: #d97706;
}

.task-tag-item.tag-email {
  background: rgba(107, 114, 128, 0.15);
  color: #4b5563;
}

.task-tag-item.tag-default {
  background: var(--glass-bg);
  color: var(--text-secondary);
}

/* Day Sidebar */
.day-sidebar {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

/* Daily Focus Card */
.daily-focus-card {
  background: linear-gradient(135deg, var(--color-primary) 0%, var(--color-primary-hover) 100%);
  border-radius: 16px;
  padding: 20px;
  color: var(--bg-card);
}

.focus-label {
  font-size: 10px;
  font-weight: 600;
  letter-spacing: 1px;
  opacity: 0.8;
}

.focus-title {
  font-size: 16px;
  font-weight: 600;
  margin: 8px 0 16px 0;
  line-height: 1.4;
}

.focus-progress {
  display: flex;
  align-items: center;
  gap: 12px;
}

.progress-bar {
  flex: 1;
  height: 6px;
  background: rgba(255, 255, 255, 0.2);
  border-radius: 3px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background: var(--bg-card);
  border-radius: 3px;
  transition: width 0.3s ease;
}

.progress-text {
  font-size: 12px;
  font-weight: 600;
}

/* Stats Section */
.stats-section {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.stat-item {
  background: var(--bg-card);
  border-radius: 12px;
  padding: 16px;
}

.stat-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 8px;
}

.stat-icon {
  width: 24px;
  height: 24px;
  border-radius: 6px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.stat-icon.efficiency {
  background: var(--color-primary-light);
  color: var(--color-primary);
}

.stat-icon.intensity {
  background: var(--color-warning-bg);
  color: var(--color-warning);
}

.stat-icon svg {
  width: 14px;
  height: 14px;
}

.stat-label-small {
  font-size: 10px;
  font-weight: 600;
  color: var(--text-tertiary);
  letter-spacing: 0.5px;
}

.stat-value-large {
  font-size: 24px;
  font-weight: 700;
  color: var(--text-primary);
  margin-bottom: 4px;
}

.stat-desc {
  font-size: 12px;
  color: var(--text-secondary);
}

/* Mini Calendar */
.mini-calendar {
  background: var(--bg-card);
  border-radius: 12px;
  padding: 16px;
}

.mini-calendar-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.mini-calendar-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
}

.mini-calendar-nav {
  display: flex;
  gap: 4px;
}

.mini-calendar-nav button {
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  background: transparent;
  border-radius: 4px;
  cursor: pointer;
  color: var(--text-secondary);
  transition: all 0.2s ease;
}

.mini-calendar-nav button:hover {
  background: var(--bg-input);
}

.mini-calendar-nav svg {
  width: 12px;
  height: 12px;
}

.mini-calendar-grid {
  display: grid;
  grid-template-columns: repeat(7, 1fr);
  gap: 4px;
}

.mini-calendar-day {
  aspect-ratio: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 12px;
  font-weight: 500;
  color: var(--text-tertiary);
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.mini-calendar-day.is-current {
  color: #1e293b;
}

.mini-calendar-day.is-today {
  background: var(--color-primary);
  color: var(--bg-card);
}

.mini-calendar-day.is-selected {
  background: var(--bg-input);
  color: var(--color-primary);
  font-weight: 600;
}

.mini-calendar-day:hover:not(.is-today):not(.is-selected) {
  background: var(--bg-input);
}

/* Upcoming Section */
.upcoming-section {
  background: var(--bg-card);
  border-radius: 12px;
  padding: 16px;
}

.upcoming-label {
  font-size: 10px;
  font-weight: 600;
  color: var(--text-tertiary);
  letter-spacing: 0.5px;
  display: block;
  margin-bottom: 12px;
}

.upcoming-item {
  display: flex;
  gap: 12px;
  align-items: flex-start;
}

.upcoming-indicator {
  width: 3px;
  height: 40px;
  background: var(--color-primary);
  border-radius: 2px;
  flex-shrink: 0;
}

.upcoming-content {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.upcoming-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
}

.upcoming-time {
  font-size: 12px;
  color: var(--text-secondary);
}

/* Floating Action Button */
.fab-add-btn {
  position: fixed;
  bottom: 32px;
  right: 32px;
  width: 56px;
  height: 56px;
  border-radius: 50%;
  background: var(--color-primary);
  border: none;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  box-shadow: 0 4px 16px rgba(0, 113, 227, 0.3);
  transition: all 0.2s ease;
  z-index: 100;
}

.fab-add-btn:hover {
  transform: scale(1.05);
  box-shadow: 0 6px 20px rgba(0, 102, 204, 0.4);
}

.fab-add-btn svg {
  width: 24px;
  height: 24px;
  color: var(--bg-card);
}

/* Week View */
.week-view {
  padding: 8px 0;
}

.week-grid {
  background: var(--bg-card);
  border-radius: 12px;
  overflow: hidden;
}

.week-grid .weekday-headers {
  display: grid;
  grid-template-columns: repeat(7, 1fr);
  gap: 8px;
  padding: 16px 16px 12px 16px;
  margin-bottom: 0;
  background: var(--bg-card);
  border-bottom: 1px solid var(--bg-input);
}

.week-days {
  display: grid;
  grid-template-columns: repeat(7, 1fr);
  gap: 8px;
  padding: 16px;
}

.week-day {
  min-height: 140px;
  padding: 12px;
  border-radius: 12px;
  background: var(--bg-card);
  border: 1px solid var(--bg-input);
  transition: all 0.2s ease;
  cursor: pointer;
  display: flex;
  flex-direction: column;
  position: relative;
}

.week-day:hover {
  border-color: var(--border-color);
  box-shadow: 0 2px 8px var(--bg-hover);
}

.week-day.weekend {
  background: var(--bg-card);
}

.week-day.holiday {
  background: var(--color-success-bg);
  border: 2px solid var(--color-success);
}

.week-day.makeup {
  background: var(--bg-card)7ed;
  border: 2px solid #fb923c;
}

.week-day.today {
  background: linear-gradient(135deg, var(--color-primary) 0%, var(--color-primary-hover) 100%);
  color: var(--bg-card);
  border: none;
}

.week-day.today.weekend {
  background: linear-gradient(135deg, var(--text-secondary) 0%, var(--text-secondary) 100%);
}

.week-day.today.holiday {
  background: linear-gradient(135deg, var(--color-success) 0%, var(--color-success) 100%);
}

.week-day.today.makeup {
  background: linear-gradient(135deg, #ea580c 0%, #c2410c 100%);
}

.day-header-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.day-number {
  font-size: 14px;
  font-weight: 600;
  color: #1e293b;
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
}

.day-number.is-today {
  background: var(--color-primary);
  color: var(--bg-card);
}

.week-day.today .day-number {
  color: var(--bg-card);
}

.week-day.today .day-number.is-today {
  background: rgba(255, 255, 255, 0.3);
}

.day-type-indicator {
  font-size: 10px;
  font-weight: 600;
  padding: 2px 6px;
  border-radius: 4px;
}

.day-type-indicator.holiday {
  background: var(--color-success-bg);
  color: var(--color-success);
}

.day-type-indicator.makeup {
  background: var(--color-warning-bg);
  color: #c2410c;
}

.day-type-indicator.weekend {
  background: var(--border-color);
  color: var(--text-secondary);
}

.week-day.today .day-type-indicator {
  background: rgba(255, 255, 255, 0.2);
  color: var(--bg-card);
}

.day-tasks {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.task-tag {
  padding: 4px 8px;
  border-radius: 6px;
  font-size: 11px;
  font-weight: 500;
  var(--bg-card)-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  display: flex;
  align-items: center;
  gap: 6px;
}

.task-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  flex-shrink: 0;
}

.task-tag.blue {
  background: var(--color-primary-light);
  color: #1d4ed8;
  font-weight: 600;
}

.task-tag.blue .task-dot {
  background: var(--color-primary-hover);
}

.task-tag.green {
  background: var(--color-success-bg);
  color: var(--color-success);
  font-weight: 600;
}

.task-tag.green .task-dot {
  background: var(--color-success);
}

.task-tag.orange {
  background: var(--color-warning-bg);
  color: #b45309;
  font-weight: 600;
}

.task-tag.orange .task-dot {
  background: #d97706;
}

.task-tag.purple {
  background: rgba(139, 92, 246, 0.18);
  color: #6d28d9;
  font-weight: 600;
}

.task-tag.purple .task-dot {
  background: #7c3aed;
}

.week-day.today .task-tag {
  background: rgba(255, 255, 255, 0.2);
  color: var(--bg-card);
}

.week-day.today .task-tag .task-dot {
  background: var(--bg-card);
}

.more-tasks {
  font-size: 10px;
  color: var(--text-tertiary);
  padding: 2px 8px;
}

.week-day.today .more-tasks {
  color: rgba(255, 255, 255, 0.7);
}

.today-badge {
  position: absolute;
  top: 8px;
  right: 8px;
  font-size: 10px;
  font-weight: 600;
  padding: 2px 8px;
  background: rgba(255, 255, 255, 0.3);
  border-radius: 4px;
}

/* Month View */
.month-view {
  padding: 8px 0;
}

.calendar-days {
  display: grid;
  grid-template-columns: repeat(7, 1fr);
  gap: 8px;
}

.day-cell {
  min-height: 100px;
  padding: 10px;
  border-radius: 12px;
  background: var(--bg-card);
  border: 1px solid var(--bg-input);
  transition: all 0.2s ease;
  cursor: pointer;
  display: flex;
  flex-direction: column;
}

.day-cell:hover {
  border-color: var(--border-color);
  box-shadow: 0 2px 8px var(--bg-hover);
}

.day-cell.other-month {
  background: var(--bg-card);
  opacity: 0.6;
}

.day-cell.weekend {
  background: var(--bg-card);
}

.day-cell.holiday {
  background: var(--color-success-bg);
  border: 2px solid var(--color-success);
}

.day-cell.makeup {
  background: var(--bg-card)7ed;
  border: 2px solid #fb923c;
}

.day-cell.today {
  background: linear-gradient(135deg, var(--color-primary) 0%, var(--color-primary-hover) 100%);
  color: var(--bg-card);
  border: none;
}

.day-cell.today.weekend {
  background: linear-gradient(135deg, var(--text-secondary) 0%, var(--text-secondary) 100%);
}

.day-cell.today.holiday {
  background: linear-gradient(135deg, var(--color-success) 0%, var(--color-success) 100%);
}

.day-cell.today.makeup {
  background: linear-gradient(135deg, #ea580c 0%, #c2410c 100%);
}

.day-cell .day-header-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 6px;
}

.day-cell .day-number {
  font-size: 13px;
  font-weight: 600;
  color: #1e293b;
}

.day-cell .day-number.is-today {
  background: var(--color-primary);
  color: var(--bg-card);
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
}

.day-cell.today .day-number {
  color: var(--bg-card);
}

.day-cell.today .day-number.is-today {
  background: rgba(255, 255, 255, 0.3);
}

.day-cell .day-type-indicator {
  font-size: 9px;
  font-weight: 600;
  padding: 1px 4px;
  border-radius: 3px;
}

.day-cell .day-type-indicator.holiday {
  background: var(--color-success-bg);
  color: var(--color-success);
}

.day-cell .day-type-indicator.makeup {
  background: var(--color-warning-bg);
  color: #c2410c;
}

.day-cell .day-type-indicator.weekend {
  background: var(--border-color);
  color: var(--text-secondary);
}

.day-cell.today .day-type-indicator {
  background: rgba(255, 255, 255, 0.2);
  color: var(--bg-card);
}

.day-cell .day-tasks {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 3px;
}

.day-cell .task-tag {
  padding: 3px 6px;
  border-radius: 4px;
  font-size: 10px;
  font-weight: 600;
  var(--bg-card)-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  display: flex;
  align-items: center;
  gap: 4px;
}

.day-cell .task-tag.blue {
  background: var(--color-primary-light);
  color: #1e40af;
}

.day-cell .task-tag.green {
  background: var(--color-success-bg);
  color: #065f46;
}

.day-cell .task-tag.orange {
  background: var(--color-warning-bg);
  color: #92400e;
}

.day-cell .task-tag.purple {
  background: rgba(139, 92, 246, 0.15);
  color: #5b21b6;
}

.day-cell .task-dot {
  width: 5px;
  height: 5px;
  border-radius: 50%;
  flex-shrink: 0;
}

.day-cell .task-tag.blue .task-dot {
  background: var(--color-primary-hover);
}

.day-cell .task-tag.green .task-dot {
  background: var(--color-success);
}

.day-cell .task-tag.orange .task-dot {
  background: #d97706;
}

.day-cell .task-tag.purple .task-dot {
  background: #7c3aed;
}

.day-cell.today .task-tag {
  background: rgba(255, 255, 255, 0.25);
  color: var(--bg-card);
}

.day-cell.today .task-tag .task-dot {
  background: var(--bg-card);
}

.day-cell .more-tasks {
  font-size: 9px;
  color: var(--text-tertiary);
  padding: 1px 6px;
}

.day-cell.today .more-tasks {
  color: rgba(255, 255, 255, 0.7);
}

/* 响应式布局 */
@media (max-width: 1200px) {
  .day-view-container {
    grid-template-columns: 1fr 280px;
    gap: 16px;
  }
  
  .day-sidebar {
    width: 280px;
  }
}

@media (max-width: 992px) {
  .day-view-container {
    grid-template-columns: 1fr;
    gap: 20px;
  }
  
  .day-sidebar {
    width: 100%;
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 16px;
  }
  
  .daily-focus-card,
  .stats-section,
  .mini-calendar,
  .upcoming-section {
    width: 100%;
  }
}

@media (max-width: 768px) {
  .calendar-page {
    padding: 16px;
  }
  
  .page-header {
    flex-wrap: wrap;
    gap: 12px;
  }
  
  .header-nav-center {
    order: 3;
    width: 100%;
    justify-content: flex-start;
  }
  
  .day-header-row {
    flex-direction: column;
    gap: 12px;
  }
  
  .day-date-title {
    font-size: 18px;
  }
  
  .timeline-task-card {
    left: 50px;
    padding: 6px 10px;
  }
  
  .hour-label {
    width: 40px;
    font-size: 11px;
  }
  
  .task-card-title {
    font-size: 12px;
  }
  
  .fab-add-btn {
    width: 48px;
    height: 48px;
    bottom: 20px;
    right: 20px;
  }
}

@media (max-width: 480px) {
  .view-switcher {
    padding: 3px;
  }
  
  .view-btn {
    padding: 6px 12px;
    font-size: 12px;
  }
  
  .day-sidebar {
    grid-template-columns: 1fr;
  }
  
  .timeline-task-card {
    left: 45px;
    padding: 4px 8px;
    border-radius: 8px;
  }
  
  .task-card-title {
    font-size: 11px;
  }
  
  .task-card-time {
    font-size: 10px;
  }
}
</style>
