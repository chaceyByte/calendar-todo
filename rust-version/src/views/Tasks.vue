<template>
  <div class="tasks-page">
    <!-- Header -->
    <header class="page-header">
      <div class="header-content">
        <h1 class="title">任务看板</h1>
        <p class="subtitle">{{ currentDate }} • 第 {{ currentWeek }} 周</p>
      </div>
      <div class="header-actions">
        <div class="search-box">
          <svg class="search-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="11" cy="11" r="8" />
            <path d="m21 21-4.35-4.35" />
          </svg>
          <input type="text" placeholder="搜索任务..." class="search-input" v-model="searchQuery" @input="handleSearch" />
        </div>
        <div class="action-buttons">
          <button class="action-btn" @click="showFilter = true">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <polygon points="22 3 2 3 10 12.46 10 19 14 21 14 12.46 22 3" />
            </svg>
          </button>
        </div>
      </div>
    </header>

    <!-- Kanban Board -->
    <div class="kanban-board">
      <!-- Planning Column -->
      <div class="kanban-column" :class="{ 'drag-over': dragOverStatus === 'planning' }" data-status="planning"
        @dragover="handleDragOver" @dragenter="handleDragEnter($event, 'planning')"
        @dragleave="handleDragLeave($event, 'planning')" @drop="handleDrop($event, 'planning')">
        <div class="column-header">
          <div class="header-left">
            <div class="status-dot planning"></div>
            <h3 class="column-title">规划中</h3>
            <span class="task-count">{{ planningTasks.length }}</span>
          </div>
          <button class="more-btn">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <circle cx="12" cy="12" r="1" />
              <circle cx="19" cy="12" r="1" />
              <circle cx="5" cy="12" r="1" />
            </svg>
          </button>
        </div>
        <div class="task-list">
          <div v-for="task in planningTasks" :key="task.id" class="task-card"
            :class="{ 'dragging': dragTask?.id === task.id }" :data-id="task.id" draggable="true"
            @dragstart="handleDragStart($event, task)" @dragend="handleDragEnd"
            @click="openEditDialog(task)">
            <div class="card-tags">
              <span v-for="tag in task.tags" :key="tag.id" class="tag"
                :style="{ background: tag.color + '20', color: tag.color }">{{ tag.name }}</span>
              <span v-if="task.isImportant" class="tag important">重要</span>
              <span v-if="task.isUrgent" class="tag urgent">紧急</span>
            </div>
            <h4 class="card-title">{{ task.title }}</h4>
            <p v-if="task.description" class="card-desc">{{ task.description }}</p>
            <div class="card-footer">
              <div class="assignee"></div>
              <div class="task-meta">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" />
                </svg>
                <span>{{ task.progress || 0 }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- In Progress Column -->
      <div class="kanban-column" :class="{ 'drag-over': dragOverStatus === 'in_progress' }" data-status="in_progress"
        @dragover="handleDragOver" @dragenter="handleDragEnter($event, 'in_progress')"
        @dragleave="handleDragLeave($event, 'in_progress')" @drop="handleDrop($event, 'in_progress')">
        <div class="column-header">
          <div class="header-left">
            <div class="status-dot in-progress"></div>
            <h3 class="column-title">进行中</h3>
            <span class="task-count">{{ inProgressTasks.length }}</span>
          </div>
          <button class="more-btn">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <circle cx="12" cy="12" r="1" />
              <circle cx="19" cy="12" r="1" />
              <circle cx="5" cy="12" r="1" />
            </svg>
          </button>
        </div>
        <div class="task-list">
          <div v-for="task in inProgressTasks" :key="task.id" class="task-card"
            :class="{ 'dragging': dragTask?.id === task.id }" :data-id="task.id" draggable="true"
            @dragstart="handleDragStart($event, task)" @dragend="handleDragEnd"
            @click="openEditDialog(task)">
            <div class="card-tags">
              <span v-for="tag in task.tags" :key="tag.id" class="tag"
                :style="{ background: tag.color + '20', color: tag.color }">{{ tag.name }}</span>
              <span v-if="task.isImportant" class="tag important">重要</span>
              <span v-if="task.isUrgent" class="tag urgent">紧急</span>
            </div>
            <h4 class="card-title">{{ task.title }}</h4>
            <div v-if="task.progress > 0" class="progress-bar">
              <div class="progress-fill" :style="{ width: task.progress + '%' }"></div>
            </div>
            <div class="card-footer">
              <div class="assignees">
                <div class="assignee"></div>
              </div>
              <span v-if="task.progress > 0" class="progress-text">{{ task.progress }}%</span>
            </div>
          </div>
        </div>
      </div>

      <!-- Archived Column -->
      <div class="kanban-column archived" :class="{ 'drag-over': dragOverStatus === 'archived' }" data-status="archived"
        @dragover="handleDragOver" @dragenter="handleDragEnter($event, 'archived')"
        @dragleave="handleDragLeave($event, 'archived')" @drop="handleDrop($event, 'archived')">
        <div class="column-header">
          <div class="header-left">
            <div class="status-dot archived"></div>
            <h3 class="column-title">已归档</h3>
            <span class="task-count">{{ archivedTasks.length }}</span>
          </div>
        </div>
        <div class="archived-list">
          <div v-for="task in archivedTasks" :key="task.id" class="archived-item"
            :class="{ 'dragging': dragTask?.id === task.id }" :data-id="task.id" draggable="true"
            @dragstart="handleDragStart($event, task)" @dragend="handleDragEnd">
            <div class="item-content">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <polyline points="20 6 9 17 4 12" />
              </svg>
              <span>{{ task.title }}</span>
            </div>
            <span class="item-date">{{ formatDate(task.archivedAt) }}</span>
          </div>
        </div>
        <button v-if="archivedTasks.length > 10" class="load-more-btn" @click="loadMoreArchived">
          加载更多历史记录
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import dayjs from 'dayjs'
import weekOfYear from 'dayjs/plugin/weekOfYear'

dayjs.extend(weekOfYear);

interface Tag {
  id: number
  name: string
  color: string
}

interface Task {
  id: number
  title: string
  description?: string
  quadrant: string
  status: string
  progress: number
  isImportant: boolean
  isUrgent: boolean
  startAt?: string
  dueAt?: string
  userId: number
  createdAt: string
  updatedAt: string
  archived: boolean
  archivedAt?: string
  tags: Tag[]
}

const tasks = ref<Task[]>([])
const searchQuery = ref('')
const showFilter = ref(false)

const filteredTasks = computed(() => {
  if (!searchQuery.value.trim()) {
    return tasks.value
  }
  const query = searchQuery.value.toLowerCase()
  return tasks.value.filter(task =>
    task.title.toLowerCase().includes(query) ||
    (task.description && task.description.toLowerCase().includes(query)) ||
    task.tags.some(tag => tag.name.toLowerCase().includes(query))
  )
})

const planningTasks = computed(() => filteredTasks.value.filter(t => t.status === 'planning'))
const inProgressTasks = computed(() => filteredTasks.value.filter(t => t.status === 'in_progress'))
const archivedTasks = computed(() => filteredTasks.value.filter(t =>
  t.status === 'archived' ||
  t.status === 'paused' ||
  t.archived
))

const currentDate = computed(() => dayjs().format('MMMM D, YYYY'))
const currentWeek = computed(() => dayjs().week())

const loadTasks = async () => {
  try {
    const result = await invoke<Task[]>('get_all_tasks', { includeArchived: true })
    tasks.value = result
  } catch (error) {
    console.error('加载任务失败:', error)
  }
}

const handleSearch = () => {
  // 搜索过滤是响应式的，通过 filteredTasks computed 属性实现
}

// 使用全局对话框打开编辑任务
const openEditDialog = (task: Task) => {
  // 调用全局对话框的编辑方法
  if ((window as any).openEditTaskDialog) {
    (window as any).openEditTaskDialog(task)
  }
}

const dragTask = ref<Task | null>(null)
const dragOverStatus = ref<string | null>(null)
const dragEnterCounter = ref<Record<string, number>>({
  planning: 0,
  in_progress: 0,
  archived: 0
})

const handleDragStart = (e: DragEvent, task: Task) => {
  dragTask.value = task

  if (e.dataTransfer) {
    e.dataTransfer.effectAllowed = 'move'
    e.dataTransfer.setData('text/plain', task.id.toString())
  }

  setTimeout(() => {
    const el = document.querySelector(`[data-id="${task.id}"]`)
    if (el) {
      el.classList.add('dragging')
    }
  }, 0)
}

const handleDragOver = (e: DragEvent) => {
  e.preventDefault()
  if (e.dataTransfer) {
    e.dataTransfer.dropEffect = 'move'
  }
}

const handleDragEnter = (e: DragEvent, targetStatus: string) => {
  e.preventDefault()
  dragEnterCounter.value[targetStatus]++
  if (dragEnterCounter.value[targetStatus] === 1) {
    dragOverStatus.value = targetStatus
  }
}

const handleDragLeave = (e: DragEvent, targetStatus: string) => {
  e.preventDefault()
  dragEnterCounter.value[targetStatus] = Math.max(0, dragEnterCounter.value[targetStatus] - 1)
  if (dragEnterCounter.value[targetStatus] === 0) {
    dragOverStatus.value = null
  }
}

const handleDragEnd = () => {
  dragTask.value = null
  dragOverStatus.value = null
  dragEnterCounter.value = { planning: 0, in_progress: 0, archived: 0 }
  const draggingElements = document.querySelectorAll('.dragging')
  draggingElements.forEach(el => {
    el.classList.remove('dragging')
  })
}

const handleDrop = async (e: DragEvent, targetStatus: string) => {
  e.preventDefault()
  e.stopPropagation()

  const task = dragTask.value
  if (!task) return

  const taskId = task.id
  const oldStatus = task.status

  if (oldStatus === targetStatus) {
    dragOverStatus.value = null
    dragEnterCounter.value = { planning: 0, in_progress: 0, archived: 0 }
    dragTask.value = null
    return
  }

  const isFromArchived = task.archived || oldStatus === 'archived' || oldStatus === 'paused'
  const isArchiving = targetStatus === 'archived'
  const isRework = isFromArchived && targetStatus === 'planning'

  try {
    const taskIndex = tasks.value.findIndex(t => t.id === taskId)
    if (taskIndex === -1) return

    const updatedTask = {
      ...task,
      status: targetStatus,
      archived: isArchiving ? true : (targetStatus === 'planning' ? false : task.archived),
      progress: isRework ? 0 : task.progress,
      archivedAt: isArchiving ? new Date().toISOString() : (targetStatus === 'planning' ? undefined : task.archivedAt)
    }
    tasks.value.splice(taskIndex, 1, updatedTask)

    const requestPayload: any = {
      id: taskId,
      status: targetStatus,
      archived: isArchiving ? true : (isFromArchived && targetStatus !== 'archived' ? false : undefined)
    }

    if (isRework) {
      requestPayload.progress = 0
    }

    await invoke('update_task', { request: requestPayload })
  } catch (error) {
    console.error('更新任务状态失败:', error)
    await loadTasks()
  } finally {
    dragOverStatus.value = null
    dragEnterCounter.value = { planning: 0, in_progress: 0, archived: 0 }
    dragTask.value = null
  }
}

const loadMoreArchived = () => {
  console.log('加载更多归档任务')
}

const formatDate = (dateStr?: string) => {
  if (!dateStr) return ''
  return dayjs(dateStr).format('MM/DD')
}

// 监听任务保存事件，刷新任务列表
const handleTaskSaved = () => {
  loadTasks()
}

onMounted(() => {
  loadTasks()
  window.addEventListener('task-saved', handleTaskSaved)
})

onUnmounted(() => {
  window.removeEventListener('task-saved', handleTaskSaved)
})
</script>

<style scoped>
.tasks-page {
  padding: 32px 40px;
  min-height: 100vh;
  background: var(--bg-page);
  display: flex;
  flex-direction: column;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 32px;
}

.header-content .title {
  font-size: 28px;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0 0 8px 0;
}

.header-content .subtitle {
  font-size: 14px;
  color: var(--text-secondary);
  margin: 0;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 16px;
}

.search-box {
  position: relative;
  display: flex;
  align-items: center;
}

.search-icon {
  position: absolute;
  left: 12px;
  width: 18px;
  height: 18px;
  color: var(--text-secondary);
}

.search-input {
  width: 280px;
  padding: 10px 16px 10px 40px;
  background: var(--bg-card);
  border: 1px solid var(--bg-input);
  border-radius: 10px;
  font-size: 14px;
  color: var(--text-primary);
  transition: all 0.2s ease;
}

.search-input::placeholder {
  color: var(--text-secondary);
}

.search-input:focus {
  outline: none;
  border-color: var(--color-primary);
  box-shadow: 0 0 0 3px rgba(0, 102, 204, 0.1);
}

.action-buttons {
  display: flex;
  align-items: center;
  gap: 12px;
}

.action-btn {
  width: 40px;
  height: 40px;
  background: var(--bg-card);
  border: 1px solid var(--bg-input);
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.2s ease;
}

.action-btn:hover {
  background: var(--bg-page);
  border-color: #d0d7de;
}

.action-btn svg {
  width: 18px;
  height: 18px;
  color: var(--text-secondary);
}

.kanban-board {
  flex: 1;
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 24px;
  position: relative;
}

.kanban-column {
  background: var(--bg-card);
  border-radius: 16px;
  padding: 20px;
  box-shadow: 0 1px 3px var(--bg-hover);
  display: flex;
  flex-direction: column;
  max-height: calc(100vh - 180px);
  transition: all 0.2s ease;
}

.kanban-column.drag-over {
  background: var(--bg-hover);
  box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.3), 0 4px 12px var(--border-color);
}

.column-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 12px;
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
}

.status-dot.planning {
  background: #8b5cf6;
}

.status-dot.in-progress {
  background: var(--color-primary);
}

.status-dot.archived {
  background: var(--text-secondary);
}

.column-title {
  font-size: 15px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
}

.task-count {
  padding: 2px 8px;
  background: var(--bg-input);
  border-radius: 10px;
  font-size: 12px;
  font-weight: 600;
  color: var(--text-secondary);
}

.more-btn {
  width: 32px;
  height: 32px;
  background: transparent;
  border: none;
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.2s ease;
}

.more-btn:hover {
  background: var(--bg-input);
}

.more-btn svg {
  width: 18px;
  height: 18px;
  color: var(--text-secondary);
}

.task-list {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 16px;
  overflow-y: auto;
  min-height: 100px;
  padding: 4px;
}

.task-list:empty::before {
  content: '';
  display: block;
  min-height: 60px;
  border: 2px dashed var(--bg-input);
  border-radius: 8px;
}

.task-card {
  background: var(--bg-page);
  border-radius: 12px;
  padding: 16px;
  transition: all 0.2s ease;
  cursor: pointer;
}

.task-card:hover {
  background: var(--bg-input);
  transform: translateY(-2px);
  box-shadow: 0 4px 12px var(--border-color);
}

.task-card.dragging {
  opacity: 0.5;
  transform: scale(0.95) rotate(2deg);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.2);
  cursor: grabbing;
}

.card-tags {
  display: flex;
  gap: 8px;
  margin-bottom: 12px;
  flex-wrap: wrap;
}

.tag {
  padding: 4px 10px;
  border-radius: 6px;
  font-size: 11px;
  font-weight: 600;
}

.tag.important {
  background: var(--color-warning-bg);
  color: var(--color-warning);
}

.tag.urgent {
  background: var(--color-error-bg);
  color: var(--color-error);
}

.card-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 8px 0;
}

.card-desc {
  font-size: 12px;
  color: var(--text-secondary);
  line-height: 1.6;
  margin: 0 0 12px 0;
}

.card-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: 12px;
}

.assignee {
  width: 28px;
  height: 28px;
  border-radius: 50%;
  background: linear-gradient(135deg, var(--bg-input), #d0d7de);
  border: 2px solid var(--bg-card);
}

.assignees {
  display: flex;
  align-items: center;
}

.assignees .assignee:last-child {
  margin-left: -8px;
}

.task-meta {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 12px;
  color: var(--text-secondary);
}

.task-meta svg {
  width: 14px;
  height: 14px;
}

.progress-bar {
  height: 4px;
  background: var(--bg-input);
  border-radius: 2px;
  overflow: hidden;
  margin-top: 12px;
}

.progress-fill {
  height: 100%;
  background: linear-gradient(90deg, var(--color-primary), #8b5cf6);
  border-radius: 2px;
  transition: width 0.3s ease;
}

.progress-text {
  font-size: 12px;
  font-weight: 600;
  color: var(--color-primary);
}

.kanban-column.archived {
  background: var(--bg-card);
}

.archived-list {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 8px;
  overflow-y: auto;
  min-height: 100px;
}

.archived-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px;
  background: var(--bg-card);
  border-radius: 8px;
  transition: all 0.2s ease;
  cursor: default;
}

.archived-item:hover {
  background: var(--bg-card);
}

.archived-item.dragging {
  opacity: 0.5;
  transform: scale(0.95);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.2);
  cursor: grabbing;
}

.item-content {
  display: flex;
  align-items: center;
  gap: 10px;
  flex: 1;
}

.item-content svg {
  width: 16px;
  height: 16px;
  color: var(--color-success);
  flex-shrink: 0;
}

.item-content span {
  font-size: 13px;
  color: var(--text-secondary);
}

.item-date {
  font-size: 12px;
  color: var(--text-secondary);
}

.load-more-btn {
  margin-top: 12px;
  padding: 12px;
  background: transparent;
  border: 1px dashed #d0d7de;
  border-radius: 8px;
  font-size: 13px;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.2s ease;
}

.load-more-btn:hover {
  border-color: var(--color-primary);
  color: var(--color-primary);
  background: var(--bg-hover);
}
</style>
