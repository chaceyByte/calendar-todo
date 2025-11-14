<template>
  <div class="tasks-container">
    <!-- 顶部控制栏 -->
    <div class="tasks-header">
      <div class="header-left">
        <h2>任务看板</h2>
      </div>
      <div class="header-right">
        <el-button type="primary" @click="showAddTaskDialog">
          <el-icon><plus /></el-icon>
          添加任务
        </el-button>
        <div class="staging-queue">
          <el-badge :value="stagingTasks.length" :max="99">
            <el-button @click="toggleStagingPanel">
              <el-icon><clock /></el-icon>
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
            <el-icon><close /></el-icon>
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
        
        <div class="column-content">
          <div 
            v-for="task in getTasksByStatus(column.id)" 
            :key="task.id"
            class="task-card"
            draggable="true"
            @dragstart="handleDragStart($event, task, column.id)"
            @contextmenu="(e) => showTaskContextMenu(e, task)"
          >
            <div class="task-header">
              <span class="task-title">{{ task.title }}</span>
              <el-dropdown trigger="click">
                <el-icon><more /></el-icon>
                <template #dropdown>
                  <el-dropdown-menu>
                    <el-dropdown-item @click="editTask(task)">编辑</el-dropdown-item>
                    <el-dropdown-item @click="addToStaging(task)">添加到暂存</el-dropdown-item>
                    <el-dropdown-item @click="() => handlePauseTask(task.id)">暂停</el-dropdown-item>
                    <el-dropdown-item @click="addTagsToTask(task)">添加标签</el-dropdown-item>
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
          </div>
          
          <!-- 添加任务按钮 -->
          <div v-if="column.id !== 'completed'" class="add-task-btn">
            <el-button 
              text 
              @click="showAddTaskDialog(column.id)"
              class="full-width"
            >
              <el-icon><plus /></el-icon>
              添加任务
            </el-button>
          </div>
        </div>
      </div>
    </div>

    <!-- 添加快捷键提示 -->
    <div class="shortcut-hint">
      <el-tag size="small">快捷键: Alt + Enter 快速添加任务</el-tag>
    </div>

    <!-- 添加/编辑任务对话框 -->
    <el-dialog 
      v-model="taskDialog.visible" 
      :title="taskDialog.isEdit ? '编辑任务' : '添加任务'"
      width="500px"
    >
      <el-form :model="taskForm" label-width="80px">
        <el-form-item label="标题">
          <el-input v-model="taskForm.title" placeholder="请输入任务标题" />
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
            <el-option label="计划中" value="planning" />
            <el-option label="制作中" value="in-progress" />
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
        <el-icon><edit /></el-icon>
        编辑
      </div>
      <div class="menu-item" @click="() => handlePauseTask(taskContextMenu.task.id)">
        <el-icon><video-pause /></el-icon>
        暂停
      </div>
      <div class="menu-item" @click="addTagsToTask(taskContextMenu.task)">
        <el-icon><price-tag /></el-icon>
        添加标签
      </div>
      <div class="menu-item" @click="deleteTask(taskContextMenu.task.id)" style="color: #f56c6c;">
        <el-icon><delete /></el-icon>
        删除
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, onUnmounted, computed } from 'vue'
import dayjs from 'dayjs'
import { 
  Plus, Clock, Close, More, Edit, VideoPause, PriceTag, Delete 
} from '@element-plus/icons-vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { useTaskStore } from '@/stores/task'

interface Task {
  id: number
  title: string
  description: string
  status: 'planning' | 'in-progress' | 'completed' | 'paused'
  progress: number
  tags: string[]
  createdAt: string
  updatedAt: string
}

interface Column {
  id: string
  title: string
}

const columns: Column[] = [
  { id: 'planning', title: '计划中' },
  { id: 'in-progress', title: '制作中' },
  { id: 'completed', title: '已完成' }
]

const taskStore = useTaskStore()

// 直接使用store中的任务数据
const tasks = computed(() => taskStore.tasks)

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
  tags: [] as string[]
})

const taskContextMenu = reactive({
  visible: false,
  x: 0,
  y: 0,
  task: {} as Task
})

const getTasksByStatus = (status: string) => {
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
    tags: []
  })
}

const editTask = (task: Task) => {
  taskDialog.isEdit = true
  taskDialog.visible = true
  Object.assign(taskForm, { ...task })
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
        tags: []
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
    dragTask.value = { task, source }
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
  
  const { task, source } = dragTask.value
  
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
    if (!stagingTasks.value.find(t => t.id === task.id)) {
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
  ElMessage.info('添加标签功能开发中')
}

const removeTagFromTask = (taskId: number, tag: string) => {
  const task = tasks.value.find(t => t.id === taskId)
  if (task) {
    task.tags = task.tags.filter(t => t !== tag)
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

// 快捷键支持
const handleKeydown = (e: KeyboardEvent) => {
  if (e.altKey && e.key === 'Enter') {
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
    // 现在tasks已经直接引用了store中的数据，无需额外赋值
  } catch (error) {
    console.error('加载任务失败:', error)
    ElMessage.error('加载任务失败')
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

// 更新移动任务功能，使其调用API
// 处理暂停任务
const handlePauseTask = async (taskId: number) => {
  try {
    await taskStore.pauseTask(taskId)
    // 刷新任务列表和暂存队列显示
    await loadTasks()
    await loadStagingTasks()
    ElMessage.success('任务已暂停')
  } catch (error) {
    console.error('暂停任务失败:', error)
    ElMessage.error('暂停任务失败，请重试')
  }
}

const moveTaskToColumn = async (task: Task, targetStatus: string) => {
  try {
    // 检查任务是否在暂存队列中
    const isFromStaging = stagingTasks.value.some(t => t.id === task.id)
    
    if (isFromStaging) {
      // 从暂存队列移动到看板
      await moveTaskFromStaging(task, targetStatus)
      // 刷新暂存队列显示
      await loadStagingTasks()
    } else {
      // 在看板内移动
      await taskStore.updateTask(task.id, {
        status: targetStatus as any,
        progress: targetStatus === 'completed' ? 100 : task.progress
      })
      await loadTasks()
    }
  } catch (error) {
    console.error('移动任务失败:', error)
    ElMessage.error('移动任务失败')
  }
}

onMounted(() => {
  document.addEventListener('keydown', handleKeydown)
  document.addEventListener('click', closeTaskContextMenu)
  // 初始化时加载任务和暂存队列
  loadTasks()
  loadStagingTasks()
})

onUnmounted(() => {
  document.removeEventListener('keydown', handleKeydown)
  document.removeEventListener('click', closeTaskContextMenu)
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