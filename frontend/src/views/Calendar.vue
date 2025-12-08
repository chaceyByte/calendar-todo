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
            <el-icon><arrow-left /></el-icon>
            上个月
          </el-button>
          <el-button @click="nextMonth">
            下个月
            <el-icon><arrow-right /></el-icon>
          </el-button>
        </el-button-group>

        <span class="current-month">{{ currentMonthText }}</span>

        <el-button-group>
          <el-button @click="exportDailyReport" type="primary">
            <el-icon><document /></el-icon>
            导出日报
          </el-button>
          <el-button @click="exportWeeklyReport" type="success">
            <el-icon><files /></el-icon>
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
                <el-icon><clock /></el-icon>
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
                <el-icon><circle /></el-icon>
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
      @click="closeContextMenu"
    >
      <div class="menu-item" @click="exportDayReport">
        <el-icon><document /></el-icon>
        导出日报
      </div>
      <div class="menu-item" @click="viewDayTasks">
        <el-icon><view /></el-icon>
        查看任务
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, nextTick } from 'vue'
import dayjs from 'dayjs'
import { ArrowLeft, ArrowRight, Document, Files, View, Clock, Circle } from '@element-plus/icons-vue'
import { ElMessage } from 'element-plus'
import { useTaskStore } from '@/stores/task'
import { useActivityStore } from '@/stores/activity'

interface Task {
  id: number
  title: string
  status: 'planning' | 'in-progress' | 'completed' | 'paused'
  startDate?: string
  endDate?: string
  createdAt: string
  updatedAt: string
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
// 手动点击获取焦点
const handleClickToFocus = () => {
  if (calendarContainer.value) {
    calendarContainer.value.focus()
    console.log('🖱️ 手动点击获取焦点成功')
  }
}

// 原生document事件监听器 - 最终解决方案
const handleGlobalKeyDown = (e: KeyboardEvent) => {
  console.group('🎹 GLOBAL Keyboard Event - Native Document Listener')
  console.log(`⏰ [${new Date().toISOString()}] 全局键盘事件触发`)

  // 详细事件信息
  console.log('📋 事件详细信息:', {
    key: e.key,
    code: e.code,
    keyCode: e.keyCode,
    ctrlKey: e.ctrlKey,
    metaKey: e.metaKey,
    shiftKey: e.shiftKey,
    altKey: e.altKey,
    repeat: e.repeat,
    isComposing: e.isComposing,
    targetElement: e.target?.tagName,
    targetClass: e.target?.className
  })

  console.log('🎯 当前焦点状态:', {
    documentHasFocus: document.hasFocus(),
    activeElement: document.activeElement?.tagName,
    isCalendarFocused: calendarContainer.value === document.activeElement
  })

  // 平台检测
  const isMac = navigator.platform.includes('Mac')
  console.log('??️ 平台信息:', {
    platform: navigator.platform,
    isMac,
    userAgent: navigator.userAgent?.substring(0, 80) + '...'
  })

  // 测试按键 - 任何时候按?键都会响应
  if (e.key === '?') {
    console.log('🎉 GLOBAL: 键盘事件监听正常工作！')
    ElMessage.success('全局键盘事件监听正常！')
    e.preventDefault()
    console.groupEnd()
    return
  }

  // macOS Cmd键检测
  const isMacCmd = isMac && e.metaKey
  const isWindowsCtrl = !isMac && e.ctrlKey
  const isUndoShortcut = (isMacCmd || isWindowsCtrl) &&
                         e.key === 'z' &&
                         !e.shiftKey &&
                         !e.altKey

  console.log('🔍 快捷键分析:', {
    isUndoShortcut,
    detectedAs: isMac ? 'Cmd+Z' : 'Ctrl+Z',
    isMacCmd,
    isWindowsCtrl
  })

  if (isUndoShortcut) {
    console.log('✅ 检测到撤销快捷键:', isMac ? 'Cmd+Z' : 'Ctrl+Z')

    // 立即阻止默认行为和传播
    e.preventDefault()
    e.stopPropagation()
    e.stopImmediatePropagation()

    console.log('🚀 开始执行撤销操作...')
    console.log('📊 Store状态:', {
      undoStackLength: taskStore.undoStack?.length || 0,
      redoStackLength: taskStore.redoStack?.length || 0
    })

    // 执行撤销操作
    taskStore.undoLastOperation()
      .then(success => {
        console.log(`✅ 撤销操作结果: ${success ? '成功' : '失败或无操作可撤销'}`)
        if (success) {
          ElMessage.success('撤销操作成功')
        } else {
          ElMessage.info('没有可撤销的操作')
        }
      })
      .catch(error => {
        console.error('❌ 撤销操作错误:', error)
        ElMessage.error('撤销操作失败')
      })
      .finally(() => {
        console.groupEnd()
      })

  } else if (e.key === 'Escape' && contextMenu.value.visible) {
    console.log('✅ 检测到ESC键 - 关闭右键菜单')
    closeContextMenu()
    e.preventDefault()
    e.stopPropagation()
    console.groupEnd()
  } else {
    console.log('ℹ️ 其他按键 - 不处理')
    console.groupEnd()
  }
}

// 确保组件获得焦点（增强版）
const focusCalendar = () => {
  nextTick(() => {
    if (calendarContainer.value) {
      // 多次尝试确保焦点设置成功
      let focusAttempts = 0
      const maxAttempts = 5
      const focusDelay = 200 // 增加延迟时间

      const attemptFocus = () => {
        focusAttempts++
        calendarContainer.value?.focus({ preventScroll: true })

        // 添加视觉反馈
        if (calendarContainer.value) {
          calendarContainer.value.style.boxShadow = '0 0 0 2px #3b82f6'
          setTimeout(() => {
            if (calendarContainer.value) {
              calendarContainer.value.style.boxShadow = ''
            }
          }, 500)
        }

        console.group('🎯 焦点设置尝试')
        console.log(`尝试次数: ${focusAttempts}/${maxAttempts}`)
        console.log('当前焦点状态:', {
          documentHasFocus: document.hasFocus(),
          activeElement: document.activeElement?.tagName,
          isCalendarFocused: calendarContainer.value === document.activeElement,
          calendarTabIndex: calendarContainer.value?.tabIndex,
          calendarVisible: calendarContainer.value?.offsetParent !== null
        })

        if (calendarContainer.value === document.activeElement) {
          console.log('✅ 焦点设置成功')
          console.groupEnd()
          return true
        } else if (focusAttempts < maxAttempts) {
          console.log('🔄 焦点设置未成功，再次尝试...')
          console.groupEnd()
          setTimeout(attemptFocus, 100)
          return false
        } else {
          console.log('⚠️ 焦点设置最终未成功，可能由于无痕模式限制')
          console.log('💡 建议: 用户可能需要手动点击日历区域以激活键盘事件')
          console.groupEnd()
          return false
        }
      }

      attemptFocus()
    }
  })
}

// 任务和活动数据
const tasks = ref<Task[]>([])
const activities = ref<ActivityRecord[]>([])

// 加载数据
const loadData = async () => {
  try {
    tasks.value = await taskStore.fetchTasks()

    // 获取所有任务的活动记录
    activities.value = []
    for (const task of tasks.value) {
      try {
        const taskActivities = await activityStore.getTaskActivities(task.id)
        activities.value.push(...taskActivities)
      } catch (error) {
        console.error(`获取任务 ${task.id} 的活动记录失败:`, error)
      }
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
        return currentDay.isSameOrAfter(taskStart) && currentDay.isSameOrBefore(taskEnd)
      } else if (taskStart) {
        return currentDay.isSameOrAfter(taskStart)
      } else if (taskEnd) {
        return currentDay.isSameOrBefore(taskEnd)
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

const exportDayReport = () => {
  if (contextMenu.value.selectedDay) {
    ElMessage.success(`导出 ${contextMenu.value.selectedDay.date} 的日报`)
  }
  closeContextMenu()
}

const viewDayTasks = () => {
  if (contextMenu.value.selectedDay) {
    ElMessage.info(`查看 ${contextMenu.value.selectedDay.date} 的任务`)
  }
  closeContextMenu()
}

const exportDailyReport = () => {
  ElMessage.success('导出日报成功')
}

const exportWeeklyReport = () => {
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
const handleClickOutside = (e: MouseEvent) => {
  if (contextMenu.value.visible) {
    closeContextMenu()
  }
onMounted(() => {
  document.addEventListener('click', handleClickOutside)

  // 添加全局键盘事件监听 - 最可靠的解决方案
  console.group('🎯 全局键盘事件监听器设置')
  document.addEventListener('keydown', handleGlobalKeyDown, {
    capture: true,
    passive: false
  })
  console.log('✅ 全局键盘事件监听器已添加 (capture: true, passive: false)')
  console.log('🎯 监听器特点:', {
    bubblePhase: 'capture (最优先)',
    passive: false,
    alwaysWorks: '是 (不受焦点限制)',
    scope: '整个文档'
  })
  console.groupEnd()

  // 添加用户友好的提示
  setTimeout(() => {
    ElMessage.info('现在可以在任意位置按 ? 键测试全局键盘事件')
  }, 1000)

  // 添加焦点测试
  setTimeout(() => {
    if (calendarContainer.value) {
      calendarContainer.value.focus()
      console.log('🔍 焦点测试:', {
        success: calendarContainer.value === document.activeElement,
        tabIndex: calendarContainer.value.tabIndex
      })
    }
  }, 1000)

  // 立即测试一次键盘监听
  console.log('🧪 立即触发测试事件...')
  setTimeout(() => {
    console.log('🔄 发送虚拟键盘事件测试...')
    const testEvent = new KeyboardEvent('keydown', {
      key: '?',
      bubbles: true,
      cancelable: true
    })
    document.dispatchEvent(testEvent)
  }, 500)

  loadData()
})

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside)
  // 移除全局键盘事件监听
  document.removeEventListener('keydown', handleGlobalKeyDown, { capture: true })
  console.log('🔕 日历组件已卸载 - 全局键盘事件监听器已移除')
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