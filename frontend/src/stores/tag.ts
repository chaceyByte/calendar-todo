import { defineStore } from 'pinia';
import { ref } from 'vue';

export interface Tag {
  id: number;
  name: string;
  color?: string;
}

export const useTagStore = defineStore('tag', () => {
  const tags = ref<Tag[]>([]);

  // 添加标签
  const addTag = async (tag: Tag) => {
    try {
      const response = await fetch('/api/tags', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(tag),
      });
      if (!response.ok) throw new Error('添加标签失败');
      const newTag = await response.json();
      tags.value.push(newTag);
    } catch (error) {
      console.error('添加标签失败:', error);
      throw error;
    }
  };

  // 更新标签
  const updateTag = async (id: number, updatedTag: Partial<Tag>) => {
    try {
      const response = await fetch(`/api/tags/${id}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(updatedTag),
      });
      if (!response.ok) throw new Error('更新标签失败');
      const updated = await response.json();
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
      const response = await fetch(`/api/tags/${id}`, { method: 'DELETE' });
      if (!response.ok) throw new Error('删除标签失败');
      tags.value = tags.value.filter(tag => tag.id !== id);
    } catch (error) {
      console.error('删除标签失败:', error);
      throw error;
    }
  };

  // 获取所有标签
  const fetchTags = async () => {
    try {
      const response = await fetch('/api/tags');
      if (!response.ok) throw new Error('获取标签失败');
      tags.value = await response.json();
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