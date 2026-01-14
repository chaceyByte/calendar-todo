import { defineStore } from 'pinia'
import { ref } from 'vue'
import {
  createTask as apiCreateTask,
  updateTask as apiUpdateTask,
  deleteTask as apiDeleteTask,
  getTasks,
  undoTask as apiUndoTask,
  resumeTask as apiResumeTask,
  pauseTask as apiPauseTask,
  updateTaskTags as apiUpdateTaskTags,
  addTaskTag as apiAddTaskTag,
  removeTaskTag as apiRemoveTaskTag,
  addTaskToStaging,
  removeTaskFromStaging,
  getStagingTasks,
  type Task,
  type CreateTaskData
} from '@/api/task'

export type TaskStatus = 'planning' | 'in-progress' | 'completed' | 'cancelled'

interface UndoOperation {
  type: 'create' | 'update' | 'delete' | 'pause' | 'resume' | 'update_tags' | 'remove_tag'
  taskId: number
  previousData?: any
  newData?: any
  timestamp: number
}

export const useTaskStore = defineStore('task', () => {
  const tasks = ref<Task[]>([])
  const undoStack = ref<UndoOperation[]>([])
  const maxUndoSteps = 5

  const pushToUndoStack = (operation: UndoOperation) => {
    undoStack.value.push(operation)
    if (undoStack.value.length > maxUndoSteps) {
      undoStack.value.shift()
    }
  }

  const undoTaskActions = async (taskId: number, depth: number = 5) => {
    try {
      const response = await apiUndoTask(taskId, depth)
      console.log('撤销任务操作成功:', response)
      return true
    } catch (error) {
      console.error('撤销任务操作失败:', error)
      return false
    }
  }

  const undoLastOperation = async () => {
    if (undoStack.value.length === 0) return
    
    const lastOperation = undoStack.value.pop()
    if (!lastOperation) return
    
    try {
      switch (lastOperation.type) {
        case 'create':
          if (lastOperation.taskId) {
            await apiDeleteTask(lastOperation.taskId)
          }
          break
        
        case 'update':
          if (lastOperation.taskId && lastOperation.previousData) {
            await apiUpdateTask(lastOperation.taskId, lastOperation.previousData)
          }
          break
        
        case 'delete':
          if (lastOperation.previousData) {
            const result = await apiCreateTask(lastOperation.previousData)
            console.log('任务已恢复:', result)
          }
          break
        
        case 'pause':
          if (lastOperation.taskId) {
            await apiResumeTask(lastOperation.taskId)
          }
          break
        
        case 'resume':
          if (lastOperation.taskId && lastOperation.previousData) {
            await apiPauseTask(lastOperation.taskId)
          }
          break
        
        case 'update_tags':
          if (lastOperation.taskId && lastOperation.previousData) {
            await apiUpdateTaskTags(lastOperation.taskId, { tagIds: lastOperation.previousData.tagIds })
          }
          break
        
        case 'remove_tag':
          if (lastOperation.taskId && lastOperation.previousData) {
            await apiAddTaskTag(lastOperation.taskId, { tagIds: [lastOperation.previousData.tagId] })
          }
          break
      }
      
      await fetchTasks()
    } catch (error) {
      console.error('撤销操作失败:', error)
      pushToUndoStack(lastOperation)
    }
  }

  const addTask = async (task: Omit<Task, 'id' | 'createdAt' | 'updatedAt' | 'completed'>) => {
    try {
      const taskData = {
        title: task.title,
        description: task.description || '',
        status: task.status,
        urgency: task.urgency || '一般',
        progress: task.progress || 0,
        priority: task.priority || 'medium',
        tags: (task.tags || []).map(tag => String(tag))
      }
      console.log('发送创建任务请求，请求数据:', taskData)
      const newTask = await apiCreateTask(taskData)
      pushToUndoStack({
        type: 'create',
        taskId: newTask.id,
        previousData: null,
        newData: newTask,
        timestamp: Date.now()
      })
      tasks.value.push(newTask)
      await fetchTasks()
      return true
    } catch (error) {
      console.error('添加任务失败:', error)
      return false
    }
  }

  const updateTask = async (id: number, updatedTask: Partial<Task>) => {
    try {
      const currentTask = tasks.value.find(task => task.id === id)
      if (!currentTask) throw new Error('任务不存在')
      const previousData = { ...currentTask }
      
      // 确保必填字段存在
      const taskData: Task = {
        ...currentTask,
        ...updatedTask
      }
      
      const updated = await apiUpdateTask(id, taskData)
      const taskIndex = tasks.value.findIndex(task => task.id === id)
      if (taskIndex !== -1) {
        tasks.value[taskIndex] = updated
      }
      pushToUndoStack({
        type: 'update',
        taskId: id,
        previousData,
        newData: updated,
        timestamp: Date.now()
      })
      return updated
    } catch (error) {
      console.error('更新任务失败:', error)
      throw error
    }
  }

  const deleteTask = async (id: number) => {
    try {
      const taskToDelete = tasks.value.find(task => task.id === id)
      if (!taskToDelete) throw new Error('任务不存在')
      
      await apiDeleteTask(id)
      tasks.value = tasks.value.filter(task => task.id !== id)
      
      pushToUndoStack({
        type: 'delete',
        taskId: id,
        previousData: taskToDelete,
        newData: null,
        timestamp: Date.now()
      })
      
      return true
    } catch (error) {
      console.error('删除任务失败:', error)
      return false
    }
  }

  const fetchTasks = async () => {
    try {
      const fetchedTasks = await getTasks()
      tasks.value = fetchedTasks
      return fetchedTasks
    } catch (error) {
      console.error('获取任务列表失败:', error)
      return []
    }
  }

  const addToStaging = async (taskId: number) => {
    try {
      await addTaskToStaging(taskId)
      return true
    } catch (error) {
      console.error('添加到暂存区失败:', error)
      return false
    }
  }

  const removeFromStaging = async (taskId: number) => {
    try {
      await removeTaskFromStaging(taskId)
      return true
    } catch (error) {
      console.error('从暂存区移除失败:', error)
      return false
    }
  }

  const fetchStagingTasks = async () => {
    try {
      return await getStagingTasks()
    } catch (error) {
      console.error('获取暂存任务失败:', error)
      return []
    }
  }

  const pauseTask = async (id: number) => {
    try {
      const currentTask = tasks.value.find(task => task.id === id)
      if (!currentTask) throw new Error('任务不存在')
      
      await apiPauseTask(id)
      
      pushToUndoStack({
        type: 'pause',
        taskId: id,
        previousData: { status: currentTask.status },
        timestamp: Date.now()
      })
      
      await fetchTasks()
      return true
    } catch (error) {
      console.error('暂停任务失败:', error)
      return false
    }
  }

  const resumeTask = async (id: number) => {
    try {
      const currentTask = tasks.value.find(task => task.id === id)
      if (!currentTask) throw new Error('任务不存在')
      
      await apiResumeTask(id)
      
      pushToUndoStack({
        type: 'resume',
        taskId: id,
        previousData: { status: currentTask.status },
        timestamp: Date.now()
      })
      
      await fetchTasks()
      return true
    } catch (error) {
      console.error('恢复任务失败:', error)
      return false
    }
  }

  const updateTaskTags = async (taskId: number, tagIds: number[]) => {
    try {
      const currentTask = tasks.value.find(task => task.id === taskId)
      if (!currentTask) throw new Error('任务不存在')
      
      const previousData = { tagIds: currentTask.tags || [] }
      await apiUpdateTaskTags(taskId, { tagIds })
      
      pushToUndoStack({
        type: 'update_tags',
        taskId,
        previousData,
        timestamp: Date.now()
      })
      
      await fetchTasks()
      return true
    } catch (error) {
      console.error('更新任务标签失败:', error)
      return false
    }
  }

  const removeTagFromTask = async (taskId: number, tagName: string) => {
    try {
      const currentTask = tasks.value.find(task => task.id === taskId)
      if (!currentTask) throw new Error('任务不存在')
      
      await apiRemoveTaskTag(taskId, tagName)
      
      pushToUndoStack({
        type: 'remove_tag',
        taskId,
        previousData: { tagId: tagName },
        timestamp: Date.now()
      })
      
      await fetchTasks()
      return true
    } catch (error) {
      console.error('移除任务标签失败:', error)
      return false
    }
  }

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
  }
})