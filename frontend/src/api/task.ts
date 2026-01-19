import request from '@/utils/request'

// 任务相关的类型定义
export interface Tag {
  id: number
  name: string
  color?: string
}

// 用于创建任务的接口（发送给后端的数据格式）
export interface CreateTaskData {
  title: string
  description?: string
  startTime?: string
  endTime?: string
  status: 'planning' | 'in-progress' | 'completed' | 'cancelled'
  priority?: '-high' | '-middle' | '-low' | 'low' | 'middle' | 'high'
  urgency?: '-high' | '-middle' | '-low' | 'low' | 'middle' | 'high'
  progress?: number
  tags?: string[]  // 标签ID的字符串数组
  completed?: boolean
}

// 从后端获取的任务数据格式
export interface Task {
  id?: number
  title: string
  description?: string
  startTime?: string
  endTime?: string
  status: 'planning' | 'in-progress' | 'completed' | 'cancelled'
  priority?: '-high' | '-middle' | '-low' | 'low' | 'middle' | 'high'
  urgency?: '-high' | '-middle' | '-low' | 'low' | 'middle' | 'high'
  progress?: number
  tags?: Tag[]  // 获取时返回的是Tag对象数组
  tagIds?: number[]
  completed?: boolean
  createdAt?: string
  updatedAt?: string
  archivedAt?: string
}

export interface TaskOperation {
  type: 'create' | 'update' | 'delete' | 'start' | 'pause' | 'resume' | 'add_tag' | 'remove_tag'
  taskId?: number
  previousData?: any
  timestamp: number
}

export interface StagingTask {
  id?: number
  originalTaskId?: number
  title: string
  description?: string
  startTime?: string
  endTime?: string
  status: 'pending' | 'in-progress' | 'completed' | 'cancelled'
  priority?: 'low' | 'medium' | 'high'
  tags?: Tag[]
  tagIds?: number[]
}

// 任务相关的API函数
export const createTask = async (taskData: CreateTaskData) => {
  try {
    const response = await request.post('/tasks', taskData)
    return response.data || response
  } catch (error) {
    console.error('创建任务失败:', error)
    throw error
  }
}

export const updateTask = async (id: number, updatedTask: Task) => {
  try {
    const response = await request.put(`/tasks/${id}`, updatedTask)
    return response.data || response
  } catch (error) {
    console.error('更新任务失败:', error)
    throw error
  }
}

export const deleteTask = async (id: number) => {
  try {
    await request.delete(`/tasks/${id}`)
  } catch (error) {
    console.error('删除任务失败:', error)
    throw error
  }
}

export const getTasks = async () => {
  try {
    const response = await request.get('/tasks')
    return response.data || response
  } catch (error) {
    console.error('获取任务列表失败:', error)
    throw error
  }
}

export const undoTask = async (taskId: number, depth?: number) => {
  try {
    const response = await request.post(`/tasks/${taskId}/undo`, { depth })
    return response.data || response
  } catch (error) {
    console.error('撤销任务操作失败:', error)
    throw error
  }
}

export const resumeTask = async (taskId: number) => {
  try {
    const response = await request.post(`/tasks/${taskId}/resume`)
    return response.data || response
  } catch (error) {
    console.error('恢复任务失败:', error)
    throw error
  }
}

export const pauseTask = async (taskId: number) => {
  try {
    const response = await request.post(`/tasks/${taskId}/pause`)
    return response.data || response
  } catch (error) {
    console.error('暂停任务失败:', error)
    throw error
  }
}

export const updateTaskTags = async (taskId: number, { tagIds }: { tagIds: number[] }) => {
  try {
    const response = await request.put(`/tasks/${taskId}/tags`, { tagIds })
    return response.data || response
  } catch (error) {
    console.error('更新任务标签失败:', error)
    throw error
  }
}

export const addTaskTag = async (taskId: number, { tagIds }: { tagIds: number[] }) => {
  try {
    const response = await request.post(`/tasks/${taskId}/tags`, { tagIds })
    return response.data || response
  } catch (error) {
    console.error('添加任务标签失败:', error)
    throw error
  }
}

export const removeTaskTag = async (taskId: number, tagName: string) => {
  try {
    const response = await request.delete(`/tasks/${taskId}/tags/${encodeURIComponent(tagName)}`)
    return response.data || response
  } catch (error) {
    console.error('删除任务标签失败:', error)
    throw error
  }
}

// 暂存任务相关API
export const addTaskToStaging = async (taskId: number) => {
  try {
    const response = await request.post(`/tasks/${taskId}/staging`)
    return response.data || response
  } catch (error) {
    console.error('添加任务到暂存区失败:', error)
    throw error
  }
}

export const removeTaskFromStaging = async (taskId: number) => {
  try {
    const response = await request.delete(`/tasks/${taskId}/staging`)
    return response.data || response
  } catch (error) {
    console.error('从暂存区移除任务失败:', error)
    throw error
  }
}

export const getStagingTasks = async () => {
  try {
    const response = await request.get('/tasks/staging')
    return response.data || response
  } catch (error) {
    console.error('获取暂存任务失败:', error)
    throw error
  }
}

// 归档任务相关API
export interface ArchivedTasksResponse {
  data: Task[]
  total: number
  page: number
  size: number
}

export const getArchivedTasks = async (page: number = 1, size: number = 20, keyword?: string) => {
  try {
    const params: any = { page, size }
    if (keyword) {
      params.keyword = keyword
    }
    const response = await request.get('/tasks/archived', { params })
    return response.data || response
  } catch (error) {
    console.error('获取归档任务失败:', error)
    throw error
  }
}

// 四象限任务相关API
export const getQuadrantTasks = async () => {
  try {
    const response = await request.get('/tasks/quadrant')
    return response.data || response
  } catch (error) {
    console.error('获取四象限任务失败:', error)
    throw error
  }
}

export const updateTaskPriorityUrgency = async (id: number, priority: string, urgency: string) => {
  try {
    const response = await request.put(`/tasks/${id}/priority-urgency`, null, {
      params: { priority, urgency }
    })
    return response.data || response
  } catch (error) {
    console.error('更新任务重要紧急程度失败:', error)
    throw error
  }
}

export const archiveTask = async (id: number) => {
  try {
    const response = await request.post(`/tasks/${id}/archive`)
    return response.data || response
  } catch (error) {
    console.error('归档任务失败:', error)
    throw error
  }
}

export const unarchiveTask = async (id: number) => {
  try {
    const response = await request.post(`/tasks/${id}/unarchive`)
    return response.data || response
  } catch (error) {
    console.error('取消归档任务失败:', error)
    throw error
  }
}