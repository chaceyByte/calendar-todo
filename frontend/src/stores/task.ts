import { defineStore } from 'pinia';
import { ref } from 'vue';

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
      
      const response = await fetch('/api/tasks', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(taskData),
      });
      if (!response.ok) throw new Error('添加任务失败');
      const result = await response.json();
      const newTask = result.data;
      tasks.value.push(newTask);
    } catch (error) {
      console.error('添加任务失败:', error);
      throw error;
    }
  };

  // 更新任务
  const updateTask = async (id: number, updatedTask: Partial<Task>) => {
    try {
      const response = await fetch(`/api/tasks/${id}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(updatedTask),
      });
      if (!response.ok) throw new Error('更新任务失败');
      const result = await response.json();
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
      const response = await fetch(`/api/tasks/${id}`, { 
        method: 'DELETE'
      });
      if (!response.ok) throw new Error('删除任务失败');
      tasks.value = tasks.value.filter(task => task.id !== id);
    } catch (error) {
      console.error('删除任务失败:', error);
      throw error;
    }
  };

  // 获取所有任务
  const fetchTasks = async () => {
    try {
      const response = await fetch('/api/tasks');
      if (!response.ok) throw new Error('获取任务失败');
      const result = await response.json();
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
      const response = await fetch(`/api/tasks/${taskId}/staging`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' }
      });
      if (!response.ok) throw new Error('添加到暂存失败');
      const result = await response.json();
      return result.data;
    } catch (error) {
      console.error('添加到暂存失败:', error);
      throw error;
    }
  };

  const removeFromStaging = async (taskId: number) => {
    try {
      const response = await fetch(`/api/tasks/${taskId}/staging`, {
        method: 'DELETE'
      });
      if (!response.ok) throw new Error('从暂存移除失败');
      const result = await response.json();
      return result.data;
    } catch (error) {
      console.error('从暂存移除失败:', error);
      throw error;
    }
  };

  const fetchStagingTasks = async () => {
    try {
      const response = await fetch('/api/tasks/staging');
      if (!response.ok) throw new Error('获取暂存任务失败');
      const result = await response.json();
      return result.data || [];
    } catch (error) {
      console.error('获取暂存任务失败:', error);
      throw error;
    }
  };

  // 暂停任务
  const pauseTask = async (id: number) => {
    try {
      const response = await fetch(`/api/tasks/${id}/pause`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' }
      });
      if (!response.ok) throw new Error('暂停任务失败');
      const result = await response.json();
      const updated = result.data;
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
  };
});