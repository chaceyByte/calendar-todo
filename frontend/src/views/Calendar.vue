<template>
  <div class="calendar-container">
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
              'has-tasks': day.tasks.length > 0
            }
          ]"
          @contextmenu="(e) => handleDayContextMenu(e, day)"
        >
          <div class="day-header">
            <span class="day-number">{{ day.day }}</span>
            <el-badge 
              v-if="day.tasks.length > 0" 
              :value="day.tasks.length" 
              class="task-badge" 
            />
          </div>
          
          <div class="day-tasks">
            <div 
              v-for="task in day.tasks.slice(0, 3)" 
              :key="task.id"
              :class="['task-item', `status-${task.status}`]"
              :title="task.title"
            >
              {{ task.title }}
            </div>
            <div v-if="day.tasks.length > 3" class="more-tasks">
              +{{ day.tasks.length - 3 }}更多
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
import { ref, computed, onMounted, onUnmounted } from 'vue'
import dayjs from 'dayjs'
import { ArrowLeft, ArrowRight, Document, Files, View } from '@element-plus/icons-vue'

interface Task {
  id: number
  title: string
  status: 'planning' | 'in-progress' | 'completed'
  startDate: string
  endDate: string
}

interface CalendarDay {
  date: string
  day: number
  isToday: boolean
  isCurrentMonth: boolean
  tasks: Task[]
}

const currentDate = ref(dayjs())
const contextMenu = ref({
  visible: false,
  x: 0,
  y: 0,
  selectedDay: null as CalendarDay | null
})

const weekDays = ['日', '一', '二', '三', '四', '五', '六']

// 模拟任务数据
const mockTasks: Task[] = [
  { id: 1, title: '项目会议', status: 'completed', startDate: '2024-01-15', endDate: '2024-01-15' },
  { id: 2, title: '前端开发', status: 'in-progress', startDate: '2024-01-10', endDate: '2024-01-20' },
  { id: 3, title: '数据库设计', status: 'planning', startDate: '2024-01-18', endDate: '2024-01-25' },
  { id: 4, title: '测试计划', status: 'planning', startDate: '2024-01-22', endDate: '2024-01-28' }
]

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
    const dayTasks = mockTasks.filter(task => 
      currentDay.isSameOrAfter(dayjs(task.startDate)) && 
      currentDay.isSameOrBefore(dayjs(task.endDate))
    )
    
    days.push({
      date: dateStr,
      day: currentDay.date(),
      isToday: currentDay.isSame(dayjs(), 'day'),
      isCurrentMonth: currentDay.isSame(currentDate.value, 'month'),
      tasks: dayTasks
    })
    
    currentDay = currentDay.add(1, 'day')
  }
  
  return days
})

const prevMonth = () => {
  currentDate.value = currentDate.value.subtract(1, 'month')
}

const nextMonth = () => {
  currentDate.value = currentDate.value.add(1, 'month')
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

// 点击其他地方关闭右键菜单
const handleClickOutside = (e: MouseEvent) => {
  if (contextMenu.value.visible) {
    closeContextMenu()
  }
}

onMounted(() => {
  document.addEventListener('click', handleClickOutside)
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
}

.calendar-header {
  margin-bottom: 24px;
}

.header-controls {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.current-month {
  font-size: 18px;
  font-weight: 600;
  color: #303133;
}

.calendar-body {
  flex: 1;
  display: flex;
  flex-direction: column;
}

.week-header {
  display: grid;
  grid-template-columns: repeat(7, 1fr);
  gap: 1px;
  margin-bottom: 8px;
}

.week-day {
  text-align: center;
  padding: 12px 0;
  background: #f5f7fa;
  font-weight: 600;
  color: #606266;
}

.calendar-grid {
  flex: 1;
  display: grid;
  grid-template-columns: repeat(7, 1fr);
  gap: 1px;
  background: #e4e7ed;
}

.calendar-day {
  background: white;
  min-height: 120px;
  padding: 8px;
  cursor: pointer;
  transition: all 0.3s;
}

.calendar-day:hover {
  background: #f5f7fa;
  transform: translateY(-2px);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.calendar-day:not(.current-month) {
  background: #fafafa;
  color: #c0c4cc;
}

.calendar-day.today {
  background: #ecf5ff;
  border: 2px solid #409eff;
}

.calendar-day.has-tasks {
  border-left: 3px solid #409eff;
}

.day-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.day-number {
  font-size: 14px;
  font-weight: 600;
}

.day-tasks {
  max-height: 80px;
  overflow: hidden;
}

.task-item {
  font-size: 12px;
  padding: 2px 4px;
  margin-bottom: 2px;
  border-radius: 2px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.task-item.status-planning {
  background: #f0f9ff;
  color: #1890ff;
}

.task-item.status-in-progress {
  background: #fef7ec;
  color: #e6a23c;
}

.task-item.status-completed {
  background: #f0f9eb;
  color: #67c23a;
}

.more-tasks {
  font-size: 11px;
  color: #909399;
  text-align: center;
  margin-top: 4px;
}

.context-menu {
  position: fixed;
  background: white;
  border: 1px solid #e4e7ed;
  border-radius: 4px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.1);
  z-index: 2000;
  min-width: 120px;
}

.menu-item {
  padding: 8px 12px;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 8px;
  transition: background-color 0.3s;
}

.menu-item:hover {
  background: #f5f7fa;
}
</style>