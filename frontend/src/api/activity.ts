import request from '@/utils/request'

export interface ActivityRecord {
  id?: number
  taskId: number
  taskTitle?: string
  startTime: string
  endTime?: string
  duration?: number
  status: 'running' | 'completed' | 'cancelled'
  notes?: string
  tags?: string[]
  type: 'auto' | 'manual'
  initialStatus?: string
}

export interface ActivityReport {
  date: string
  duration: number
  tasks: {
    taskId: number
    taskTitle: string
    duration: number
  }[]
}

export interface WeeklyReport {
  weekStart: string
  weekEnd: string
  dailyReports: ActivityReport[]
  totalDuration: number
  topTasks: {
    taskId: number
    taskTitle: string
    duration: number
  }[]
}

// 活动相关的API函数
export const startActivity = async (taskId: number, notes?: string) => {
  try {
    const response = await request.post('/activities/start', {
      taskId,
      notes
    })
    return response.data || response
  } catch (error) {
    console.error('开始活动失败:', error)
    throw error
  }
}

export const endActivity = async (taskId: number) => {
  try {
    const response = await request.post(`/activities/end/${taskId}`)
    return response.data || response
  } catch (error) {
    console.error('结束活动失败:', error)
    throw error
  }
}

export const addManualActivity = async (activity: ActivityRecord) => {
  try {
    const response = await request.post('/activities/manual', activity)
    return response.data || response
  } catch (error) {
    console.error('添加手动活动失败:', error)
    throw error
  }
}

export const getActivityByTask = async (taskId: number) => {
  try {
    const response = await request.get(`/activities/task/${taskId}`)
    return response.data || response
  } catch (error) {
    console.error('获取任务活动失败:', error)
    throw error
  }
}

export const getAllActivities = async () => {
  try {
    const response = await request.get('/activities/all')
    return response.data || response
  } catch (error) {
    console.error('获取所有活动失败:', error)
    throw error
  }
}

export const getCurrentActivity = async (taskId: number) => {
  try {
    const response = await request.get(`/activities/current/${taskId}`)
    return response.data || response
  } catch (error) {
    console.error('获取当前活动失败:', error)
    throw error
  }
}

export const getDailyReport = async (date?: string) => {
  try {
    const response = await request.get('/activities/report/daily', {
      params: { date }
    })
    return response.data || response
  } catch (error) {
    console.error('获取每日报告失败:', error)
    throw error
  }
}

export const getWeeklyReport = async (weekStart?: string, weekEnd?: string) => {
  try {
    const response: any = await request.get('/activities/report/weekly', {
      params: { weekStart, weekEnd }
    })
    return response.data || response
  } catch (error) {
    console.error('获取每周报告失败:', error)
    throw error
  }
}