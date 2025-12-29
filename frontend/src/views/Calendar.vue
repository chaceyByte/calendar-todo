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
          <el-button @click="goToToday" type="primary">
            今天
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
            @dblclick="(e) => handleDayDoubleClick(e, day)"
        >
          <div class="day-header">
            <div class="day-date-info">
              <div class="date-main">
                <span :class="['day-number', { 'is-weekend': day.isWeekend }]">{{ formatDayHeader(day.date) }}</span>
                <span v-if="day.holidayType" :class="['holiday-badge', day.holidayType.toLowerCase()]">
                  {{ day.holidayType === 'REST' ? '休' : '补' }}
                </span>
              </div>
              <span :class="['lunar-date', { 'is-weekend': day.isWeekend }]">{{ formatLunarDate(day.date) }}</span>
            </div>
            <div class="day-indicators">
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
                  v-for="task in day.tasks.slice(0, 3)"
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

            <div v-if="day.tasks.length > 3 || day.activities.length > 2" class="more-items">
              +{{ (day.tasks.length - 3) + (day.activities.length - 2) }}更多
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
      <div class="menu-item" @click="() => copyDayTasks()">
        <el-icon>
          <document/>
        </el-icon>
        复制任务
      </div>
      <div v-if="isFridaySaturdayOrSunday(contextMenu.selectedDay?.date)" class="menu-item" @click="() => exportWeekReportFromContextMenu()">
        <el-icon>
          <files/>
        </el-icon>
        导出周报
      </div>
    </div>

    <!-- 任务详情弹窗 -->
    <el-dialog
        v-model="taskDialog.visible"
        :title="`${taskDialog.date} 任务详情`"
        width="600px"
        :close-on-click-modal="false"
    >
      <div class="task-dialog-content">
        <div v-if="taskDialog.tasks.length === 0" class="no-tasks">
          该日期暂无任务
        </div>
        <div v-else class="task-list">
          <div
              v-for="(taskGroup, index) in groupTasksByTag(taskDialog.tasks)"
              :key="index"
              class="task-group"
          >
            <div class="tag-title">{{ taskGroup.tag }}</div>
            <div class="task-items">
              <div
                  v-for="task in taskGroup.tasks"
                  :key="task.id"
                  class="task-plain"
              >
                -- {{ task.title }}{{ task.description ? '; ' + task.description : '' }}
              </div>
            </div>
          </div>
        </div>
      </div>
      <template #footer>
        <el-button @click="taskDialog.visible = false">关闭</el-button>
        <el-button type="primary" @click="copyDayTasks(taskDialog.date)">复制任务</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import {computed, onMounted, onUnmounted, ref} from 'vue'
import dayjs from 'dayjs'
import {getLunar} from 'chinese-lunar-calendar'
import {ArrowLeft, ArrowRight, Clock, Document, Files} from '@element-plus/icons-vue'
import {useTaskStore} from '@/stores/task'
import {useActivityStore} from '@/stores/activity'
import {ElMessage, ElMessageBox} from 'element-plus/es'
import {getHolidaysByYear} from '@/api/calendar'

interface Task {
  id: number
  title: string
  description?: string
  status: 'planning' | 'in-progress' | 'completed' | 'cancelled'
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

interface HolidayConfig {
  id?: number
  year: string
  date: string
  type: 'REST' | 'WORK'
  description: string
}

interface CalendarDay {
  date: string
  day: number
  isToday: boolean
  isWeekend: boolean
  isCurrentMonth: boolean
  tasks: Task[]
  activities: ActivityRecord[]
  totalActivityTime: number
  holidayType?: 'REST' | 'WORK' | null
  holidayDescription?: string
}

const currentDate = ref(dayjs())
const contextMenu = ref({
  visible: false,
  x: 0,
  y: 0,
  selectedDay: null as CalendarDay | null
})
const taskDialog = ref({
  visible: false,
  date: '',
  tasks: [] as Task[]
})
const calendarContainer = ref<HTMLElement | null>(null)

// 休息日配置数据
const holidays = ref<HolidayConfig[]>([])

const weekDays = ['一', '二', '三', '四', '五', '六', '日']

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

    // 使用批量接口获取所有任务
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

    // 加载休息日配置
    await loadHolidays()
  } catch (error) {
    console.error('加载数据失败:', error)
    ElMessage.error('加载数据失败')
  }
}

// 加载休息日配置（调用后端API）
const loadHolidays = async () => {
  try {
    const currentYear = parseInt(currentDate.value.format('YYYY'))

    // 使用calendar API组件获取休息日配置
    holidays.value = await getHolidaysByYear(currentYear)
    console.log(`Loaded ${holidays.value.length} holiday configurations from API`)

  } catch (error) {
    console.error('加载休息日配置失败:', error)
    // 如果后端接口失败，返回空数组
    holidays.value = []
  }
}

// 获取日期的休息日类型
const getHolidayType = (dateStr: string): HolidayConfig | null => {
  return holidays.value.find(h => h.date === dateStr) || null
}

const currentMonthText = computed(() => {
  return currentDate.value.format('YYYY年MM月')
})

const calendarDays = computed(() => {
  const days: CalendarDay[] = []
  const startOfMonth = currentDate.value.startOf('month')
  const endOfMonth = currentDate.value.endOf('month')
  const startDate = startOfMonth.startOf('week').add(1, 'day') // 调整为从周一开始
  const endDate = endOfMonth.endOf('week').add(1, 'day') // 调整为从周一开始

  // 优化：预先创建任务日期映射，避免N+1查询
  const taskDateMap: { [date: string]: Task[] } = {}
  const activityDateMap: { [date: string]: ActivityRecord[] } = {}

  // 初始化所有日期的空数组
  let currentDay = startDate.clone()
  while (currentDay.isBefore(endDate) || currentDay.isSame(endDate)) {
    const dateStr = currentDay.format('YYYY-MM-DD')
    taskDateMap[dateStr] = []
    activityDateMap[dateStr] = []
    currentDay = currentDay.add(1, 'day')
  }

  // 批量分配任务到对应日期 - 基于任务生命周期活动记录
  tasks.value.forEach(task => {
    // 获取该任务的所有活动记录
    const taskActivities = activities.value.filter(activity => activity.taskId === task.id)

    if (taskActivities.length === 0) {
      // 如果没有活动记录，使用任务的创建日期
      const taskCreated = task.createdAt ? dayjs(task.createdAt) : null
      if (taskCreated) {
        const createdDateStr = taskCreated.format('YYYY-MM-DD')
        if (taskDateMap[createdDateStr]) {
          taskDateMap[createdDateStr].push(task)
        }
      }
      return
    }

    // 按开始时间排序活动记录
    const sortedActivities = [...taskActivities].sort((a, b) =>
        dayjs(a.startTime).isBefore(dayjs(b.startTime)) ? -1 : 1
    )

    // 分析任务的生命周期，确定任务在哪些日期处于活动状态
    const activeDates = new Set<string>()

    // 遍历活动记录，确定任务的活动时间段
    for (let i = 0; i < sortedActivities.length; i++) {
      const activity = sortedActivities[i]
      const activityDate = dayjs(activity.startTime)

      // 根据活动类型确定任务状态
      switch (activity.activityType) {
        case 'CREATED':
          // 任务创建，标记创建日期
          activeDates.add(activityDate.format('YYYY-MM-DD'))
          break

        case 'STARTED':
          // 任务开始进行，标记从开始时间到下一个状态变更的日期
          const startDate = activityDate
          let endDate = dayjs() // 默认显示到今天为止

          // 查找下一个状态变更
          for (let j = i + 1; j < sortedActivities.length; j++) {
            const nextActivity = sortedActivities[j]
            if (['PAUSED', 'COMPLETED'].includes(nextActivity.activityType)) {
              endDate = dayjs(nextActivity.startTime)
              break
            }
          }

          // 标记这个时间段内的所有日期（包括今天及之前的日期）
          let currentDay = startDate.clone()
          while (currentDay.isBefore(endDate) || currentDay.isSame(endDate)) {
            // 只显示今天及之前的日期，不显示未来日期
            if (!currentDay.isAfter(dayjs(), 'day')) {
              activeDates.add(currentDay.format('YYYY-MM-DD'))
            }
            currentDay = currentDay.add(1, 'day')
          }
          break

        case 'RESUMED':
          // 任务恢复，标记从恢复时间到下一个状态变更的日期
          const resumeDate = activityDate
          let resumeEndDate = dayjs() // 默认显示到今天为止

          // 查找下一个状态变更
          for (let j = i + 1; j < sortedActivities.length; j++) {
            const nextActivity = sortedActivities[j]
            if (['PAUSED', 'COMPLETED'].includes(nextActivity.activityType)) {
              resumeEndDate = dayjs(nextActivity.startTime)
              break
            }
          }

          // 标记这个时间段内的所有日期（包括今天及之前的日期）
          let resumeCurrentDay = resumeDate.clone()
          while (resumeCurrentDay.isBefore(resumeEndDate) || resumeCurrentDay.isSame(resumeEndDate)) {
            // 只显示今天及之前的日期，不显示未来日期
            if (!resumeCurrentDay.isAfter(dayjs(), 'day')) {
              activeDates.add(resumeCurrentDay.format('YYYY-MM-DD'))
            }
            resumeCurrentDay = resumeCurrentDay.add(1, 'day')
          }
          break

        case 'COMPLETED':
          // 任务完成，标记完成日期（仅显示今天及之前的完成日期）
          if (!activityDate.isAfter(dayjs(), 'day')) {
            activeDates.add(activityDate.format('YYYY-MM-DD'))
          }
          break

          // PAUSED 状态不标记日期，因为任务处于暂停状态
      }
    }

    // 特殊处理：对于上周开始但本周仍在进行的任务，需要确保显示到今天
    // 检查任务是否处于进行中状态且有开始记录
    if (task.status === 'in-progress') {
      const startedActivity = sortedActivities.find(a => a.activityType === 'STARTED')
      if (startedActivity) {
        const startDate = dayjs(startedActivity.startTime)
        
        // 如果任务在上周或更早开始，且没有完成记录，则确保显示到今天
        const hasCompletedActivity = sortedActivities.some(a => a.activityType === 'COMPLETED')
        if (startDate.isBefore(dayjs(), 'day') && !hasCompletedActivity) {
          let currentDay = startDate.clone()
          // 从开始日期到今天的所有日期
          while (currentDay.isSameOrBefore(dayjs(), 'day')) {
            activeDates.add(currentDay.format('YYYY-MM-DD'))
            currentDay = currentDay.add(1, 'day')
          }
        }
      }
    }

    // 将任务分配到对应的活动日期
    activeDates.forEach(dateStr => {
      if (taskDateMap[dateStr]) {
        taskDateMap[dateStr].push(task)
      }
    })
  })

  // 批量分配活动记录到对应日期 - 只显示非任务状态变更的活动
  activities.value.forEach(activity => {
    // 过滤掉任务状态变更的活动记录（CREATED, STARTED, COMPLETED, PAUSED, RESUMED）
    const statusChangeTypes = ['CREATED', 'STARTED', 'COMPLETED', 'PAUSED', 'RESUMED']
    if (statusChangeTypes.includes(activity.activityType)) {
      return
    }

    const activityDate = dayjs(activity.startTime)
    const dateStr = activityDate.format('YYYY-MM-DD')
    if (activityDateMap[dateStr]) {
      activityDateMap[dateStr].push(activity)
    }
  })

  // 构建日历天数
  currentDay = startDate.clone()
  while (currentDay.isBefore(endDate) || currentDay.isSame(endDate)) {
    const dateStr = currentDay.format('YYYY-MM-DD')
    const dayTasks = taskDateMap[dateStr] || []
    const dayActivities = activityDateMap[dateStr] || []

    // 计算当天总活动时间
    const totalActivityTime = dayActivities.reduce((total, activity) =>
        total + (activity.durationMinutes || 0), 0
    )

    // 获取休息日信息
    const holidayInfo = getHolidayType(dateStr)

    days.push({
      date: dateStr,
      day: currentDay.date(),
      isToday: currentDay.isSame(dayjs(), 'day'),
      isCurrentMonth: currentDay.isSame(currentDate.value, 'month'),
      isWeekend: currentDay.day() === 0 || currentDay.day() === 6,
      tasks: dayTasks,
      activities: dayActivities,
      totalActivityTime,
      holidayType: holidayInfo?.type || null,
      holidayDescription: holidayInfo?.description || ''
    })

    currentDay = currentDay.add(1, 'day')
  }

  return days
})

const prevMonth = () => {
  currentDate.value = currentDate.value.subtract(1, 'month')
  loadData()
}

const goToToday = () => {
  currentDate.value = dayjs()
  loadData()
}

const nextMonth = () => {
  currentDate.value = currentDate.value.add(1, 'month')
  loadData()
}

const handleDayContextMenu = (e: MouseEvent, day: CalendarDay) => {
  e.preventDefault()
  // 只有有任务时才显示右键菜单
  if (day.tasks.length === 0) {
    return
  }
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
    'cancelled': '已取消'
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

const exportDailyReport = () => {
  const selectedDateStr = dayjs().format('YYYY-MM-DD') // 使用当前日期，而不是日历显示的日期
  copyActiveTasks(selectedDateStr)
  ElMessage.success('导出日报成功')
}

// 复制指定周的活动任务到剪切板
const copyActiveTasksForWeek = async (targetDate?: string) => {
  // 如果传入了目标日期，使用该日期所在的周；否则使用当前日期（当前周）
  const baseDate = targetDate ? dayjs(targetDate) : dayjs()
  const weekStart = baseDate.startOf('week').add(1, 'day') // 调整为从周一开始
  const weekEnd = baseDate.endOf('week').add(1, 'day') // 调整为从周一开始
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
  copyActiveTasksForWeek() // 使用当前日期所在的周
  ElMessage.success('导出周报成功')
}

// 判断日期是否为周五、周六或周日
const isFridaySaturdayOrSunday = (dateStr: string): boolean => {
  if (!dateStr) return false
  const date = dayjs(dateStr)
  const dayOfWeek = date.day() // 0=周日, 1=周一, ..., 6=周六
  return dayOfWeek === 5 || dayOfWeek === 6 || dayOfWeek === 0
}

// 从右键菜单导出周报
const exportWeekReportFromContextMenu = () => {
  if (!contextMenu.value.selectedDay) {
    ElMessage.error('未选择日期')
    return
  }
  copyActiveTasksForWeek(contextMenu.value.selectedDay.date)
  ElMessage.success('导出周报成功')
  closeContextMenu()
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

// 双击日期格子打开任务详情
const handleDayDoubleClick = (e: MouseEvent, day: CalendarDay) => {
  e.stopPropagation()
  // 只有有任务时才显示弹窗
  if (day.tasks.length === 0) {
    return
  }
  taskDialog.value = {
    visible: true,
    date: day.date,
    tasks: day.tasks
  }
}

// 按标签分组任务（支持多对多关系）
const groupTasksByTag = (tasks: Task[]) => {
  const groups: { [key: string]: Task[] } = {}

  tasks.forEach(task => {
    // 如果任务有多个标签，任务会在多个标签组中出现
    if (task.tags && task.tags.length > 0) {
      task.tags.forEach(tag => {
        if (!groups[tag]) {
          groups[tag] = []
        }
        groups[tag].push(task)
      })
    } else {
      // 没有标签的任务归为未分类
      const tag = '未分类'
      if (!groups[tag]) {
        groups[tag] = []
      }
      groups[tag].push(task)
    }
  })

  return Object.keys(groups).map(tag => ({
    tag,
    tasks: groups[tag]
  }))
}

// 格式化日期header为"xx月xx日"格式
const formatDayHeader = (dateStr: string) => {
  const date = dayjs(dateStr)
  return date.format('MM月DD日')
}

// 格式化农历日期
const formatLunarDate = (dateStr: string) => {
  try {
    const date = dayjs(dateStr)
    const lunar = getLunar(date.year(), date.month() + 1, date.date())

    // 使用库提供的格式化字符串，或者根据日期判断显示月份
    if (lunar.lunarDate === 1) {
      // 初一显示月份
      const monthNames = ['正', '二', '三', '四', '五', '六', '七', '八', '九', '十', '冬', '腊']
      return monthNames[lunar.lunarMonth - 1] + '月'
    } else {
      // 其他日期显示日期
      return lunar.dateStr.replace(/^\S+月/, '') // 移除月份部分，只保留日期
    }
  } catch (error) {
    console.error('农历日期格式化错误:', error)
    return '初一'
  }
}

// 复制指定日期的任务
const copyDayTasks = async (dateStr?: string) => {
  if (!dateStr) {
    if (contextMenu.value.selectedDay) {
      dateStr = contextMenu.value.selectedDay.date
    } else if (taskDialog.value.visible) {
      dateStr = taskDialog.value.date
    } else {
      closeContextMenu()
      return
    }
  }

  const targetDay = calendarDays.value.find(day => day.date === dateStr)
  if (!targetDay || targetDay.tasks.length === 0) {
    ElMessage.info(`${dateStr} 没有任务`)
    closeContextMenu()
    return
  }

  const taskGroups = groupTasksByTag(targetDay.tasks)
  let clipboardText = `${dateStr} 任务\n\n`

  taskGroups.forEach((group, groupIndex) => {
    clipboardText += `- tag ${groupIndex + 1} ${group.tag}\n`
    group.tasks.forEach((task, index) => {
      clipboardText += `  ${index + 1}. ${task.title} | ${task.description || '无描述'}\n`
    })
    clipboardText += '\n'
  })

  // 使用现代的 Clipboard API 复制到剪切板
  try {
    if (navigator.clipboard && navigator.clipboard.writeText) {
      await navigator.clipboard.writeText(clipboardText)
      ElMessage.success(`已复制 ${targetDay.tasks.length} 个任务到剪切板`)
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
  min-height: 0;
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
  grid-auto-rows: minmax(120px, 1fr);
  min-height: 0;
  overflow-y: auto;
}

.calendar-day {
  padding: 12px;
  border-right: 1px solid #f1f5f9;
  border-bottom: 1px solid #f1f5f9;
  cursor: pointer;
  transition: all 0.2s ease;
  position: relative;
  overflow: hidden;
  min-height: 120px;
  display: flex;
  flex-direction: column;
}

.calendar-day:hover {
  background: #f8fafc;
  z-index: 1;
}

.calendar-day:not(.current-month) {
  background: #fafbfc;
  color: #cbd5e1;
}

.calendar-day:not(.current-month) .day-number {
  color: #cbd5e1;
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

/* 去除任务相关的左侧边框 */

.calendar-day.has-activities {
  border-right: 3px solid #10b981;
}

.day-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  padding: 4px 8px;
  border-radius: 4px;
  margin: -4px -8px 8px -8px;
}

.day-date-info {
  width: 100%;
  display: flex;
  /*align-items: center;*/
  justify-content: space-between;
}

.date-main {
  display: flex;
  align-items: center;
  gap: 4px;
}

.is-weekend {
  color: #EC3333 !important;
}

/* 休息日标签样式 */
.holiday-badge {
  display: inline-block;
  padding: 1px 3px;
  border-radius: 2px;
  font-size: 8px;
  font-weight: bold;
  line-height: 1;
  min-width: 12px;
  text-align: center;
}

.holiday-badge.rest {
  background-color: #1359E6; /* 蓝色底色 */
  color: white; /* 白色字体 */
  font-size: 12px;
  font-weight: 500;
}

.holiday-badge.work {
  background-color: #EC3333; /* 红色底色 */
  color: white; /* 白色字体 */
  font-size: 12px;
  font-weight: 500;
}

.day-number {
  font-size: 12px;
  font-weight: 700;
  color: #1f2937;
  line-height: 1.2;
  font-family: 'Alibaba PuHuiTi', 'Alibaba DingTalk Font', 'Source Han Sans CN', 'Noto Sans SC', sans-serif;
}

.lunar-date {
  font-size: 8px;
  font-weight: 400;
  color: #6b7280;
  line-height: 1;
  margin-top: 1px;
  font-family: 'Alibaba PuHuiTi', 'Alibaba DingTalk Font', 'Source Han Sans CN', 'Noto Sans SC', sans-serif;
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
  flex: 1;
  min-height: 0;
  overflow: hidden;
}

.day-tasks {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.task-item {
  font-size: 12px;
  padding: 6px 8px 6px 16px;
  border-radius: 0 4px 4px 0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  font-weight: 500;
  box-shadow: 0 1px 1px rgba(0, 0, 0, 0.05);
  border-left: 3px solid transparent;
  height: 29px;
  line-height: 17px;
  position: relative;
  display: flex;
  align-items: center;
  justify-content: flex-end;
  text-align: right;
  max-width: calc(100% - 8px);
  width: fit-content;
  align-self: flex-end;
  margin-left: auto;
}

.task-item::before {
  content: '';
  position: absolute;
  left: 6px;
  top: 50%;
  transform: translateY(-50%);
  width: 8px;
  height: 8px;
  background: #f97316;
  border-radius: 50%;
}

.task-item.status-planning {
  background: #dbeafe;
  color: #1e40af;
  border-radius: 20px 0 0 20px;
}

.task-item.status-in-progress {
  background: #fef3c7;
  color: #92400e;
  border-radius: 20px 0 0 20px;
}

.task-item.status-completed {
  background: #f3f4f6;
  color: #6b7280;
  border-radius: 20px 0 0 20px;
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

/* 任务弹窗样式 */
.task-dialog-content {
  max-height: 400px;
  overflow-y: auto;
  padding: 0 10px;
}

.no-tasks {
  text-align: center;
  color: #6b7280;
  padding: 40px 0;
  font-size: 16px;
}

.task-group {
  margin-bottom: 20px;
}

.tag-title {
  font-weight: 600;
  color: #374151;
  margin-bottom: 8px;
  font-size: 15px;
  padding-left: 8px;
  border-left: 3px solid #3b82f6;
}

.task-items {
  margin-left: 20px;
}

.task-dialog-content .task-plain {
  margin-bottom: 8px;
  font-size: 14px;
  line-height: 1.4;
  color: #333;
}

.task-number {
  font-weight: 600;
  color: #6b7280;
  min-width: 20px;
  margin-right: 8px;
}

.task-content {
  flex: 1;
}

.task-title {
  font-weight: 500;
  color: #1f2937;
  margin-bottom: 4px;
}

.task-description {
  color: #6b7280;
  font-size: 13px;
  line-height: 1.4;
}
</style>