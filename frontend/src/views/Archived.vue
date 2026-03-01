<template>
  <div class="archived-container">
    <!-- 页面头部 - 专业列表风格 -->
    <div class="page-header">
      <div class="header-content">
        <h1 class="page-title">归档任务</h1>
        <p class="page-subtitle">回顾已完成的成就，分析历史数据</p>
      </div>
      <div class="header-stats">
        <div class="stat-card">
          <div class="stat-icon">
            <el-icon><check /></el-icon>
          </div>
          <div class="stat-info">
            <span class="stat-number">{{ total }}</span>
            <span class="stat-label">已完成</span>
          </div>
        </div>
        <div class="stat-card">
          <div class="stat-icon">
            <el-icon><timer /></el-icon>
          </div>
          <div class="stat-info">
            <span class="stat-number">{{ Math.floor(total / 10) }}</span>
            <span class="stat-label">工作天</span>
          </div>
        </div>
      </div>
    </div>

    <!-- 搜索和筛选区域 - 专业风格 -->
    <div class="search-section">
      <div class="search-controls">
        <div class="search-input-wrapper">
          <el-icon class="search-icon"><search /></el-icon>
          <el-input
            v-model="searchKeyword"
            placeholder="搜索任务标题、描述或标签..."
            clearable
            @clear="handleSearch"
            @keyup.enter="handleSearch"
            class="search-input"
          />
        </div>
        <div class="action-buttons">
          <el-button type="primary" class="btn-search" @click="handleSearch">
            <el-icon><search /></el-icon>
            搜索
          </el-button>
          <el-button class="btn-refresh" @click="refreshData">
            <el-icon><refresh /></el-icon>
            刷新
          </el-button>
        </div>
      </div>
    </div>

    <!-- 任务列表区域 -->
    <div class="task-section">
      <!-- 空状态 -->
      <div v-if="tasks.length === 0" class="empty-state">
        <div class="empty-content">
          <div class="empty-icon">
            <el-icon><document /></el-icon>
          </div>
          <h3 class="empty-title">暂无归档任务</h3>
          <p class="empty-description">完成你的第一个任务，它将出现在这里</p>
        </div>
      </div>

      <!-- 任务列表表格 -->
      <div v-else class="task-list-container">
        <!-- 列表头部 -->
        <div class="list-header">
          <div class="header-column task-title-col">任务标题</div>
          <div class="header-column task-tags-col">标签</div>
          <div class="header-column task-date-col">创建时间</div>
          <div class="header-column task-progress-col">进度</div>
          <div class="header-column task-actions-col">操作</div>
        </div>

        <!-- 任务列表 -->
        <div class="task-list">
          <div
            v-for="task in tasks"
            :key="task.id"
            class="task-list-item"
            @click="viewTaskDetails(task)"
          >
            <!-- 任务标题和描述 -->
            <div class="list-column task-title-col">
              <div class="task-title-wrapper">
                <h4 class="task-title">{{ task.title }}</h4>
                <p class="task-description">{{ task.description || '暂无描述' }}</p>
              </div>
            </div>

            <!-- 标签 -->
            <div class="list-column task-tags-col">
              <div v-if="task.tags && task.tags.length > 0" class="tag-list">
                <el-tag
                  v-for="tag in task.tags.slice(0, 3)"
                  :key="tag"
                  size="small"
                  class="tag-item"
                  :style="{ backgroundColor: getTagColor(tag) }"
                >
                  {{ tag }}
                </el-tag>
                <el-tag
                  v-if="task.tags.length > 3"
                  size="small"
                  class="tag-more"
                >
                  +{{ task.tags.length - 3 }}
                </el-tag>
              </div>
              <span v-else class="no-tags">无标签</span>
            </div>

            <!-- 日期信息 -->
            <div class="list-column task-date-col">
              <div class="date-info">
                <div class="date-item">
                  <el-icon class="date-icon"><calendar /></el-icon>
                  <span class="date-text">{{ formatDate(task.createdAt) }}</span>
                </div>
                <div class="date-item">
                  <el-icon class="date-icon"><clock /></el-icon>
                  <span class="date-text">{{ formatDate(task.updatedAt) }}</span>
                </div>
              </div>
            </div>

            <!-- 进度 -->
            <div class="list-column task-progress-col">
              <div class="progress-info">
                <el-progress
                  :percentage="task.progress || 100"
                  :show-text="false"
                  :stroke-width="6"
                  status="success"
                  class="progress-bar"
                />
                <span class="progress-text">{{ task.progress || 100 }}%</span>
              </div>
            </div>

            <!-- 操作按钮 -->
            <div class="list-column task-actions-col">
              <div class="action-buttons" @click.stop>
                <el-button
                  size="small"
                  type="primary"
                  class="btn-detail"
                  @click="viewTaskDetails(task)"
                >
                  <el-icon><view /></el-icon>
                  详情
                </el-button>
                <el-button
                  size="small"
                  class="btn-activity"
                  @click="showActivityDrawer(task)"
                >
                  <el-icon><timer /></el-icon>
                  记录
                </el-button>
                <el-dropdown trigger="click" class="more-dropdown">
                  <el-button size="small" class="btn-more">
                    <el-icon><more /></el-icon>
                  </el-button>
                  <template #dropdown>
                    <el-dropdown-menu>
                      <el-dropdown-item @click="deleteTask(task.id)">
                        <el-icon><delete /></el-icon>
                        删除任务
                      </el-dropdown-item>
                    </el-dropdown-menu>
                  </template>
                </el-dropdown>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 分页组件 -->
    <div v-if="total > 0" class="pagination-section">
      <el-pagination
        v-model:current-page="currentPage"
        v-model:page-size="pageSize"
        :page-sizes="[12, 24, 48, 96]"
        :total="total"
        layout="total, sizes, prev, pager, next, jumper"
        @size-change="handleSizeChange"
        @current-change="handleCurrentChange"
        class="modern-pagination"
      />
    </div>

    <!-- 任务详情对话框 -->
    <el-dialog
      v-model="detailDialog.visible"
      :title="`任务详情 - ${detailDialog.task?.title}`"
      width="600px"
      class="detail-dialog"
    >
      <div v-if="detailDialog.task" class="task-detail">
        <div class="detail-header">
          <h3 class="detail-title">{{ detailDialog.task.title }}</h3>
          <el-tag type="success" size="large" class="status-badge">
            <el-icon><trophy /></el-icon>
            已完成
          </el-tag>
        </div>
        
        <el-descriptions :column="1" border class="detail-descriptions">
          <el-descriptions-item label="任务描述">
            <span class="description-text">
              {{ detailDialog.task.description || '暂无描述' }}
            </span>
          </el-descriptions-item>
          <el-descriptions-item label="完成进度">
            <el-progress
              :percentage="detailDialog.task.progress || 100"
              status="success"
              :stroke-width="8"
              class="detail-progress"
            />
          </el-descriptions-item>
          <el-descriptions-item label="创建时间">
            <span class="time-text">{{ formatDateTime(detailDialog.task.createdAt) }}</span>
          </el-descriptions-item>
          <el-descriptions-item label="完成时间">
            <span class="time-text">{{ formatDateTime(detailDialog.task.updatedAt) }}</span>
          </el-descriptions-item>
          <el-descriptions-item v-if="detailDialog.task.tags && detailDialog.task.tags.length > 0" label="任务标签">
            <div class="tag-list">
              <el-tag
                v-for="tag in detailDialog.task.tags"
                :key="tag"
                size="small"
                class="detail-tag"
                :style="{ backgroundColor: getTagColor(tag) }"
              >
                {{ tag }}
              </el-tag>
            </div>
          </el-descriptions-item>
        </el-descriptions>
      </div>
      <template #footer>
        <el-button @click="detailDialog.visible = false" round>关闭</el-button>
      </template>
    </el-dialog>

    <!-- 活动记录抽屉 -->
    <el-drawer
      v-model="activityDrawer.visible"
      :title="`${activityDrawer.taskTitle} - 活动记录`"
      direction="rtl"
      size="500px"
      class="activity-drawer"
    >
      <div class="drawer-header">
        <h3 class="drawer-title">活动历史</h3>
        <p class="drawer-subtitle">查看任务执行过程中的详细记录</p>
      </div>
      
      <div class="activity-timeline">
        <el-timeline>
          <el-timeline-item
            v-for="activity in activities"
            :key="activity.id"
            :timestamp="formatDateTime(activity.startTime)"
            :type="getActivityTimelineType(activity)"
            class="timeline-item"
          >
            <div class="activity-content" :data-type="getActivityDataType(activity)">
              <div class="activity-title">
                {{ getActivityDescription(activity) }}
              </div>
              <div class="activity-details">
                <div class="activity-time-range" v-if="activity.startTime">
                  <span class="time-label">时间范围:</span>
                  <span class="time-value">{{ formatDateTime(activity.startTime) }} - {{ activity.endTime ? formatDateTime(activity.endTime) : '进行中' }}</span>
                </div>
                <div class="activity-duration" v-if="activity.duration">
                  <span class="duration-label">持续时间:</span>
                  <span class="duration-value">{{ activityStore.formatDuration(activity.duration) }}</span>
                </div>
                <div class="activity-type" v-if="activity.notes || activity.description">
                  <span class="type-label">活动类型:</span>
                  <span class="type-value">{{ getActivityTypeDescription(activity.notes || activity.description || '') }}</span>
                </div>
              </div>
            </div>
          </el-timeline-item>
        </el-timeline>
      </div>

      <!-- 活动统计 -->
      <div class="activity-stats" v-if="activities.length > 0">
        <el-card class="stats-card">
          <template #header>
            <div class="stats-header">
              <el-icon><data-analysis /></el-icon>
              <span>活动统计</span>
            </div>
          </template>
          <div class="stats-grid">
            <div class="stat-item">
              <div class="stat-icon">⏱️</div>
              <div class="stat-content">
                <span class="stat-label">总活动时间</span>
                <span class="stat-value">{{ activityStore.formatDuration(getTotalActivityTime()) }}</span>
              </div>
            </div>
            <div class="stat-item">
              <div class="stat-icon">📝</div>
              <div class="stat-content">
                <span class="stat-label">活动记录</span>
                <span class="stat-value">{{ activities.length }} 条</span>
              </div>
            </div>
            <div class="stat-item">
              <div class="stat-icon">📅</div>
              <div class="stat-content">
                <span class="stat-label">工作天数</span>
                <span class="stat-value">{{ getWorkDaysCount() }} 天</span>
              </div>
            </div>
          </div>
        </el-card>
      </div>

      <div class="activity-actions">
        <el-button type="primary" @click="showManualActivityDialog({ id: activityDrawer.taskId } as Task)" round>
          <el-icon><plus /></el-icon>
          添加活动记录
        </el-button>
      </div>
    </el-drawer>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, computed } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus/es'
import {
  Search,
  Refresh,
  DocumentRemove,
  More,
  Timer,
  Delete,
  Plus,
  Check,
  Clock,
  TrendCharts,
  View,
  Trophy,
  DataAnalysis,
  Document,
  Calendar
} from '@element-plus/icons-vue'
import { getArchivedTasks, deleteTask as deleteTaskApi, updateTask } from '@/api/task'
import { useActivityStore, type ActivityRecord } from '@/stores/activity'
import dayjs from 'dayjs'

interface Task {
  id: number
  title: string
  description: string
  status: string
  progress: number
  tags: string[]
  createdAt: string
  updatedAt: string
}

const activityStore = useActivityStore()

// 响应式数据
const tasks = ref<Task[]>([])
const total = ref(0)
const currentPage = ref(1)
const pageSize = ref(12)
const searchKeyword = ref('')
const activities = ref<ActivityRecord[]>([])

// 标签颜色映射 - 生成协调的颜色
const tagColorMap = ref<Record<string, string>>({})

// 对话框状态
const detailDialog = reactive({
  visible: false,
  task: null as Task | null
})

const editDialog = reactive({
  visible: false,
  form: {
    id: 0,
    title: '',
    description: '',
    progress: 100
  }
})

const activityDrawer = reactive({
  visible: false,
  taskId: 0,
  taskTitle: ''
})

const manualActivityDialog = reactive({
  visible: false,
  taskId: 0,
  form: {
    activityType: 'WORK',
    startTime: '',
    endTime: '',
    description: ''
  }
})

// 生成标签颜色
const generateTagColor = (tag: string) => {
  if (tagColorMap.value[tag]) {
    return tagColorMap.value[tag]
  }
  
  // 基于标签名称生成颜色
  const colors = [
    'rgba(13, 148, 136, 0.15)',  // Teal
    'rgba(249, 115, 22, 0.15)',  // Orange
    'rgba(16, 185, 129, 0.15)',  // Green
    'rgba(59, 130, 246, 0.15)',  // Blue
    'rgba(139, 92, 246, 0.15)',  // Purple
    'rgba(236, 72, 153, 0.15)',  // Pink
    'rgba(245, 158, 11, 0.15)',  // Amber
    'rgba(107, 114, 128, 0.15)'  // Gray
  ]
  
  const hash = tag.split('').reduce((acc, char) => char.charCodeAt(0) + ((acc << 5) - acc), 0)
  const color = colors[Math.abs(hash) % colors.length]
  tagColorMap.value[tag] = color
  return color
}

// 获取标签颜色
const getTagColor = (tag: string) => {
  return tagColorMap.value[tag] || generateTagColor(tag)
}

// 获取活动类型描述
const getActivityTypeDescription = (activityType: string) => {
  if (activityType.includes('CREATED') || activityType.includes('创建')) {
    return '创建'
  } else if (activityType.includes('STARTED') || activityType.includes('开始')) {
    return '开始'
  } else if (activityType.includes('COMPLETED') || activityType.includes('完成')) {
    return '完成'
  } else if (activityType.includes('WORK') || activityType.includes('工作')) {
    return '工作'
  } else if (activityType.includes('MEETING') || activityType.includes('会议')) {
    return '会议'
  } else if (activityType.includes('STUDY') || activityType.includes('学习')) {
    return '学习'
  } else {
    return '其他'
  }
}

// 计算总活动时间
const getTotalActivityTime = () => {
  return activities.value
    .filter(a => a.duration)
    .reduce((total, activity) => total + (activity.duration || 0), 0)
}

const getWorkDaysCount = () => {
  const uniqueDays = new Set()
  activities.value.forEach(activity => {
    if (activity.startTime) {
      uniqueDays.add(activity.startTime.split(' ')[0])
    }
  })
  return uniqueDays.size
}



// 加载归档任务
const loadArchivedTasks = async () => {
  try {
    const data = await getArchivedTasks(currentPage.value, pageSize.value, searchKeyword.value)
    if (data && data.records) {
      tasks.value = data.records || []
      total.value = data.total || 0
      
      // 为标签生成颜色
      tasks.value.forEach(task => {
        if (task.tags) {
          task.tags.forEach(tag => generateTagColor(tag))
        }
      })
    }
  } catch (error) {
    console.error('加载归档任务失败:', error)
    ElMessage.error('加载归档任务失败')
  }
}

// 搜索处理
const handleSearch = () => {
  currentPage.value = 1
  loadArchivedTasks()
}

// 刷新数据
const refreshData = () => {
  loadArchivedTasks()
}

// 分页处理
const handleSizeChange = (size: number) => {
  pageSize.value = size
  currentPage.value = 1
  loadArchivedTasks()
}

const handleCurrentChange = (page: number) => {
  currentPage.value = page
  loadArchivedTasks()
}

// 查看任务详情
const viewTaskDetails = (task: Task) => {
  detailDialog.task = task
  detailDialog.visible = true
}

// 保存任务编辑
const saveTaskEdit = async () => {
  try {
    await updateTask(editDialog.form.id, {
      title: editDialog.form.title,
      description: editDialog.form.description,
      progress: editDialog.form.progress,
      status: 'completed'
    })
    
    ElMessage.success('任务更新成功')
    editDialog.visible = false
    loadArchivedTasks()
  } catch (error) {
    console.error('更新任务失败:', error)
    ElMessage.error('更新任务失败')
  }
}

// 删除任务
const deleteTask = async (taskId: number) => {
  try {
    await ElMessageBox.confirm('确定要删除这个任务吗？此操作不可恢复。', '确认删除', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })

    await deleteTaskApi(taskId)
    ElMessage.success('任务删除成功')
    loadArchivedTasks()
  } catch (error) {
    if (error !== 'cancel') {
      console.error('删除任务失败:', error)
      ElMessage.error('删除任务失败')
    }
  }
}

// 查看活动记录
const showActivityDrawer = async (task: Task) => {
  activityDrawer.taskId = task.id
  activityDrawer.taskTitle = task.title
  activityDrawer.visible = true

  try {
    activities.value = await activityStore.getTaskActivities(task.id)
  } catch (error) {
    console.error('获取活动记录失败:', error)
    ElMessage.error('获取活动记录失败')
  }
}

// 格式化日期
const formatDate = (date: string) => {
  return dayjs(date).format('YYYY-MM-DD')
}

const formatDateTime = (date: string) => {
  return dayjs(date).format('YYYY-MM-DD HH:mm:ss')
}



// 获取活动时间线类型 - 基于数据库字段
const getActivityTimelineType = (activity: ActivityRecord) => {
  const activityType = activity.notes || activity.description || ''
  
  if (activityType.includes('CREATED') || activityType.includes('创建')) {
    return 'primary'
  } else if (activityType.includes('STARTED') || activityType.includes('开始')) {
    return 'success'
  } else if (activityType.includes('COMPLETED') || activityType.includes('完成')) {
    return 'warning'
  } else if (activityType.includes('WORK') || activityType.includes('工作')) {
    return 'primary'
  } else if (activityType.includes('MEETING') || activityType.includes('会议')) {
    return 'success'
  } else if (activityType.includes('STUDY') || activityType.includes('学习')) {
    return 'info'
  } else {
    return 'default'
  }
}

// 获取活动数据类型 - 用于CSS样式
const getActivityDataType = (activity: ActivityRecord) => {
  const activityType = activity.notes || activity.description || ''
  
  if (activityType.includes('CREATED') || activityType.includes('创建')) {
    return 'created'
  } else if (activityType.includes('STARTED') || activityType.includes('开始')) {
    return 'started'
  } else if (activityType.includes('COMPLETED') || activityType.includes('完成')) {
    return 'completed'
  } else if (activityType.includes('WORK') || activityType.includes('工作')) {
    return 'work'
  } else if (activityType.includes('MEETING') || activityType.includes('会议')) {
    return 'meeting'
  } else if (activityType.includes('STUDY') || activityType.includes('学习')) {
    return 'study'
  } else {
    return 'other'
  }
}

// 获取活动描述 - 基于数据库字段
const getActivityDescription = (activity: ActivityRecord) => {
  if (activity.description) {
    return activity.description
  }
  
  if (activity.notes) {
    return activity.notes
  }
  
  if (activity.notes?.includes('CREATED') || activity.description?.includes('创建')) {
    return '任务创建'
  } else if (activity.notes?.includes('STARTED') || activity.description?.includes('开始')) {
    return '开始任务'
  } else if (activity.notes?.includes('COMPLETED') || activity.description?.includes('完成')) {
    return '任务完成'
  }
  
  return '活动记录'
}

// 手动添加活动记录相关方法
const showManualActivityDialog = (task: Task) => {
  manualActivityDialog.taskId = task.id
  manualActivityDialog.form = {
    activityType: 'WORK',
    startTime: '',
    endTime: '',
    description: ''
  }
  manualActivityDialog.visible = true
}

const saveManualActivity = async () => {
  try {
    if (!manualActivityDialog.form.startTime || !manualActivityDialog.form.endTime) {
      ElMessage.warning('请选择开始和结束时间')
      return
    }

    if (new Date(manualActivityDialog.form.startTime) >= new Date(manualActivityDialog.form.endTime)) {
      ElMessage.warning('结束时间必须晚于开始时间')
      return
    }

    await activityStore.addManualActivity({
      taskId: manualActivityDialog.taskId,
      taskTitle: '',
      startTime: manualActivityDialog.form.startTime,
      endTime: manualActivityDialog.form.endTime,
      notes: `${manualActivityDialog.form.activityType}: ${manualActivityDialog.form.description}`
    })

    activities.value = await activityStore.getTaskActivities(manualActivityDialog.taskId)

    manualActivityDialog.visible = false
    ElMessage.success('活动记录已添加')
  } catch (error) {
    console.error('添加活动记录失败:', error)
    ElMessage.error('添加活动记录失败，请重试')
  }
}

// 组件挂载时加载数据
onMounted(() => {
  loadArchivedTasks()
})
</script>

<style scoped>
.archived-container {
  padding: 32px;
  max-width: 1400px;
  margin: 0 auto;
  background: #FAF5FF;
  min-height: 100vh;
}

/* 页面头部 - 专业列表风格 */
.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 32px;
  padding-bottom: 24px;
  border-bottom: 1px solid #E9D5FF;
}

.header-content {
  flex: 1;
}

.page-title {
  margin: 0 0 8px 0;
  font-size: 32px;
  font-weight: 700;
  color: #4C1D95;
  letter-spacing: -0.02em;
}

.page-subtitle {
  margin: 0;
  color: #6B21A8;
  font-size: 16px;
  font-weight: 500;
}

.header-stats {
  display: flex;
  gap: 16px;
}

.stat-card {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 16px 20px;
  background: white;
  border-radius: 12px;
  border: 1px solid #F3E8FF;
  box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1);
  transition: all 0.2s ease-in-out;
  cursor: pointer;
}

.stat-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
  border-color: #7C3AED;
}

.stat-icon {
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #7C3AED;
  color: white;
  border-radius: 8px;
  font-size: 18px;
}

.stat-info {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
}

.stat-number {
  font-size: 24px;
  font-weight: 700;
  color: #4C1D95;
}

.stat-label {
  font-size: 12px;
  color: #6B21A8;
  font-weight: 500;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

/* 搜索和筛选区域 - 专业风格 */
.search-section {
  margin-bottom: 24px;
  padding: 20px 24px;
  background: white;
  border-radius: 12px;
  border: 1px solid #F3E8FF;
  box-shadow: 0 1px 2px 0 rgba(0, 0, 0, 0.05);
}

.search-controls {
  display: flex;
  gap: 16px;
  align-items: center;
}

.search-input-wrapper {
  position: relative;
  flex: 1;
  max-width: 500px;
}

.search-icon {
  position: absolute;
  left: 12px;
  top: 50%;
  transform: translateY(-50%);
  color: #6B21A8;
  z-index: 1;
}

:deep(.search-input .el-input__wrapper) {
  padding-left: 40px;
  border-radius: 8px;
  border: 1px solid #E9D5FF;
  background: white;
  transition: all 0.2s ease-in-out;
}

:deep(.search-input .el-input__wrapper:hover) {
  border-color: #7C3AED;
}

:deep(.search-input .el-input__wrapper.is-focus) {
  border-color: #7C3AED;
  box-shadow: 0 0 0 3px rgba(124, 58, 237, 0.1);
}

.action-buttons {
  display: flex;
  gap: 8px;
}

:deep(.btn-search) {
  background: #7C3AED;
  border: none;
  font-weight: 500;
  border-radius: 6px;
}

:deep(.btn-search:hover) {
  background: #6D28D9;
  transform: translateY(-1px);
}

:deep(.btn-refresh) {
  border: 1px solid #E9D5FF;
  font-weight: 500;
  border-radius: 6px;
}

:deep(.btn-refresh:hover) {
  border-color: #7C3AED;
  color: #7C3AED;
  transform: translateY(-1px);
}

/* 任务列表区域 */
.task-section {
  min-height: 400px;
}

/* 空状态 */
.empty-state {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 80px 40px;
  background: white;
  border-radius: 12px;
  border: 1px solid #F3E8FF;
}

.empty-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
  text-align: center;
}

.empty-icon {
  width: 80px;
  height: 80px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #F3E8FF;
  color: #7C3AED;
  border-radius: 16px;
  font-size: 32px;
}

.empty-title {
  margin: 0;
  font-size: 20px;
  font-weight: 600;
  color: #4C1D95;
}

.empty-description {
  margin: 0;
  color: #6B21A8;
  font-size: 14px;
}

/* 任务列表表格 */
.task-list-container {
  background: white;
  border-radius: 12px;
  border: 1px solid #F3E8FF;
  overflow: hidden;
  box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1);
}

/* 列表头部 */
.list-header {
  display: grid;
  grid-template-columns: 2fr 1fr 1fr 1fr 200px;
  gap: 16px;
  padding: 16px 24px;
  background: #F8FAFC;
  border-bottom: 1px solid #E2E8F0;
  font-weight: 600;
  color: #4C1D95;
  font-size: 14px;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.header-column {
  display: flex;
  align-items: center;
}

/* 任务列表项 */
.task-list {
  max-height: 600px;
  overflow-y: auto;
}

.task-list-item {
  display: grid;
  grid-template-columns: 2fr 1fr 1fr 1fr 200px;
  gap: 16px;
  padding: 20px 24px;
  border-bottom: 1px solid #F1F5F9;
  cursor: pointer;
  transition: all 0.2s ease-in-out;
  position: relative;
}

.task-list-item:hover {
  background: #F8FAFC;
  transform: translateX(4px);
}

.task-list-item:last-child {
  border-bottom: none;
}

.list-column {
  display: flex;
  align-items: center;
}

/* 任务标题列 */
.task-title-col {
  align-items: flex-start;
}

.task-title-wrapper {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.task-title {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: #1F2937;
  line-height: 1.4;
}

.task-description {
  margin: 0;
  color: #6B7280;
  font-size: 14px;
  line-height: 1.5;
  display: -webkit-box;
  -webkit-line-clamp: 1;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

/* 标签列 */
.task-tags-col {
  flex-wrap: wrap;
  gap: 4px;
}

.tag-list {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
}

.tag-item {
  border: none;
  font-size: 11px;
  font-weight: 500;
  color: #7C3AED;
  background: rgba(124, 58, 237, 0.1);
  border-radius: 6px;
  padding: 2px 6px;
}

.tag-more {
  border: none;
  font-size: 11px;
  color: #6B7280;
  background: rgba(107, 114, 128, 0.1);
  border-radius: 6px;
  padding: 2px 6px;
}

.no-tags {
  color: #9CA3AF;
  font-size: 12px;
  font-style: italic;
}

/* 日期列 */
.task-date-col {
  flex-direction: column;
  gap: 4px;
}

.date-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.date-item {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 12px;
  color: #6B7280;
}

.date-icon {
  color: #7C3AED;
  font-size: 12px;
}

.date-text {
  font-size: 12px;
  color: #6B7280;
}

/* 进度列 */
.task-progress-col {
  flex-direction: column;
  gap: 6px;
}

.progress-info {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.progress-bar {
  width: 100%;
}

.progress-text {
  font-size: 12px;
  color: #6B7280;
  font-weight: 500;
  text-align: center;
}

/* 操作列 */
.task-actions-col {
  justify-content: flex-end;
}

.action-buttons {
  display: flex;
  gap: 8px;
  align-items: center;
}

:deep(.btn-detail) {
  background: #7C3AED;
  border: none;
  font-weight: 500;
  border-radius: 6px;
  font-size: 12px;
  padding: 6px 12px;
}

:deep(.btn-detail:hover) {
  background: #6D28D9;
  transform: translateY(-1px);
}

:deep(.btn-activity) {
  border: 1px solid #E9D5FF;
  font-weight: 500;
  border-radius: 6px;
  font-size: 12px;
  padding: 6px 12px;
  color: #7C3AED;
}

:deep(.btn-activity:hover) {
  border-color: #7C3AED;
  background: #7C3AED;
  color: white;
  transform: translateY(-1px);
}

:deep(.btn-more) {
  border: 1px solid #E9D5FF;
  border-radius: 6px;
  font-size: 12px;
  padding: 6px 8px;
  color: #6B7280;
}

:deep(.btn-more:hover) {
  border-color: #7C3AED;
  color: #7C3AED;
}

/* 分页区域 */
.pagination-section {
  display: flex;
  justify-content: center;
  margin-top: 32px;
}

:deep(.modern-pagination) {
  background: white;
  border: 1px solid #F3E8FF;
  border-radius: 8px;
  padding: 8px 16px;
}

/* 响应式设计 */
@media (max-width: 1200px) {
  .list-header,
  .task-list-item {
    grid-template-columns: 2fr 1fr 1fr 120px 180px;
  }
}

@media (max-width: 1024px) {
  .list-header,
  .task-list-item {
    grid-template-columns: 2fr 1fr 150px 120px 160px;
  }
  
  .task-tags-col {
    flex-direction: column;
    align-items: flex-start;
  }
}

@media (max-width: 768px) {
  .archived-container {
    padding: 20px;
  }
  
  .page-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 20px;
  }
  
  .page-title {
    font-size: 24px;
  }
  
  .header-stats {
    width: 100%;
    justify-content: space-between;
  }
  
  .search-controls {
    flex-direction: column;
    gap: 12px;
  }
  
  .search-input-wrapper {
    width: 100%;
    max-width: none;
  }
  
  .list-header {
    display: none;
  }
  
  .task-list-item {
    display: flex;
    flex-direction: column;
    gap: 12px;
    padding: 16px;
  }
  
  .action-buttons {
    justify-content: flex-start;
    width: 100%;
  }
  
  .task-title-col,
  .task-tags-col,
  .task-date-col,
  .task-progress-col,
  .task-actions-col {
    width: 100%;
  }
  
  .task-tags-col {
    order: 2;
  }
  
  .task-date-col {
    order: 3;
  }
  
  .task-progress-col {
    order: 4;
  }
  
  .task-actions-col {
    order: 5;
  }
}

@media (max-width: 480px) {
  .header-stats {
    flex-direction: column;
    gap: 12px;
  }
  
  .stat-card {
    width: 100%;
    justify-content: flex-start;
  }
  
  .action-buttons {
    flex-direction: column;
    gap: 8px;
  }
  
  :deep(.btn-detail),
  :deep(.btn-activity) {
    width: 100%;
  }
}
</style>