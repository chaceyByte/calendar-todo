<script setup lang="ts">
import { ref, onMounted, reactive } from 'vue'
import { ElMessage } from 'element-plus'
import { getQuadrantTasks, updateTaskPriorityUrgency, createTask, deleteTask, type Task } from '@/api/task'

// 任务数据
const tasks = ref<Task[]>([])

// 象限定义
const quadrants = [
  {
    id: 'not-urgent-important',
    title: '第二象限',
    desc: '重要但不紧急 (制定计划)',
    bgColor: 'bg-amber-50',
    accentColor: 'bg-amber-500',
    hexColor: '#f59e0b'
  },
  {
    id: 'urgent-important',
    title: '第一象限',
    desc: '紧急且重要 (立即执行)',
    bgColor: 'bg-rose-50',
    accentColor: 'bg-rose-500',
    hexColor: '#f43f5e'
  },
  {
    id: 'not-urgent-not-important',
    title: '第三象限',
    desc: '不紧急且不重要 (稍后或取消)',
    bgColor: 'bg-blue-50',
    accentColor: 'bg-blue-500',
    hexColor: '#3b82f6'
  },
  {
    id: 'urgent-not-important',
    title: '第四象限',
    desc: '不重要但紧急 (委派或减少)',
    bgColor: 'bg-emerald-50',
    accentColor: 'bg-emerald-500',
    hexColor: '#10b981'
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

const onDragStart = (task) => {
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
  <div class="quadrant-container">

    <!-- 中心完成区域 -->
    <div
        class="finish-center w-24 h-24 rounded-full bg-slate-900 shadow-2xl flex items-center justify-center border-4 border-white transition-all duration-300"
        :class="{'scale-125 bg-green-600': isOverCenter}"
        @dragover.prevent="onDragOverCenter"
        @dragleave="isOverCenter = false"
        @drop="onDropCenter"
    >
      <div class="text-white text-center">
        <p class="text-[10px] font-bold uppercase">拖入</p>
        <p class="text-xs uppercase">完成</p>
      </div>
    </div>

    <!-- 四个象限 -->
    <div
        v-for="quad in quadrants"
        :key="quad.id"
        :class="[quad.bgColor, 'quadrant-item', getQuadrantClass(quad.id), 'rounded-xl p-3 flex flex-col border-2 border-transparent transition-all overflow-hidden shadow-sm']"
        @dragover.prevent
        @drop="onDropQuadrant($event, quad.id)"
    >
      <!-- 第一、二象限的头部 -->
      <div v-if="['not-urgent-important', 'urgent-important'].includes(quad.id)"
           :class="['quadrant-header', getTextPositionClass(quad.id)]">
        <div class="header-content">
          <h2 class="font-black text-slate-800 flex items-center gap-2">
            <span class="w-2 h-6" :class="quad.accentColor"></span>
            {{ quad.title }}
          </h2>
          <p class="text-[10px] text-slate-500 font-bold uppercase mt-1">{{ quad.desc }}</p>
        </div>
        <button
            @click="openAddModal(quad.id)"
            class="w-8 h-8 rounded-full bg-white flex items-center justify-center shadow-sm hover:scale-110 active:scale-95 transition-transform"
        >
          <span class="text-xl font-bold">+</span>
        </button>
      </div>

      <!-- 任务列表 -->
      <div class="flex-1 overflow-y-auto no-scrollbar space-y-0.5 px-5 py-2"
           :class="{'pb-16': ['urgent-not-important', 'not-urgent-not-important'].includes(quad.id)}">
        <div
            v-for="task in tasks.filter(t => getTaskQuadrant(t) === quad.id && t.status !== 'completed')"
            :key="task.id"
            draggable="true"
            @dragstart="onDragStart(task)"
            @dragend="onDragEnd"
            class="task-card bg-white rounded-lg shadow border-l-4 ml-2 mr-1 hover:shadow-md transition-all duration-200 hover:-translate-y-0.5"
            :style="{ borderLeftColor: quad.hexColor }"
        >
          <p class="text-sm font-medium text-slate-800 px-5 py-2">{{ task.title || task.description }}</p>
        </div>

        <!-- 空状态 -->
        <div v-if="tasks.filter(t => getTaskQuadrant(t) === quad.id && t.status !== 'completed').length === 0 && !loading"
             class="h-full flex items-center justify-center opacity-20 italic text-xs">
          暂无任务
        </div>
      </div>

      <!-- 第三、四象限的头部（放在任务列表下方） -->
      <div v-if="['urgent-not-important', 'not-urgent-not-important'].includes(quad.id)"
           :class="['quadrant-bottom-header', getTextPositionClass(quad.id)]">
        <div class="header-content">
          <h2 class="font-black text-slate-800 flex items-center gap-2">
            <span class="w-2 h-6" :class="quad.accentColor"></span>
            {{ quad.title }}
          </h2>
          <p class="text-[10px] text-slate-500 font-bold uppercase mt-1">{{ quad.desc }}</p>
        </div>
        <button
            @click="openAddModal(quad.id)"
            class="w-8 h-8 rounded-full bg-white flex items-center justify-center shadow-sm hover:scale-110 active:scale-95 transition-transform"
        >
          <span class="text-xl font-bold">+</span>
        </button>
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
.quadrant-container {
  height: 100%;
  display: grid;
  grid-template-columns: 1fr 1fr;
  grid-template-rows: 1fr 1fr;
  gap: 12px;
  position: relative;
  align-items: stretch;
  justify-items: stretch;
}

.drop-zone-active {
  filter: brightness(0.9);
  outline: 2px dashed #6366f1;
}

.finish-center {
  position: absolute;
  top: calc(50% - 48px); /* 减去按钮高度的一半 */
  left: calc(50% - 48px); /* 减去按钮宽度的一半 */
  z-index: 60; /* 确保在缺口之上 */
  transform-origin: center;
}

.task-card {
  cursor: grab;
  transition: transform 0.1s, box-shadow 0.1s;
  margin: 0.5rem;
}

.task-card:active {
  cursor: grabbing;
}

.dragging {
  opacity: 0.5;
  transform: scale(0.95);
}

/* 象限缺口样式 */
.quadrant-item {
  position: relative;
}

/* 使用不同的圆角组合创建缺口效果 */
.quadrant-top-left {
  border-radius: 12px 12px 0 12px;
}

.quadrant-top-right {
  border-radius: 12px 12px 12px 0;
}

.quadrant-bottom-left {
  border-radius: 12px 0 12px 12px;
  padding-top: 3rem;
}

.quadrant-bottom-right {
  border-radius: 0 12px 12px 12px;
  padding-top: 3rem;
}

/* 象限头部文字定位 */
.quadrant-header {
  display: flex;
  margin-bottom: 1rem;
  position: relative;
}

.text-top-left .header-content {
  margin-right: auto;
}

.text-top-right .header-content {
  order: 2;
  margin-left: auto;
}

.text-top-right button {
  order: 1;
  margin-right: 0.5rem;
}

.text-bottom-left {
  display: flex;
  justify-content: flex-start;
  align-items: flex-end;
  margin-top: auto;
}

.text-bottom-left .header-content {
  text-align: left;
}

.text-bottom-left button {
  margin-left: auto;
}

.text-bottom-right {
  display: flex;
  justify-content: flex-end;
  align-items: flex-end;
  margin-top: auto;
}

.text-bottom-right .header-content {
  text-align: right;
  margin-left: auto;
}

.text-bottom-right button {
  order: -1;
  margin-right: 0.5rem;
}

/* 隐藏滚动条 */
.no-scrollbar::-webkit-scrollbar {
  display: none;
}

.no-scrollbar {
  -ms-overflow-style: none;
  scrollbar-width: none;
}
</style>