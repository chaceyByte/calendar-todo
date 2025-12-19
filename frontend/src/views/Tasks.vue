<template>
  <div class="tasks-container">
    <!-- 顶部控制栏 -->
    <div class="tasks-header">
      <div class="header-left">
        <h2>任务看板</h2>
      </div>
      <div class="header-right">
        <el-button type="primary" @click="showAddTaskDialog">
          <el-icon>
            <plus/>
          </el-icon>
          添加任务
        </el-button>
        <div class="staging-queue">
          <el-badge :value="stagingTasks.length" :max="99">
            <el-button @click="toggleStagingPanel">
              <el-icon>
                <clock/>
              </el-icon>
              暂存队列
            </el-button>
          </el-badge>
        </div>
      </div>
    </div>

    <!-- 暂存队列面板 -->
    <transition name="slide-right">
      <div v-if="showStaging" class="staging-panel">
        <div class="panel-header">
          <h3>暂存队列</h3>
          <el-button text @click="toggleStagingPanel">
            <el-icon>
              <close/>
            </el-icon>
          </el-button>
        </div>
        <div class="staging-list">
          <div
              v-for="task in stagingTasks"
              :key="task.id"
              class="staging-item"
              draggable="true"
              @dragstart="handleDragStart($event, task, 'staging')"
          >
            <div class="task-content">
              <span class="task-title">{{ task.title }}</span>
              <div class="task-actions">
                <el-button size="small" text @click="moveTaskToColumn(task, 'planning')">
                  添加到计划
                </el-button>
                <el-button size="small" text @click="removeFromStaging(task.id)">
                  删除
                </el-button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </transition>

    <!-- 任务看板 -->
    <div class="kanban-board" :class="{ 'with-staging': showStaging }">
      <div
          v-for="column in columns"
          :key="column.id"
          class="kanban-column"
          @dragover="handleDragOver"
          @drop="handleDrop($event, column.id)"
      >
        <div class="column-header">
          <h3>{{ column.title }}</h3>
          <span class="task-count">{{ getTasksByStatus(column.id).length }}</span>
        </div>

        <div class="column-content" :ref="el => setColumnRef(el, column.id)">
          <div
              v-for="task in getTasksByStatus(column.id)"
              :key="task.id"
              class="task-card"
              draggable="true"
              @dragstart="handleDragStart($event, task as any, column.id)"
              @contextmenu="(e) => showTaskContextMenu(e, task as any)"
          >
            <div class="task-header">
              <span class="task-title">{{ task.title }}</span>
              <el-dropdown trigger="click">
                <el-icon>
                  <more/>
                </el-icon>
                <template #dropdown>
                  <el-dropdown-menu>
                    <el-dropdown-item @click="editTask(task as any)">
                      <el-icon>
                        <edit/>
                      </el-icon>
                      编辑
                    </el-dropdown-item>
                    <el-dropdown-item @click="showActivityDrawer(task as any)">
                      <el-icon>
                        <view/>
                      </el-icon>
                      查看活动记录
                    </el-dropdown-item>
                    <el-dropdown-item @click="addToStaging(task as any)">添加到暂存</el-dropdown-item>
                    <el-dropdown-item v-if="task.status !== 'cancelled'" @click="() => handlePauseTask(task.id)">暂停
                    </el-dropdown-item>
                    <el-dropdown-item v-if="task.status === 'cancelled'" @click="() => handleResumeTask(task.id)">恢复
                    </el-dropdown-item>
                    <el-dropdown-item @click="addTagsToTask(task as any)">添加标签</el-dropdown-item>
                    <el-dropdown-item @click="showManualActivityDialog(task as any)">添加活动记录</el-dropdown-item>
                    <el-dropdown-item @click="deleteTask(task.id)" divided>删除</el-dropdown-item>
                  </el-dropdown-menu>
                </template>
              </el-dropdown>
            </div>

            <div class="task-body">
              <p class="task-description">{{ task.description }}</p>
              <div v-if="task.tags && task.tags.length > 0" class="task-tags">
                <el-tag
                    v-for="tag in task.tags"
                    :key="tag"
                    size="small"
                    closable
                    @close="removeTagFromTask(task.id, tag)"
                >
                  {{ tag }}
                </el-tag>
              </div>
            </div>

            <div class="task-footer">
              <span class="task-date">{{ formatDate(task.createdAt) }}</span>
              <el-progress
                  v-if="task.progress > 0"
                  :percentage="task.progress"
                  :show-text="false"
                  :stroke-width="2"
              />
            </div>

            <!-- 活动状态指示器 -->
            <div class="task-activity-indicator" @click="showActivityDrawer(task as any)">
              <div class="activity-status" :class="getActivityStatusClass(task as any)">
                <el-icon>
                  <timer/>
                </el-icon>
                <span class="activity-text">{{ getActivityStatusText(task as any) }}</span>
              </div>
              <div class="activity-time" v-if="getCurrentActivity(task.id)">
                {{ formatActivityTime(getCurrentActivity(task.id)) }}
              </div>
            </div>
          </div>

          <!-- 添加任务按钮 -->
          <div v-if="column.id !== 'completed'" class="add-task-btn">
            <el-button
                text
                @click="showAddTaskDialog(column.id)"
                class="full-width"
            >
              <el-icon>
                <plus/>
              </el-icon>
              添加任务
            </el-button>
          </div>

          <!-- 加载更多按钮（仅对已完成列） -->
          <div v-if="column.id === 'completed' && completedPagination.hasMore" class="load-more-btn">
            <el-button
                text
                @click="loadMoreCompletedTasks"
                class="full-width"
                :loading="completedPagination.isLoading"
            >
              <el-icon>
                <plus/>
              </el-icon>
              {{ completedPagination.isLoading ? '加载中...' : '加载更多' }}
            </el-button>
          </div>

          <!-- 已加载完成提示 -->
          <div
              v-if="column.id === 'completed' && !completedPagination.hasMore && loadedCompletedTasks && loadedCompletedTasks.length > 0"
              class="load-complete-hint">
            <el-tag size="small" type="info">已加载全部 {{ loadedCompletedTasks.length }} 个任务</el-tag>
          </div>
        </div>
      </div>
    </div>

    <!-- 添加快捷键提示 -->
    <div class="shortcut-hint">
      <el-tag size="small">快捷键: Alt + Enter 快速添加任务 | Ctrl+Z 撤销操作</el-tag>
    </div>

    <!-- 添加/编辑任务对话框 -->
    <el-dialog
        v-model="taskDialog.visible"
        :title="taskDialog.isEdit ? '编辑任务' : '添加任务'"
        width="500px"
    >
      <el-form :model="taskForm" label-width="80px">
        <el-form-item label="标题">
          <el-input v-model="taskForm.title" placeholder="请输入任务标题"/>
        </el-form-item>
        <el-form-item label="描述">
          <el-input
              v-model="taskForm.description"
              type="textarea"
              :rows="3"
              placeholder="请输入任务描述"
          />
        </el-form-item>
        <el-form-item label="状态">
          <el-select v-model="taskForm.status" placeholder="请选择状态">
            <el-option label="计划中" value="planning"/>
            <el-option label="制作中" value="in_progress"/>
            <el-option label="已完成" value="completed"/>
          </el-select>
        </el-form-item>
      </el-form>

      <template #footer>
        <el-button @click="taskDialog.visible = false">取消</el-button>
        <el-button type="primary" @click="saveTask">保存</el-button>
      </template>
    </el-dialog>

    <!-- 任务右键菜单 -->
    <div
        v-if="taskContextMenu.visible"
        class="task-context-menu"
        :style="{ left: taskContextMenu.x + 'px', top: taskContextMenu.y + 'px' }"
        @click="closeTaskContextMenu"
    >
      <div class="menu-item" @click="editTask(taskContextMenu.task)">
        <el-icon>
          <edit/>
        </el-icon>
        编辑
      </div>
      <div class="menu-item" @click="() => handlePauseTask(taskContextMenu.task.id)">
        <el-icon>
          <video-pause/>
        </el-icon>
        暂停
      </div>
      <div class="menu-item" @click="addTagsToTask(taskContextMenu.task)">
        <el-icon>
          <price-tag/>
        </el-icon>
        添加标签
      </div>
      <div class="menu-item" @click="deleteTask(taskContextMenu.task.id)" style="color: #f56c6c;">
        <el-icon>
          <delete/>
        </el-icon>
        删除
      </div>
    </div>

    <!-- 添加标签对话框 -->
    <el-dialog
        v-model="tagDialog.visible"
        title="添加标签"
        width="400px"
    >
      <el-form label-width="80px">
        <el-form-item label="选择标签">
          <el-select
              v-model="tagDialog.selectedTags"
              multiple
              filterable
              default-first-option
              placeholder="选择标签"
              style="width: 100%"
          >
            <el-option
                v-for="tag in availableTags"
                :key="tag.id"
                :label="tag.name"
                :value="tag.id"
            >
              <span style="float: left">{{ tag.name }}</span>
              <span style="float: right; color: #8492a6; font-size: 13px">{{ tag.taskCount || 0 }} 个任务</span>
            </el-option>
          </el-select>
        </el-form-item>
      </el-form>

      <template #footer>
        <el-button @click="tagDialog.visible = false">取消</el-button>
        <el-button type="primary" @click="saveTagsToTask">保存</el-button>
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
            <div class="activity-content">
              <div class="activity-title">
                {{ getActivityDescription(activity) }}
              </div>
              <div class="activity-description" v-if="activity.description">
                {{ activity.description }}
              </div>
              <div class="activity-duration" v-if="activity.durationMinutes">
                持续时间: {{ activityStore.formatDuration(activity.durationMinutes) }}
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
            <span>实际工作天数:</span>
            <span class="stat-value">{{ getWorkDaysCount() }} 天</span>
          </div>
        </el-card>
      </div>

      <div class="activity-actions">
        <el-button type="primary" @click="showManualActivityDialog({ id: activityDrawer.taskId } as Task)">
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
import {computed, onMounted, onUnmounted, reactive, ref} from 'vue'
import dayjs from 'dayjs'
import {ElMessage, ElMessageBox} from 'element-plus/es'
import {Clock, Close, Delete, Edit, More, Plus, Timer, VideoPause, PriceTag, View} from '@element-plus/icons-vue'

import {type Task, useTaskStore} from '@/stores/task'
import {useTagStore} from '@/stores/tag'
import {useActivityStore} from '@/stores/activity'

interface Column {
  id: string
  title: string
}

interface PaginationState {
  pageSize: number
  currentPage: number
  hasMore: boolean
  isLoading: boolean
}

const columns: Column[] = [
  {id: 'planning', title: '计划中'},
  {id: 'in_progress', title: '制作中'},
  {id: 'completed', title: '已完成'}
]

const taskStore = useTaskStore()
const tagStore = useTagStore()
const activityStore = useActivityStore()

// 使用store中的任务数据
const tasks = computed(() => taskStore.tasks)


const availableTags = ref([])

// 分页状态管理
const completedPagination = reactive<PaginationState>({
  pageSize: 5,
  currentPage: 1,
  hasMore: false,
  isLoading: false
})

// 已加载的已完成任务列表
const loadedCompletedTasks = ref<Task[]>([])

// 列引用管理
const columnRefs = ref<Record<string, HTMLElement>>({})

const stagingTasks = ref<Task[]>([])
const showStaging = ref(false)
const dragTask = ref<{ task: Task, source: string } | null>(null)

const taskDialog = reactive({
  visible: false,
  isEdit: false,
  targetColumn: 'planning' as string
})

const taskForm = reactive({
  id: 0,
  title: '',
  description: '',
  status: 'planning' as string,
  progress: 0,
  tags: [] as string[],
  completed: false
})

const taskContextMenu = reactive({
  visible: false,
  x: 0,
  y: 0,
  task: {} as Task
})

const tagDialog = reactive({
  visible: false,
  selectedTags: [] as number[],
  currentTaskId: 0
})

// 活动记录相关状态
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

const activities = ref([])

const getTasksByStatus = (status: string) => {
  if (status === 'completed') {
    // 对于已完成任务，返回已加载的任务列表
    return loadedCompletedTasks.value || []
  }
  // 对于其他状态，返回所有任务
  return tasks.value?.filter(task => task.status === status) || []
}

const toggleStagingPanel = () => {
  showStaging.value = !showStaging.value
}

const showAddTaskDialog = (columnId?: string) => {
  taskDialog.targetColumn = columnId || 'planning'
  taskDialog.isEdit = false
  taskDialog.visible = true

  // 重置表单，确保状态默认为"planning"
  Object.assign(taskForm, {
    id: 0,
    title: '',
    description: '',
    status: 'planning',
    progress: 0,
    tags: [],
    completed: false
  })
}

const editTask = (task: Task) => {
  taskDialog.isEdit = true
  taskDialog.visible = true
  Object.assign(taskForm, {...task})
}

const saveTask = async () => {
  if (!taskForm.title.trim()) {
    ElMessage.warning('请输入任务标题')
    return
  }

  try {
    if (taskDialog.isEdit) {
      // 编辑任务
      await taskStore.updateTask(taskForm.id, {
        title: taskForm.title,
        description: taskForm.description,
        status: taskForm.status as any,
        progress: taskForm.progress
      })
      ElMessage.success('任务更新成功')
    } else {
      // 添加新任务
      const newTask = {
        title: taskForm.title,
        description: taskForm.description,
        status: taskForm.status as any,
        progress: 0,
        tags: [],
        completed: false
      }
      await taskStore.addTask(newTask)
      ElMessage.success('任务添加成功')
    }

    // 刷新任务列表
    await loadTasks()
    taskDialog.visible = false
  } catch (error) {
    console.error('操作失败:', error)
    ElMessage.error('操作失败，请重试')
  }
}

// 拖拽功能
const handleDragStart = (e: DragEvent, task: Task, source: string) => {
  if (e.dataTransfer) {
    e.dataTransfer.effectAllowed = 'move'
    dragTask.value = {task, source}
  }
}

const handleDragOver = (e: DragEvent) => {
  e.preventDefault()
  if (e.dataTransfer) {
    e.dataTransfer.dropEffect = 'move'
  }
}

const handleDrop = async (e: DragEvent, targetStatus: string) => {
  e.preventDefault()

  if (!dragTask.value) return

  const {task, source} = dragTask.value

  try {
    if (source === 'staging') {
      // 从暂存队列移动到看板
      await moveTaskFromStaging(task, targetStatus)
    } else {
      // 在看板内移动
      await moveTaskToColumn(task, targetStatus)
    }
  } catch (error) {
    console.error('拖拽操作失败:', error)
    ElMessage.error('操作失败，请重试')
  }

  dragTask.value = null
}


const moveTaskFromStaging = async (task: Task, targetStatus: string) => {
  try {
    // 从暂存队列移除
    await removeFromStaging(task.id)

    // 更新任务状态到目标列
    await taskStore.updateTask(task.id, {
      status: targetStatus as any,
      progress: targetStatus === 'completed' ? 100 : task.progress
    })

    // 刷新任务列表
    await loadTasks()
    ElMessage.success('任务已移动到看板')
  } catch (error) {
    console.error('移动任务失败:', error)
    ElMessage.error('移动任务失败，请重试')
  }
}

const removeFromStaging = async (taskId: number) => {
  try {
    // 调用后端API
    await taskStore.removeFromStaging(taskId)

    // 刷新暂存队列显示
    await loadStagingTasks()
    ElMessage.success('任务已从暂存队列移除')
  } catch (error) {
    console.error('从暂存移除失败:', error)
    ElMessage.error('从暂存移除失败，请重试')
  }
}

// 添加任务到暂存队列
const addToStaging = async (task: Task) => {
  try {
    // 检查是否已经在暂存队列中
    if (!stagingTasks.value?.find(t => t.id === task.id)) {
      // 调用后端API
      await taskStore.addToStaging(task.id)

      // 刷新暂存队列显示
      await loadStagingTasks()
      ElMessage.success('任务已添加到暂存队列')
    } else {
      ElMessage.warning('任务已在暂存队列中')
    }
  } catch (error) {
    console.error('添加到暂存失败:', error)
    ElMessage.error('添加到暂存失败，请重试')
  }
}


const deleteTask = async (taskId: number) => {
  try {
    await ElMessageBox.confirm('确定要删除这个任务吗？', '确认删除', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })

    await taskStore.deleteTask(taskId)
    await loadTasks()
    ElMessage.success('任务删除成功')
  } catch (error) {
    if (error !== 'cancel') {
      console.error('删除失败:', error)
      ElMessage.error('删除失败，请重试')
    }
  }
}

const addTagsToTask = (task: Task) => {
  tagDialog.currentTaskId = task.id
  // 将标签名称转换为标签ID
  tagDialog.selectedTags = (task.tags || []).map(tagName => {
    const tag = availableTags.value.find(t => t.name === tagName)
    return tag ? tag.id : 0
  }).filter(id => id > 0)
  tagDialog.visible = true
}

const removeTagFromTask = async (taskId: number, tagName: string) => {
  try {
    const task = tasks.value?.find(t => t.id === taskId)
    if (task) {
      // 调用API移除标签
      await taskStore.removeTagFromTask(taskId, tagName)
      // 更新本地状态
      task.tags = task.tags.filter(t => t !== tagName)
      ElMessage.success('标签已移除')
    }
  } catch (error) {
    console.error('移除标签失败:', error)
    ElMessage.error('移除标签失败，请重试')
  }
}

// 右键菜单
const showTaskContextMenu = (e: MouseEvent, task: Task) => {
  e.preventDefault()
  taskContextMenu.visible = true
  taskContextMenu.x = e.clientX
  taskContextMenu.y = e.clientY
  taskContextMenu.task = task
}

const closeTaskContextMenu = () => {
  taskContextMenu.visible = false
}

// 全局键盘事件监听器
const handleGlobalKeyDown = async (e: KeyboardEvent) => {
  // 检测撤销快捷键 (Ctrl+Z 或 Cmd+Z)
  const isMac = navigator.platform.includes('Mac')
  const isUndoShortcut =
      ((isMac && e.metaKey) || (!isMac && e.ctrlKey)) &&
      e.key === 'z' &&
      !e.shiftKey &&
      !e.altKey

  if (isUndoShortcut) {
    e.preventDefault()
    e.stopPropagation()

    try {
      // 获取当前选中的任务或最近移动的任务
      const selectedTaskId = localStorage.getItem('lastMovedTaskId')

      if (!selectedTaskId) {
        ElMessage.info('没有可撤销的任务操作')
        return
      }

      console.log('开始撤销任务:', selectedTaskId)
      ElMessage.info('正在撤销操作...')

      // 使用 taskStore 中封装的撤销方法
      const success = await taskStore.undoTaskActions(parseInt(selectedTaskId), 5)

      if (success) {
        // 刷新任务列表和暂存队列
        await Promise.all([
          loadTasks(),
          loadStagingTasks()
        ])
        // 清除最后移动的任务ID
        localStorage.removeItem('lastMovedTaskId')
        console.log('撤销操作完成，界面已更新')
        ElMessage.success('撤销操作成功')
      }
    } catch (error: any) {
      console.error('撤销操作错误:', error)
      const errorMessage = error.message || '撤销操作失败'
      ElMessage.error(errorMessage)
    }
  }
  // Alt+Enter 快速添加任务
  else if (e.altKey && e.key === 'Enter') {
    e.preventDefault()
    showAddTaskDialog()
  }
}

const formatDate = (date: string) => {
  return dayjs(date).format('MM-DD')
}

// 加载任务列表
const loadTasks = async () => {
  try {
    await taskStore.fetchTasks()
    // 初始化已完成任务分页
    initCompletedTasksPagination()
  } catch (error) {
    console.error('加载任务失败:', error)
    ElMessage.error('加载任务失败')
  }
}

// 初始化已完成任务分页
const initCompletedTasksPagination = () => {
  const allCompletedTasks = tasks.value?.filter(task => task.status === 'completed') || []

  // 重置分页状态
  completedPagination.currentPage = 1
  completedPagination.hasMore = allCompletedTasks.length > completedPagination.pageSize

  // 加载第一页数据
  loadedCompletedTasks.value = allCompletedTasks.slice(0, completedPagination.pageSize)
}

// 加载更多已完成任务
const loadMoreCompletedTasks = () => {
  if (completedPagination.isLoading || !completedPagination.hasMore) {
    return
  }

  completedPagination.isLoading = true

  const allCompletedTasks = tasks.value?.filter(task => task.status === 'completed') || []
  const nextPage = completedPagination.currentPage + 1
  const startIndex = (nextPage - 1) * completedPagination.pageSize
  const endIndex = startIndex + completedPagination.pageSize

  const newTasks = allCompletedTasks.slice(startIndex, endIndex)

  // 模拟异步加载
  setTimeout(() => {
    loadedCompletedTasks.value = [...(loadedCompletedTasks.value || []), ...newTasks]
    completedPagination.currentPage = nextPage
    completedPagination.hasMore = endIndex < allCompletedTasks.length
    completedPagination.isLoading = false
  }, 300)
}

// 设置列引用
const setColumnRef = (el: Element | ComponentPublicInstance | null, columnId: string) => {
  if (el && 'tagName' in el) { // 确保是 HTMLElement
    columnRefs.value[columnId] = el as HTMLElement

    // 仅为已完成列添加滚动监听
    if (columnId === 'completed' && el) {
      setupScrollListener(el as HTMLElement)
    }
  }
}

// 设置滚动监听
const setupScrollListener = (element: HTMLElement) => {
  element.addEventListener('scroll', handleColumnScroll)
}

// 处理列滚动事件
const handleColumnScroll = (event: Event) => {
  const target = event.target as HTMLElement
  const {scrollTop, scrollHeight, clientHeight} = target

  // 当滚动到底部附近时自动加载更多
  if (scrollHeight - scrollTop - clientHeight < 100 && !completedPagination.isLoading && completedPagination.hasMore) {
    loadMoreCompletedTasks()
  }
}

// 加载可用标签
const loadAvailableTags = async () => {
  try {
    availableTags.value = await tagStore.fetchTags()
  } catch (error) {
    console.error('加载标签失败:', error)
    ElMessage.error('加载标签失败')
  }
}

// 加载暂存队列
const loadStagingTasks = async () => {
  try {
    const stagingTasksData = await taskStore.fetchStagingTasks()
    stagingTasks.value = stagingTasksData
  } catch (error) {
    console.error('加载暂存队列失败:', error)
    ElMessage.error('加载暂存队列失败')
  }
}

// 处理暂停任务
const handlePauseTask = async (taskId: number) => {
  try {
    await taskStore.pauseTask(taskId)
    // 刷新任务列表和暂存队列显示
    await loadTasks()
    await loadStagingTasks()

    // 记录最后操作的任务ID，用于撤销
    localStorage.setItem('lastMovedTaskId', taskId.toString())
    ElMessage.success('任务已暂停并添加到暂存队列')
  } catch (error) {
    console.error('暂停任务失败:', error)
    ElMessage.error('暂停任务失败，请重试')
  }
}

const moveTaskToColumn = async (task: Task, targetStatus: string) => {
  try {
    // 检查任务是否在暂存队列中
    // 确保 stagingTasks 是数组再使用 some 方法
    const isFromStaging = Array.isArray(stagingTasks.value) && stagingTasks.value?.some(t => t.id === task.id)

    if (isFromStaging) {
      // 从暂存队列移动到看板
      await moveTaskFromStaging(task, targetStatus)
      // 刷新暂存队列显示
      await loadStagingTasks()

      // 记录最后移动的任务ID，用于撤销
      localStorage.setItem('lastMovedTaskId', task.id.toString())
    } else {
      // 在看板内移动
      await taskStore.updateTask(task.id, {
        status: targetStatus as any,
        progress: targetStatus === 'completed' ? 100 : task.progress
      })

      // 如果任务状态发生变化，重新初始化分页
      if (task.status !== targetStatus) {
        await loadTasks()
      }

      // 记录最后移动的任务ID，用于撤销
      localStorage.setItem('lastMovedTaskId', task.id.toString())
    }
  } catch (error) {
    console.error('移动任务失败:', error)
    ElMessage.error('移动任务失败')
  }
}

// 保存标签到任务
const saveTagsToTask = async () => {
  try {
    await taskStore.updateTaskTags(tagDialog.currentTaskId, tagDialog.selectedTags)

    // 更新本地任务状态 - 将标签ID转换为标签名称
    const task = tasks.value?.find(t => t.id === tagDialog.currentTaskId)
    if (task) {
      task.tags = tagDialog.selectedTags.map(tagId => {
        const tag = availableTags.value.find(t => t.id === tagId)
        return tag ? tag.name : ''
      }).filter(name => name !== '')
    }

    tagDialog.visible = false
    ElMessage.success('标签已更新')
  } catch (error) {
    console.error('更新标签失败:', error)
    ElMessage.error('更新标签失败，请重试')
  }
}

// 活动记录相关方法
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

const handleResumeTask = async (taskId: number) => {
  try {
    await taskStore.resumeTask(taskId)
    // 刷新任务列表和暂存队列显示
    await loadTasks()
    await loadStagingTasks()

    // 记录最后操作的任务ID，用于撤销
    localStorage.setItem('lastMovedTaskId', taskId.toString())
    ElMessage.success('任务已恢复并从暂存队列移除')
  } catch (error) {
    console.error('恢复任务失败:', error)
    ElMessage.error('恢复任务失败，请重试')
  }
}

// 获取任务当前活动
const getCurrentActivity = (taskId: number) => {
  // 检查是否有当前活动，并且是否属于这个任务
  if (activityStore.currentActivity && activityStore.currentActivity.taskId === taskId) {
    return activityStore.currentActivity
  }
  return null
}

// 获取活动状态文本
const getActivityStatusText = (task: Task) => {
  const currentActivity = getCurrentActivity(task.id)
  if (currentActivity) {
    // 根据活动状态返回描述文本
    switch (currentActivity.status) {
      case 'running':
        return '进行中'
      case 'completed':
        return '已完成'
      case 'cancelled':
        return '已取消'
      default:
        return '活动记录'
    }
  }

  // 根据任务状态返回默认文本
  switch (task.status) {
    case 'planning':
      return '计划中'
    case 'in_progress':
      return '进行中'
    case 'completed':
      return '已完成'
    default:
      return '未知状态'
  }
}

// 获取活动状态样式类
const getActivityStatusClass = (task: Task) => {
  // 根据任务状态返回默认样式
  switch (task.status) {
    case 'planning':
      return 'activity-planning'
    case 'in_progress':
      return 'activity-in-progress'
    case 'completed':
      return 'activity-completed'
    case 'cancelled':
      return 'activity-cancelled'
    default:
      return 'activity-unknown'
  }
}

// 格式化活动时间
const formatActivityTime = (activity: any) => {
  if (!activity || !activity.startTime) return ''

  const startTime = new Date(activity.startTime)
  const now = new Date()
  const diffMs = now.getTime() - startTime.getTime()
  const diffMins = Math.floor(diffMs / (1000 * 60))

  if (diffMins < 60) {
    return `${diffMins}分钟`
  } else if (diffMins < 1440) { // 24小时
    const hours = Math.floor(diffMins / 60)
    const mins = diffMins % 60
    return `${hours}小时${mins > 0 ? mins + '分钟' : ''}`
  } else {
    return dayjs(startTime).format('MM-DD HH:mm')
  }
}

// 格式化日期时间
const formatDateTime = (dateTime: string) => {
  return dayjs(dateTime).format('YYYY-MM-DD HH:mm')
}

// 获取活动时间线类型
const getActivityTimelineType = (activity: any) => {
  switch (activity.status) {
    case 'running':
      return 'primary'
    case 'completed':
      return 'success'
    case 'cancelled':
      return 'warning'
    default:
      return 'info'
  }
}

// 获取活动描述
const getActivityDescription = (activity: any) => {
  // 检查notes字段，如果包含类型信息则提取
  if (activity.notes) {
    // 尝试解析notes字段中的活动类型，例如 "WORK: 活动描述"
    const match = activity.notes.match(/^(WORK|MEETING|STUDY|OTHER):\s*(.*)$/)
    if (match) {
      const [, type, description] = match
      return getActivityTypeDescription(type) + (description ? ` - ${description}` : '')
    }
  }

  // 如果无法从notes中解析，则根据状态返回描述
  switch (activity.status) {
    case 'running':
      return '进行中'
    case 'completed':
      return '已完成'
    case 'cancelled':
      return '已取消'
    default:
      return '活动记录'
  }
}

// 获取活动类型描述
const getActivityTypeDescription = (type: string) => {
  switch (type) {
    case 'CREATED':
      return '创建'
    case 'STARTED':
      return '开始'
    case 'PAUSED':
      return '暂停'
    case 'RESUMED':
      return '恢复'
    case 'COMPLETED':
      return '完成'
    case 'WORK':
      return '工作'
    case 'MEETING':
      return '会议'
    case 'STUDY':
      return '学习'
    case 'OTHER':
      return '其他'
    default:
      return type
  }
}

// 计算总活动时间
const getTotalActivityTime = () => {
  return activities.value
      .filter(a => a.durationMinutes)
      .reduce((total, activity) => total + (activity.durationMinutes || 0), 0)
}

// 计算工作天数
const getWorkDaysCount = () => {
  const uniqueDays = new Set()
  activities.value.forEach(activity => {
    if (activity.startTime) {
      uniqueDays.add(activity.startTime.split(' ')[0])
    }
  })
  return uniqueDays.size
}

onMounted(() => {
  document.addEventListener('keydown', handleGlobalKeyDown, {capture: true})
  document.addEventListener('click', closeTaskContextMenu)
  // 初始化时加载任务和暂存队列
  loadTasks()
  loadStagingTasks()
  loadAvailableTags()
})

onUnmounted(() => {
  document.removeEventListener('keydown', handleGlobalKeyDown, {capture: true})
  document.removeEventListener('click', closeTaskContextMenu)

  // 清理滚动监听器
  Object.values(columnRefs.value).forEach(element => {
    if (element) {
      element.removeEventListener('scroll', handleColumnScroll)
    }
  })
})
</script>

<style scoped>
.tasks-container {
  height: 100%;
  display: flex;
  flex-direction: column;
  position: relative;
}

.tasks-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 16px;
}

.staging-queue {
  position: relative;
}

.staging-panel {
  position: absolute;
  right: 0;
  top: 0;
  bottom: 0;
  width: 300px;
  background: white;
  border-left: 1px solid #e4e7ed;
  z-index: 10;
  display: flex;
  flex-direction: column;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px;
  border-bottom: 1px solid #e4e7ed;
}

.staging-list {
  flex: 1;
  overflow-y: auto;
  padding: 8px;
}

.staging-item {
  background: #f5f7fa;
  border-radius: 6px;
  padding: 12px;
  margin-bottom: 8px;
  cursor: move;
}

.task-content {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.task-actions {
  display: flex;
  gap: 4px;
}

.kanban-board {
  flex: 1;
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 24px;
  transition: all 0.3s;
}

.kanban-board.with-staging {
  margin-right: 300px;
}

.kanban-column {
  background: #f8f9fa;
  border-radius: 8px;
  padding: 16px;
  display: flex;
  flex-direction: column;
}

.column-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.column-header h3 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
}

.task-count {
  background: #409eff;
  color: white;
  border-radius: 10px;
  padding: 2px 8px;
  font-size: 12px;
}

.column-content {
  flex: 1;
  overflow-y: auto;
}

.task-card {
  background: white;
  border-radius: 6px;
  padding: 12px;
  margin-bottom: 12px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  cursor: move;
  transition: all 0.3s;
}

.task-card:hover {
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
  transform: translateY(-1px);
}

.task-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 8px;
}

.task-title {
  font-weight: 600;
  font-size: 14px;
  flex: 1;
  margin-right: 8px;
}

.task-description {
  font-size: 12px;
  color: #606266;
  margin-bottom: 8px;
  line-height: 1.4;
}

.task-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
  margin-bottom: 8px;
}

.task-footer {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.task-date {
  font-size: 11px;
  color: #909399;
}

.add-task-btn {
  margin-top: 8px;
}

.load-more-btn {
  margin-top: 8px;
}

.load-complete-hint {
  text-align: center;
  margin-top: 8px;
  padding: 4px;
}

.full-width {
  width: 100%;
}

.shortcut-hint {
  position: fixed;
  bottom: 20px;
  right: 20px;
}

.task-context-menu {
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

/* 活动状态指示器 */
.task-activity-indicator {
  margin-top: 8px;
  padding: 6px 8px;
  background: #f5f7fa;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.3s;
}

.task-activity-indicator:hover {
  background: #e6f7ff;
}

.activity-status {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 12px;
}

.activity-text {
  font-weight: 500;
}

.activity-time {
  font-size: 11px;
  color: #909399;
  margin-top: 2px;
}

/* 活动状态样式 */
.activity-planning {
  color: #909399;
}

.activity-in-progress {
  color: #409eff;
}

.activity-completed {
  color: #67c23a;
}

.activity-paused {
  color: #e6a23c;
}

.activity-unknown {
  color: #f56c6c;
}

.activity-created {
  color: #409eff;
}

.activity-started {
  color: #67c23a;
}

.activity-resumed {
  color: #409eff;
}

.activity-work {
  color: #409eff;
}

.activity-meeting {
  color: #e6a23c;
}

.activity-study {
  color: #67c23a;
}

.activity-other {
  color: #909399;
}

/* 活动记录抽屉样式 */
.activity-timeline {
  margin-bottom: 20px;
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

.activity-stats {
  margin-bottom: 20px;
}

.stat-item {
  display: flex;
  justify-content: space-between;
  margin-bottom: 8px;
}

.stat-value {
  font-weight: 600;
}

.activity-actions {
  text-align: center;
}

/* 动画 */
.slide-right-enter-active,
.slide-right-leave-active {
  transition: transform 0.3s ease;
}

.slide-right-enter-from,
.slide-right-leave-to {
  transform: translateX(100%);
}
</style>