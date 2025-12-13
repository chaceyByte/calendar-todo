import { defineStore } from 'pinia';
import { ref } from 'vue';
import request from '@/utils/request';
import { useActivityStore } from './activity';
import { ElMessage } from 'element-plus';

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

// 撤销操作类型定义
interface UndoOperation {
  type: 'create' | 'update' | 'delete' | 'pause' | 'resume' | 'update_tags' | 'remove_tag';
  taskId: number;
  previousData?: any; // 操作前的数据状态
  newData?: any;      // 操作后的数据状态
  timestamp: number;  // 操作时间戳
}

export const useTaskStore = defineStore('task', () => {
  const tasks = ref<Task[]>([]);
  const activityStore = useActivityStore();

  // 撤销栈 - 最多存储5个操作
  const maxUndoSteps = 5;

  // 从localStorage初始化撤销栈
  const storedUndoStack = localStorage.getItem('taskUndoStack');
  const undoStack = ref<UndoOperation[]>(
    storedUndoStack ? JSON.parse(storedUndoStack) : []
  );

  // 保存撤销栈到localStorage
  const saveUndoStack = () => {
    localStorage.setItem('taskUndoStack', JSON.stringify(undoStack.value));
  };

  // 添加到撤销栈
  const pushToUndoStack = (operation: UndoOperation) => {
    undoStack.value.unshift(operation); // 新操作添加到栈顶

    // 保持栈大小不超过最大值
    if (undoStack.value.length > maxUndoSteps) {
      undoStack.value.pop();
    }

    // 保存到localStorage
    saveUndoStack();
  };

  // 撤销操作
  const undoTaskActions = async (taskId: number, depth: number = 5) => {
    try {
      console.log('调用撤销API:', `/api/tasks/${taskId}/undo`, { depth });
      const response = await request.post(`/api/tasks/${taskId}/undo`, { depth });
      console.log('撤销API响应:', response);
      
      // 响应拦截器已经返回了 response.data，所以 response 就是后端的 ApiResponse 结构
      // 后端成功响应格式: { success: true, message: "成功撤销最近操作", data: ... }
      if (response.success === true) {
        // 刷新任务列表
        await fetchTasks();
        console.log('撤销操作成功，任务列表已刷新');
        // 不在这里显示成功消息，让调用方处理
        return true;
      } else {
        const errorMsg = response.message || '撤销操作失败';
        console.error('撤销操作失败:', errorMsg);
        ElMessage.error(errorMsg);
        return false;
      }
    } catch (error: any) {
      console.error('撤销操作异常:', error);
      // 尝试从错误响应中获取错误信息
      const errorMessage = error.message || '撤销操作失败';
      console.error('详细错误信息:', errorMessage);
      // 不在这里显示错误消息，让调用方处理
      throw new Error(errorMessage);
    }
  };

  // 保留原来的本地撤销栈方法，但不再使用
  const undoLastOperation = async () => {
    if (undoStack.value.length === 0) {
      console.warn('撤销栈为空，没有可撤销的操作')
      ElMessage.warning('没有可撤销的操作');
      return false;
    }

    const operation = undoStack.value.shift()!;
    console.log('当前撤销栈:', undoStack.value)
    console.log('开始撤销操作:', operation)

    try {

      // 根据操作类型执行不同的撤销逻辑
      switch (operation.type) {
        case 'create':
          console.log('撤销创建操作，删除任务:', operation.taskId)
          // 撤销创建 = 删除任务
          await request.delete(`/api/tasks/${operation.taskId}`);
          tasks.value = tasks.value.filter(task => task.id !== operation.taskId);
          console.log('任务删除成功')
          break;

        case 'update':
          console.log('撤销更新操作，恢复任务状态:', operation.taskId)
          // 撤销更新 = 恢复之前的状态
          if (operation.previousData) {
            console.log('恢复数据:', operation.previousData)
            await request.put(`/api/tasks/${operation.taskId}`, operation.previousData);
            const taskIndex = tasks.value.findIndex(task => task.id === operation.taskId);
            if (taskIndex !== -1) {
              tasks.value[taskIndex] = { ...tasks.value[taskIndex], ...operation.previousData };
              console.log('任务状态恢复成功')
            } else {
              console.warn('任务不存在:', operation.taskId)
            }
          }
          break;

        case 'delete':
          console.log('撤销删除操作，恢复任务:', operation.taskId)
          // 撤销删除 = 重新创建任务
          if (operation.previousData) {
            console.log('恢复数据:', operation.previousData)
            const result = await request.post('/api/tasks', operation.previousData);
            const restoredTask = result.data || result;
            tasks.value.push(restoredTask);
            console.log('任务恢复成功:', restoredTask)
          }
          break;

        case 'pause':
          // 撤销暂停 = 恢复任务
          await request.post(`/api/tasks/${operation.taskId}/resume`);
          const taskIndex = tasks.value.findIndex(task => task.id === operation.taskId);
          if (taskIndex !== -1) {
            tasks.value[taskIndex].status = 'in-progress';
          }
          break;

        case 'resume':
          // 撤销恢复 = 重新暂停任务
          await request.post(`/api/tasks/${operation.taskId}/pause`);
          const pauseIndex = tasks.value.findIndex(task => task.id === operation.taskId);
          if (pauseIndex !== -1) {
            tasks.value[pauseIndex].status = 'paused';
          }
          break;

        case 'update_tags':
          // 撤销标签更新
          if (operation.previousData && operation.previousData.tagIds) {
            await request.put(`/api/tasks/${operation.taskId}/tags`, {
              tagIds: operation.previousData.tagIds
            });
          }
          break;

        case 'remove_tag':
          // 撤销标签移除 = 重新添加标签
          if (operation.previousData && operation.previousData.tagName) {
            await request.post(`/api/tasks/${operation.taskId}/tags`, {
              tagIds: [operation.previousData.tagName]
            });
          }
          break;
      }

      // 更新localStorage
      saveUndoStack();

      ElMessage.success('撤销操作成功');
      return true;
    } catch (error: any) {
      console.error('撤销操作失败:', error);

      // 将操作重新放回栈中，因为撤销失败
      undoStack.value.unshift(operation);

      // 更新localStorage
      saveUndoStack();

      const errorMessage = error.response?.data?.message || error.message || '撤销操作失败';
      ElMessage.error(`撤销失败: ${errorMessage}`);
      return false;
    }
  };

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

      const result = await request.post('/api/tasks', taskData);
      const newTask = result.data || result;

      // 记录撤销操作
      pushToUndoStack({
        type: 'create',
        taskId: newTask.id,
        previousData: null,
        newData: newTask,
        timestamp: Date.now()
      });

      // 添加任务后自动记录创建活动
      try {
        if (newTask && newTask.id) {
          await activityStore.startActivity(newTask.id, 'CREATED', '任务创建');
        }
      } catch (error) {
        console.log('记录创建活动失败:', error);
      }
      tasks.value.push(newTask);
      return newTask;
    } catch (error) {
      console.error('添加任务失败:', error);
      throw error;
    }
  };

  // 更新任务
  const updateTask = async (id: number, updatedTask: Partial<Task>) => {
    try {
      // 先保存当前状态用于撤销
      const currentTask = tasks.value.find(task => task.id === id);
      if (!currentTask) {
        throw new Error('任务不存在');
      }

      const previousData = { ...currentTask };

      const result = await request.put(`/api/tasks/${id}`, updatedTask);
      const updated = result.data || result;
      const taskIndex = tasks.value.findIndex(task => task.id === id);
      if (taskIndex !== -1) {
        tasks.value[taskIndex] = updated;
      }

      // 记录撤销操作
      pushToUndoStack({
        type: 'update',
        taskId: id,
        previousData,
        newData: updated,
        timestamp: Date.now()
      });

      return updated;
    } catch (error) {
      console.error('更新任务失败:', error);
      throw error;
    }
  };

  // 删除任务
  const deleteTask = async (id: number) => {
    try {
      // 先保存当前状态用于撤销
      const taskToDelete = tasks.value.find(task => task.id === id);
      if (!taskToDelete) {
        throw new Error('任务不存在');
      }

      await request.delete(`/tasks/${id}`);

      // 记录撤销操作
      pushToUndoStack({
        type: 'delete',
        taskId: id,
        previousData: taskToDelete,
        newData: null,
        timestamp: Date.now()
      });

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
      tasks.value = response.data || response || [];
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
      return response.data || response;
    } catch (error) {
      console.error('添加到暂存失败:', error);
      throw error;
    }
  };

  const removeFromStaging = async (taskId: number) => {
    try {
      const response = await request.delete(`/api/tasks/${taskId}/staging`);
      return response.data || response;
    } catch (error) {
      console.error('从暂存移除失败:', error);
      throw error;
    }
  };

  const fetchStagingTasks = async () => {
    try {
      const response = await request.get('/api/tasks/staging');
      return response.data || response || [];
    } catch (error) {
      console.error('获取暂存任务失败:', error);
      throw error;
    }
  };

  // 暂停任务
  const pauseTask = async (id: number) => {
    try {
      // 先保存当前状态用于撤销
      const currentTask = tasks.value.find(task => task.id === id);
      if (!currentTask) {
        throw new Error('任务不存在');
      }

      // 先结束当前活动
      try {
        await activityStore.endActivity(id);
      } catch (error) {
        console.log('没有进行中的活动或结束活动失败:', error);
      }

      const result = await request.post(`/api/tasks/${id}/pause`);
      const updated = result.data || result;
      const taskIndex = tasks.value.findIndex(task => task.id === id);
      if (taskIndex !== -1) {
        tasks.value[taskIndex] = updated;
      }

      // 记录撤销操作
      pushToUndoStack({
        type: 'pause',
        taskId: id,
        previousData: currentTask,
        newData: updated,
        timestamp: Date.now()
      });

      // 暂停后自动添加到暂存队列
      try {
        await addToStaging(id);
      } catch (error) {
        console.log('添加到暂存队列失败:', error);
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
      // 先保存当前状态用于撤销
      const currentTask = tasks.value.find(task => task.id === id);
      if (!currentTask) {
        throw new Error('任务不存在');
      }

      const result = await request.post(`/api/tasks/${id}/resume`);
      const updated = result.data || result;

      // 记录撤销操作
      pushToUndoStack({
        type: 'resume',
        taskId: id,
        previousData: currentTask,
        newData: updated,
        timestamp: Date.now()
      });

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

      // 恢复后从暂存队列移除
      try {
        await removeFromStaging(id);
      } catch (error) {
        console.log('从暂存队列移除失败:', error);
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
      // 保存当前标签状态用于撤销
      const currentTask = tasks.value.find(task => task.id === taskId);
      const previousTagIds = currentTask?.tags || [];

      const result = await request.put(`/api/tasks/${taskId}/tags`, { tagIds });
      const updated = result.data || result;
      const taskIndex = tasks.value.findIndex(task => task.id === taskId);
      if (taskIndex !== -1) {
        tasks.value[taskIndex] = updated;
      }

      // 记录撤销操作
      pushToUndoStack({
        type: 'update_tags',
        taskId: taskId,
        previousData: { tagIds: previousTagIds },
        newData: { tagIds },
        timestamp: Date.now()
      });

      return updated;
    } catch (error) {
      console.error('更新任务标签失败:', error);
      throw error;
    }
  };

  // 从任务中移除标签
  const removeTagFromTask = async (taskId: number, tagName: string) => {
    try {
      // 保存当前状态用于撤销
      const currentTask = tasks.value.find(task => task.id === taskId);
      const previousTagIds = currentTask?.tags || [];

      const result = await request.delete(`/api/tasks/${taskId}/tags/${encodeURIComponent(tagName)}`);
      const updated = result.data || result;
      const taskIndex = tasks.value.findIndex(task => task.id === taskId);
      if (taskIndex !== -1) {
        tasks.value[taskIndex] = updated;
      }

      // 记录撤销操作
      pushToUndoStack({
        type: 'remove_tag',
        taskId: taskId,
        previousData: { tagName, tagIds: previousTagIds },
        newData: { tagIds: updated.tags || [] },
        timestamp: Date.now()
      });

      return updated;
    } catch (error) {
      console.error('移除任务标签失败:', error);
      throw error;
    }
  };

  return {
    tasks,
    undoStack,
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
    pushToUndoStack,
    undoLastOperation,
    undoTaskActions
  };
});