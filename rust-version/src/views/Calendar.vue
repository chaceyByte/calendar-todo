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

          <!-- Gantt Chart View -->
          <GanttChart 
            :tasks="dayTasks"
            :work-day-start="workDayStart"
            :work-day-end="workDayEnd"
            @edit-task="editTask"
          />
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
                  @click="editTask(task)"
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
        <!-- Month Navigation Header -->
        <div class="month-nav-header">
          <div class="month-title-block">
            <h2 class="month-date-title">{{ currentDate.format('YYYY年MM月') }}</h2>
          </div>
          <div class="month-nav">
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

        <!-- Weekday Headers -->
        <div class="weekday-headers">
          <div v-for="day in weekDaysWithNames" :key="day.key" class="weekday" :class="{ 'is-weekend': day.isWeekend }">
            {{ day.weekday }}
          </div>
        </div>

        <!-- Calendar Days -->
        <div v-if="loading" class="loading-indicator">
          <div class="spinner"></div>
          <span>Loading calendar...</span>
        </div>
        <div v-else-if="!calendarData?.days || calendarData.days.length === 0" class="empty-calendar">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="3" y="4" width="18" height="18" rx="2" ry="2"/>
            <line x1="16" y1="2" x2="16" y2="6"/>
            <line x1="8" y1="2" x2="8" y2="6"/>
            <line x1="3" y1="10" x2="21" y2="10"/>
          </svg>
          <p>无法加载日历数据</p>
        </div>
        <div v-else class="calendar-days">
          <div
            v-for="day in calendarData.days"
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
                @click="editTask(task)"
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
import GanttChart, { type GanttTask, type TimeSegment } from '../components/GanttChart.vue'

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

interface RawTask {
  id: number
  title: string
  quadrant?: number | string
  status?: number | string
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

// 上午工作时段（用于计算 workDayStart）
const morningSession = computed(() => {
  if (!dayWorkHours.value) {
    return { start: 8.5, end: 12 }
  }
  const start = parseTime(dayWorkHours.value.morning_session.start_time)
  const end = parseTime(dayWorkHours.value.morning_session.end_time)
  return {
    start: start.hour + start.minute / 60,
    end: end.hour + end.minute / 60
  }
})

// 下午工作时段（用于计算 workDayEnd）
const afternoonSession = computed(() => {
  if (!dayWorkHours.value) {
    return { start: 13.5, end: 17.5 }
  }
  const start = parseTime(dayWorkHours.value.afternoon_session.start_time)
  const end = parseTime(dayWorkHours.value.afternoon_session.end_time)
  return {
    start: start.hour + start.minute / 60,
    end: end.hour + end.minute / 60
  }
})

const weekdays = ['周一', '周二', '周三', '周四', '周五', '周六', '周日']

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
    const dayData = calendarData.value?.days?.find(d => d.date === dateStr)

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

// 将 UTC 时间字符串转换为本地时间的分钟数（从0点开始）
const timeStrToMinutes = (timeStr: string): number => {
  const normalized = timeStr.endsWith('Z') || timeStr.includes('+') ? timeStr : timeStr + 'Z'
  const d = dayjs(normalized)
  return d.hour() * 60 + d.minute()
}

// 从日期字符串提取时间部分的分钟数
const extractTimeMinutes = (dateTimeStr: string): number => {
  // 支持 ISO 格式 (2024-01-01T08:30:00Z) 或纯时间格式 (08:30:00)
  if (dateTimeStr.includes('T')) {
    return timeStrToMinutes(dateTimeStr)
  }
  const parts = dateTimeStr.split(':')
  return parseInt(parts[0]) * 60 + parseInt(parts[1])
}

// 获取任务标签
const getTaskTags = (quadrant: number): string[] => {
  switch (quadrant) {
    case 1: return ['#URGENT', '#CORE']
    case 2: return ['#STRATEGY', '#DEEPWORK']
    case 3: return ['#EXTERNAL']
    default: return ['#EMAIL']
  }
}

// 解析象限值（支持字符串和数字）
const parseQuadrant = (quadrant: number | string | undefined): number => {
  if (typeof quadrant === 'number') return quadrant
  if (typeof quadrant === 'string') {
    const map: Record<string, number> = {
      'ImportantUrgent': 1,
      'ImportantNotUrgent': 2,
      'NotImportantUrgent': 3,
      'NotImportantNotUrgent': 4
    }
    return map[quadrant] || 1
  }
  return 1
}

// Day view tasks - 甘特图格式，合并所有未归档任务的时间段
const dayTasks = computed((): GanttTask[] => {
  const dateStr = currentDate.value.format('YYYY-MM-DD')
  const isToday = currentDate.value.isSame(dayjs(), 'day')
  const taskMap = new Map<number, GanttTask>()

  // 工作开始时间的分钟数
  const workStartMinutes = Math.floor(workDayStart.value) * 60
  const workEndMinutes = Math.ceil(workDayEnd.value) * 60

  // 辅助：添加或合并任务段
  const addTaskSegment = (
    taskId: number,
    title: string,
    quadrant: number,
    status: string,
    startMinutes: number,
    endMinutes: number,
    isRunning: boolean = false
  ) => {
    // 限制在工作时间范围内
    const clampedStart = Math.max(startMinutes, workStartMinutes)
    const clampedEnd = Math.min(endMinutes, workEndMinutes)
    if (clampedStart >= clampedEnd) return

    const existing = taskMap.get(taskId)
    const segment: TimeSegment = {
      startMinutes: clampedStart,
      endMinutes: clampedEnd,
      isRunning
    }

    if (existing) {
      existing.segments.push(segment)
      // 如果有任意段是 running，任务状态更新为 in_progress
      if (isRunning) existing.taskStatus = 'in_progress'
    } else {
      taskMap.set(taskId, {
        id: taskId,
        title,
        taskQuadrant: quadrant,
        taskStatus: isRunning ? 'in_progress' : status,
        tags: getTaskTags(quadrant),
        segments: [segment]
      })
    }
  }

  // 1. 处理当天工作记录 - 每个记录生成一个时间段
  dayWorkRecords.value.forEach(record => {
    if (record.task_status === 0) return // 跳过规划中

    const startMinutes = extractTimeMinutes(record.start_time)
    const endMinutes = record.end_time
      ? extractTimeMinutes(record.end_time)
      : (isToday ? dayjs().hour() * 60 + dayjs().minute() : workEndMinutes)

    addTaskSegment(
      record.task_id,
      record.task_title,
      record.task_quadrant,
      getStatusText(record.task_status),
      startMinutes,
      endMinutes,
      !record.end_time && isToday
    )
  })

  // 2. 从日历数据中获取当天的任务（补充未在工作记录中的）
  const dayData = calendarData.value?.days.find(d => d.date === dateStr)
  if (dayData?.tasks) {
    dayData.tasks.forEach(t => {
      if (taskMap.has(t.task_id)) return // 已有工作记录，跳过
      if (!t.start_time) return

      const startMinutes = extractTimeMinutes(t.start_time)
      const endMinutes = t.end_time
        ? extractTimeMinutes(t.end_time)
        : (isToday ? dayjs().hour() * 60 + dayjs().minute() : workEndMinutes)

      addTaskSegment(
        t.task_id,
        t.title,
        t.task_quadrant,
        t.task_status,
        startMinutes,
        endMinutes,
        !t.end_time && isToday
      )
    })
  }

  // 3. 处理进行中的跨天任务（status = 1，未归档）
  // 如果没有今天的工作记录，从今天工作开始时间起算
  inProgressTasks.value.forEach((task: RawTask) => {
    if (taskMap.has(task.id)) return // 已有今天工作记录

    const quadrant = parseQuadrant(task.quadrant)
    const nowMinutes = isToday ? dayjs().hour() * 60 + dayjs().minute() : workEndMinutes

    addTaskSegment(
      task.id,
      task.title,
      quadrant,
      'in_progress',
      workStartMinutes,
      nowMinutes,
      isToday
    )
  })

  // 4. 处理当天归档的任务
  archivedTasks.value.forEach((task: RawTask) => {
    if (taskMap.has(task.id)) return // 已有今天工作记录

    const quadrant = parseQuadrant(task.quadrant)

    addTaskSegment(
      task.id,
      task.title,
      quadrant,
      'archived',
      workStartMinutes,
      workEndMinutes,
      false
    )
  })

  // 对每个任务的 segments 按开始时间排序并合并重叠段
  const result: GanttTask[] = []
  taskMap.forEach(task => {
    task.segments.sort((a, b) => a.startMinutes - b.startMinutes)

    // 合并重叠或相邻的段
    const merged: TimeSegment[] = []
    task.segments.forEach(seg => {
      const last = merged[merged.length - 1]
      if (last && seg.startMinutes <= last.endMinutes) {
        // 重叠或相邻，合并
        last.endMinutes = Math.max(last.endMinutes, seg.endMinutes)
        if (seg.isRunning) last.isRunning = true
      } else {
        merged.push({ ...seg })
      }
    })

    task.segments = merged
    result.push(task)
  })

  // 按任务 ID 排序（稳定展示）
  result.sort((a, b) => a.id - b.id)

  return result
})

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

const editTask = (taskOrId: number | { id: number; title?: string }) => {
  const id = typeof taskOrId === 'number' ? taskOrId : taskOrId.id
  if ((window as any).openEditTaskDialog) {
    (window as any).openEditTaskDialog({ id })
  }
}

watch(currentDate, async (newDate, oldDate) => {
  if (currentView.value === 'day' && oldDate && newDate.format('YYYY-MM-DD') !== oldDate.format('YYYY-MM-DD')) {
    await loadDayWorkRecords()
    await loadDayWorkHours()
    await loadInProgressTasks()
    await loadArchivedTasks()
  }
  await loadCalendarData()
}, { immediate: true })

watch(currentView, async (newView) => {
  if (newView === 'day') {
    await Promise.all([
      loadDayWorkRecords(),
      loadDayWorkHours(),
      loadInProgressTasks(),
      loadArchivedTasks()
    ])
  }
  await loadCalendarData()
})

watch(() => currentDate.value.format('YYYY-MM'), () => {
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
  white-space: nowrap;
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

.month-nav-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.month-title-block {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.month-date-title {
  font-size: 20px;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0;
}

.month-nav {
  display: flex;
  align-items: center;
  gap: 8px;
}

.loading-indicator {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 0;
  gap: 12px;
}

.spinner {
  width: 40px;
  height: 40px;
  border: 3px solid var(--bg-input);
  border-top-color: var(--color-primary);
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.loading-indicator span {
  font-size: 14px;
  color: var(--text-secondary);
}

.empty-calendar {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 0;
  gap: 12px;
  color: var(--text-tertiary);
}

.empty-calendar svg {
  width: 48px;
  height: 48px;
}

.empty-calendar p {
  font-size: 14px;
  margin: 0;
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
  white-space: nowrap;
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
