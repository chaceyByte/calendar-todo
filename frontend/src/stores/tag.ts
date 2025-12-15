import {defineStore} from 'pinia';
import {ref} from 'vue';
import {
    createTag as apiCreateTag,
    updateTag as apiUpdateTag,
    deleteTag as apiDeleteTag,
    getTags,
    type Tag as ApiTag
} from '@/api/tag';

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
            const newTag = await apiCreateTag(tag);
            tags.value.push(newTag);
        } catch (error) {
            console.error('添加标签失败:', error);
            throw error;
        }
    };

    // 更新标签
    const updateTag = async (id: number, updatedTag: Partial<Tag>) => {
        try {
            const updated = await apiUpdateTag(id, updatedTag as ApiTag);
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
            await apiDeleteTag(id);
            tags.value = tags.value.filter(tag => tag.id !== id);
        } catch (error) {
            console.error('删除标签失败:', error);
            throw error;
        }
    };

    // 获取所有标签
    const fetchTags = async () => {
        try {
            tags.value = await getTags();
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