<template>
  <div class="reports-container">
    <!-- 页面头部 -->
    <div class="reports-header">
      <h2>活动报告</h2>
      <div class="header-actions">
        <el-button type="primary" @click="exportReport">
          <el-icon><download /></el-icon>
          导出报告
        </el-button>
      </div>
    </div>

    <!-- 报告类型选择 -->
    <el-tabs v-model="reportTab" @tab-change="handleTabChange">
      <el-tab-pane label="日报" name="daily">
        <div class="daily-report">
          <div class="report-controls">
            <el-date-picker
              v-model="selectedDate"
              type="date"
              placeholder="选择日期"
              @change="loadDailyReport"
              :disabled-date="disableFutureDates"
            />
            <el-button @click="setToday">今天</el-button>
            <el-button @click="setYesterday">昨天</el-button>
          </div>
          
          <el-card class="report-content" v-if="dailyReport">
            <template #header>
              {{ formatDate(selectedDate) }} 活动报告
            </template>
            
            <div class="report-summary">
              <div class="summary-item">
                <span>总活动时间:</span>
                <span class="summary-value">{{ activityStore.formatDuration(dailyReport.totalTime) }}</span>
              </div>
              <div class="summary-item">
                <span>完成任务:</span>
                <span class="summary-value">{{ dailyReport.completedTasks }}</span>
              </div>
              <div class="summary-item">
                <span>活动任务:</span>
                <span class="summary-value">{{ dailyReport.activeTasks }}</span>
              </div>
            </div>
            
            <div class="task-activities" v-if="dailyReport.taskActivities.length > 0">
              <h4>任务活动详情</h4>
              <el-table :data="dailyReport.taskActivities" style="width: 100%">
                <el-table-column prop="taskTitle" label="任务" />
                <el-table-column prop="duration" label="活动时间">
                  <template #default="row">
                    {{ activityStore.formatDuration(row.duration) }}
                  </template>
                </el-table-column>
                <el-table-column prop="status" label="状态">
                  <template #default="row">
                    <el-tag :type="getStatusType(row.status)">
                      {{ getStatusText(row.status) }}
                    </el-tag>
                  </template>
                </el-table-column>
                <el-table-column label="操作" width="120">
                  <template #default="row">
                    <el-button size="small" text @click="viewTaskActivities(row.taskId)">
                      查看详情
                    </el-button>
                  </template>
                </el-table-column>
              </el-table>
            </div>
            
            <div v-else class="no-data">
              <el-empty description="当天没有活动记录" />
            </div>
          </el-card>
        </div>
      </el-tab-pane>
      
      <el-tab-pane label="周报" name="weekly">
        <div class="weekly-report">
          <div class="report-controls">
            <el-date-picker
              v-model="weekStart"
              type="week"
              placeholder="选择周"
              format="YYYY 第 ww 周"
              @change="loadWeeklyReport"
              :disabled-date="disableFutureDates"
            />
            <el-button @click="setThisWeek">本周</el-button>
            <el-button @click="setLastWeek">上周</el-button>
          </div>
          
          <el-card class="report-content" v-if="weeklyReport">
            <template #header>
              {{ formatDateRange(weekStart, weekEnd) }} 活动报告
            </template>
            
            <div class="report-summary">
              <div class="summary-item">
                <span>总活动时间:</span>
                <span class="summary-value">{{ activityStore.formatDuration(weeklyReport.totalTime) }}</span>
              </div>
              <div class="summary-item">
                <span>完成任务:</span>
                <span class="summary-value">{{ weeklyReport.completedTasks }}</span>
              </div>
            </div>
            
            <!-- 每日统计图表 -->
            <div class="daily-chart" v-if="weeklyReport.dailySummaries">
              <h4>每日活动统计</h4>
              <div class="chart-container">
                <div 
                  v-for="(day, date) in weeklyReport.dailySummaries" 
                  :key="date"
                  class="day-bar"
                >
                  <div class="bar-container">
                    <div 
                      class="bar" 
                      :style="{ height: getBarHeight(day.totalTime, maxDailyTime) + '%' }"
                    ></div>
                  </div>
                  <div class="day-label">{{ formatDate(date) }}</div>
                  <div class="day-time">{{ activityStore.formatDuration(day.totalTime) }}</div>
                </div>
              </div>
            </div>
            
            <!-- 任务活动详情 -->
            <div class="task-activities" v-if="weeklyReport.taskActivities.length > 0">
              <h4>任务活动详情</h4>
              <el-table :data="weeklyReport.taskActivities" style="width: 100%">
                <el-table-column prop="taskTitle" label="任务" />
                <el-table-column prop="totalDuration" label="总活动时间">
                  <template #default="row">
                    {{ activityStore.formatDuration(row.totalDuration) }}
                  </template>
                </el-table-column>
                <el-table-column prop="status" label="状态">
                  <template #default="row">
                    <el-tag :type="getStatusType(row.status)">
                      {{ getStatusText(row.status) }}
                    </el-tag>
                  </template>
                </el-table-column>
                <el-table-column label="操作" width="120">
                  <template #default="row">
                    <el-button size="small" text @click="viewTaskActivities(row.taskId)">
                      查看详情
                    </el-button>
                  </template>
                </el-table-column>
              </el-table>
            </div>
            
            <div v-else class="no-data">
              <el-empty description="本周没有活动记录" />
            </div>
          </el-card>
        </div>
      </el-tab-pane>
    </el-tabs>

    <!-- 任务活动详情对话框 -->
    <el-dialog
      v-model="taskActivitiesDialog.visible"
      :title="`${taskActivitiesDialog.taskTitle} - 活动记录`"
      width="600px"
    >
      <el-timeline>
        <el-timeline-item
          v-for="activity in taskActivitiesDialog.activities"
          :key="activity.id"
          :timestamp="formatDateTime(activity.startTime)"
          :type="getActivityTimelineType(activity.activityType)"
        >
          <div class="activity-content">
            <div class="activity-title">
              {{ activityStore.getActivityTypeDescription(activity.activityType) }}
            </div>
            <div class="activity-description" v-if="activity.description">
              {{ activity.description }}
            </div>
            <div class="activity-duration" v-if="activity.duration">
              持续时间: {{ activityStore.formatDuration(activity.duration) }}
            </div>
          </div>
        </el-timeline-item>
      </el-timeline>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue'
import dayjs from 'dayjs'
import { Download } from '@element-plus/icons-vue'
import { ElMessage } from 'element-plus'
import { useActivityStore } from '@/stores/activity'
import { useTaskStore } from '@/stores/task'

const activityStore = useActivityStore()
const taskStore = useTaskStore()

// 报告类型
const reportTab = ref('daily')

// 日期选择
const selectedDate = ref(new Date())
const weekStart = ref(getWeekStart(new Date()))
const weekEnd = ref(getWeekEnd(new Date()))

// 报告数据
const dailyReport = ref(null)
const weeklyReport = ref(null)

// 任务活动详情对话框
const taskActivitiesDialog = reactive({
  visible: false,
  taskId: 0,
  taskTitle: '',
  activities: []
})

// 计算属性
const maxDailyTime = computed(() => {
  if (!weeklyReport.value || !weeklyReport.value.dailySummaries) return 0
  
  let max = 0
  Object.values(weeklyReport.value.dailySummaries).forEach((day: any) => {
    if (day.totalTime > max) max = day.totalTime
  })
  return max || 1 // 避免除以0
})

// 方法
const getWeekStart = (date: Date) => {
  const d = new Date(date)
  const day = d.getDay()
  const diff = d.getDate() - day + (day === 0 ? -6 : 1) // 调整为周一开始
  return new Date(d.setDate(diff))
}

const getWeekEnd = (date: Date) => {
  const start = getWeekStart(date)
  const end = new Date(start)
  end.setDate(start.getDate() + 6)
  return end
}

const formatDate = (date: Date | string) => {
  return dayjs(date).format('YYYY-MM-DD')
}

const formatDateRange = (start: Date | string, end: Date | string) => {
  return `${formatDate(start)} ~ ${formatDate(end)}`
}

const formatDateTime = (dateTime: string) => {
  return dayjs(dateTime).format('YYYY-MM-DD HH:mm')
}

const disableFutureDates = (time: Date) => {
  return time.getTime() > Date.now()
}

const setToday = () => {
  selectedDate.value = new Date()
  loadDailyReport()
}

const setYesterday = () => {
  const yesterday = new Date()
  yesterday.setDate(yesterday.getDate() - 1)
  selectedDate.value = yesterday
  loadDailyReport()
}

const setThisWeek = () => {
  weekStart.value = getWeekStart(new Date())
  weekEnd.value = getWeekEnd(new Date())
  loadWeeklyReport()
}

const setLastWeek = () => {
  const lastWeek = new Date()
  lastWeek.setDate(lastWeek.getDate() - 7)
  weekStart.value = getWeekStart(lastWeek)
  weekEnd.value = getWeekEnd(lastWeek)
  loadWeeklyReport()
}

const loadDailyReport = async () => {
  try {
    dailyReport.value = await activityStore.getDailyReport(formatDate(selectedDate.value))
  } catch (error) {
    console.error('加载日报失败:', error)
    ElMessage.error('加载日报失败')
  }
}

const loadWeeklyReport = async () => {
  try {
    weeklyReport.value = await activityStore.getWeeklyReport(formatDate(weekStart.value))
  } catch (error) {
    console.error('加载周报失败:', error)
    ElMessage.error('加载周报失败')
  }
}

const handleTabChange = (tabName: string) => {
  if (tabName === 'daily') {
    loadDailyReport()
  } else if (tabName === 'weekly') {
    loadWeeklyReport()
  }
}

const viewTaskActivities = async (taskId: number) => {
  try {
    // 获取任务信息
    const tasks = await taskStore.fetchTasks()
    const task = tasks.find(t => t.id === taskId)
    
    if (task) {
      taskActivitiesDialog.taskId = taskId
      taskActivitiesDialog.taskTitle = task.title
      taskActivitiesDialog.activities = await activityStore.getTaskActivities(taskId)
      taskActivitiesDialog.visible = true
    }
  } catch (error) {
    console.error('获取任务活动记录失败:', error)
    ElMessage.error('获取任务活动记录失败')
  }
}

const getStatusType = (status: string) => {
  switch (status) {
    case 'planning':
      return 'info'
    case 'in-progress':
      return 'primary'
    case 'completed':
      return 'success'
    case 'paused':
      return 'warning'
    default:
      return 'info'
  }
}

const getStatusText = (status: string) => {
  switch (status) {
    case 'planning':
      return '计划中'
    case 'in-progress':
      return '进行中'
    case 'completed':
      return '已完成'
    case 'paused':
      return '已暂停'
    default:
      return '未知'
  }
}

const getActivityTimelineType = (activityType: string) => {
  switch (activityType) {
    case 'CREATED':
      return 'primary'
    case 'STARTED':
      return 'success'
    case 'PAUSED':
      return 'warning'
    case 'RESUMED':
      return 'info'
    case 'COMPLETED':
      return 'success'
    case 'WORK':
      return 'primary'
    case 'MEETING':
      return 'warning'
    case 'STUDY':
      return 'info'
    default:
      return 'info'
  }
}

const getBarHeight = (value: number, max: number) => {
  return max > 0 ? (value / max) * 100 : 0
}

const exportReport = () => {
  ElMessage.info('导出功能开发中...')
}

// 初始化
onMounted(() => {
  loadDailyReport()
})
</script>

<style scoped>
.reports-container {
  height: 100%;
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.reports-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.report-controls {
  display: flex;
  align-items: center;
  gap: 16px;
  margin-bottom: 24px;
}

.report-content {
  margin-bottom: 24px;
}

.report-summary {
  display: flex;
  gap: 24px;
  margin-bottom: 24px;
}

.summary-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 16px;
  background: #f5f7fa;
  border-radius: 8px;
  min-width: 120px;
}

.summary-value {
  font-size: 24px;
  font-weight: 600;
  margin-top: 8px;
}

.task-activities {
  margin-top: 24px;
}

.task-activities h4 {
  margin-bottom: 16px;
}

.no-data {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 200px;
}

.daily-chart {
  margin: 24px 0;
}

.chart-container {
  display: flex;
  align-items: flex-end;
  height: 200px;
  gap: 16px;
  padding: 16px 0;
}

.day-bar {
  display: flex;
  flex-direction: column;
  align-items: center;
  flex: 1;
}

.bar-container {
  height: 150px;
  width: 40px;
  background: #f5f7fa;
  border-radius: 4px;
  position: relative;
  display: flex;
  align-items: flex-end;
}

.bar {
  width: 100%;
  background: #409eff;
  border-radius: 4px;
  transition: height 0.3s;
}

.day-label {
  margin-top: 8px;
  font-size: 12px;
  color: #606266;
}

.day-time {
  font-size: 12px;
  color: #909399;
}

.activity-content {
  padding-bottom: 10px;
}

.activity-title {
  font-weight: 600;
  margin-bottom: 4px;
}

.activity-description {
  font-size: 12px;
  color: #606266;
  margin-bottom: 4px;
}

.activity-duration {
  font-size: 12px;
  color: #909399;
}
</style>