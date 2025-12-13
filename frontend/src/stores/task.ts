import { defineStore } from 'pinia'
import { ref } from 'vue'
import request from '@/utils/request'
import { useActivityStore } from './activity'

export type TaskStatus = 'planning' | 'in-progress' | 'completed' | 'paused'

export interface Task {
  id: number
  title: string
  description?: string
  status: TaskStatus
  progress: number
  priority?: 'low' | 'medium' | 'high'
  startDate?: string
  endDate?: string
  tags?: string[]
  createdAt: string
  updatedAt: string
  completed: boolean
}

interface UndoOperation {
  type: 'create' | 'update' | 'delete' | 'pause' | 'resume' | 'update_tags' | 'remove_tag'
  taskId: number
  previousData?: any
  newData?: any
  timestamp: number
}

export const useTaskStore = defineStore('task', () => {
  const tasks = ref<Task[]>([])
  const activityStore = useActivityStore()

  const maxUndoSteps = 5
  const storedUndoStack = localStorage.getItem('taskUndoStack')
  const undoStack = ref<UndoOperation[]>
    (storedUndoStack ? JSON.parse(storedUndoStack) : [])

  const saveUndoStack = () => {
    localStorage.setItem('taskUndoStack', JSON.stringify(undoStack.value))
  }

  const pushToUndoStack = (operation: UndoOperation) => {
    undoStack.value.unshift(operation)
    if (undoStack.value.length > maxUndoSteps) {
      undoStack.value.pop()
    }
    saveUndoStack()
  }

  const undoTaskActions = async (taskId: number, depth: number = 5) => {
    try {
      const response = await request.post(`/api/tasks/${taskId}/undo`, { depth })
      // request.ts 拦截器返回的是 response.data，所以 response 已经是后端返回的数据结构
      if (response && (response as any).success === true) {
        await fetchTasks()
        return true
      } else {
        const errorMsg = (response as any)?.message || '撤销操作失败'
        ElMessage.error(errorMsg)
        return false
      }
    } catch (error: any) {
      const errorMessage = error.message || '撤销操作失败'
      throw new Error(errorMessage)
    }
  }

  const undoLastOperation = async () => {
    if (undoStack.value.length === 0) {
      ElMessage.warning('没有可撤销的操作')
      return false
    }
    const operation = undoStack.value.shift()!
    try {
      switch (operation.type) {
        case 'create':
          await request.delete(`/api/tasks/${operation.taskId}`)
          tasks.value = tasks.value.filter(task => task.id !== operation.taskId)
          break
        case 'update':
          if (operation.previousData) {
            await request.put(`/api/tasks/${operation.taskId}`, operation.previousData)
            const taskIndex = tasks.value.findIndex(task => task.id === operation.taskId)
            if (taskIndex !== -1) {
              tasks.value[taskIndex] = { ...tasks.value[taskIndex], ...operation.previousData }
            }
          }
          break
        case 'delete':
          if (operation.previousData) {
            const result = await request.post('/api/tasks', operation.previousData)
            const restoredTask = result.data || result
            tasks.value.push(restoredTask)
          }
          break
        case 'pause':
          await request.post(`/api/tasks/${operation.taskId}/resume`)
          const taskIndexPause = tasks.value.findIndex(task => task.id === operation.taskId)
          if (taskIndexPause !== -1) {
            tasks.value[taskIndexPause].status = 'in-progress'
          }
          break
        case 'resume':
          await request.post(`/api/tasks/${operation.taskId}/pause`)
          const pauseIndex = tasks.value.findIndex(task => task.id === operation.taskId)
          if (pauseIndex !== -1) {
            tasks.value[pauseIndex].status = 'paused'
          }
          break
        case 'update_tags':
          if (operation.previousData && operation.previousData.tagIds) {
            await request.put(`/api/tasks/${operation.taskId}/tags`, {
              tagIds: operation.previousData.tagIds
            })
          }
          break
        case 'remove_tag':
          if (operation.previousData && operation.previousData.tagName) {
            await request.post(`/api/tasks/${operation.taskId}/tags`, {
              tagIds: [operation.previousData.tagName]
            })
          }
          break
      }
      saveUndoStack()
      ElMessage.success('撤销操作成功')
      return true
    } catch (error: any) {
      undoStack.value.unshift(operation)
      saveUndoStack()
      const errorMessage = error.response?.data?.message || error.message || '撤销操作失败'
      ElMessage.error(`撤销失败: ${errorMessage}`)
      return false
    }
  }

  const addTask = async (task: Omit<Task, 'id' | 'createdAt' | 'updatedAt' | 'completed'>) => {
    try {
      const taskData = {
        title: task.title,
        description: task.description || '',
        status: task.status,
        progress: task.progress || 0,
        priority: task.priority || 'medium'
      }
      const result = await request.post('/api/tasks', taskData)
      const newTask = result.data || result
      pushToUndoStack({
        type: 'create',
        taskId: newTask.id,
        previousData: null,
        newData: newTask,
        timestamp: Date.now()
      })
      try {
        if (newTask && newTask.id) {
          await activityStore.startActivity(newTask.id, 'CREATED', '任务创建')
        }
      } catch (error) {
        console.log('记录创建活动失败:', error)
      }
      tasks.value.push(newTask)
      return newTask
    } catch (error) {
      console.error('添加任务失败:', error)
      throw error
    }
  }

  const updateTask = async (id: number, updatedTask: Partial<Task>) => {
    try {
      const currentTask = tasks.value.find(task => task.id === id)
      if (!currentTask) throw new Error('任务不存在')
      const previousData = { ...currentTask }
      const result = await request.put(`/api/tasks/${id}`, updatedTask)
      const updated = result.data || result
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
      await request.delete(`/api/tasks/${id}`)
      pushToUndoStack({
        type: 'delete',
        taskId: id,
        previousData: taskToDelete,
        newData: null,
        timestamp: Date.now()
      })
      tasks.value = tasks.value.filter(task => task.id !== id)
    } catch (error) {
      console.error('删除任务失败:', error)
      throw error
    }
  }

  const fetchTasks = async () => {
    try {
      const response = await request.get('/api/tasks')
      tasks.value = response.data || response || []
      return tasks.value
    } catch (error) {
      console.error('获取任务失败:', error)
      throw error
    }
  }

  const addToStaging = async (taskId: number) => {
    try {
      const response = await request.post(`/api/tasks/${taskId}/staging`)
      return response.data || response
    } catch (error) {
      console.error('添加到暂存失败:', error)
      throw error
    }
  }

  const removeFromStaging = async (taskId: number) => {
    try {
      const response = await request.delete(`/api/tasks/${taskId}/staging`)
      return response.data || response
    } catch (error) {
      console.error('从暂存移除失败:', error)
      throw error
    }
  }

  const fetchStagingTasks = async () => {
    try {
      const response = await request.get('/api/tasks/staging')
      return response.data || response || []
    } catch (error) {
      console.error('获取暂存任务失败:', error)
      throw error
    }
  }

  const pauseTask = async (id: number) => {
    try {
      const currentTask = tasks.value.find(task => task.id === id)
      if (!currentTask) throw new Error('任务不存在')
      try {
        await activityStore.endActivity(id)
      } catch (error) {
        console.log('没有进行中的活动或结束活动失败:', error)
      }
      const result = await request.post(`/api/tasks/${id}/pause`)
      const updated = result.data || result
      const taskIndex = tasks.value.findIndex(task => task.id === id)
      if (taskIndex !== -1) {
        tasks.value[taskIndex] = updated
      }
      pushToUndoStack({
        type: 'pause',
        taskId: id,
        previousData: currentTask,
        newData: updated,
        timestamp: Date.now()
      })
      try {
        await addToStaging(id)
      } catch (error) {
        console.log('添加到暂存队列失败:', error)
      }
      return updated
    } catch (error) {
      console.error('暂停任务失败:', error)
      throw error
    }
  }

  const resumeTask = async (id: number) => {
    try {
      const currentTask = tasks.value.find(task => task.id === id)
      if (!currentTask) throw new Error('任务不存在')
      const result = await request.post(`/api/tasks/${id}/resume`)
      const updated = result.data || result
      pushToUndoStack({
        type: 'resume',
        taskId: id,
        previousData: currentTask,
        newData: updated,
        timestamp: Date.now()
      })
      try {
        await activityStore.startActivity(id, 'RESUMED', '任务恢复')
      } catch (error) {
        console.log('开始恢复活动失败:', error)
      }
      const taskIndex = tasks.value.findIndex(task => task.id === id)
      if (taskIndex !== -1) {
        tasks.value[taskIndex] = updated
      }
      try {
        await removeFromStaging(id)
      } catch (error) {
        console.log('从暂存队列移除失败:', error)
      }
      return updated
    } catch (error) {
      console.error('恢复任务失败:', error)
      throw error
    }
  }

  const updateTaskTags = async (taskId: number, tagIds: number[]) => {
    try {
      const currentTask = tasks.value.find(task => task.id === taskId)
      const previousTagIds = currentTask?.tags || []
      const result = await request.put(`/api/tasks/${taskId}/tags`, { tagIds })
      const updated = result.data || result
      const taskIndex = tasks.value.findIndex(task => task.id === taskId)
      if (taskIndex !== -1) {
        tasks.value[taskIndex] = updated
      }
      pushToUndoStack({
        type: 'update_tags',
        taskId: taskId,
        previousData: { tagIds: previousTagIds },
        newData: { tagIds },
        timestamp: Date.now()
      })
      return updated
    } catch (error) {
      console.error('更新任务标签失败:', error)
      throw error
    }
  }

  const removeTagFromTask = async (taskId: number, tagName: string) => {
    try {
      const currentTask = tasks.value.find(task => task.id === taskId)
      const previousTagIds = currentTask?.tags || []
      const result = await request.delete(`/api/tasks/${taskId}/tags/${encodeURIComponent(tagName)}`)
      const updated = result.data || result
      const taskIndex = tasks.value.findIndex(task => task.id === taskId)
      if (taskIndex !== -1) {
        tasks.value[taskIndex] = updated
      }
      pushToUndoStack({
        type: 'remove_tag',
        taskId: taskId,
        previousData: { tagName, tagIds: previousTagIds },
        newData: { tagIds: updated.tags || [] },
        timestamp: Date.now()
      })
      return updated
    } catch (error) {
      console.error('移除任务标签失败:', error)
      throw error
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