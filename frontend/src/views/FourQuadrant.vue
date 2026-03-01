<script setup lang="ts">
import { ref, onMounted, reactive } from 'vue'
import { ElMessage } from 'element-plus'
import { getQuadrantTasks, updateTaskPriorityUrgency, createTask, deleteTask, type Task } from '@/api/task'
import { Check, Plus, Sort, Document } from '@element-plus/icons-vue'

// 任务数据
const tasks = ref<Task[]>([])

// 象限定义 - 使用绿色主题配色方案
const quadrants = [
  {
    id: 'not-urgent-important',
    title: '第二象限',
    desc: '重要但不紧急 (制定计划)',
    bgColor: 'bg-amber-50',
    accentColor: 'bg-amber-500',
    hexColor: '#f59e0b',
    gradient: 'linear-gradient(135deg, #FEF3C7 0%, #FDE68A 100%)',
    borderColor: '#F59E0B'
  },
  {
    id: 'urgent-important',
    title: '第一象限',
    desc: '紧急且重要 (立即执行)',
    bgColor: 'bg-rose-50',
    accentColor: 'bg-rose-500',
    hexColor: '#f43f5e',
    gradient: 'linear-gradient(135deg, #FECDD3 0%, #FDA4AF 100%)',
    borderColor: '#F43F5E'
  },
  {
    id: 'not-urgent-not-important',
    title: '第三象限',
    desc: '不紧急且不重要 (稍后或取消)',
    bgColor: 'bg-blue-50',
    accentColor: 'bg-blue-500',
    hexColor: '#3b82f6',
    gradient: 'linear-gradient(135deg, #DBEAFE 0%, #93C5FD 100%)',
    borderColor: '#3B82F6'
  },
  {
    id: 'urgent-not-important',
    title: '第四象限',
    desc: '不重要但紧急 (委派或减少)',
    bgColor: 'bg-emerald-50',
    accentColor: 'bg-emerald-500',
    hexColor: '#10b981',
    gradient: 'linear-gradient(135deg, #D1FAE5 0%, #A7F3D0 100%)',
    borderColor: '#10B981'
  }
]

// 象限ID到优先级和紧急程度的映射
const quadrantToPriorityUrgency = (quadrantId: string) => {
  const map: Record<string, { priority: string; urgency: string }> = {
    'not-urgent-important': { priority: 'high', urgency: '-high' },
    'urgent-important': { priority: 'high', urgency: 'high' },
    'not-urgent-not-important': { priority: '-high', urgency: '-high' },
    'urgent-not-important': { priority: '-high', urgency: 'high' }
  }
  return map[quadrantId] || { priority: 'middle', urgency: 'middle' }
}

// 优先级和紧急程度到象限ID的映射
const priorityUrgencyToQuadrant = (priority: string, urgency: string) => {
  const map: Record<string, string> = {
    'high,-high': 'not-urgent-important',
    'high,high': 'urgent-important',
    '-high,-high': 'not-urgent-not-important',
    '-high,high': 'urgent-not-important'
  }
  return map[`${priority},${urgency}`] || 'not-urgent-not-important'
}

// 计算任务所属的象限
const getTaskQuadrant = (task: Task) => {
  if (!task.priority || !task.urgency) return 'not-urgent-not-important'
  return priorityUrgencyToQuadrant(task.priority, task.urgency)
}

// 根据任务状态筛选任务
const getFilteredTasks = (quadrantId: string) => {
  return tasks.value.filter(t => getTaskQuadrant(t) === quadrantId && t.status !== 'completed')
}

const draggedTask = ref<Task | null>(null)
const isOverCenter = ref(false)
const showModal = ref(false)
const showSuccess = ref(false)
const currentTargetQuadrant = ref('')
const loading = ref(false)

// 任务表单
const taskForm = reactive({
  title: '',
  description: '',
  status: 'planning',
  priority: 'high',
  urgency: 'high',
  progress: 0
})

// 加载任务数据
const loadTasks = async () => {
  try {
    loading.value = true
    tasks.value = await getQuadrantTasks()
  } catch (error) {
    console.error('加载任务失败:', error)
  } finally {
    loading.value = false
  }
}

// 组件挂载时加载任务
onMounted(() => {
  loadTasks()
})

const onDragStart = (task: Task) => {
  draggedTask.value = task;
};

const onDragEnd = () => {
  // draggedTask.value = null;
  isOverCenter.value = false;
};

const onDragOverCenter = () => {
  isOverCenter.value = true;
};

const onDropCenter = async () => {
  if (draggedTask.value && draggedTask.value.id) {
    try {
      // 删除任务
      await deleteTask(draggedTask.value.id)
      // 从本地列表中移除
      tasks.value = tasks.value.filter(t => t.id !== draggedTask.value!.id)
      showSuccess.value = true
      setTimeout(() => showSuccess.value = false, 2000)
    } catch (error) {
      console.error('完成任务失败:', error)
    }
  }
  isOverCenter.value = false
  draggedTask.value = null
}

const onDropQuadrant = async (e: DragEvent, quadrantId: string) => {
  if (draggedTask.value && draggedTask.value.id) {
    try {
      // 获取目标象限的priority和urgency
      const { priority, urgency } = quadrantToPriorityUrgency(quadrantId)
      // 调用API更新
      await updateTaskPriorityUrgency(draggedTask.value.id!, priority, urgency)

      // 更新本地数据
      const task = tasks.value.find(t => t.id === draggedTask.value!.id)
      if (task) {
        task.priority = priority as any
        task.urgency = urgency as any
      }
    } catch (error) {
      console.error('更新任务失败:', error)
    }
  }
}

const openAddModal = (quadId: string) => {
  currentTargetQuadrant.value = quadId
  // 根据象限设置默认的优先级和紧急程度
  const { priority, urgency } = quadrantToPriorityUrgency(quadId)
  Object.assign(taskForm, {
    title: '',
    description: '',
    status: 'planning',
    priority: priority as any,
    urgency: urgency as any,
    progress: 0
  })
  showModal.value = true
}

const addTask = async () => {
  if (!taskForm.title.trim()) {
    ElMessage.warning('请输入任务标题')
    return
  }

  try {
    const newTask = await createTask({
      title: taskForm.title,
      description: taskForm.description,
      status: taskForm.status,
      priority: taskForm.priority,
      urgency: taskForm.urgency,
      progress: taskForm.progress,
      tags: [],
      completed: taskForm.status === 'completed'
    })
    tasks.value.push(newTask)
    showModal.value = false
    ElMessage.success('任务添加成功')
  } catch (error) {
    console.error('创建任务失败:', error)
    ElMessage.error('创建任务失败')
  }
}

const getQuadrantClass = (quadrantId) => {
  const classMap = {
    'not-urgent-important': 'quadrant-top-left',
    'urgent-important': 'quadrant-top-right',
    'not-urgent-not-important': 'quadrant-bottom-left',
    'urgent-not-important': 'quadrant-bottom-right'
  };
  return classMap[quadrantId] || '';
};

const getTextPositionClass = (quadrantId) => {
  const classMap = {
    'not-urgent-important': 'text-top-left',
    'urgent-important': 'text-top-right',
    'not-urgent-not-important': 'text-bottom-left',
    'urgent-not-important': 'text-bottom-right'
  };
  return classMap[quadrantId] || '';
};
</script>

<template>
  <div class="quadrant-page-container">
    <!-- 页面标题 -->
<!--    <div class="page-header">-->
<!--      <h1 class="page-title">四象限任务管理</h1>-->
<!--      <p class="page-subtitle">基于艾森豪威尔矩阵，科学管理任务优先级</p>-->
<!--    </div>-->

    <!-- 主四象限容器 -->
    <div class="quadrant-main-container">
      <!-- 中心完成区域 -->
      <div
          class="finish-center"
          :class="{'active': isOverCenter}"
          @dragover.prevent="onDragOverCenter"
          @dragleave="isOverCenter = false"
          @drop="onDropCenter"
      >
        <div class="finish-content">
          <div class="finish-icon">
            <el-icon><check /></el-icon>
          </div>
          <div class="finish-text">
            <p class="finish-title">拖入完成</p>
            <p class="finish-subtitle">完成任务</p>
          </div>
        </div>
      </div>

      <!-- 四个象限 -->
      <div
          v-for="quad in quadrants"
          :key="quad.id"
          class="quadrant-item"
          :class="getQuadrantClass(quad.id)"
          :style="{ background: quad.gradient, borderColor: quad.borderColor }"
          @dragover.prevent
          @drop="onDropQuadrant($event, quad.id)"
      >
        <!-- 象限头部 -->
        <div class="quadrant-header" :class="getTextPositionClass(quad.id)">
          <div class="header-content">
            <div class="quadrant-badge" :style="{ backgroundColor: quad.borderColor }"></div>
            <div class="header-text">
              <h3 class="quadrant-title">{{ quad.title }}</h3>
              <p class="quadrant-desc">{{ quad.desc }}</p>
            </div>
          </div>
          <button
              @click="openAddModal(quad.id)"
              class="add-task-btn"
              :style="{ backgroundColor: quad.borderColor }"
          >
            <el-icon><plus /></el-icon>
          </button>
        </div>

        <!-- 任务列表 -->
        <div class="task-list" :class="{'bottom-space': ['urgent-not-important', 'not-urgent-not-important'].includes(quad.id)}">
          <div class="task-list-content">
            <div
                v-for="task in getFilteredTasks(quad.id)"
                :key="task.id"
                draggable="true"
                @dragstart="onDragStart(task)"
                @dragend="onDragEnd"
                class="task-card"
                :style="{ borderLeftColor: quad.borderColor }"
            >
              <div class="task-content">
                <div class="task-title">{{ task.title || task.description }}</div>
                <div class="task-drag-handle">
                  <el-icon><sort /></el-icon>
                </div>
              </div>
            </div>

            <!-- 空状态 -->
            <div v-if="getFilteredTasks(quad.id).length === 0 && !loading"
                class="empty-state">
              <div class="empty-icon">
                <el-icon><document /></el-icon>
              </div>
              <p class="empty-text">暂无任务</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>

  <!-- 新增任务弹窗 -->
  <el-dialog
      v-model="showModal"
      title="添加新任务"
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
          <el-option label="制作中" value="in-progress"/>
          <el-option label="已完成" value="completed"/>
        </el-select>
      </el-form-item>
      <el-form-item label="紧急程度">
        <el-select v-model="taskForm.urgency" placeholder="请选择紧急程度" disabled>
          <el-option label="非紧急" value="-high"/>
          <el-option label="一般" value="middle"/>
          <el-option label="紧急" value="high"/>
        </el-select>
      </el-form-item>
      <el-form-item label="重要程度">
        <el-select v-model="taskForm.priority" placeholder="请选择重要程度" disabled>
          <el-option label="不重要" value="-high"/>
          <el-option label="一般" value="middle"/>
          <el-option label="重要" value="high"/>
        </el-select>
      </el-form-item>
    </el-form>

    <template #footer>
      <el-button @click="showModal = false">取消</el-button>
      <el-button type="primary" @click="addTask">保存</el-button>
    </template>
  </el-dialog>

  <!-- 完成反馈提示 -->
  <div v-if="showSuccess"
       class="fixed bottom-10 left-1/2 -translate-x-1/2 bg-green-600 text-white px-6 py-3 rounded-full font-bold shadow-xl animate-bounce z-[110]">
    任务已达成！ 🎉
  </div>
</template>

<style scoped>
/* 页面整体布局 */
.quadrant-page-container {
  min-height: 100vh;
  background: linear-gradient(135deg, #F0FDFA 0%, #E0F2F1 100%);
  padding: 32px;
  overflow: hidden;
}

/* 页面标题 */
.page-header {
  text-align: center;
  margin-bottom: 40px;
}

.page-title {
  font-size: 36px;
  font-weight: 700;
  color: #134E4A;
  margin: 0 0 8px 0;
  letter-spacing: -0.02em;
}

.page-subtitle {
  font-size: 16px;
  color: #0D9488;
  margin: 0;
  font-weight: 500;
}

/* 主四象限容器 */
.quadrant-main-container {
  display: grid;
  grid-template-columns: 1fr 1fr;
  grid-template-rows: 1fr 1fr;
  gap: 20px;
  position: relative;
  max-width: 1200px;
  margin: 0 auto;
  height: calc(100vh - 200px);
  min-height: 500px;
  max-height: 700px;
}

/* 中心完成区域 */
.finish-center {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 120px;
  height: 120px;
  background: linear-gradient(135deg, #0D9488 0%, #14B8A6 100%);
  border-radius: 50%;
  border: 4px solid white;
  box-shadow: 0 20px 40px rgba(13, 148, 136, 0.3);
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.3s ease;
  z-index: 60;
}

.finish-center.active {
  background: linear-gradient(135deg, #F97316 0%, #FB923C 100%);
  transform: translate(-50%, -50%) scale(1.1);
  box-shadow: 0 25px 50px rgba(249, 115, 22, 0.4);
}

.finish-content {
  text-align: center;
  color: white;
}

.finish-icon {
  font-size: 24px;
  margin-bottom: 8px;
}

.finish-title {
  font-size: 12px;
  font-weight: 700;
  margin: 0;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.finish-subtitle {
  font-size: 10px;
  margin: 0;
  opacity: 0.9;
}

/* 象限项样式 */
.quadrant-item {
  position: relative;
  border-radius: 16px;
  border: 2px solid;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  transition: all 0.3s ease;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.quadrant-item:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.15);
}

/* 象限缺口效果 */
.quadrant-top-left {
  border-radius: 16px 16px 8px 16px;
}

.quadrant-top-right {
  border-radius: 16px 16px 16px 8px;
}

.quadrant-bottom-left {
  border-radius: 16px 8px 16px 16px;
}

.quadrant-bottom-right {
  border-radius: 8px 16px 16px 16px;
}

/* 象限头部 */
.quadrant-header {
  display: flex;
  align-items: center;
  padding: 20px;
  min-height: 80px;
}

.text-top-left {
  justify-content: space-between;
}

.text-top-right {
  justify-content: space-between;
  flex-direction: row-reverse;
}

.text-bottom-left {
  justify-content: space-between;
  margin-top: auto;
}

.text-bottom-right {
  justify-content: space-between;
  flex-direction: row-reverse;
  margin-top: auto;
}

.header-content {
  display: flex;
  align-items: center;
  gap: 12px;
}

.quadrant-badge {
  width: 4px;
  height: 24px;
  border-radius: 2px;
  flex-shrink: 0;
}

.header-text {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.quadrant-title {
  font-size: 18px;
  font-weight: 700;
  color: #1F2937;
  margin: 0;
  line-height: 1.2;
}

.quadrant-desc {
  font-size: 12px;
  color: #6B7280;
  margin: 0;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

/* 添加任务按钮 */
.add-task-btn {
  width: 40px;
  height: 40px;
  border-radius: 10px;
  border: none;
  color: white;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.2s ease;
  font-size: 16px;
}

.add-task-btn:hover {
  transform: scale(1.1);
}

/* 任务列表 */
.task-list {
  flex: 1;
  overflow: hidden;
  padding: 0 20px 20px;
  display: flex;
  flex-direction: column;
}

.bottom-space {
  padding-bottom: 60px;
}

.task-list-content {
  flex: 1;
  overflow-y: auto;
  padding-right: 4px;
}

.task-list-content::-webkit-scrollbar {
  width: 4px;
}

.task-list-content::-webkit-scrollbar-track {
  background: rgba(0, 0, 0, 0.05);
  border-radius: 2px;
}

.task-list-content::-webkit-scrollbar-thumb {
  background: rgba(0, 0, 0, 0.2);
  border-radius: 2px;
}

.task-list-content::-webkit-scrollbar-thumb:hover {
  background: rgba(0, 0, 0, 0.3);
}

/* 任务卡片 */
.task-card {
  background: white;
  border-radius: 8px;
  border-left: 4px solid;
  margin-bottom: 8px;
  cursor: grab;
  transition: all 0.2s ease;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.task-card:hover {
  transform: translateX(4px);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.15);
}

.task-card:active {
  cursor: grabbing;
}

.task-content {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
}

.task-title {
  font-size: 14px;
  font-weight: 500;
  color: #1F2937;
  line-height: 1.4;
  flex: 1;
  margin: 0;
}

.task-drag-handle {
  color: #9CA3AF;
  font-size: 14px;
  opacity: 0.6;
  transition: opacity 0.2s ease;
}

.task-card:hover .task-drag-handle {
  opacity: 1;
}

/* 空状态 */
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 120px;
  color: #9CA3AF;
}

.empty-icon {
  font-size: 32px;
  margin-bottom: 8px;
  opacity: 0.5;
}

.empty-text {
  font-size: 12px;
  font-style: italic;
  margin: 0;
}

/* 响应式设计 */
@media (max-width: 1024px) {
  .quadrant-main-container {
    height: calc(100vh - 180px);
    gap: 16px;
    max-height: 600px;
  }
  
  .quadrant-header {
    padding: 16px;
    min-height: 70px;
  }
  
  .task-list {
    padding: 0 16px 16px;
  }
}

@media (max-width: 768px) {
  .quadrant-page-container {
    padding: 20px;
  }
  
  .page-title {
    font-size: 28px;
  }
  
  .quadrant-main-container {
    grid-template-columns: 1fr;
    grid-template-rows: repeat(4, 1fr);
    height: auto;
    min-height: 800px;
    max-height: none;
  }
  
  .finish-center {
    display: none;
  }
  
  .quadrant-item {
    min-height: 200px;
  }
  
  .quadrant-title {
    font-size: 16px;
  }
  
  .quadrant-desc {
    font-size: 11px;
  }
}

@media (max-width: 480px) {
  .quadrant-page-container {
    padding: 16px;
  }
  
  .page-title {
    font-size: 24px;
  }
  
  .quadrant-header {
    padding: 12px;
    min-height: 60px;
  }
  
  .task-list {
    padding: 0 12px 12px;
  }
  
  .task-content {
    padding: 10px 12px;
  }
  
  .task-title {
    font-size: 13px;
  }
}
</style>