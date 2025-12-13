import {defineStore} from 'pinia';
import {ref} from 'vue';
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
            if ((response as any).success === false) throw new Error((response as any).message || '添加标签失败');
            const newTag = (response as any).data;
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
            const updated = response.data || response;
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
            await request.delete(`/api/tags/${id}`);
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
            if ((response as any).success === false) throw new Error((response as any).message || '获取标签失败');
            tags.value = (response as any).data;
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