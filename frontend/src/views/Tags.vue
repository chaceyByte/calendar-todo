<template>
  <div class="tags-container">
    <!-- 页面头部 -->
    <div class="tags-header">
      <div class="header-content">
        <h1 class="page-title">标签管理</h1>
        <p class="page-subtitle">高效管理您的任务标签，让工作更有条理</p>
      </div>
      <el-button type="primary" class="add-btn" @click="showAddTagDialog">
        <el-icon>
          <plus/>
        </el-icon>
        添加标签
      </el-button>
    </div>

    <!-- 搜索区域 -->
    <div class="search-section">
      <el-input
          v-model="searchKeyword"
          placeholder="搜索标签名称..."
          class="search-input"
          clearable
      >
        <template #prefix>
          <el-icon>
            <search/>
          </el-icon>
        </template>
      </el-input>
    </div>

    <!-- 标签卡片网格 -->
    <div class="tags-grid">
      <div
          v-for="tag in filteredTags"
          :key="tag.id"
          class="tag-card"
          :style="{ borderLeftColor: tag.color || '#7C3AED' }"
      >
        <div class="tag-content">
          <div class="tag-main">
            <div class="tag-name-wrapper">
              <div class="tag-color-dot" :style="{ backgroundColor: tag.color || '#7C3AED' }"></div>
              <span class="tag-name">{{ tag.name }}</span>
            </div>
            <div class="tag-stats">
              <div class="task-count">
                <span class="count-number">{{ tag.taskCount || 0 }}</span>
                <span class="count-label">任务</span>
              </div>
            </div>
          </div>
          
          <div class="tag-meta">
            <div class="create-time">
              <el-icon><calendar /></el-icon>
              <span>{{ formatDate(tag.createdAt) }}</span>
            </div>
          </div>
        </div>
        
        <div class="tag-actions">
          <el-button
              size="small"
              text
              class="action-btn"
              @click="editTag(tag)"
          >
            <el-icon><edit /></el-icon>
            编辑
          </el-button>
          <el-button
              size="small"
              text
              type="danger"
              class="action-btn"
              @click="deleteTag(tag.id)"
          >
            <el-icon><delete /></el-icon>
            删除
          </el-button>
        </div>
      </div>

      <!-- 空状态 -->
      <div v-if="filteredTags.length === 0" class="empty-state">
        <div class="empty-icon">
          <el-icon><price-tag /></el-icon>
        </div>
        <h3 class="empty-title">暂无标签</h3>
        <p class="empty-description">创建您的第一个标签来更好地组织任务</p>
        <el-button type="primary" @click="showAddTagDialog">
          创建标签
        </el-button>
      </div>
    </div>

    <!-- 添加/编辑标签对话框 -->
    <el-dialog
        v-model="tagDialog.visible"
        :title="tagDialog.isEdit ? '编辑标签' : '创建新标签'"
        width="500px"
        class="tag-dialog"
    >
      <el-form :model="tagForm" label-width="80px">
        <el-form-item label="标签名称">
          <el-input
              v-model="tagForm.name"
              placeholder="请输入标签名称"
              maxlength="20"
              show-word-limit
          />
        </el-form-item>
        <el-form-item label="标签颜色">
          <div class="color-picker-wrapper">
            <el-color-picker v-model="tagForm.color" show-alpha :predefine="predefineColors"/>
            <div class="color-presets">
              <div
                  v-for="color in colorPresets"
                  :key="color"
                  class="color-preset"
                  :style="{ backgroundColor: color }"
                  @click="tagForm.color = color"
              ></div>
            </div>
          </div>
        </el-form-item>
      </el-form>

      <template #footer>
        <el-button @click="tagDialog.visible = false">取消</el-button>
        <el-button type="primary" @click="saveTag">保存</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import {computed, onMounted, reactive, ref} from 'vue';
import dayjs from 'dayjs';
import {Plus, Search, Edit, Delete, Calendar, PriceTag} from '@element-plus/icons-vue';
import {ElMessage, ElMessageBox} from 'element-plus';
import {useTagStore} from '@/stores/tag';

interface Tag {
  id: number
  name: string
  color?: string
  userId?: number
  createdAt?: string
  updatedAt?: string
  taskCount?: number  // 后端现在返回这个字段
}

const tagStore = useTagStore();
const tags = ref<Tag[]>([]);

onMounted(async () => {
  tags.value = await tagStore.fetchTags();
});

const searchKeyword = ref('')

const tagDialog = reactive({
  visible: false,
  isEdit: false
})

const tagForm = reactive({
  id: 0,
  name: '',
  color: '#409eff'
})

const filteredTags = computed(() => {
  if (!searchKeyword.value.trim()) {
    return tags.value
  }

  const keyword = searchKeyword.value.toLowerCase()
  return tags.value.filter(tag =>
      tag.name.toLowerCase().includes(keyword)
  )
})

const getTagType = (name: string) => {
  const types = ['', 'success', 'warning', 'danger', 'info']
  const index = name.length % types.length
  return types[index]
}

const showAddTagDialog = () => {
  tagDialog.isEdit = false
  tagDialog.visible = true

  // 重置表单
  Object.assign(tagForm, {
    id: 0,
    name: '',
    color: '#409eff'
  })
}

const editTag = (tag: Tag) => {
  tagDialog.isEdit = true
  tagDialog.visible = true
  Object.assign(tagForm, {...tag})
}

const saveTag = async () => {
  if (!tagForm.name.trim()) {
    ElMessage.warning('请输入标签名称')
    return
  }

  try {
    if (tagDialog.isEdit) {
      await tagStore.updateTag(tagForm.id, {
        name: tagForm.name,
        color: tagForm.color
      });
      ElMessage.success('标签更新成功');
    } else {
      await tagStore.addTag({
        id: 0, // 临时ID，后端会生成新的
        name: tagForm.name,
        color: tagForm.color
      } as any);
      ElMessage.success('标签添加成功');
    }
    tags.value = await tagStore.fetchTags();
    tagDialog.visible = false;
  } catch (error) {
    console.error('操作失败:', error);
    ElMessage.error('操作失败');
  }
}

const deleteTag = async (tagId: number) => {
  try {
    await ElMessageBox.confirm(
      '确定要删除这个标签吗？', 
      '确认删除',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
      }
    );
    await tagStore.deleteTag(tagId);
    tags.value = await tagStore.fetchTags();
    ElMessage.success('删除成功');
  } catch (error) {
    console.error('删除失败:', error);
    ElMessage.error('删除失败');
  }
}

const formatDate = (date: string) => {
  return dayjs(date).format('YYYY-MM-DD HH:mm')
}

// 颜色预设
const colorPresets = [
  '#7C3AED', '#A78BFA', '#F97316', '#10B981', 
  '#3B82F6', '#EF4444', '#F59E0B', '#8B5CF6'
]

const predefineColors = [
  '#7C3AED', '#A78BFA', '#F97316', '#10B981', 
  '#3B82F6', '#EF4444', '#F59E0B', '#8B5CF6',
  '#FF6B6B', '#4ECDC4', '#45B7D1', '#96CEB4',
  '#FECA57', '#FF9FF3', '#54A0FF', '#5F27CD'
]
</script>

<style scoped>
.tags-container {
  min-height: 100vh;
  background: linear-gradient(135deg, #FAF5FF 0%, #F3E8FF 100%);
  padding: 32px;
  overflow-x: hidden;
}

/* 页面头部 */
.tags-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-end;
  margin-bottom: 32px;
}

.header-content {
  flex: 1;
}

.page-title {
  font-size: 36px;
  font-weight: 700;
  color: #4C1D95;
  margin: 0 0 8px 0;
  letter-spacing: -0.02em;
}

.page-subtitle {
  font-size: 16px;
  color: #7C3AED;
  margin: 0;
  font-weight: 500;
  opacity: 0.8;
}

.add-btn {
  background: linear-gradient(135deg, #7C3AED 0%, #A78BFA 100%);
  border: none;
  border-radius: 12px;
  padding: 12px 24px;
  font-weight: 600;
  box-shadow: 0 4px 12px rgba(124, 58, 237, 0.3);
  transition: all 0.3s ease;
}

.add-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 20px rgba(124, 58, 237, 0.4);
}

/* 搜索区域 */
.search-section {
  margin-bottom: 24px;
  max-width: 400px;
}

.search-input {
  border-radius: 12px;
  border: 1px solid #E9D5FF;
  background: white;
}

.search-input:deep(.el-input__inner) {
  border: none;
  background: transparent;
}

/* 标签网格 */
.tags-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
  gap: 24px;
  margin-bottom: 40px;
}

/* 标签卡片 */
.tag-card {
  background: white;
  border-radius: 16px;
  border-left: 4px solid;
  padding: 24px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
  transition: all 0.3s ease;
  cursor: pointer;
  position: relative;
  overflow: hidden;
}

.tag-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.12);
}

.tag-card::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 1px;
  background: linear-gradient(90deg, transparent, rgba(124, 58, 237, 0.1), transparent);
}

.tag-content {
  margin-bottom: 20px;
}

.tag-main {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 16px;
}

.tag-name-wrapper {
  display: flex;
  align-items: center;
  gap: 12px;
}

.tag-color-dot {
  width: 16px;
  height: 16px;
  border-radius: 50%;
  flex-shrink: 0;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.tag-name {
  font-size: 18px;
  font-weight: 600;
  color: #1F2937;
  line-height: 1.4;
}

.tag-stats {
  text-align: right;
}

.task-count {
  display: flex;
  flex-direction: column;
  align-items: center;
}

.count-number {
  font-size: 24px;
  font-weight: 700;
  color: #7C3AED;
  line-height: 1;
}

.count-label {
  font-size: 12px;
  color: #6B7280;
  font-weight: 500;
  margin-top: 4px;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

/* 标签元信息 */
.tag-meta {
  display: flex;
  align-items: center;
  gap: 16px;
}

.create-time {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 14px;
  color: #6B7280;
}

.create-time .el-icon {
  font-size: 16px;
  color: #9CA3AF;
}

/* 标签操作 */
.tag-actions {
  display: flex;
  gap: 8px;
  justify-content: flex-end;
  border-top: 1px solid #F3F4F6;
  padding-top: 16px;
}

.action-btn {
  border-radius: 8px;
  padding: 8px 16px;
  font-size: 14px;
  font-weight: 500;
  transition: all 0.2s ease;
}

.action-btn:hover {
  background: rgba(124, 58, 237, 0.05);
}

/* 空状态 */
.empty-state {
  grid-column: 1 / -1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 80px 40px;
  text-align: center;
  background: white;
  border-radius: 16px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
}

.empty-icon {
  font-size: 64px;
  color: #A78BFA;
  margin-bottom: 24px;
  opacity: 0.6;
}

.empty-title {
  font-size: 24px;
  font-weight: 600;
  color: #1F2937;
  margin: 0 0 8px 0;
}

.empty-description {
  font-size: 16px;
  color: #6B7280;
  margin: 0 0 24px 0;
  max-width: 400px;
  line-height: 1.5;
}

/* 对话框样式 */
.tag-dialog:deep(.el-dialog) {
  border-radius: 16px;
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.15);
}

.color-picker-wrapper {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.color-presets {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.color-preset {
  width: 32px;
  height: 32px;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s ease;
  border: 2px solid transparent;
}

.color-preset:hover {
  transform: scale(1.1);
  border-color: #E5E7EB;
}

/* 响应式设计 */
@media (max-width: 1024px) {
  .tags-grid {
    grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
    gap: 20px;
  }
  
  .tags-container {
    padding: 24px;
  }
}

@media (max-width: 768px) {
  .tags-container {
    padding: 20px;
  }
  
  .tags-header {
    flex-direction: column;
    align-items: stretch;
    gap: 16px;
  }
  
  .page-title {
    font-size: 28px;
  }
  
  .tags-grid {
    grid-template-columns: 1fr;
    gap: 16px;
  }
  
  .tag-card {
    padding: 20px;
  }
  
  .tag-name {
    font-size: 16px;
  }
  
  .count-number {
    font-size: 20px;
  }
}

@media (max-width: 480px) {
  .tags-container {
    padding: 16px;
  }
  
  .page-title {
    font-size: 24px;
  }
  
  .tag-card {
    padding: 16px;
  }
  
  .tag-main {
    flex-direction: column;
    align-items: flex-start;
    gap: 12px;
  }
  
  .tag-stats {
    text-align: left;
  }
  
  .task-count {
    flex-direction: row;
    align-items: center;
    gap: 8px;
  }
  
  .count-label {
    margin-top: 0;
  }
}
</style>