<template>
  <div class="tags-container">
    <!-- 页面头部 -->
    <div class="tags-header">
      <h2>标签管理</h2>
      <el-button type="primary" @click="showAddTagDialog">
        <el-icon>
          <plus/>
        </el-icon>
        添加标签
      </el-button>
    </div>

    <!-- 标签列表 -->
    <el-card class="tags-list">
      <template #header>
        <div class="card-header">
          <span>标签列表</span>
          <el-input
              v-model="searchKeyword"
              placeholder="搜索标签"
              style="width: 200px;"
              clearable
          >
            <template #prefix>
              <el-icon>
                <search/>
              </el-icon>
            </template>
          </el-input>
        </div>
      </template>

      <el-table :data="filteredTags" style="width: 100%">
        <el-table-column prop="name" label="标签名称" min-width="200" width ="600">
          <template #default="{ row }">
            <el-tag :type="getTagType(row.name)">{{ row.name }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="createdAt" label="创建时间" width="280">
          <template #default="{ row }">
            {{ formatDate(row.createdAt) }}
          </template>
        </el-table-column>
        <el-table-column prop="taskCount" label="任务数量" width="200">
          <template #default="{ row }">
            <el-badge :value="row.taskCount" :max="99"/>
          </template>
        </el-table-column>
        <el-table-column label="操作" width="200" fixed="right">
          <template #default="{ row }">
            <el-button
                size="small"
                text
                @click="editTag(row)"
            >
              编辑
            </el-button>
            <el-button
                size="small"
                text
                type="danger"
                @click="deleteTag(row.id)"
            >
              删除
            </el-button>
          </template>
        </el-table-column>
      </el-table>
    </el-card>

    <!-- 添加/编辑标签对话框 -->
    <el-dialog
        v-model="tagDialog.visible"
        :title="tagDialog.isEdit ? '编辑标签' : '添加标签'"
        width="400px"
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
          <el-color-picker v-model="tagForm.color"/>
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
import {Plus, Search} from '@element-plus/icons-vue';
import {ElMessage, ElMessageBox} from 'element-plus';
import {useTagStore} from '@/stores/tag';

interface Tag {
  id: number
  name: string
  color?: string
  userId: number
  createdAt: string
  updatedAt: string
  taskCount: number  // 后端现在返回这个字段
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
        name: tagForm.name,
        color: tagForm.color
      });
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
</script>

<style scoped>
.tags-container {
  height: 100%;
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.tags-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.tags-list {
  flex: 1;
}

/* 标签样式 */
:deep(.el-tag) {
  margin-right: 8px;
  margin-bottom: 4px;
}
</style>