<script setup lang="ts">
import {ref} from 'vue'

const tasks = ref([
  {id: 1, content: '完成周报 PPT', quadrant: 'urgent-important'},
  {id: 2, content: '制定下季度学习计划', quadrant: 'not-urgent-important'},
  {id: 3, content: '回复社群不紧急消息', quadrant: 'urgent-not-important'},
  {id: 4, content: '清理桌面杂物', quadrant: 'not-urgent-not-important'}
]);

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
];

const draggedTask = ref(null);
const isOverCenter = ref(false);
const showModal = ref(false);
const showSuccess = ref(false);
const currentTargetQuadrant = ref('');
const newTaskContent = ref('');

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

const onDropCenter = () => {
  if (draggedTask.value) {
    tasks.value = tasks.value.filter(t => t.id !== draggedTask.value.id);
    showSuccess.value = true;
    setTimeout(() => showSuccess.value = false, 2000);
  }
  isOverCenter.value = false;
};

const onDropQuadrant = (e, quadrantId) => {
  if (draggedTask.value) {
    const task = tasks.value.find(t => t.id === draggedTask.value.id);
    if (task) task.quadrant = quadrantId;
  }
};

const openAddModal = (quadId) => {
  currentTargetQuadrant.value = quadId;
  showModal.value = true;
};

const addTask = () => {
  if (newTaskContent.value.trim()) {
    tasks.value.push({
      id: Date.now(),
      content: newTaskContent.value,
      quadrant: currentTargetQuadrant.value
    });
    newTaskContent.value = '';
    showModal.value = false;
  }
};

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
        :class="[quad.bgColor, 'quadrant-item', getQuadrantClass(quad.id), 'rounded-xl p-4 flex flex-col border-2 border-transparent transition-all overflow-hidden shadow-sm']"
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
      <div class="flex-1 overflow-y-auto no-scrollbar space-y-2">
        <div
            v-for="(task, index) in tasks.filter(t => t.quadrant === quad.id)"
            :key="task.id"
            draggable="true"
            @dragstart="onDragStart(task)"
            @dragend="onDragEnd"
            class="task-card bg-white p-3 rounded-lg shadow-sm border-l-4"
            :style="{ borderLeftColor: quad.hexColor }"
        >
          <p class="text-sm font-medium text-slate-700 px-5">{{ task.content }}</p>
        </div>

        <!-- 空状态 -->
        <div v-if="tasks.filter(t => t.quadrant === quad.id).length === 0"
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
  <div v-if="showModal"
       class="fixed inset-0 bg-slate-900/60 backdrop-blur-sm z-[100] flex items-center justify-center p-4">
    <div class="bg-white rounded-2xl w-full max-w-md p-6 shadow-2xl">
      <h3 class="text-lg font-bold mb-4">添加新任务</h3>
      <textarea
          v-model="newTaskContent"
          placeholder="输入任务内容..."
          class="w-full h-32 p-4 bg-slate-50 border border-slate-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-indigo-500 mb-6"
      ></textarea>
      <div class="flex gap-3">
        <button @click="showModal = false"
                class="flex-1 py-3 font-bold text-slate-500 hover:bg-slate-100 rounded-xl transition-colors">取消
        </button>
        <button @click="addTask"
                class="flex-1 py-3 font-bold bg-indigo-600 text-white rounded-xl hover:bg-indigo-700 transition-colors shadow-lg shadow-indigo-200">
          创建
        </button>
      </div>
    </div>
  </div>

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
}

.quadrant-bottom-right {
  border-radius: 0 12px 12px 12px;
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