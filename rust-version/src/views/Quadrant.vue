<template>
  <div class="quadrant-page">
    <!-- Toast 提示 -->
    <div v-if="toastMessage" class="toast" :class="toastType">
      {{ toastMessage }}
    </div>

    <!-- Header -->
    <header class="page-header">
      <div class="header-content">
        <p class="subtitle">Cognitive Productivity</p>
        <h1 class="title">艾森豪威尔矩阵</h1>
      </div>
    </header>

    <!-- Eisenhower Matrix Grid -->
    <div class="matrix-container">
      <div class="matrix-grid">
        <!-- Quadrant 2: Important but Not Urgent -->
        <div 
          class="quadrant q2" 
          :class="{ 'drag-over': dragOverQuadrant === 'q2' }"
          data-quadrant="q2"
          @dragover="handleQuadrantDragOver"
          @dragenter="handleQuadrantDragEnter($event, 'q2')"
          @dragleave="handleQuadrantDragLeave($event, 'q2')"
          @drop="handleQuadrantDrop($event, 'q2')"
        >
          <div class="quadrant-header">
            <div class="header-top">
              <span class="quadrant-label">Quadrant 2</span>
              <h3 class="quadrant-title">重要不紧急</h3>
            </div>
            <span class="action-badge schedule">制定计划 (SCHEDULE)</span>
          </div>
          <div class="task-list">
            <div 
              v-for="task in q2Tasks" 
              :key="task.id" 
              class="task-card" 
              :class="{ 'dragging': draggedTask?.id === task.id }"
              :data-id="task.id"
              draggable="true"
              @dragstart="handleDragStart($event, task)"
              @dragend="handleDragEnd"
            >
              <h4 class="task-title">{{ task.title }}</h4>
              <p v-if="task.description" class="task-desc">{{ task.description }}</p>
              <div class="task-tags">
                <span v-if="task.isImportant" class="tag important">重要</span>
                <span v-if="task.isUrgent" class="tag urgent">紧急</span>
                <span v-for="tag in task.tags" :key="tag.id" class="tag" :style="{ background: tag.color + '20', color: tag.color }">
                  {{ tag.name }}
                </span>
              </div>
            </div>
            <div v-if="q2Tasks.length === 0" class="empty-hint">
              <span>拖拽任务到此处</span>
            </div>
          </div>
          <div class="quadrant-footer">
            <span class="task-count">{{ q2Tasks.length }} 个任务</span>
          </div>
        </div>

        <!-- Quadrant 1: Important & Urgent -->
        <div 
          class="quadrant q1" 
          :class="{ 'drag-over': dragOverQuadrant === 'q1' }"
          data-quadrant="q1"
          @dragover="handleQuadrantDragOver"
          @dragenter="handleQuadrantDragEnter($event, 'q1')"
          @dragleave="handleQuadrantDragLeave($event, 'q1')"
          @drop="handleQuadrantDrop($event, 'q1')"
        >
          <div class="quadrant-header">
            <div class="header-top">
              <span class="quadrant-label">Quadrant 1</span>
              <h3 class="quadrant-title">重要且紧急</h3>
            </div>
            <span class="action-badge do">立即处理 (DO)</span>
          </div>
          <div class="task-list">
            <div 
              v-for="task in q1Tasks" 
              :key="task.id" 
              class="task-card" 
              :class="{ 'dragging': draggedTask?.id === task.id }"
              :data-id="task.id"
              draggable="true"
              @dragstart="handleDragStart($event, task)"
              @dragend="handleDragEnd"
            >
              <h4 class="task-title">{{ task.title }}</h4>
              <p v-if="task.description" class="task-desc">{{ task.description }}</p>
              <div class="task-tags">
                <span v-if="task.isImportant" class="tag important">重要</span>
                <span v-if="task.isUrgent" class="tag urgent">紧急</span>
                <span v-for="tag in task.tags" :key="tag.id" class="tag" :style="{ background: tag.color + '20', color: tag.color }">
                  {{ tag.name }}
                </span>
              </div>
            </div>
            <div v-if="q1Tasks.length === 0" class="empty-hint">
              <span>拖拽任务到此处</span>
            </div>
          </div>
          <div class="quadrant-footer">
            <span class="task-count">{{ q1Tasks.length }} 个任务</span>
          </div>
        </div>

        <!-- Quadrant 3: Not Important & Not Urgent -->
        <div 
          class="quadrant q3" 
          :class="{ 'drag-over': dragOverQuadrant === 'q3' }"
          data-quadrant="q3"
          @dragover="handleQuadrantDragOver"
          @dragenter="handleQuadrantDragEnter($event, 'q3')"
          @dragleave="handleQuadrantDragLeave($event, 'q3')"
          @drop="handleQuadrantDrop($event, 'q3')"
        >
          <div class="quadrant-header">
            <div class="header-top">
              <span class="quadrant-label">Quadrant 3</span>
              <h3 class="quadrant-title">不重要不紧急</h3>
            </div>
            <span class="action-badge delete">尽量消除 (DELETE)</span>
          </div>
          <div class="task-list">
            <div 
              v-for="task in q3Tasks" 
              :key="task.id" 
              class="task-card" 
              :class="{ 'dragging': draggedTask?.id === task.id }"
              :data-id="task.id"
              draggable="true"
              @dragstart="handleDragStart($event, task)"
              @dragend="handleDragEnd"
            >
              <h4 class="task-title">{{ task.title }}</h4>
              <p v-if="task.description" class="task-desc">{{ task.description }}</p>
              <div class="task-tags">
                <span v-if="task.isImportant" class="tag important">重要</span>
                <span v-if="task.isUrgent" class="tag urgent">紧急</span>
                <span v-for="tag in task.tags" :key="tag.id" class="tag" :style="{ background: tag.color + '20', color: tag.color }">
                  {{ tag.name }}
                </span>
              </div>
            </div>
            <div v-if="q3Tasks.length === 0" class="empty-hint">
              <span>拖拽任务到此处</span>
            </div>
          </div>
          <div class="quadrant-footer">
            <span class="task-count">{{ q3Tasks.length }} 个任务</span>
          </div>
        </div>

        <!-- Quadrant 4: Not Important but Urgent -->
        <div 
          class="quadrant q4" 
          :class="{ 'drag-over': dragOverQuadrant === 'q4' }"
          data-quadrant="q4"
          @dragover="handleQuadrantDragOver"
          @dragenter="handleQuadrantDragEnter($event, 'q4')"
          @dragleave="handleQuadrantDragLeave($event, 'q4')"
          @drop="handleQuadrantDrop($event, 'q4')"
        >
          <div class="quadrant-header">
            <div class="header-top">
              <span class="quadrant-label">Quadrant 4</span>
              <h3 class="quadrant-title">不重要但紧急</h3>
            </div>
            <span class="action-badge delegate">交由他人 (DELEGATE)</span>
          </div>
          <div class="task-list">
            <div 
              v-for="task in q4Tasks" 
              :key="task.id" 
              class="task-card" 
              :class="{ 'dragging': draggedTask?.id === task.id }"
              :data-id="task.id"
              draggable="true"
              @dragstart="handleDragStart($event, task)"
              @dragend="handleDragEnd"
            >
              <h4 class="task-title">{{ task.title }}</h4>
              <p v-if="task.description" class="task-desc">{{ task.description }}</p>
              <div class="task-tags">
                <span v-if="task.isImportant" class="tag important">重要</span>
                <span v-if="task.isUrgent" class="tag urgent">紧急</span>
                <span v-for="tag in task.tags" :key="tag.id" class="tag" :style="{ background: tag.color + '20', color: tag.color }">
                  {{ tag.name }}
                </span>
              </div>
            </div>
            <div v-if="q4Tasks.length === 0" class="empty-hint">
              <span>拖拽任务到此处</span>
            </div>
          </div>
          <div class="quadrant-footer">
            <span class="task-count">{{ q4Tasks.length }} 个任务</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Footer Status Bar -->
    <footer class="status-bar">
      <div class="status-left">
        <div class="status-item urgent">
          <span class="status-dot"></span>
          <span>{{ urgentCount }} 紧急任务</span>
        </div>
        <div class="status-item planned">
          <span class="status-dot"></span>
          <span>{{ totalCount }} 总任务</span>
        </div>
        <div class="status-item archived">
          <span class="status-dot"></span>
          <span>{{ archivedCount }} 已归档</span>
        </div>
      </div>
      <!-- <div class="status-right">
        <span v-if="lastSyncTime" class="sync-time">最后同步: {{ formatSyncTime(lastSyncTime) }}</span>
        <span v-else class="sync-time">加载中...</span>
      </div> -->
    </footer>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

// 任务类型定义
interface Tag {
  id: number
  name: string
  color: string
}

interface Task {
  id: number
  title: string
  description?: string
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

// 响应式数据
const tasks = ref<Task[]>([])
const draggedTask = ref<Task | null>(null)
const dragOverQuadrant = ref<string | null>(null)
const isDragOverArchive = ref(false)
const isLoading = ref(false)
const lastSyncTime = ref<Date | null>(null)
const toastMessage = ref('')
const toastType = ref<'success' | 'error'>('success')

// 拖拽计数器（处理嵌套元素的拖拽事件）
const dragEnterCounter = ref<Record<string, number>>({
  q1: 0,
  q2: 0,
  q3: 0,
  q4: 0,
  archive: 0
})

// 计算属性：各象限任务（只显示进行中的任务）
const q1Tasks = computed(() => tasks.value.filter(t => t.isImportant && t.isUrgent && !t.archived && t.status === 'in_progress'))
const q2Tasks = computed(() => tasks.value.filter(t => t.isImportant && !t.isUrgent && !t.archived && t.status === 'in_progress'))
const q3Tasks = computed(() => tasks.value.filter(t => !t.isImportant && !t.isUrgent && !t.archived && t.status === 'in_progress'))
const q4Tasks = computed(() => tasks.value.filter(t => !t.isImportant && t.isUrgent && !t.archived && t.status === 'in_progress'))

// 计算属性：统计信息（只统计进行中的任务）
const urgentCount = computed(() => tasks.value.filter(t => t.isUrgent && !t.archived && t.status === 'in_progress').length)
const totalCount = computed(() => tasks.value.filter(t => !t.archived && t.status === 'in_progress').length)
const archivedCount = computed(() => tasks.value.filter(t => t.archived).length)

// Toast 提示
function showToast(message: string, type: 'success' | 'error' = 'success') {
  toastMessage.value = message
  toastType.value = type
  setTimeout(() => {
    toastMessage.value = ''
  }, 3000)
}

// 格式化同步时间
function formatSyncTime(date: Date): string {
  const now = new Date()
  const diff = Math.floor((now.getTime() - date.getTime()) / 1000)
  
  if (diff < 60) return '刚刚'
  if (diff < 3600) return `${Math.floor(diff / 60)} 分钟前`
  if (diff < 86400) return `${Math.floor(diff / 3600)} 小时前`
  return `${Math.floor(diff / 86400)} 天前`
}

// 加载任务数据
async function loadTasks() {
  isLoading.value = true
  try {
    const result = await invoke<Task[]>('get_all_tasks', { includeArchived: true })
    tasks.value = result
    lastSyncTime.value = new Date()
    console.log('✅ 任务加载成功:', result.length)
  } catch (error) {
    console.error('❌ 加载任务失败:', error)
    showToast('加载任务失败', 'error')
  } finally {
    isLoading.value = false
  }
}

// 获取象限对应的重要性和紧急性
function getQuadrantConfig(quadrant: string): { isImportant: boolean; isUrgent: boolean } {
  switch (quadrant) {
    case 'q1': return { isImportant: true, isUrgent: true }
    case 'q2': return { isImportant: true, isUrgent: false }
    case 'q3': return { isImportant: false, isUrgent: false }
    case 'q4': return { isImportant: false, isUrgent: true }
    default: return { isImportant: false, isUrgent: false }
  }
}

// 拖拽开始
function handleDragStart(event: DragEvent, task: Task) {
  draggedTask.value = task
  
  if (event.dataTransfer) {
    event.dataTransfer.effectAllowed = 'move'
    event.dataTransfer.setData('text/plain', task.id.toString())
    // 设置拖拽图像
    const target = event.target as HTMLElement
    if (target) {
      event.dataTransfer.setDragImage(target, 0, 0)
    }
  }
  
  console.log('📝 开始拖拽任务:', task.title)
}

// 拖拽结束
function handleDragEnd() {
  draggedTask.value = null
  dragOverQuadrant.value = null
  isDragOverArchive.value = false
  // 重置计数器
  dragEnterCounter.value = { q1: 0, q2: 0, q3: 0, q4: 0, archive: 0 }
  console.log('📝 拖拽结束')
}

// 象限拖拽悬停
function handleQuadrantDragOver(event: DragEvent) {
  event.preventDefault()
  if (event.dataTransfer) {
    event.dataTransfer.dropEffect = 'move'
  }
}

// 象限拖拽进入
function handleQuadrantDragEnter(event: DragEvent, quadrant: string) {
  event.preventDefault()
  dragEnterCounter.value[quadrant]++
  
  if (dragEnterCounter.value[quadrant] === 1) {
    dragOverQuadrant.value = quadrant
    console.log('📝 进入象限:', quadrant)
  }
}

// 象限拖拽离开
function handleQuadrantDragLeave(event: DragEvent, quadrant: string) {
  event.preventDefault()
  dragEnterCounter.value[quadrant] = Math.max(0, dragEnterCounter.value[quadrant] - 1)
  
  if (dragEnterCounter.value[quadrant] === 0) {
    // 检查是否真的离开了象限（不是进入了子元素）
    const relatedTarget = event.relatedTarget as HTMLElement
    const currentTarget = event.currentTarget as HTMLElement
    if (!currentTarget.contains(relatedTarget)) {
      dragOverQuadrant.value = null
      console.log('📝 离开象限:', quadrant)
    }
  }
}

// 象限放置
async function handleQuadrantDrop(event: DragEvent, targetQuadrant: string) {
  event.preventDefault()
  
  const task = draggedTask.value
  if (!task) return
  
  // 重置状态
  dragOverQuadrant.value = null
  dragEnterCounter.value = { q1: 0, q2: 0, q3: 0, q4: 0, archive: 0 }
  
  // 获取目标象限的配置
  const config = getQuadrantConfig(targetQuadrant)
  
  // 检查是否真的需要更新
  if (task.isImportant === config.isImportant && task.isUrgent === config.isUrgent) {
    console.log('📝 任务已在该象限，无需更新')
    draggedTask.value = null
    return
  }
  
  // 乐观更新：先更新本地状态
  const taskIndex = tasks.value.findIndex(t => t.id === task.id)
  if (taskIndex !== -1) {
    tasks.value[taskIndex] = {
      ...task,
      isImportant: config.isImportant,
      isUrgent: config.isUrgent
    }
  }
  
  console.log(`📝 移动任务到 ${targetQuadrant}:`, task.title)
  
  try {
    // 调用后端API更新任务
    await invoke('update_task_quadrant', {
      taskId: task.id,
      isImportant: config.isImportant,
      isUrgent: config.isUrgent
    })
    
    lastSyncTime.value = new Date()
    showToast('任务移动成功', 'success')
    console.log('✅ 任务象限更新成功')
  } catch (error) {
    console.error('❌ 更新任务象限失败:', error)
    showToast('移动任务失败，请重试', 'error')
    // 回滚本地状态
    await loadTasks()
  } finally {
    draggedTask.value = null
  }
}

// 归档按钮拖拽悬停
function handleArchiveDragOver(event: DragEvent) {
  event.preventDefault()
  if (event.dataTransfer) {
    event.dataTransfer.dropEffect = 'move'
  }
}

// 归档按钮拖拽进入
function handleArchiveDragEnter(event: DragEvent) {
  event.preventDefault()
  dragEnterCounter.value.archive++
  
  if (dragEnterCounter.value.archive === 1) {
    isDragOverArchive.value = true
    console.log('📝 进入归档区域')
  }
}

// 归档按钮拖拽离开
function handleArchiveDragLeave(event: DragEvent) {
  event.preventDefault()
  dragEnterCounter.value.archive = Math.max(0, dragEnterCounter.value.archive - 1)
  
  if (dragEnterCounter.value.archive === 0) {
    const relatedTarget = event.relatedTarget as HTMLElement
    const currentTarget = event.currentTarget as HTMLElement
    if (!currentTarget.contains(relatedTarget)) {
      isDragOverArchive.value = false
      console.log('📝 离开归档区域')
    }
  }
}

// 归档放置
async function handleArchiveDrop(event: DragEvent) {
  event.preventDefault()
  
  const task = draggedTask.value
  if (!task) return
  
  // 重置状态
  isDragOverArchive.value = false
  dragEnterCounter.value = { q1: 0, q2: 0, q3: 0, q4: 0, archive: 0 }
  
  // 检查任务是否已归档
  if (task.archived) {
    console.log('📝 任务已归档，无需重复操作')
    draggedTask.value = null
    return
  }
  
  // 乐观更新：先更新本地状态
  const taskIndex = tasks.value.findIndex(t => t.id === task.id)
  if (taskIndex !== -1) {
    tasks.value[taskIndex] = {
      ...task,
      archived: true,
      archivedAt: new Date().toISOString()
    }
  }
  
  console.log('📝 归档任务:', task.title)
  
  try {
    // 调用后端API归档任务
    await invoke('archive_task', {
      taskId: task.id,
      archived: true
    })
    
    lastSyncTime.value = new Date()
    showToast('任务已归档', 'success')
    console.log('✅ 任务归档成功')
  } catch (error) {
    console.error('❌ 归档任务失败:', error)
    showToast('归档任务失败，请重试', 'error')
    // 回滚本地状态
    await loadTasks()
  } finally {
    draggedTask.value = null
  }
}

// 组件挂载时加载数据
onMounted(() => {
  loadTasks()
})
</script>

<style scoped>
.quadrant-page {
  padding: 32px 40px;
  min-height: 100vh;
  background: var(--bg-page);
  display: flex;
  flex-direction: column;
}

/* Toast 提示 */
.toast {
  position: fixed;
  top: 20px;
  right: 20px;
  padding: 12px 24px;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  color: var(--bg-card);
  z-index: 1000;
  animation: slideIn 0.3s ease, fadeOut 0.3s ease 2.7s;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.toast.success {
  background: var(--color-success);
}

.toast.error {
  background: var(--color-error);
}

@keyframes slideIn {
  from {
    transform: translateX(100%);
    opacity: 0;
  }
  to {
    transform: translateX(0);
    opacity: 1;
  }
}

@keyframes fadeOut {
  from {
    opacity: 1;
  }
  to {
    opacity: 0;
  }
}

/* Header */
.page-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 24px;
}

.header-content .subtitle {
  font-size: 12px;
  font-weight: 600;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 1px;
  margin: 0 0 8px 0;
}

.header-content .title {
  font-size: 28px;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0;
}

/* Matrix Container */
.matrix-container {
  flex: 1;
  display: flex;
  flex-direction: column;
}

.matrix-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  grid-template-rows: 1fr 1fr;
  gap: 16px;
  flex: 1;
  position: relative;
}

/* Quadrant */
.quadrant {
  background: var(--bg-card);
  border-radius: 16px;
  padding: 20px;
  display: flex;
  flex-direction: column;
  box-shadow: 0 1px 3px var(--bg-hover);
  position: relative;
  overflow: hidden;
  transition: all 0.2s ease;
}

.quadrant.drag-over {
  background: rgba(102, 126, 234, 0.05);
  box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.3), 0 4px 12px var(--border-color);
  transform: scale(1.01);
}

.quadrant::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 4px;
}

.quadrant.q1::before {
  background: linear-gradient(90deg, var(--color-error), #f97316);
}

.quadrant.q2::before {
  background: linear-gradient(90deg, #3b82f6, #8b5cf6);
}

.quadrant.q3::before {
  background: linear-gradient(90deg, var(--color-warning), #eab308);
}

.quadrant.q4::before {
  background: linear-gradient(90deg, var(--text-secondary), #9ca3af);
}

.quadrant-header {
  margin-bottom: 16px;
}

.header-top {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 8px;
}

.quadrant-label {
  font-size: 11px;
  font-weight: 700;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.quadrant-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
}

.action-badge {
  display: inline-block;
  padding: 4px 10px;
  border-radius: 6px;
  font-size: 11px;
  font-weight: 600;
  letter-spacing: 0.3px;
}

.action-badge.do {
  background: rgba(239, 68, 68, 0.1);
  color: var(--color-error);
}

.action-badge.schedule {
  background: rgba(59, 130, 246, 0.1);
  color: #3b82f6;
}

.action-badge.delegate {
  background: rgba(245, 158, 11, 0.1);
  color: var(--color-warning);
}

.action-badge.delete {
  background: rgba(107, 114, 128, 0.1);
  color: var(--text-secondary);
}

/* Task List */
.task-list {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 12px;
  overflow-y: auto;
  min-height: 100px;
  padding: 4px;
}

.empty-hint {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 80px;
  border: 2px dashed #e8ecf1;
  border-radius: 10px;
  color: var(--text-secondary);
  font-size: 13px;
}

.task-card {
  padding: 14px;
  background: var(--bg-page);
  border-radius: 10px;
  border-left: 3px solid transparent;
  transition: all 0.2s ease;
  cursor: grab;
  user-select: none;
}

.task-card:active {
  cursor: grabbing;
}

.task-card.dragging {
  opacity: 0.5;
  transform: scale(0.98);
  cursor: grabbing;
}

.task-card:hover {
  background: #e8ecf1;
  transform: translateX(2px);
}

.q1 .task-card {
  border-left-color: var(--color-error);
}

.q2 .task-card {
  border-left-color: #3b82f6;
}

.q3 .task-card {
  border-left-color: var(--color-warning);
}

.q4 .task-card {
  border-left-color: var(--text-secondary);
}

.task-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 6px 0;
}

.task-desc {
  font-size: 12px;
  color: var(--text-secondary);
  line-height: 1.5;
  margin: 0 0 10px 0;
}

.task-tags {
  display: flex;
  gap: 6px;
  flex-wrap: wrap;
}

.tag {
  padding: 3px 8px;
  border-radius: 4px;
  font-size: 10px;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.3px;
}

.tag.important {
  background: rgba(245, 158, 11, 0.15);
  color: var(--color-warning);
}

.tag.urgent {
  background: rgba(239, 68, 68, 0.15);
  color: var(--color-error);
}

.quadrant-footer {
  margin-top: 12px;
  padding-top: 12px;
  border-top: 1px solid var(--bg-input);
}

.task-count {
  font-size: 12px;
  color: var(--text-secondary);
  font-weight: 500;
}

/* Center Archive Button */
.center-archive {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  z-index: 10;
  border-radius: 50%;
}

.archive-btn {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  padding: 16px 20px;
  background: var(--bg-card);
  border: 2px solid #e8ecf1;
  border-radius: 50%;
  width: 80px;
  height: 80px;
  cursor: pointer;
  transition: all 0.2s ease;
  box-shadow: 0 2px 8px var(--border-color);
}

.archive-btn:hover {
  border-color: var(--color-primary);
  background: rgba(0, 102, 204, 0.02);
}

.archive-btn.drag-over {
  border-color: var(--color-primary);
  background: rgba(0, 102, 204, 0.1);
  transform: scale(1.1);
  box-shadow: 0 4px 16px rgba(0, 102, 204, 0.2);
}

.archive-btn.drag-over svg,
.archive-btn.drag-over span {
  color: var(--color-primary);
}

.archive-btn svg {
  width: 24px;
  height: 24px;
  color: var(--text-secondary);
}

.archive-btn:hover svg {
  color: var(--color-primary);
}

.archive-btn span {
  font-size: 10px;
  font-weight: 600;
  color: var(--text-secondary);
}

.archive-btn:hover span {
  color: var(--color-primary);
}

/* Status Bar */
.status-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: 24px;
  padding: 16px 20px;
  background: var(--bg-card);
  border-radius: 12px;
  box-shadow: 0 1px 3px var(--bg-hover);
}

.status-left {
  display: flex;
  gap: 24px;
}

.status-item {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 13px;
  font-weight: 500;
  color: var(--text-secondary);
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
}

.status-item.urgent .status-dot {
  background: var(--color-error);
}

.status-item.planned .status-dot {
  background: #3b82f6;
}

.status-item.archived .status-dot {
  background: var(--text-secondary);
}

.status-right {
  display: flex;
  align-items: center;
  gap: 16px;
}

.sync-time {
  font-size: 12px;
  color: var(--text-secondary);
}
</style>