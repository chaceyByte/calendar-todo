import { defineStore } from 'pinia';
import { ref } from 'vue';
import request from '@/utils/request';

export interface Tag {
  id: number;
  name: string;
  color?: string;
  userId?: number;
  createdAt?: string;
  updatedAt?: string;
  taskCount?: number;
}

export const useTagStore = defineStore('tag', () => {
  const tags = ref<Tag[]>([]);

  // 添加标签
  const addTag = async (tag: Tag) => {
    try {
      const response = await request.post('/api/tags', tag);
      if (response.success === false) throw new Error(response.message || '添加标签失败');
      const newTag = response.data;
      tags.value.push(newTag);
    } catch (error) {
      console.error('添加标签失败:', error);
      throw error;
    }
  };

  // 更新标签
  const updateTag = async (id: number, updatedTag: Partial<Tag>) => {
    try {
      const response = await request.put(`/api/tags/${id}`, updatedTag);
      if (response.success === false) throw new Error(response.message || '更新标签失败');
      const updated = result.data;
      const tagIndex = tags.value.findIndex(tag => tag.id === id);
      if (tagIndex !== -1) {
        tags.value[tagIndex] = updated;
      }
    } catch (error) {
      console.error('更新标签失败:', error);
      throw error;
    }
  };

  // 删除标签
  const deleteTag = async (id: number) => {
    try {
      const response = await request.delete(`/api/tags/${id}`);
      if (response.success === false) throw new Error(response.message || '删除标签失败');
      tags.value = tags.value.filter(tag => tag.id !== id);
    } catch (error) {
      console.error('删除标签失败:', error);
      throw error;
    }
  };

  // 获取所有标签
  const fetchTags = async () => {
    try {
      const response = await request.get('/api/tags');
      if (response.success === false) throw new Error(response.message || '获取标签失败');
      tags.value = response.data;
      return tags.value;
    } catch (error) {
      console.error('获取标签失败:', error);
      throw error;
    }
  };

  return {
    tags,
    addTag,
    updateTag,
    deleteTag,
    fetchTags,
  };
});