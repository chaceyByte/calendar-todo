import { defineStore } from 'pinia';
import { ref } from 'vue';

export interface Task {
  id: number;
  title: string;
  description?: string;
  dueDate?: string;
  priority?: 'low' | 'medium' | 'high';
  completed: boolean;
  tags?: number[];
}

export const useTaskStore = defineStore('task', () => {
  const tasks = ref<Task[]>([]);

  // 添加任务
  const addTask = async (task: Task) => {
    try {
      const response = await fetch('/api/tasks', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(task),
      });
      if (!response.ok) throw new Error('添加任务失败');
      const newTask = await response.json();
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
      const updated = await response.json();
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
      const response = await fetch(`/api/tasks/${id}`, { method: 'DELETE' });
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
      tasks.value = await response.json();
      return tasks.value;
    } catch (error) {
      console.error('获取任务失败:', error);
      throw error;
    }
  };

  return {
    tasks,
    addTask,
    updateTask,
    deleteTask,
    fetchTasks,
  };
});