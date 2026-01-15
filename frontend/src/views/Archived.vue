<template>
  <div class="archived-container">
    <!-- 页面头部 -->
    <div class="page-header">
      <h2>归档任务</h2>
      <p class="page-description">查看和管理已完成的任务</p>
    </div>

    <!-- 搜索和筛选区域 -->
    <div class="search-section">
      <el-row :gutter="20">
        <el-col :span="8">
          <el-input
            v-model="searchKeyword"
            placeholder="搜索任务标题或描述"
            clearable
            @clear="handleSearch"
            @keyup.enter="handleSearch"
          >
            <template #prefix>
              <el-icon><search /></el-icon>
            </template>
          </el-input>
        </el-col>
        <el-col :span="4">
          <el-button type="primary" @click="handleSearch">
            <el-icon><search /></el-icon>
            搜索
          </el-button>
        </el-col>
        <el-col :span="12" style="text-align: right;">
          <el-button @click="refreshData">
            <el-icon><refresh /></el-icon>
            刷新
          </el-button>
        </el-col>
      </el-row>
    </div>

    <!-- 任务列表 -->
    <div class="task-list">
      <el-card v-if="tasks.length === 0" class="empty-state">
        <div class="empty-content">
          <el-icon size="48" color="#909399">
            <document-remove />
          </el-icon>
          <p>暂无归档任务</p>
        </div>
      </el-card>

      <div v-else class="task-grid">
        <el-card
          v-for="task in tasks"
          :key="task.id"
          class="task-card"
          shadow="hover"
        >
          <div class="task-header">
            <h3 class="task-title">{{ task.title }}</h3>
            <el-dropdown trigger="click">
              <el-icon><more /></el-icon>
              <template #dropdown>
                <el-dropdown-menu>
                  <el-dropdown-item @click="viewTaskDetails(task)">
                    <el-icon><view /></el-icon>
                    查看详情
                  </el-dropdown-item>
                  <el-dropdown-item @click="showActivityDrawer(task)">
                    <el-icon><timer /></el-icon>
                    活动记录
                  </el-dropdown-item>
                  <el-dropdown-item @click="deleteTask(task.id)" divided>
                    <el-icon><delete /></el-icon>
                    删除
                  </el-dropdown-item>
                </el-dropdown-menu>
              </template>
            </el-dropdown>
          </div>

          <div class="task-body">
            <p class="task-description">{{ task.description || '暂无描述' }}</p>
            
            <div v-if="task.tags && task.tags.length > 0" class="task-tags">
              <el-tag
                v-for="tag in task.tags"
                :key="tag"
                size="small"
                type="info"
              >
                {{ tag }}
              </el-tag>
            </div>
          </div>

          <div class="task-footer">
            <div class="task-meta">
              <span class="task-date">{{ formatDate(task.createdAt) }}</span>
              <el-progress
                v-if="task.progress > 0"
                :percentage="task.progress"
                :show-text="false"
                :stroke-width="4"
                status="success"
              />
            </div>
            <el-tag type="success" size="small">已完成</el-tag>
          </div>
        </el-card>
      </div>
    </div>

    <!-- 分页组件 -->
    <div v-if="total > 0" class="pagination-section">
      <el-pagination
        v-model:current-page="currentPage"
        v-model:page-size="pageSize"
        :page-sizes="[10, 20, 50, 100]"
        :total="total"
        layout="total, sizes, prev, pager, next, jumper"
        @size-change="handleSizeChange"
        @current-change="handleCurrentChange"
      />
    </div>

    <!-- 任务详情对话框 -->
    <el-dialog
      v-model="detailDialog.visible"
      :title="`任务详情 - ${detailDialog.task?.title}`"
      width="600px"
    >
      <div v-if="detailDialog.task" class="task-detail">
        <el-descriptions :column="1" border>
          <el-descriptions-item label="任务标题">
            {{ detailDialog.task.title }}
          </el-descriptions-item>
          <el-descriptions-item label="任务描述">
            {{ detailDialog.task.description || '暂无描述' }}
          </el-descriptions-item>
          <el-descriptions-item label="完成进度">
            <el-progress :percentage="detailDialog.task.progress || 100" />
          </el-descriptions-item>
          <el-descriptions-item label="创建时间">
            {{ formatDateTime(detailDialog.task.createdAt) }}
          </el-descriptions-item>
          <el-descriptions-item label="更新时间">
            {{ formatDateTime(detailDialog.task.updatedAt) }}
          </el-descriptions-item>
          <el-descriptions-item v-if="detailDialog.task.tags && detailDialog.task.tags.length > 0" label="标签">
            <el-tag
              v-for="tag in detailDialog.task.tags"
              :key="tag"
              size="small"
              type="info"
              style="margin-right: 8px;"
            >
              {{ tag }}
            </el-tag>
          </el-descriptions-item>
        </el-descriptions>
      </div>
      <template #footer>
        <el-button @click="detailDialog.visible = false">关闭</el-button>
      </template>
    </el-dialog>

    <!-- 编辑任务对话框 -->
    <el-dialog
      v-model="editDialog.visible"
      title="编辑任务"
      width="500px"
    >
      <el-form :model="editDialog.form" label-width="80px">
        <el-form-item label="标题">
          <el-input v-model="editDialog.form.title" placeholder="请输入任务标题" />
        </el-form-item>
        <el-form-item label="描述">
          <el-input
            v-model="editDialog.form.description"
            type="textarea"
            :rows="3"
            placeholder="请输入任务描述"
          />
        </el-form-item>
        <el-form-item label="进度">
          <el-slider v-model="editDialog.form.progress" :min="0" :max="100" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="editDialog.visible = false">取消</el-button>
        <el-button type="primary" @click="saveTaskEdit">保存</el-button>
      </template>
    </el-dialog>

    <!-- 活动记录抽屉 -->
    <el-drawer
      v-model="activityDrawer.visible"
      :title="`${activityDrawer.taskTitle} - 活动记录`"
      direction="rtl"
      size="400px"
    >
      <div class="activity-timeline">
        <el-timeline>
          <el-timeline-item
            v-for="activity in activities"
            :key="activity.id"
            :timestamp="formatDateTime(activity.startTime)"
            :type="getActivityTimelineType(activity)"
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
        <el-card>
          <template #header>活动统计</template>
          <div class="stat-item">
            <span>总活动时间:</span>
            <span class="stat-value">
              {{ activityStore.formatDuration(getTotalActivityTime()) }}
            </span>
          </div>
          <div class="stat-item">
            <span>活动记录数量:</span>
            <span class="stat-value">{{ activities.length }} 条</span>
          </div>
          <div class="stat-item">
            <span>实际工作天数:</span>
            <span class="stat-value">{{ getWorkDaysCount() }} 天</span>
          </div>
        </el-card>
      </div>

      <div class="activity-actions">
        <el-button type="primary" @click="showManualActivityDialog({ id: activityDrawer.taskId } as Task)">
          <el-icon><plus /></el-icon>
          添加活动记录
        </el-button>
      </div>
    </el-drawer>

    <!-- 手动添加活动记录对话框 -->
    <el-dialog
      v-model="manualActivityDialog.visible"
      title="添加活动记录"
      width="500px"
    >
      <el-form :model="manualActivityDialog.form" label-width="100px">
        <el-form-item label="活动类型">
          <el-select v-model="manualActivityDialog.form.activityType" style="width: 100%">
            <el-option label="工作" value="WORK"/>
            <el-option label="会议" value="MEETING"/>
            <el-option label="学习" value="STUDY"/>
            <el-option label="其他" value="OTHER"/>
          </el-select>
        </el-form-item>
        <el-form-item label="开始时间">
          <el-date-picker
            v-model="manualActivityDialog.form.startTime"
            type="datetime"
            placeholder="选择开始时间"
            style="width: 100%"
          />
        </el-form-item>
        <el-form-item label="结束时间">
          <el-date-picker
            v-model="manualActivityDialog.form.endTime"
            type="datetime"
            placeholder="选择结束时间"
            style="width: 100%"
          />
        </el-form-item>
        <el-form-item label="描述">
          <el-input
            v-model="manualActivityDialog.form.description"
            type="textarea"
            :rows="3"
            placeholder="描述活动内容"
          />
        </el-form-item>
      </el-form>

      <template #footer>
        <el-button @click="manualActivityDialog.visible = false">取消</el-button>
        <el-button type="primary" @click="saveManualActivity">保存</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus/es'
import {
  Search,
  Refresh,
  DocumentRemove,
  More,
  Timer,
  Delete,
  Plus
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
const pageSize = ref(20)
const searchKeyword = ref('')
const activities = ref<ActivityRecord[]>([])

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

// 获取活动类型描述
const getActivityTypeDescription = (activityType: string) => {
  // 从字符串中提取活动类型
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
      // 后端返回的数据结构：response.data 包含 records 和 total 字段
      tasks.value = data.records || []
      total.value = data.total || 0
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
  // 基于数据库的 activity_type 和 description 字段
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
  // 优先使用数据库中的 description 字段
  if (activity.description) {
    return activity.description
  }
  
  // 其次使用 notes 字段
  if (activity.notes) {
    return activity.notes
  }
  
  // 根据活动类型返回默认描述
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

    // 刷新活动记录
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
  padding: 24px;
  max-width: 1200px;
  margin: 0 auto;
}

.page-header {
  margin-bottom: 32px;
}

.page-header h2 {
  margin: 0 0 8px 0;
  font-size: 24px;
  font-weight: 600;
  color: #303133;
}

.page-description {
  margin: 0;
  color: #606266;
  font-size: 14px;
}

.search-section {
  margin-bottom: 24px;
  padding: 20px;
  background: #f8f9fa;
  border-radius: 8px;
}

.task-list {
  min-height: 400px;
}

.empty-state {
  text-align: center;
  padding: 60px 20px;
}

.empty-content {
  color: #909399;
}

.task-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
  gap: 20px;
  margin-bottom: 24px;
}

.task-card {
  transition: all 0.3s ease;
}

.task-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.task-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 12px;
}

.task-title {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: #303133;
  line-height: 1.4;
  flex: 1;
  margin-right: 12px;
}

.task-body {
  margin-bottom: 16px;
}

.task-description {
  margin: 0 0 12px 0;
  color: #606266;
  font-size: 14px;
  line-height: 1.5;
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.task-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}

.task-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.task-meta {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.task-date {
  font-size: 12px;
  color: #909399;
}

.pagination-section {
  display: flex;
  justify-content: center;
  margin-top: 32px;
}

.task-detail {
  padding: 0 20px;
}

.activity-timeline {
  padding: 0 20px;
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

@media (max-width: 768px) {
  .archived-container {
    padding: 16px;
  }
  
  .task-grid {
    grid-template-columns: 1fr;
    gap: 16px;
  }
  
  .search-section {
    padding: 16px;
  }
}

/* 活动记录抽屉样式 - 基于数据库字段优化 */
.activity-timeline {
  margin-bottom: 20px;
}

.activity-content {
  padding: 8px 0;
  border-left: 3px solid transparent;
  padding-left: 12px;
}

.activity-title {
  font-weight: 600;
  margin-bottom: 8px;
  font-size: 14px;
  color: #303133;
}

.activity-details {
  background: #f8f9fa;
  border-radius: 6px;
  padding: 12px;
  margin-top: 8px;
}

.activity-time-range,
.activity-duration,
.activity-type {
  display: flex;
  align-items: center;
  margin-bottom: 6px;
  font-size: 12px;
}

.activity-time-range:last-child,
.activity-duration:last-child,
.activity-type:last-child {
  margin-bottom: 0;
}

.time-label,
.duration-label,
.type-label {
  color: #909399;
  min-width: 70px;
  margin-right: 8px;
}

.time-value,
.duration-value,
.type-value {
  color: #606266;
  font-weight: 500;
}

/* 根据活动类型设置不同的边框颜色 */
.activity-content[data-type="created"] {
  border-left-color: #409eff;
}

.activity-content[data-type="started"] {
  border-left-color: #67c23a;
}

.activity-content[data-type="completed"] {
  border-left-color: #e6a23c;
}

.activity-content[data-type="work"] {
  border-left-color: #409eff;
}

.activity-content[data-type="meeting"] {
  border-left-color: #67c23a;
}

.activity-content[data-type="study"] {
  border-left-color: #909399;
}

.activity-stats {
  margin-bottom: 20px;
}

.stat-item {
  display: flex;
  justify-content: space-between;
  margin-bottom: 8px;
  font-size: 14px;
}

.stat-value {
  font-weight: 600;
  color: #409eff;
}

.activity-actions {
  text-align: center;
  padding-top: 16px;
  border-top: 1px solid #ebeef5;
}
</style>