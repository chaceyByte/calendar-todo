import { defineStore } from 'pinia';
import { ref } from 'vue';
import request from '@/utils/request';
import { useActivityStore } from './activity';

export interface Task {
  id: number;
  title: string;
  description?: string;
  status: string; // planning, in-progress, completed, paused
  progress: number;
  priority?: 'low' | 'medium' | 'high';
  startDate?: string;
  endDate?: string;
  tags?: string[];
  createdAt: string;
  updatedAt: string;
  completed: boolean;
}

export const useTaskStore = defineStore('task', () => {
  const tasks = ref<Task[]>([]);
  const activityStore = useActivityStore();

  // 添加任务
  const addTask = async (task: Omit<Task, 'id' | 'createdAt' | 'updatedAt' | 'completed'>) => {
    try {
      const taskData = {
        title: task.title,
        description: task.description || '',
        status: task.status,
        progress: task.progress || 0,
        priority: task.priority || 'medium'
        // 暂时不发送tags字段，因为后端Task实体类不支持
      };
      
      const response = await request.post('/tasks', taskData);
      const newTask = response;
      
      // 添加任务后自动记录创建活动
      try {
        await activityStore.startActivity(newTask.id, 'CREATED', '任务创建');
      } catch (error) {
        console.log('记录创建活动失败:', error);
      }
      tasks.value.push(newTask);
    } catch (error) {
      console.error('添加任务失败:', error);
      throw error;
    }
  };

  // 更新任务
  const updateTask = async (id: number, updatedTask: Partial<Task>) => {
    try {
      const response = await request.put(`/tasks/${id}`, updatedTask);
      const result = response;
      const updated = result.data;
      const taskIndex = tasks.value.findIndex(task => task.id === id);
      if (taskIndex !== -1) {
        tasks.value[taskIndex] = updated;
      }
    } catch (error) {
      console.error('更新任务失败:', error);
      throw error;
    }
  };

  // 删除任务
  const deleteTask = async (id: number) => {
    try {
      const response = await request.delete(`/tasks/${id}`);
      tasks.value = tasks.value.filter(task => task.id !== id);
    } catch (error) {
      console.error('删除任务失败:', error);
      throw error;
    }
  };

  // 获取所有任务
  const fetchTasks = async () => {
    try {
      const response = await request.get('/api/tasks');
      if (response.success === false) throw new Error(response.message || '获取任务失败');
      const result = response;
      tasks.value = result.data || [];
      return tasks.value;
    } catch (error) {
      console.error('获取任务失败:', error);
      throw error;
    }
  };

  // 暂存任务相关功能
  const addToStaging = async (taskId: number) => {
    try {
      const response = await request.post(`/api/tasks/${taskId}/staging`);
      if (response.success === false) throw new Error(response.message || '添加到暂存失败');
      const result = response;
      return result.data;
    } catch (error) {
      console.error('添加到暂存失败:', error);
      throw error;
    }
  };

  const removeFromStaging = async (taskId: number) => {
    try {
      const response = await request.delete(`/api/tasks/${taskId}/staging`);
      if (response.success === false) throw new Error(response.message || '从暂存移除失败');
      const result = response;
      return result.data;
    } catch (error) {
      console.error('从暂存移除失败:', error);
      throw error;
    }
  };

  const fetchStagingTasks = async () => {
    try {
      const response = await request.get('/tasks/staging');
      return response || [];
    } catch (error) {
      console.error('获取暂存任务失败:', error);
      throw error;
    }
  };

  // 暂停任务
  const pauseTask = async (id: number) => {
    try {
      // 先结束当前活动
      try {
        await activityStore.endActivity(id);
      } catch (error) {
        console.log('没有进行中的活动或结束活动失败:', error);
      }
      
      const response = await request.post(`/tasks/${id}/pause`);
      const updated = response;
      const taskIndex = tasks.value.findIndex(task => task.id === id);
      if (taskIndex !== -1) {
        tasks.value[taskIndex] = updated;
      }
      return updated;
    } catch (error) {
      console.error('暂停任务失败:', error);
      throw error;
    }
  };

  // 恢复任务
  const resumeTask = async (id: number) => {
    try {
      const response = await request.post(`/tasks/${id}/resume`);
      const updated = response;
      
      // 恢复任务后开始新的活动
      try {
        await activityStore.startActivity(id, 'RESUMED', '任务恢复');
      } catch (error) {
        console.log('开始恢复活动失败:', error);
      }
      const taskIndex = tasks.value.findIndex(task => task.id === id);
      if (taskIndex !== -1) {
        tasks.value[taskIndex] = updated;
      }
      return updated;
    } catch (error) {
      console.error('恢复任务失败:', error);
      throw error;
    }
  };

  // 更新任务标签
  const updateTaskTags = async (taskId: number, tagIds: number[]) => {
    try {
      const response = await request.put(`/api/tasks/${taskId}/tags`, { tagIds });
      if (response.success === false) throw new Error(response.message || '更新任务标签失败');
      const result = response;
      const updated = result.data;
      const taskIndex = tasks.value.findIndex(task => task.id === taskId);
      if (taskIndex !== -1) {
        tasks.value[taskIndex] = updated;
      }
      return updated;
    } catch (error) {
      console.error('更新任务标签失败:', error);
      throw error;
    }
  };

  // 从任务中移除标签
  const removeTagFromTask = async (taskId: number, tagName: string) => {
    try {
      const response = await request.delete(`/api/tasks/${taskId}/tags/${encodeURIComponent(tagName)}`);
      if (response.success === false) throw new Error(response.message || '移除任务标签失败');
      const result = response;
      const updated = result.data;
      const taskIndex = tasks.value.findIndex(task => task.id === taskId);
      if (taskIndex !== -1) {
        tasks.value[taskIndex] = updated;
      }
      return updated;
    } catch (error) {
      console.error('移除任务标签失败:', error);
      throw error;
    }
  };

  return {
    tasks,
    addTask,
    updateTask,
    deleteTask,
    fetchTasks,
    addToStaging,
    removeFromStaging,
    fetchStagingTasks,
    pauseTask,
    resumeTask,
    updateTaskTags,
    removeTagFromTask,
  };
});