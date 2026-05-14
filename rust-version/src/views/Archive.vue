<template>
  <div class="archive-page">
    <!-- Header Section -->
    <header class="page-header">
      <div class="header-content">
        <h1 class="title">已归档任务</h1>
        <p class="subtitle">回顾您已经完成并归档的所有专注时刻。</p>
      </div>
      <div class="header-actions">
        <div class="search-box">
          <svg class="search-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="11" cy="11" r="8"/>
            <path d="m21 21-4.35-4.35"/>
          </svg>
          <input 
            type="text" 
            v-model="searchKeyword"
            @input="handleSearch"
            placeholder="关键词模糊搜索..." 
            class="search-input" 
          />
        </div>
        <button class="filter-btn" @click="refreshData">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polygon points="22 3 2 3 10 12.46 10 19 14 21 14 12.46 22 3"/>
          </svg>
        </button>
      </div>
    </header>

    <!-- Table Section -->
    <div class="table-section">
      <div class="table-container">
        <!-- Table Header -->
        <div class="table-header">
          <div class="th task-content">任务内容</div>
          <div class="th tags">标签</div>
          <div class="th created-time">创建时间</div>
          <div class="th archived-time">归档时间</div>
          <div class="th work-days">工作天数</div>
          <div class="th actions">操作</div>
        </div>

        <!-- Table Body -->
        <div class="table-body" v-if="!loading">
          <div v-if="tasks.length === 0" class="empty-state">
            <p>暂无已归档任务</p>
          </div>
          <template v-else>
            <div class="table-row" v-for="task in tasks" :key="task.id">
              <div class="td task-content">
                <span class="task-text">{{ task.title }}</span>
              </div>
              <div class="td tags">
                <span 
                  v-for="tag in parseTags(task.tags)" 
                  :key="tag.id"
                  class="tag"
                  :style="{ backgroundColor: tag.color + '20', color: tag.color }"
                >
                  {{ tag.name }}
                </span>
                <span v-if="!task.tags" class="tag-empty">无标签</span>
              </div>
              <div class="td created-time">{{ formatDateTime(task.created_at) }}</div>
              <div class="td archived-time">{{ formatDateTime(task.archived_at) }}</div>
              <div class="td work-days">
                <span class="work-days-value">{{ minutesToDays(task.total_work_duration_minutes).toFixed(1) }} 天</span>
              </div>
              <div class="td actions">
                <button class="action-btn" @click="viewTaskDetail(task)">
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <circle cx="12" cy="12" r="1"/>
                    <circle cx="19" cy="12" r="1"/>
                    <circle cx="5" cy="12" r="1"/>
                  </svg>
                </button>
              </div>
            </div>
          </template>
        </div>

        <!-- Loading State -->
        <div class="loading-state" v-else>
          <p>加载中...</p>
        </div>
      </div>

      <!-- Pagination -->
      <div class="pagination" v-if="!loading && total > 0">
        <span class="pagination-info">显示 {{ paginationInfo }} 共 {{ total }} 条记录</span>
        <div class="pagination-controls">
          <button 
            class="page-btn prev" 
            :disabled="page <= 1"
            @click="changePage(page - 1)"
          >
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <polyline points="15 18 9 12 15 6"/>
            </svg>
          </button>
          <button 
            v-for="p in visiblePages" 
            :key="p"
            :class="['page-btn', { active: p === page }]"
            @click="changePage(p)"
          >
            {{ p }}
          </button>
          <button 
            class="page-btn next" 
            :disabled="page >= totalPages"
            @click="changePage(page + 1)"
          >
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <polyline points="9 18 15 12 9 6"/>
            </svg>
          </button>
        </div>
      </div>
    </div>

    <!-- Stats Grid -->
    <div class="stats-grid">
      <!-- Total Archives -->
      <div class="stat-card">
        <div class="stat-header">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/>
            <polyline points="22 4 12 14.01 9 11.01"/>
          </svg>
          <span class="stat-label">总归档量</span>
        </div>
        <div class="stat-content">
          <span class="stat-value">{{ stats.total_archived }}</span>
          <span class="stat-trend">+{{ monthlyGrowth }}% vs 上月</span>
        </div>
      </div>

      <!-- Average Lifetime -->
      <div class="stat-card">
        <div class="stat-header">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="10"/>
            <polyline points="12 6 12 12 16 14"/>
          </svg>
          <span class="stat-label">平均寿命</span>
        </div>
        <div class="stat-content">
          <span class="stat-value">{{ stats.average_lifetime_days.toFixed(1) }} 天</span>
          <span class="stat-desc">任务转化速度</span>
        </div>
      </div>

      <!-- AI Insights Card -->
      <div class="ai-card">
        <div class="ai-icon">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M12 2a10 10 0 1 0 10 10A10 10 0 0 0 12 2z"/>
            <path d="M12 16v-4"/>
            <path d="M12 8h.01"/>
          </svg>
        </div>
        <div class="ai-content">
          <h3>生成任务洞察报告</h3>
          <p class="ai-description">
            利用 AI 深度分析您的已完成任务，发现提升效率的关键路径。
          </p>
          <button class="ai-btn" @click="generateAIReport">立即开始分析</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { ElMessage } from 'element-plus'
import dayjs from 'dayjs'

// ==================== 类型定义 ====================

interface Tag {
  id: number
  name: string
  color: string
}

interface ArchiveTaskItem {
  id: number
  title: string
  description?: string
  quadrant: number
  created_at: string
  archived_at?: string
  total_work_duration_minutes: number
  tags?: string
}

interface WorkDurationStats {
  total_archived: number
  average_lifetime_days: number
}

interface ArchiveTaskListResponse {
  tasks: ArchiveTaskItem[]
  total: number
  page: number
  page_size: number
  stats: WorkDurationStats
}

// ==================== 响应式状态 ====================

const tasks = ref<ArchiveTaskItem[]>([])
const total = ref(0)
const page = ref(1)
const pageSize = ref(10)
const searchKeyword = ref('')
const loading = ref(false)
const stats = ref<WorkDurationStats>({
  total_archived: 0,
  average_lifetime_days: 0
})
const monthlyGrowth = ref(12) // 模拟数据，后续可从后端获取

// ==================== 计算属性 ====================

const totalPages = computed(() => Math.ceil(total.value / pageSize.value))

const paginationInfo = computed(() => {
  const start = (page.value - 1) * pageSize.value + 1
  const end = Math.min(page.value * pageSize.value, total.value)
  return `${start}-${end}`
})

const visiblePages = computed(() => {
  const pages: number[] = []
  const maxVisible = 5
  let start = Math.max(1, page.value - Math.floor(maxVisible / 2))
  let end = Math.min(totalPages.value, start + maxVisible - 1)
  
  if (end - start + 1 < maxVisible) {
    start = Math.max(1, end - maxVisible + 1)
  }
  
  for (let i = start; i <= end; i++) {
    pages.push(i)
  }
  return pages
})

// ==================== 方法 ====================

// 获取已归档任务列表
async function fetchArchivedTasks() {
  loading.value = true
  try {
    const response = await invoke<ArchiveTaskListResponse>('get_archived_tasks', {
      page: page.value,
      pageSize: pageSize.value,
      keyword: searchKeyword.value || null
    })
    
    tasks.value = response.tasks
    total.value = response.total
    stats.value = response.stats
  } catch (error) {
    console.error('获取归档任务失败:', error)
    ElMessage.error('获取归档任务失败')
  } finally {
    loading.value = false
  }
}

// 解析标签 JSON
function parseTags(tagsJson?: string): Tag[] {
  if (!tagsJson) return []
  try {
    return JSON.parse(`[${tagsJson}]`)
  } catch {
    return []
  }
}

// 格式化日期时间
function formatDateTime(dateStr?: string): string {
  if (!dateStr) return '-'
  // 数据库时间统一为 UTC，无时区标记时补 Z 以确保 dayjs 正确转换为本地时间
  const normalized = dateStr.endsWith('Z') || dateStr.includes('+') ? dateStr : dateStr + 'Z'
  return dayjs(normalized).format('YYYY-MM-DD HH:mm')
}

// 将分钟转换为工作天数（8小时/天）
function minutesToDays(minutes: number): number {
  return minutes / 480 // 480分钟 = 8小时
}

// 搜索处理
let searchTimeout: number | null = null
function handleSearch() {
  if (searchTimeout) {
    clearTimeout(searchTimeout)
  }
  searchTimeout = window.setTimeout(() => {
    page.value = 1
    fetchArchivedTasks()
  }, 300)
}

// 切换页码
function changePage(newPage: number) {
  if (newPage < 1 || newPage > totalPages.value) return
  page.value = newPage
  fetchArchivedTasks()
}

// 刷新数据
function refreshData() {
  page.value = 1
  fetchArchivedTasks()
}

// 查看任务详情
function viewTaskDetail(task: ArchiveTaskItem) {
  // TODO: 实现查看详情功能
  console.log('查看任务详情:', task)
}

// 生成 AI 报告
function generateAIReport() {
  ElMessage.info('AI 分析功能开发中...')
}

// ==================== 生命周期 ====================

onMounted(() => {
  fetchArchivedTasks()
})
</script>

<style scoped>
.archive-page {
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

.header-actions {
  display: flex;
  gap: 12px;
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
  border: 1px solid #e8ecf1;
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

.filter-btn {
  width: 40px;
  height: 40px;
  background: var(--bg-card);
  border: 1px solid #e8ecf1;
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.2s ease;
}

.filter-btn:hover {
  background: var(--bg-page);
  border-color: #d0d7de;
}

.filter-btn svg {
  width: 18px;
  height: 18px;
  color: var(--text-secondary);
}

/* Table Section */
.table-section {
  background: var(--bg-card);
  border-radius: 16px;
  padding: 24px;
  margin-bottom: 24px;
  box-shadow: 0 1px 3px var(--bg-hover);
}

.table-container {
  overflow-x: auto;
}

.table-header {
  display: grid;
  grid-template-columns: 2fr 1.5fr 1fr 1fr 100px 80px;
  gap: 16px;
  padding: 16px;
  background: var(--bg-page);
  border-radius: 10px;
  margin-bottom: 8px;
}

.th {
  font-size: 12px;
  font-weight: 600;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.table-body {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.table-row {
  display: grid;
  grid-template-columns: 2fr 1.5fr 1fr 1fr 100px 80px;
  gap: 16px;
  padding: 16px;
  background: var(--bg-card);
  border: 1px solid #e8ecf1;
  border-radius: 10px;
  transition: all 0.2s ease;
}

.table-row:hover {
  background: var(--bg-page);
  border-color: #d0d7de;
}

.td {
  display: flex;
  align-items: center;
  font-size: 14px;
  color: var(--text-primary);
}

.task-text {
  font-weight: 500;
}

.tags {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.tag {
  padding: 4px 10px;
  border-radius: 6px;
  font-size: 12px;
  font-weight: 500;
}

.tag-empty {
  padding: 4px 10px;
  border-radius: 6px;
  font-size: 12px;
  color: var(--text-secondary);
  font-style: italic;
}

.created-time,
.archived-time {
  color: var(--text-secondary);
  font-size: 13px;
}

.work-days-value {
  font-weight: 600;
  color: var(--color-primary);
}

.action-btn {
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

.action-btn:hover {
  background: var(--bg-input);
}

.action-btn svg {
  width: 18px;
  height: 18px;
  color: var(--text-secondary);
}

.empty-state,
.loading-state {
  padding: 48px;
  text-align: center;
  color: var(--text-secondary);
}

/* Pagination */
.pagination {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: 24px;
  padding-top: 24px;
  border-top: 1px solid #e8ecf1;
}

.pagination-info {
  font-size: 13px;
  color: var(--text-secondary);
}

.pagination-controls {
  display: flex;
  align-items: center;
  gap: 8px;
}

.page-btn {
  min-width: 36px;
  height: 36px;
  padding: 0 12px;
  background: var(--bg-card);
  border: 1px solid #e8ecf1;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.2s ease;
  display: flex;
  align-items: center;
  justify-content: center;
}

.page-btn:hover:not(:disabled) {
  background: var(--bg-page);
  border-color: #d0d7de;
}

.page-btn.active {
  background: var(--color-primary);
  border-color: var(--color-primary);
  color: var(--bg-card);
}

.page-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.page-btn svg {
  width: 16px;
  height: 16px;
}

/* Stats Grid */
.stats-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 24px;
}

.stat-card {
  background: var(--bg-card);
  border-radius: 16px;
  padding: 24px;
  box-shadow: 0 1px 3px var(--bg-hover);
}

.stat-header {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 16px;
}

.stat-header svg {
  width: 20px;
  height: 20px;
  color: var(--color-primary);
}

.stat-label {
  font-size: 13px;
  color: var(--text-secondary);
  font-weight: 500;
}

.stat-content {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.stat-value {
  font-size: 32px;
  font-weight: 700;
  color: var(--text-primary);
}

.stat-trend {
  font-size: 12px;
  color: var(--color-success);
  font-weight: 500;
}

.stat-desc {
  font-size: 12px;
  color: var(--text-secondary);
}

/* AI Card */
.ai-card {
  background: linear-gradient(135deg, var(--text-primary) 0%, #2d2d44 100%);
  border-radius: 16px;
  padding: 24px;
  display: flex;
  gap: 20px;
  position: relative;
  overflow: hidden;
}

.ai-card::before {
  content: '';
  position: absolute;
  top: 0;
  right: 0;
  width: 200px;
  height: 200px;
  background: radial-gradient(circle, rgba(255, 255, 255, 0.1), transparent);
  border-radius: 50%;
  transform: translate(50%, -50%);
}

.ai-icon {
  width: 48px;
  height: 48px;
  background: rgba(255, 255, 255, 0.1);
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.ai-icon svg {
  width: 24px;
  height: 24px;
  color: var(--bg-card);
}

.ai-content {
  flex: 1;
  position: relative;
  z-index: 1;
}

.ai-content h3 {
  font-size: 16px;
  font-weight: 600;
  color: var(--bg-card);
  margin: 0 0 8px 0;
}

.ai-description {
  font-size: 13px;
  color: rgba(255, 255, 255, 0.7);
  line-height: 1.6;
  margin: 0 0 16px 0;
}

.ai-btn {
  padding: 10px 20px;
  background: var(--bg-card);
  color: var(--text-primary);
  border: none;
  border-radius: 8px;
  font-size: 13px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s ease;
}

.ai-btn:hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
}

/* Responsive */
@media (max-width: 1200px) {
  .table-header,
  .table-row {
    grid-template-columns: 2fr 1.5fr 1fr 1fr 100px 80px;
  }
}

@media (max-width: 992px) {
  .stats-grid {
    grid-template-columns: repeat(2, 1fr);
  }
  
  .table-header,
  .table-row {
    grid-template-columns: 2fr 1fr 1fr 80px;
  }
  
  .th.tags,
  .td.tags,
  .th.work-days,
  .td.work-days {
    display: none;
  }
}

@media (max-width: 768px) {
  .archive-page {
    padding: 20px;
  }
  
  .page-header {
    flex-direction: column;
    gap: 16px;
  }
  
  .search-input {
    width: 200px;
  }
  
  .stats-grid {
    grid-template-columns: 1fr;
  }
  
  .table-header,
  .table-row {
    grid-template-columns: 2fr 1fr 80px;
  }
  
  .th.created-time,
  .td.created-time,
  .th.archived-time,
  .td.archived-time {
    display: none;
  }
}
</style>