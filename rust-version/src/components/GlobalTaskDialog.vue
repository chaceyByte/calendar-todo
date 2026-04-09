<template>
  <!-- Create/Edit Task Dialog -->
  <div v-if="showDialog" class="dialog-overlay" @click.self="closeDialog">
    <div class="dialog-content">
      <div class="dialog-header">
        <h3>{{ isEditing ? '编辑任务' : '新建任务' }}</h3>
        <button class="close-btn" @click="closeDialog">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M18 6L6 18M6 6l12 12" />
          </svg>
        </button>
      </div>
      <div class="dialog-body">
        <div class="form-group">
          <label>任务标题</label>
          <input v-model="taskForm.title" type="text" placeholder="输入任务标题..." class="form-input" />
        </div>
        <div class="form-group">
          <label>任务描述</label>
          <textarea v-model="taskForm.description" placeholder="输入任务描述..." class="form-textarea" rows="3"></textarea>
        </div>
        <!-- 标签选择器 -->
        <div class="form-group">
          <label>标签</label>
          <div class="tag-selector" ref="tagSelectorRef">
            <!-- 已选标签展示 -->
            <div class="selected-tags">
              <span v-for="tag in selectedTags" :key="tag.id" class="selected-tag"
                :style="{ background: tag.color + '20', color: tag.color, borderColor: tag.color }">
                {{ tag.name }}
                <button class="remove-tag-btn" @click="removeTag(tag)" :style="{ color: tag.color }">
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M18 6L6 18M6 6l12 12" />
                  </svg>
                </button>
              </span>
              <span v-if="selectedTags.length === 0" class="no-tags-hint">未选择标签</span>
            </div>
            <!-- 标签搜索输入框 -->
            <div class="tag-search-wrapper">
              <input v-model="tagSearchQuery" type="text" placeholder="搜索标签..." class="tag-search-input"
                @focus="showTagDropdown = true" @keydown.esc="showTagDropdown = false" />
              <svg class="tag-search-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <circle cx="11" cy="11" r="8" />
                <path d="m21 21-4.35-4.35" />
              </svg>
              <!-- 标签下拉列表 -->
              <div v-if="showTagDropdown" class="tag-dropdown">
                <div v-if="filteredAvailableTags.length === 0" class="tag-dropdown-empty">
                  {{ tagSearchQuery ? '未找到匹配的标签' : '暂无可用标签' }}
                </div>
                <div v-else class="tag-dropdown-list">
                  <div v-for="tag in filteredAvailableTags" :key="tag.id" class="tag-option"
                    :class="{ selected: isTagSelected(tag.id) }" @click="toggleTag(tag)" @mousedown.prevent>
                    <span class="tag-color-dot" :style="{ background: tag.color }"></span>
                    <span class="tag-name">{{ tag.name }}</span>
                    <span v-if="isTagSelected(tag.id)" class="tag-check-icon">
                      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <polyline points="20 6 9 17 4 12" />
                      </svg>
                    </span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
        <div class="form-row">
          <div class="form-group">
            <label>状态</label>
            <select v-model="taskForm.status" class="form-select">
              <option value="planning">规划中</option>
              <option value="in_progress">进行中</option>
              <option value="paused">已暂停</option>
              <option value="completed">已完成</option>
              <option value="archived">已归档</option>
            </select>
          </div>
        </div>
        <!-- 四象限选择 -->
        <div class="form-group">
          <label>四象限分类</label>
          <div class="quadrant-selector">
            <div class="quadrant-option" :class="{ active: taskForm.isImportant && taskForm.isUrgent }"
              @click="setQuadrant(true, true)">
              <div class="quadrant-dot important-urgent"></div>
              <span>重要且紧急</span>
            </div>
            <div class="quadrant-option" :class="{ active: taskForm.isImportant && !taskForm.isUrgent }"
              @click="setQuadrant(true, false)">
              <div class="quadrant-dot important"></div>
              <span>重要不紧急</span>
            </div>
            <div class="quadrant-option" :class="{ active: !taskForm.isImportant && taskForm.isUrgent }"
              @click="setQuadrant(false, true)">
              <div class="quadrant-dot urgent"></div>
              <span>紧急不重要</span>
            </div>
            <div class="quadrant-option" :class="{ active: !taskForm.isImportant && !taskForm.isUrgent }"
              @click="setQuadrant(false, false)">
              <div class="quadrant-dot normal"></div>
              <span>不重要不紧急</span>
            </div>
          </div>
        </div>
        <div class="form-group">
          <label>进度 ({{ taskForm.progress }}%)</label>
          <input v-model.number="taskForm.progress" type="range" min="0" max="100" class="form-range" />
        </div>
        <div class="form-row">
          <div class="form-group">
            <label>开始时间</label>
            <input v-model="taskForm.startAt" type="datetime-local" class="form-input" />
          </div>
          <div class="form-group">
            <label>截止时间</label>
            <input v-model="taskForm.dueAt" type="datetime-local" class="form-input" />
          </div>
        </div>
      </div>
      <div class="dialog-footer">
        <button v-if="isEditing" class="btn-danger" @click="deleteCurrentTask">删除</button>
        <div class="dialog-actions">
          <button class="btn-secondary" @click="closeDialog">取消</button>
          <button class="btn-primary" @click="saveTask">保存</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import dayjs from 'dayjs'
import { newTaskEvent } from '../utils/eventBus'

interface Tag {
  id: number
  name: string
  color: string
}

interface TagWithCount extends Tag {
  taskCount: number
}

const showDialog = ref(false)
const isEditing = ref(false)
const currentTaskId = ref<number | null>(null)

// 标签选择器相关
const allTags = ref<TagWithCount[]>([])
const selectedTags = ref<Tag[]>([])
const originalTagIds = ref<number[]>([])
const tagSearchQuery = ref('')
const showTagDropdown = ref(false)
const tagSelectorRef = ref<HTMLElement | null>(null)

const taskForm = ref({
  title: '',
  description: '',
  status: 'planning',
  isImportant: false,
  isUrgent: false,
  progress: 0,
  startAt: '',
  dueAt: ''
})

// 标签选择器计算属性
const filteredAvailableTags = computed(() => {
  if (!tagSearchQuery.value.trim()) {
    return allTags.value
  }
  const query = tagSearchQuery.value.toLowerCase()
  return allTags.value.filter(tag => tag.name.toLowerCase().includes(query))
})

// 加载标签
const loadTags = async () => {
  try {
    const result = await invoke<TagWithCount[]>('get_tags')
    allTags.value = result
  } catch (error) {
    console.error('加载标签失败:', error)
  }
}

const isTagSelected = (tagId: number) => {
  return selectedTags.value.some(tag => tag.id === tagId)
}

const toggleTag = (tag: TagWithCount) => {
  const index = selectedTags.value.findIndex(t => t.id === tag.id)
  if (index === -1) {
    selectedTags.value.push({
      id: tag.id,
      name: tag.name,
      color: tag.color
    })
  } else {
    selectedTags.value.splice(index, 1)
  }
}

const removeTag = (tag: Tag) => {
  const index = selectedTags.value.findIndex(t => t.id === tag.id)
  if (index !== -1) {
    selectedTags.value.splice(index, 1)
  }
}

// 点击外部关闭标签下拉列表
const handleClickOutside = (event: MouseEvent) => {
  if (tagSelectorRef.value && !tagSelectorRef.value.contains(event.target as Node)) {
    showTagDropdown.value = false
  }
}

// 打开创建对话框
const openCreateDialog = () => {
  isEditing.value = false
  currentTaskId.value = null
  selectedTags.value = []
  originalTagIds.value = []
  tagSearchQuery.value = ''
  showTagDropdown.value = false
  taskForm.value = {
    title: '',
    description: '',
    status: 'planning',
    isImportant: false,
    isUrgent: false,
    progress: 0,
    startAt: '',
    dueAt: ''
  }
  showDialog.value = true
}

// 关闭对话框
const closeDialog = () => {
  showDialog.value = false
}

const setQuadrant = (isImportant: boolean, isUrgent: boolean) => {
  taskForm.value.isImportant = isImportant
  taskForm.value.isUrgent = isUrgent
}

const saveTask = async () => {
  try {
    if (isEditing.value && currentTaskId.value) {
      // 计算需要添加和删除的标签
      const currentTagIds = selectedTags.value.map(t => t.id)
      const addTagIds = currentTagIds.filter(id => !originalTagIds.value.includes(id))
      const removeTagIds = originalTagIds.value.filter(id => !currentTagIds.includes(id))

      await invoke('update_task', {
        request: {
          id: currentTaskId.value,
          title: taskForm.value.title || undefined,
          description: taskForm.value.description || undefined,
          status: taskForm.value.status,
          isImportant: taskForm.value.isImportant,
          isUrgent: taskForm.value.isUrgent,
          progress: taskForm.value.progress,
          startAt: taskForm.value.startAt || undefined,
          dueAt: taskForm.value.dueAt || undefined,
          archived: taskForm.value.status === 'archived',
          addTagIds: addTagIds.length > 0 ? addTagIds : undefined,
          removeTagIds: removeTagIds.length > 0 ? removeTagIds : undefined
        }
      })
    } else {
      await invoke('create_task', {
        request: {
          title: taskForm.value.title,
          description: taskForm.value.description || undefined,
          status: taskForm.value.status,
          isImportant: taskForm.value.isImportant,
          isUrgent: taskForm.value.isUrgent,
          startAt: taskForm.value.startAt || undefined,
          dueAt: taskForm.value.dueAt || undefined,
          tagIds: selectedTags.value.map(t => t.id)
        }
      })
    }
    
    // 发送任务已保存事件，通知各页面刷新
    window.dispatchEvent(new CustomEvent('task-saved'))
    
    closeDialog()
  } catch (error) {
    console.error('保存任务失败:', error)
    alert('保存任务失败：' + error)
  }
}

const deleteCurrentTask = async () => {
  if (!currentTaskId.value) return
  if (!confirm('确定要删除这个任务吗？')) return

  try {
    await invoke('delete_task', { taskId: currentTaskId.value })
    
    // 发送任务已删除事件
    window.dispatchEvent(new CustomEvent('task-saved'))
    
    closeDialog()
  } catch (error) {
    console.error('删除任务失败:', error)
    alert('删除任务失败：' + error)
  }
}

// 监听新任务事件
watch(newTaskEvent, () => {
  openCreateDialog()
})

// 监听编辑任务事件
const editTaskEvent = ref(0)
const taskToEdit = ref<any>(null)

// 暴露给全局使用
;(window as any).openEditTaskDialog = (task: any) => {
  taskToEdit.value = task
  editTaskEvent.value++
}

watch(editTaskEvent, () => {
  if (taskToEdit.value) {
    openEditDialog(taskToEdit.value)
    taskToEdit.value = null
  }
})

// 监听下拉列表显示状态，控制点击外部事件监听
watch(showTagDropdown, (isOpen) => {
  if (isOpen) {
    setTimeout(() => {
      document.addEventListener('click', handleClickOutside)
    }, 0)
  } else {
    document.removeEventListener('click', handleClickOutside)
  }
})

onMounted(() => {
  loadTags()
})

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside)
})
</script>

<style scoped>
.dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.dialog-content {
  background: var(--bg-card);
  border-radius: 16px;
  width: 480px;
  max-height: 90vh;
  overflow-y: auto;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
}

.dialog-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 24px;
  border-bottom: 1px solid #e8ecf1;
}

.dialog-header h3 {
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
}

.close-btn {
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

.close-btn:hover {
  background: var(--bg-input);
}

.close-btn svg {
  width: 18px;
  height: 18px;
  color: var(--text-secondary);
}

.dialog-body {
  padding: 24px;
}

.form-group {
  margin-bottom: 20px;
}

.form-group label {
  display: block;
  font-size: 13px;
  font-weight: 500;
  color: var(--text-secondary);
  margin-bottom: 8px;
}

.form-input,
.form-textarea,
.form-select {
  width: 100%;
  padding: 10px 14px;
  background: var(--bg-page);
  border: 1px solid #e8ecf1;
  border-radius: 10px;
  font-size: 14px;
  color: var(--text-primary);
  transition: all 0.2s ease;
}

.form-input:focus,
.form-textarea:focus,
.form-select:focus {
  outline: none;
  border-color: var(--color-primary);
  background: var(--bg-card);
}

.form-textarea {
  resize: vertical;
  min-height: 80px;
}

.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
}

.form-range {
  width: 100%;
  height: 6px;
  border-radius: 3px;
  background: #e8ecf1;
  outline: none;
  -webkit-appearance: none;
}

.form-range::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background: var(--color-primary);
  cursor: pointer;
}

.dialog-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 24px;
  border-top: 1px solid #e8ecf1;
}

.dialog-actions {
  display: flex;
  gap: 12px;
}

.btn-primary,
.btn-secondary,
.btn-danger {
  padding: 10px 20px;
  border-radius: 10px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  border: none;
}

.btn-primary {
  background: var(--color-primary);
  color: var(--bg-card);
}

.btn-primary:hover {
  background: var(--color-primary-hover);
}

.btn-secondary {
  background: var(--bg-page);
  color: var(--text-secondary);
  border: 1px solid #e8ecf1;
}

.btn-secondary:hover {
  background: #e8ecf1;
}

.btn-danger {
  background: #fef2f2;
  color: var(--color-error);
}

.btn-danger:hover {
  background: #fee2e2;
}

.quadrant-selector {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 12px;
}

.quadrant-option {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 12px;
  background: var(--bg-page);
  border: 2px solid transparent;
  border-radius: 10px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.quadrant-option:hover {
  background: #e8ecf1;
}

.quadrant-option.active {
  border-color: var(--color-primary);
  background: rgba(0, 102, 204, 0.05);
}

.quadrant-dot {
  width: 12px;
  height: 12px;
  border-radius: 50%;
}

.quadrant-dot.important-urgent {
  background: var(--color-error);
}

.quadrant-dot.important {
  background: var(--color-warning);
}

.quadrant-dot.urgent {
  background: #3b82f6;
}

.quadrant-dot.normal {
  background: var(--color-success);
}

.quadrant-option span {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-secondary);
}

.quadrant-option.active span {
  color: var(--color-primary);
}

/* 标签选择器样式 */
.tag-selector {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.selected-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  min-height: 32px;
}

.selected-tag {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 6px 10px;
  border-radius: 8px;
  font-size: 13px;
  font-weight: 500;
  border: 1px solid;
  transition: all 0.2s ease;
}

.remove-tag-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 16px;
  height: 16px;
  background: transparent;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  padding: 0;
  opacity: 0.7;
  transition: all 0.2s ease;
}

.remove-tag-btn:hover {
  opacity: 1;
  background: rgba(0, 0, 0, 0.1);
}

.remove-tag-btn svg {
  width: 12px;
  height: 12px;
}

.no-tags-hint {
  font-size: 13px;
  color: var(--text-secondary);
  font-style: italic;
}

.tag-search-wrapper {
  position: relative;
}

.tag-search-input {
  width: 100%;
  padding: 10px 14px 10px 40px;
  background: var(--bg-page);
  border: 1px solid #e8ecf1;
  border-radius: 10px;
  font-size: 14px;
  color: var(--text-primary);
  transition: all 0.2s ease;
}

.tag-search-input:focus {
  outline: none;
  border-color: var(--color-primary);
  background: var(--bg-card);
}

.tag-search-input::placeholder {
  color: var(--text-secondary);
}

.tag-search-icon {
  position: absolute;
  left: 12px;
  top: 50%;
  transform: translateY(-50%);
  width: 18px;
  height: 18px;
  color: var(--text-secondary);
  pointer-events: none;
}

.tag-dropdown {
  position: absolute;
  top: 100%;
  left: 0;
  right: 0;
  margin-top: 4px;
  background: var(--bg-card);
  border: 1px solid #e8ecf1;
  border-radius: 10px;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.1);
  max-height: 200px;
  overflow-y: auto;
  z-index: 100;
}

.tag-dropdown-empty {
  padding: 16px;
  text-align: center;
  font-size: 13px;
  color: var(--text-secondary);
}

.tag-dropdown-list {
  padding: 8px;
}

.tag-option {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 12px;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.15s ease;
}

.tag-option:hover {
  background: var(--bg-page);
}

.tag-option.selected {
  background: var(--color-primary-light);
}

.tag-color-dot {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  flex-shrink: 0;
}

.tag-name {
  flex: 1;
  font-size: 14px;
  color: var(--text-primary);
}

.tag-option.selected .tag-name {
  color: var(--color-primary);
  font-weight: 500;
}

.tag-check-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 18px;
  height: 18px;
  color: var(--color-primary);
}

.tag-check-icon svg {
  width: 14px;
  height: 14px;
}
</style>
