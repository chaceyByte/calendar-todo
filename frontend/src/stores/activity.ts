import {defineStore} from 'pinia'
import {computed, ref} from 'vue'
import request from '@/utils/request'

export interface ActivityRecord {
    id: number
    taskId: number
    startTime: string
    endTime?: string
    activityType: string
    description?: string
    durationMinutes?: number
    createdAt: string
}

export interface DailyReport {
    date: string
    totalTime: number
    completedTasks: number
    activeTasks: number
    taskActivities: TaskActivityDetail[]
}

export interface TaskActivityDetail {
    taskId: number
    taskTitle: string
    duration: number
    status: string
    activities: ActivityDetail[]
}

export interface ActivityDetail {
    id: number
    activityType: string
    description?: string
    startTime: string
    endTime?: string
    duration?: number
}

export interface WeeklyReport {
    weekStart: string
    weekEnd: string
    totalTime: number
    completedTasks: number
    dailySummaries: Map<string, DaySummary>
    taskActivities: WeeklyTaskActivityDetail[]
}

export interface DaySummary {
    date: string
    totalTime: number
    completedTasks: number
    activeTasks: number
}

export interface WeeklyTaskActivityDetail {
    taskId: number
    taskTitle: string
    totalDuration: number
    dailyDurations: Map<string, number>
    status: string
}

export const useActivityStore = defineStore('activity', () => {
    // 状态
    const activities = ref<ActivityRecord[]>([])
    const currentActivities = ref<Map<number, ActivityRecord>>(new Map())
    const dailyReport = ref<DailyReport | null>(null)
    const weeklyReport = ref<WeeklyReport | null>(null)
    const loading = ref(false)

    // 计算属性
    const totalActiveTime = computed(() => {
        return activities.value
            .filter(a => a.durationMinutes)
            .reduce((total, activity) => total + (activity.durationMinutes || 0), 0)
    })

    // 开始活动记录
    const startActivity = async (taskId: number, activityType: string, description?: string) => {
        try {
            loading.value = true
            const activity: ActivityRecord = await request.post('/api/activities/start', {
                taskId,
                activityType,
                description
            }) as any

            activities.value.unshift(activity)
            currentActivities.value.set(taskId, activity)

            return activity
        } catch (error) {
            console.error('开始活动记录失败:', error)
            throw error
        } finally {
            loading.value = false
        }
    }

    // 结束任务当前活动
    const endActivity = async (taskId: number) => {
        try {
            loading.value = true
            const response = await request.post(`/api/activities/end/${taskId}`) as any
            const activity: ActivityRecord = response.data
            // 更新活动列表中的对应记录
            const index = activities.value.findIndex(a => a.id === activity.id)
            if (index !== -1) {
                activities.value[index] = activity
            } else {
                activities.value.unshift(activity)
            }

            // 从当前活动映射中移除
            currentActivities.value.delete(taskId)

            return activity
        } catch (error) {
            console.error('结束活动记录失败:', error)
            throw error
        } finally {
            loading.value = false
        }
    }

    // 添加手动活动记录
    const addManualActivity = async (data: {
        taskId: number
        activityType: string
        startTime: string
        endTime: string
        description?: string
    }) => {
        try {
            loading.value = true
            const response = await request.post('/api/activities/manual', data) as any

            const activity: ActivityRecord = response.data
            activities.value.unshift(activity)

            return activity
        } catch (error) {
            console.error('添加手动活动记录失败:', error)
            throw error
        } finally {
            loading.value = false
        }
    }

    // 获取任务活动记录
    const getTaskActivities = async (taskId: number) => {
        try {
            loading.value = true
            const response = await request.get(`/api/activities/task/${taskId}`) as any

            return response.data as ActivityRecord[]
        } catch (error) {
            console.error('获取任务活动记录失败:', error)
            throw error
        } finally {
            loading.value = false
        }
    }

    // 获取所有活动记录
    const getAllActivities = async () => {
        try {
            loading.value = true
            const response = await request.get('/api/activities/all') as any

            return response.data as ActivityRecord[]
        } catch (error) {
            console.error('获取所有活动记录失败:', error)
            throw error
        } finally {
            loading.value = false
        }
    }

    // 获取任务当前活动
    const getCurrentActivity = async (taskId: number) => {
        try {
            const response = await request.get(`/api/activities/current/${taskId}`) as any

            if (response.data) {
                currentActivities.value.set(taskId, response.data)
                return response.data as ActivityRecord
            } else {
                currentActivities.value.delete(taskId)
                return null
            }
        } catch (error) {
            console.error('获取任务当前活动失败:', error)
            throw error
        }
    }

    // 获取日报数据
    const getDailyReport = async (date: string) => {
        try {
            loading.value = true
            const response = await request.get('/api/activities/report/daily', {
                params: {date}
            }) as any

            dailyReport.value = response.data as DailyReport
            return dailyReport.value
        } catch (error) {
            console.error('获取日报数据失败:', error)
            throw error
        } finally {
            loading.value = false
        }
    }

    // 获取周报数据
    const getWeeklyReport = async (weekStart: string) => {
        try {
            loading.value = true
            const response: any = await request.get('/api/activities/report/weekly', {
                params: {weekStart}
            })

            weeklyReport.value = {} as WeeklyReport
            Object.assign(weeklyReport.value, response)
            return weeklyReport.value
        } catch (error) {
            console.error('获取周报数据失败:', error)
            throw error
        } finally {
            loading.value = false
        }
    }

    // 格式化持续时间
    const formatDuration = (minutes: number): string => {
        if (!minutes || minutes <= 0) return '0分钟'

        const hours = Math.floor(minutes / 60)
        const mins = minutes % 60

        if (hours > 0) {
            return `${hours}小时${mins > 0 ? mins + '分钟' : ''}`
        } else {
            return `${mins}分钟`
        }
    }

    // 获取活动类型描述
    const getActivityTypeDescription = (type: string): string => {
        const typeMap: Record<string, string> = {
            'CREATED': '创建',
            'STARTED': '开始',
            'PAUSED': '暂停',
            'RESUMED': '恢复',
            'COMPLETED': '完成',
            'WORK': '工作',
            'MEETING': '会议',
            'STUDY': '学习',
            'OTHER': '其他'
        }

        return typeMap[type] || type
    }

    // 获取活动状态样式类
    const getActivityStatusClass = (activity: ActivityRecord): string => {
        if (activity.endTime) {
            return 'activity-completed'
        }

        switch (activity.activityType) {
            case 'CREATED':
                return 'activity-created'
            case 'STARTED':
                return 'activity-started'
            case 'PAUSED':
                return 'activity-paused'
            case 'RESUMED':
                return 'activity-resumed'
            case 'COMPLETED':
                return 'activity-completed'
            case 'WORK':
                return 'activity-work'
            case 'MEETING':
                return 'activity-meeting'
            case 'STUDY':
                return 'activity-study'
            default:
                return 'activity-other'
        }
    }

    return {
        // 状态
        activities,
        currentActivities,
        dailyReport,
        weeklyReport,
        loading,

        // 计算属性
        totalActiveTime,

        // 方法
        startActivity,
        endActivity,
        addManualActivity,
        getTaskActivities,
        getAllActivities,
        getCurrentActivity,
        getDailyReport,
        getWeeklyReport,
        formatDuration,
        getActivityTypeDescription,
        getActivityStatusClass
    }
})