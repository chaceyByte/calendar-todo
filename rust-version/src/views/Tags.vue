<template>
  <div class="tags-page">
    <!-- Header Section -->
    <header class="page-header">
      <div class="header-content">
        <h1 class="title">标签管理</h1>
        <p class="subtitle">整理和归类您的认知资产</p>
      </div>
      <button class="create-btn" @click="handleCreateTag">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M12 5v14M5 12h14"/>
        </svg>
        创建新标签
      </button>
    </header>

    <!-- Tags Grid -->
    <div class="tags-grid">
      <!-- Tag Cards -->
      <div
        v-for="tag in tags"
        :key="tag.id"
        class="tag-card"
        :class="tag.badge.toLowerCase()"
        @click="handleEditTag(tag.id)"
      >
        <div class="tag-header">
          <span class="tag-badge" :class="tag.badge.toLowerCase()">{{ tag.badge }}</span>
          <button class="delete-btn" @click.stop="handleDeleteTag(tag.id)">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <polyline points="3 6 5 6 21 6"/>
              <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/>
            </svg>
          </button>
        </div>
        <h3 class="tag-name">{{ tag.name }}</h3>
        <div class="tag-footer">
          <span class="task-count" :class="{ empty: tag.taskCount === 0 }">
            {{ tag.taskCount > 0 ? `${tag.taskCount} 个任务使用中` : '无任务使用 - 可安全删除' }}
          </span>
        </div>
      </div>

      <!-- Add New Tag Card -->
      <div class="tag-card add-new" @click="handleCreateTag">
        <div class="add-icon">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M12 5v14M5 12h14"/>
          </svg>
        </div>
        <p class="add-text">定义新维度</p>
      </div>
    </div>

    <!-- Footer Analytics -->
    <div class="footer-analytics">
      <div class="stats-section">
        <div class="stat-item">
          <div class="stat-label">
            <span class="label-text">标签总数</span>
          </div>
          <div class="stat-value">
            <span class="number">12</span>
          </div>
        </div>
        <div class="stat-item">
          <div class="stat-label">
            <span class="label-text">最活跃标签</span>
          </div>
          <span class="active-tag">深度工作</span>
        </div>
      </div>
      <div class="chart-section">
        <div class="trend-chart">
          <svg viewBox="0 0 400 80" preserveAspectRatio="none">
            <defs>
              <linearGradient id="chartGradient" x1="0%" y1="0%" x2="0%" y2="100%">
                <stop offset="0%" style="stop-color:var(--color-primary);stop-opacity:0.2"/>
                <stop offset="100%" style="stop-color:var(--color-primary);stop-opacity:0"/>
              </linearGradient>
            </defs>
            <path d="M0,60 Q50,50 100,55 T200,35 T300,45 T400,25 L400,80 L0,80 Z" fill="url(#chartGradient)"/>
            <path d="M0,60 Q50,50 100,55 T200,35 T300,45 T400,25" fill="none" stroke="var(--color-primary)" stroke-width="2"/>
          </svg>
        </div>
      </div>
    </div>

    <!-- Create/Edit Tag Modal -->
    <CreateEditTagModal
      v-model:isVisible="isCreateEditModalVisible"
      :is-edit-mode="isEditMode"
      :initial-name="editingTag?.name || ''"
      :initial-color="editingTag?.color || 'var(--color-primary)'"
      @cancel="handleCancelEdit"
      @save="handleSaveTag"
    />

    <!-- Delete Tag Modal -->
    <DeleteTagModal
      v-model:isVisible="isDeleteModalVisible"
      :tag-name="selectedTag?.name || ''"
      :task-count="selectedTag?.taskCount || 0"
      @cancel="handleCancelDelete"
      @view-tasks="handleViewTasks"
    />

    <!-- Confirm Delete Modal -->
    <ConfirmDeleteModal
      v-model:isVisible="isConfirmDeleteVisible"
      :tag-name="selectedTag?.name || ''"
      :task-count="selectedTag?.taskCount || 0"
      @cancel="handleCancelDelete"
      @confirm="handleConfirmDelete"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import CreateEditTagModal from '../components/CreateEditTagModal.vue'
import DeleteTagModal from '../components/DeleteTagModal.vue'
import ConfirmDeleteModal from '../components/ConfirmDeleteModal.vue'

interface Tag {
  id: number
  name: string
  badge: string
  taskCount: number
  color?: string
}

const tags = ref<Tag[]>([])
const isCreateEditModalVisible = ref(false)
const isDeleteModalVisible = ref(false)
const isConfirmDeleteVisible = ref(false)
const isEditMode = ref(false)
const selectedTag = ref<Tag | null>(null)
const editingTag = ref<Tag | null>(null)
const pendingDeleteTagId = ref<number | null>(null)

const fetchTags = async () => {
  try {
    const fetchedTags = await invoke<any[]>('get_tags')
    tags.value = fetchedTags.map((tag: any) => ({
      id: tag.id,
      name: tag.name,
      badge: tag.name.substring(0, 8).toUpperCase(),
      taskCount: tag.task_count || 0,
      color: tag.color || 'var(--color-primary)'
    }))
  } catch (error) {
    console.error('获取标签失败:', error)
  }
}

const handleCreateTag = () => {
  isEditMode.value = false
  editingTag.value = null
  isCreateEditModalVisible.value = true
}

const handleEditTag = (tagId: number) => {
  const tag = tags.value.find(t => t.id === tagId)
  if (tag) {
    isEditMode.value = true
    editingTag.value = tag
    isCreateEditModalVisible.value = true
  }
}

const handleCancelEdit = () => {
  editingTag.value = null
}

const handleSaveTag = async (data: { name: string; color: string }) => {
  try {
    if (isEditMode.value && editingTag.value) {
      await invoke('update_tag', {
        request: {
          id: editingTag.value.id,
          name: data.name,
          color: data.color
        }
      })
    } else {
      await invoke('create_tag', {
        request: {
          name: data.name,
          color: data.color
        }
      })
    }
    await fetchTags()
  } catch (error) {
    console.error('保存标签失败:', error)
  }
}

const handleDeleteTag = async (tagId: number) => {
  const tag = tags.value.find(t => t.id === tagId)
  if (tag) {
    selectedTag.value = tag
    try {
      const usageCount = await invoke<number>('get_tag_usage_count', { tagId })
      if (usageCount > 0) {
        selectedTag.value.taskCount = usageCount
        isDeleteModalVisible.value = true
      } else {
        pendingDeleteTagId.value = tagId
        isConfirmDeleteVisible.value = true
      }
    } catch (error) {
      console.error('删除标签失败:', error)
    }
  }
}

const handleConfirmDelete = async () => {
  if (pendingDeleteTagId.value !== null) {
    try {
      await invoke('delete_tag', { tagId: pendingDeleteTagId.value })
      await fetchTags()
    } catch (error) {
      console.error('删除标签失败:', error)
    } finally {
      pendingDeleteTagId.value = null
    }
  }
}

const handleCancelDelete = () => {
  pendingDeleteTagId.value = null
}

const handleViewTasks = () => {
  if (selectedTag.value) {
    console.log('查看标签关联的任务:', selectedTag.value.name)
  }
}

onMounted(() => {
  fetchTags()
})
</script>

<style scoped>
.tags-page {
  padding: 32px 40px;
  min-height: 100vh;
  background: var(--bg-page);
}

/* Header */
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

.create-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 20px;
  background: linear-gradient(135deg, var(--color-primary) 0%, var(--color-primary-hover) 100%);
  color: var(--bg-card);
  border: none;
  border-radius: 10px;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s ease;
}

.create-btn:hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(0, 113, 227, 0.3);
}

.create-btn svg {
  width: 18px;
  height: 18px;
}

/* Tags Grid */
.tags-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 20px;
  margin-bottom: 24px;
}

.tag-card {
  background: var(--bg-card);
  border-radius: 16px;
  padding: 24px;
  box-shadow: 0 1px 3px var(--bg-hover);
  transition: all 0.2s ease;
  cursor: pointer;
}

.tag-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px var(--border-color);
}

.tag-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.tag-badge {
  padding: 6px 12px;
  border-radius: 8px;
  font-size: 11px;
  font-weight: 700;
  letter-spacing: 0.5px;
}

.tag-badge.urgent {
  background: rgba(239, 68, 68, 0.1);
  color: var(--color-error);
}

.tag-badge.learning {
  background: rgba(59, 130, 246, 0.1);
  color: #3b82f6;
}

.tag-badge.focus {
  background: rgba(139, 92, 246, 0.1);
  color: #8b5cf6;
}

.tag-badge.unused {
  background: rgba(107, 114, 128, 0.1);
  color: var(--text-secondary);
}

.tag-badge.personal {
  background: rgba(16, 185, 129, 0.1);
  color: var(--color-success);
}

.delete-btn {
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

.delete-btn:hover {
  background: rgba(239, 68, 68, 0.1);
}

.delete-btn svg {
  width: 18px;
  height: 18px;
  color: var(--text-secondary);
}

.delete-btn:hover svg {
  color: var(--color-error);
}

.tag-name {
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 16px 0;
}

.tag-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.task-count {
  font-size: 13px;
  color: var(--text-secondary);
}

.task-count.empty {
  color: var(--text-secondary);
  font-style: italic;
}

.avatar-stack {
  display: flex;
  align-items: center;
}

.avatar {
  width: 28px;
  height: 28px;
  border-radius: 50%;
  background: #e8ecf1;
  border: 2px solid var(--bg-card);
}

.avatar.more {
  width: 28px;
  height: 28px;
  border-radius: 50%;
  background: var(--color-primary);
  color: var(--bg-card);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 10px;
  font-weight: 600;
  margin-left: -8px;
}

.progress-dots {
  display: flex;
  gap: 6px;
}

.progress-dots .dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: #e8ecf1;
}

.progress-dots .dot:first-child {
  background: #8b5cf6;
}

/* Add New Card */
.tag-card.add-new {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 140px;
  border: 2px dashed #d0d7de;
  background: transparent;
}

.tag-card.add-new:hover {
  border-color: var(--color-primary);
  background: rgba(0, 102, 204, 0.02);
}

.add-icon {
  width: 48px;
  height: 48px;
  background: var(--bg-input);
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 12px;
  transition: all 0.2s ease;
}

.tag-card.add-new:hover .add-icon {
  background: rgba(0, 102, 204, 0.1);
}

.add-icon svg {
  width: 24px;
  height: 24px;
  color: var(--text-secondary);
}

.tag-card.add-new:hover .add-icon svg {
  color: var(--color-primary);
}

.add-text {
  font-size: 14px;
  color: var(--text-secondary);
  margin: 0;
}

.tag-card.add-new:hover .add-text {
  color: var(--color-primary);
}

/* Footer Analytics */
.footer-analytics {
  background: var(--bg-card);
  border-radius: 16px;
  padding: 24px;
  display: grid;
  grid-template-columns: 1fr 2fr;
  gap: 40px;
  box-shadow: 0 1px 3px var(--bg-hover);
}

.stats-section {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.stat-item {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.stat-label {
  display: flex;
  align-items: center;
  gap: 8px;
}

.label-text {
  font-size: 13px;
  color: var(--text-secondary);
  font-weight: 500;
}

.stat-value .number {
  font-size: 32px;
  font-weight: 700;
  color: var(--text-primary);
}

.active-tag {
  font-size: 16px;
  font-weight: 600;
  color: var(--color-primary);
}

.chart-section {
  display: flex;
  align-items: center;
}

.trend-chart {
  width: 100%;
  height: 80px;
}

.trend-chart svg {
  width: 100%;
  height: 100%;
}
</style>