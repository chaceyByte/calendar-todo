<template>
  <div
      class="calendar-container"
      ref="calendarContainer"
      tabindex="0"
      @click="handleClickToFocus"
  >
    <!-- 日历头部控制栏 -->
    <div class="calendar-header">
      <div class="header-controls">
        <el-button-group>
          <el-button @click="prevMonth">
            <el-icon>
              <arrow-left/>
            </el-icon>
            上个月
          </el-button>
          <el-button @click="nextMonth">
            下个月
            <el-icon>
              <arrow-right/>
            </el-icon>
          </el-button>
        </el-button-group>

        <span class="current-month">{{ currentMonthText }}</span>

        <el-button-group>
          <el-button @click="exportDailyReport" type="primary">
            <el-icon>
              <document/>
            </el-icon>
            导出日报
          </el-button>
          <el-button @click="exportWeeklyReport" type="success">
            <el-icon>
              <files/>
            </el-icon>
            导出周报
          </el-button>
        </el-button-group>
      </div>
    </div>

    <!-- 日历主体 -->
    <div class="calendar-body">
      <!-- 星期标题 -->
      <div class="week-header">
        <div v-for="day in weekDays" :key="day" class="week-day">
          {{ day }}
        </div>
      </div>

      <!-- 日期格子 -->
      <div class="calendar-grid">
        <div
            v-for="day in calendarDays"
            :key="day.date"
            :class="[
            'calendar-day',
            {
              'today': day.isToday,
              'current-month': day.isCurrentMonth,
              'has-tasks': day.tasks.length > 0,
              'has-activities': day.activities.length > 0
            }
          ]"
            @contextmenu="(e) => handleDayContextMenu(e, day)"
        >
          <div class="day-header">
            <span class="day-number">{{ day.day }}</span>
            <div class="day-indicators">
              <el-badge
                  v-if="day.tasks.length > 0"
                  :value="day.tasks.length"
                  type="primary"
                  class="task-badge"
              />
              <div
                  v-if="day.totalActivityTime > 0"
                  class="activity-indicator"
                  :title="`活动时间: ${activityStore.formatDuration(day.totalActivityTime)}`"
              >
                <el-icon>
                  <clock/>
                </el-icon>
                <span class="activity-time">{{ formatShortDuration(day.totalActivityTime) }}</span>
              </div>
            </div>
          </div>

          <div class="day-content">
            <div class="day-tasks">
              <div
                  v-for="task in day.tasks.slice(0, 2)"
                  :key="task.id"
                  :class="['task-item', `status-${task.status}`]"
                  :title="task.title"
              >
                {{ task.title }}
              </div>
            </div>

            <div class="day-activities">
              <div
                  v-for="activity in day.activities.slice(0, 2)"
                  :key="activity.id"
                  :class="['activity-item', `type-${activity.activityType.toLowerCase()}`]"
                  :title="`${activity.description || activity.activityType} (${formatShortDuration(activity.durationMinutes || 0)})`"
              >
                <el-icon>
                  <circle/>
                </el-icon>
                <span class="activity-text">{{ activity.description || activity.activityType }}</span>
              </div>
            </div>

            <div v-if="day.tasks.length > 2 || day.activities.length > 2" class="more-items">
              +{{ (day.tasks.length - 2) + (day.activities.length - 2) }}更多
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 右键菜单 -->
    <div
        v-if="contextMenu.visible"
        class="context-menu"
        :style="{ left: contextMenu.x + 'px', top: contextMenu.y + 'px' }"
        @click="closeContextMenu">
      <div class="menu-item" @click="copyActiveTasks(contextMenu.selectedDay?.date || '')">
        <el-icon>
          <document/>
        </el-icon>
        复制日报
      </div>
      <div class="menu-item" @click="viewDayTasks">
        <el-icon>
          <view/>
        </el-icon>
        查看任务
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {computed, onMounted, onUnmounted, ref} from 'vue'
import dayjs from 'dayjs'
import {ArrowLeft, ArrowRight, Clock, Document, Files} from '@element-plus/icons-vue'
import {useTaskStore} from '@/stores/task'
import {useActivityStore} from '@/stores/activity'

interface Task {
  id: number
  title: string
  description?: string
  status: string
  progress: number
  priority?: 'low' | 'medium' | 'high'
  startDate?: string
  endDate?: string
  tags?: string[]
  createdAt: string
  updatedAt: string
  completed: boolean
}

interface ActivityRecord {
  id: number
  taskId: number
  startTime: string
  endTime?: string
  activityType: string
  description?: string
  durationMinutes?: number
}

interface CalendarDay {
  date: string
  day: number
  isToday: boolean
  isCurrentMonth: boolean
  tasks: Task[]
  activities: ActivityRecord[]
  totalActivityTime: number
}

const currentDate = ref(dayjs())
const contextMenu = ref({
  visible: false,
  x: 0,
  y: 0,
  selectedDay: null as CalendarDay | null
})
const calendarContainer = ref<HTMLElement | null>(null)

const weekDays = ['日', '一', '二', '三', '四', '五', '六']

const taskStore = useTaskStore()
const activityStore = useActivityStore()
// 手动点击获取焦点
const handleClickToFocus = () => {
  if (calendarContainer.value) {
    calendarContainer.value.focus()
  }
}


// 任务和活动数据
const tasks = ref<Task[]>([])
const activities = ref<ActivityRecord[]>([])

// 加载数据
const loadData = async () => {
  try {
    console.log('Loading data...')
    const fetchedTasks = await taskStore.fetchTasks()
    tasks.value = fetchedTasks as Task[]
    console.log(`Loaded ${tasks.value.length} tasks`)

    // 使用批量接口获取所有活动记录
    const allActivities = await activityStore.getAllActivities()
    
    // 确保allActivities是一个数组
    if (Array.isArray(allActivities)) {
      activities.value = allActivities
      console.log(`Loaded ${activities.value.length} activities`)
    } else {
      console.warn('活动记录不是数组格式:', allActivities)
      activities.value = []
    }
  } catch (error) {
    console.error('加载数据失败:', error)
    ElMessage.error('加载数据失败')
  }
}

const currentMonthText = computed(() => {
  return currentDate.value.format('YYYY年MM月')
})

const calendarDays = computed(() => {
  const days: CalendarDay[] = []
  const startOfMonth = currentDate.value.startOf('month')
  const endOfMonth = currentDate.value.endOf('month')
  const startDate = startOfMonth.startOf('week')
  const endDate = endOfMonth.endOf('week')

  let currentDay = startDate

  while (currentDay.isBefore(endDate) || currentDay.isSame(endDate)) {
    const dateStr = currentDay.format('YYYY-MM-DD')

    // 获取当天的任务
    const dayTasks = tasks.value.filter(task => {
      // 处理任务可能没有日期字段的情况
      const taskStart = task.startDate ? dayjs(task.startDate) : null
      const taskEnd = task.endDate ? dayjs(task.endDate) : null

      if (!taskStart && !taskEnd) {
        // 如果任务没有日期，检查创建日期是否在当前月份
        const taskCreated = task.createdAt ? dayjs(task.createdAt) : null
        return taskCreated && taskCreated.isSame(currentDay, 'day')
      }

      // 如果有开始或结束日期，检查是否在范围内
      if (taskStart && taskEnd) {
        return currentDay.isSame(taskStart) || currentDay.isAfter(taskStart) && currentDay.isSame(taskEnd) || currentDay.isBefore(taskEnd)
      } else if (taskStart) {
        return currentDay.isSame(taskStart) || currentDay.isAfter(taskStart)
      } else if (taskEnd) {
        return currentDay.isSame(taskEnd) || currentDay.isBefore(taskEnd)
      }

      return false
    })

    // 获取当天的活动记录
    const dayActivities = activities.value.filter(activity =>
        dayjs(activity.startTime).isSame(currentDay, 'day')
    )

    // 计算当天总活动时间
    const totalActivityTime = dayActivities.reduce((total, activity) =>
        total + (activity.durationMinutes || 0), 0
    )

    days.push({
      date: dateStr,
      day: currentDay.date(),
      isToday: currentDay.isSame(dayjs(), 'day'),
      isCurrentMonth: currentDay.isSame(currentDate.value, 'month'),
      tasks: dayTasks,
      activities: dayActivities,
      totalActivityTime
    })

    currentDay = currentDay.add(1, 'day')
  }

  return days
})

const prevMonth = () => {
  currentDate.value = currentDate.value.subtract(1, 'month')
  loadData()
}

const nextMonth = () => {
  currentDate.value = currentDate.value.add(1, 'month')
  loadData()
}

const handleDayContextMenu = (e: MouseEvent, day: CalendarDay) => {
  e.preventDefault()
  contextMenu.value = {
    visible: true,
    x: e.clientX,
    y: e.clientY,
    selectedDay: day
  }
}

const closeContextMenu = () => {
  contextMenu.value.visible = false
}

// 获取指定日期的活动任务
const getActiveTasksForDay = (dateStr: string) => {
  const targetDay = dayjs(dateStr)

  // 筛选符合条件的活动任务
  return tasks.value.filter(task => {
    // 获取任务的活动记录
    const taskActivities = activities.value.filter(activity => activity.taskId === task.id)

    // 按时间排序
    taskActivities.sort((a, b) => new Date(b.startTime).getTime() - new Date(a.startTime).getTime())

    // 如果没有任何活动记录，不算活动任务
    if (taskActivities.length === 0) {
      return false
    }

    // 条件1：今日从计划中变为制作中
    const todayPlannedToInProgress = taskActivities.some(activity => {
      const activityDate = dayjs(activity.startTime)
      return activityDate.isSame(targetDay, 'day') &&
          activity.activityType === 'STARTED' &&
          task.status === 'in-progress'
    })

    // 条件2：今日之前一直在制作中并且没有完成的
    const beforeTodayInProgress = task.status === 'in-progress' &&
        taskActivities.some(activity =>
            dayjs(activity.startTime).isBefore(targetDay, 'day') &&
            activity.activityType === 'STARTED'
        ) &&
        !taskActivities.some(activity =>
            (dayjs(activity.startTime).isSame(targetDay, 'day') || dayjs(activity.startTime).isBefore(targetDay, 'day')) &&
            activity.activityType === 'COMPLETED'
        )

    // 条件3：今天从制作中变更为已完成的
    const todayInProgressToCompleted = taskActivities.some(activity => {
      const activityDate = dayjs(activity.startTime)
      return activityDate.isSame(targetDay, 'day') &&
          activity.activityType === 'COMPLETED' &&
          task.status === 'completed'
    })

    const isActive = todayPlannedToInProgress || beforeTodayInProgress || todayInProgressToCompleted
    return isActive
  })
}

// 复制活动任务到剪切板
const copyActiveTasks = async (dateStr: string) => {
  if (!dateStr) {
    if (!contextMenu.value.selectedDay) {
      closeContextMenu()
      return
    }
    dateStr = contextMenu.value.selectedDay.date
  }
  const activeTasks = getActiveTasksForDay(dateStr)
  if (activeTasks.length === 0) {
    ElMessage.info(`${dateStr} 没有活动任务`)
    closeContextMenu()
    return
  }

  // 格式化任务信息
  let clipboardText = `${dateStr} 活动任务\n\n`
  activeTasks.forEach((task, index) => {
    clipboardText += `${index + 1}. ${task.title}\n`
    clipboardText += `   状态: ${getStatusText(task.status)}\n`
    clipboardText += `   描述: ${(task as any).description || '无'}\n`

    // 获取任务相关的活动记录
    const taskActivities = activities.value
        .filter(activity => activity.taskId === task.id)
        .filter(activity => dayjs(activity.startTime).isSame(dateStr, 'day'))
        .sort((a, b) => new Date(a.startTime).getTime() - new Date(b.startTime).getTime())

    if (taskActivities.length > 0) {
      clipboardText += `   今日活动:\n`
      taskActivities.forEach(activity => {
        const startTime = dayjs(activity.startTime).format('HH:mm')
        const endTime = activity.endTime ? dayjs(activity.endTime).format('HH:mm') : '进行中'
        const duration = activity.durationMinutes ? ` (${formatShortDuration(activity.durationMinutes)})` : ''
        clipboardText += `     - ${getActivityTypeText(activity.activityType)}: ${startTime}-${endTime}${duration}\n`
        if (activity.description) {
          clipboardText += `       描述: ${activity.description}\n`
        }
      })
    }
    clipboardText += '\n'
  })
  // 使用现代的 Clipboard API 复制到剪切板
  try {
    if (navigator.clipboard && navigator.clipboard.writeText) {
      await navigator.clipboard.writeText(clipboardText)
      ElMessage.success(`已复制 ${activeTasks.length} 个活动任务到剪切板`)
    } else {
      // 降级方案
      fallbackCopyToClipboard(clipboardText)
    }
    closeContextMenu()
  } catch (err) {
    console.error('复制失败:', err)
    // 最后的备选方案 - 弹出对话框显示文本
    ElMessageBox.alert(clipboardText, '复制内容', {
      confirmButtonText: '确定',
    }).then(() => {
      ElMessage.info('请手动复制上方内容')
      closeContextMenu()
    }).catch(() => {
      closeContextMenu()
    })
  }

  closeContextMenu()
}

// 降级的复制方案
const fallbackCopyToClipboard = (text: string) => {
  try {
    const textArea = document.createElement('textarea')
    textArea.value = text
    textArea.style.position = 'fixed'
    textArea.style.left = '-999999px'
    textArea.style.top = '-999999px'
    document.body.appendChild(textArea)
    textArea.focus()
    textArea.select()

    const successful = document.execCommand('copy')
    document.body.removeChild(textArea)

    if (successful) {
      ElMessage.success('已复制活动任务到剪切板')
    } else {
      throw new Error('复制命令执行失败')
    }
  } catch (err) {
    console.error('降级复制方案也失败了:', err)
    ElMessage.error('复制失败，请手动复制')
  }
}

// 获取状态文本
const getStatusText = (status: string) => {
  const statusMap: Record<string, string> = {
    'planning': '计划中',
    'in-progress': '制作中',
    'completed': '已完成',
    'paused': '已暂停'
  }
  return statusMap[status] || status
}

// 获取活动类型文本
const getActivityTypeText = (type: string) => {
  const typeMap: Record<string, string> = {
    'CREATED': '创建',
    'STARTED': '开始',
    'COMPLETED': '完成',
    'PAUSED': '暂停',
    'RESUMED': '恢复',
    'WORK': '工作',
    'MEETING': '会议',
    'STUDY': '学习',
    'OTHER': '其他'
  }
  return typeMap[type] || type
}

// const exportDayReport = () => {
//   if (contextMenu.value.selectedDay) {
//     ElMessage.success(`导出 ${contextMenu.value.selectedDay.date} 的日报`)
//   }
//   closeContextMenu()
// }

const viewDayTasks = () => {
  if (contextMenu.value.selectedDay) {
    ElMessage.info(`查看 ${contextMenu.value.selectedDay.date} 的任务`)
  }
  closeContextMenu()
}

const exportDailyReport = () => {
  const selectedDateStr = currentDate.value.format('YYYY-MM-DD')
  copyActiveTasks(selectedDateStr)
  ElMessage.success('导出日报成功')
}

// 复制本周活动任务到剪切板
const copyActiveTasksForWeek = async () => {
  const weekStart = currentDate.value.startOf('week')
  const weekEnd = currentDate.value.endOf('week')
  const weekStartStr = weekStart.format('YYYY-MM-DD')
  const weekEndStr = weekEnd.format('YYYY-MM-DD')

  // 收集本周所有活动任务
  const weeklyActiveTasks = []

  for (let i = 0; i <= weekEnd.diff(weekStart, 'day'); i++) {
    const currentDay = weekStart.add(i, 'day')
    const currentDayStr = currentDay.format('YYYY-MM-DD')

    // 获取当天的活动任务
    const dayActiveTasks = getActiveTasksForDay(currentDayStr)
    weeklyActiveTasks.push({
      date: currentDayStr,
      tasks: dayActiveTasks
    })
  }

  // 格式化周报内容
  let clipboardText = `${weekStartStr} 至 ${weekEndStr} 活动任务周报\n\n`

  weeklyActiveTasks.forEach(dayInfo => {
    if (dayInfo.tasks.length > 0) {
      clipboardText += `📅 ${dayInfo.date} (${dayjs(dayInfo.date).format('dddd')})\n`
      dayInfo.tasks.forEach((task, index) => {
        clipboardText += `  ${index + 1}. ${task.title}\n`
        clipboardText += `     状态: ${getStatusText(task.status)}\n`
        clipboardText += `     描述: ${task.description || '无'}\n`
      })
      clipboardText += '\n'
    }
  })

  if (weeklyActiveTasks.every(dayInfo => dayInfo.tasks.length === 0)) {
    clipboardText += '本周暂无活动任务'
  }

  // 使用现代的 Clipboard API 复制到剪切板
  try {
    if (navigator.clipboard && navigator.clipboard.writeText) {
      await navigator.clipboard.writeText(clipboardText)
      ElMessage.success('已复制本周活动任务到剪切板')
    } else {
      // 降级方案
      fallbackCopyToClipboard(clipboardText)
    }
    closeContextMenu()
  } catch (err) {
    console.error('复制失败:', err)
    // 最后的备选方案 - 弹出对话框显示文本
    ElMessageBox.alert(clipboardText, '复制内容', {
      confirmButtonText: '确定',
    }).then(() => {
      ElMessage.info('请手动复制上方内容')
      closeContextMenu()
    }).catch(() => {
      closeContextMenu()
    })
  }
}

const exportWeeklyReport = () => {
  copyActiveTasksForWeek()
  ElMessage.success('导出周报成功')
}

// 格式化短时间显示
const formatShortDuration = (minutes: number): string => {
  if (!minutes || minutes <= 0) return ''

  const hours = Math.floor(minutes / 60)
  const mins = minutes % 60

  if (hours > 0) {
    return `${hours}h${mins > 0 ? mins + 'm' : ''}`
  } else {
    return `${mins}m`
  }
}

// 点击其他地方关闭右键菜单
const handleClickOutside = (_e: MouseEvent) => {
  if (contextMenu.value.visible) {
    closeContextMenu()
  }
}

onMounted(() => {
  document.addEventListener('click', handleClickOutside)
  loadData()
})

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside)
})
</script>

<style scoped>
.calendar-container {
  height: 100%;
  display: flex;
  flex-direction: column;
  background: #f8fafc;
  padding: 20px;
  border-radius: 12px;
  outline: none;
}

.calendar-container:focus {
  box-shadow: 0 0 0 2px #3b82f6;
  transition: box-shadow 0.2s ease;
}

.calendar-header {
  margin-bottom: 24px;
  padding: 0 8px;
}

.header-controls {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.current-month {
  font-size: 20px;
  font-weight: 600;
  color: #1f2937;
  letter-spacing: 0.5px;
}

.calendar-body {
  flex: 1;
  display: flex;
  flex-direction: column;
  background: white;
  border-radius: 12px;
  overflow: hidden;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.05);
}

.week-header {
  display: grid;
  grid-template-columns: repeat(7, 1fr);
  background: #f1f5f9;
  padding: 16px 0;
}

.week-day {
  text-align: center;
  font-weight: 600;
  color: #475569;
  font-size: 14px;
}

.calendar-grid {
  flex: 1;
  display: grid;
  grid-template-columns: repeat(7, 1fr);
  grid-auto-rows: minmax(100px, 1fr);
}

.calendar-day {
  padding: 12px;
  border-right: 1px solid #f1f5f9;
  border-bottom: 1px solid #f1f5f9;
  cursor: pointer;
  transition: all 0.2s ease;
  position: relative;
  overflow: hidden;
}

.calendar-day:hover {
  background: #f8fafc;
  z-index: 1;
}

.calendar-day:not(.current-month) {
  background: #fafbfc;
  color: #94a3b8;
}

.calendar-day.today {
  background: #eff6ff;
}

.calendar-day.today::before {
  content: '';
  position: absolute;
  top: 8px;
  left: 8px;
  width: 6px;
  height: 6px;
  background: #3b82f6;
  border-radius: 50%;
}

.calendar-day.has-tasks {
  border-left: 3px solid #3b82f6;
}

.calendar-day.has-activities {
  border-right: 3px solid #10b981;
}

.day-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.day-number {
  font-size: 15px;
  font-weight: 600;
  color: #1f2937;
}

.day-indicators {
  display: flex;
  align-items: center;
  gap: 6px;
}

.task-badge {
  transform: scale(0.85);
}

.activity-indicator {
  display: flex;
  align-items: center;
  gap: 2px;
  font-size: 11px;
  color: #10b981;
  background: #ecfdf5;
  padding: 2px 6px;
  border-radius: 10px;
  font-weight: 500;
}

.activity-time {
  font-size: 10px;
}

.day-content {
  display: flex;
  flex-direction: column;
  gap: 6px;
  height: calc(100% - 30px);
  overflow: hidden;
}

.day-tasks {
  display: flex;
  flex-direction: column;
  gap: 3px;
}

.task-item {
  font-size: 12px;
  padding: 3px 6px;
  border-radius: 6px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  font-weight: 500;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
}

.task-item.status-planning {
  background: #dbeafe;
  color: #1e40af;
}

.task-item.status-in-progress {
  background: #fed7aa;
  color: #c2410c;
}

.task-item.status-completed {
  background: #d1fae5;
  color: #065f46;
}

.day-activities {
  display: flex;
  flex-direction: column;
  gap: 3px;
}

.activity-item {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 11px;
  padding: 2px 5px;
  border-radius: 4px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  background: #f3f4f6;
  color: #4b5563;
}

.activity-item.type-work {
  background: #e0f2fe;
  color: #0277bd;
}

.activity-item.type-meeting {
  background: #fff7ed;
  color: #c2410c;
}

.activity-item.type-study {
  background: #f3e8ff;
  color: #6b21a8;
}

.activity-item.type-created {
  background: #e0f2fe;
  color: #0277bd;
}

.activity-item.type-started {
  background: #dcfce7;
  color: #166534;
}

.activity-item.type-paused {
  background: #fef3c7;
  color: #d97706;
}

.activity-item.type-resumed {
  background: #e0f2fe;
  color: #0277bd;
}

.activity-item.type-completed {
  background: #dcfce7;
  color: #166534;
}

.activity-text {
  max-width: 80px;
  overflow: hidden;
  text-overflow: ellipsis;
}

.more-items {
  font-size: 11px;
  color: #6b7280;
  text-align: center;
  padding: 2px 0;
  font-weight: 500;
}

.context-menu {
  position: fixed;
  background: white;
  border: 1px solid #e5e7eb;
  border-radius: 8px;
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.1);
  z-index: 2000;
  min-width: 140px;
  overflow: hidden;
}

.menu-item {
  padding: 10px 14px;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 8px;
  transition: background-color 0.2s;
  font-size: 14px;
}

.menu-item:hover {
  background: #f9fafb;
}
</style>