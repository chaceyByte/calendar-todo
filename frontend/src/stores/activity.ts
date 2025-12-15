import {defineStore} from 'pinia'
import {computed, ref} from 'vue'
import {
    addManualActivity as apiAddManualActivity,
    endActivity as apiEndActivity,
    getActivityByTask as apiGetActivityByTask,
    getAllActivities as apiGetAllActivities,
    getCurrentActivity as apiGetCurrentActivity,
    getDailyReport as apiGetDailyReport,
    getWeeklyReport as apiGetWeeklyReport,
    startActivity as apiStartActivity
} from '@/api/activity'

export interface ActivityRecord {
    id: number
    taskId: number
    taskTitle?: string
    startTime: string
    endTime?: string
    duration?: number
    status: 'running' | 'completed' | 'cancelled'
    notes?: string
    tags?: string[]
    type: 'auto' | 'manual'
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

export const useActivityStore = defineStore('activity', () => {
    const activities = ref<ActivityRecord[]>([])
    const currentActivity = ref<ActivityRecord | null>(null)
    const dailyReport = ref<ActivityReport | null>(null)
    const weeklyReport = ref<WeeklyReport | null>(null)

    // 计算属性
    const totalDuration = computed(() => {
        return activities.value.reduce((total, activity) => {
            if (activity.duration) {
                return total + activity.duration
            }
            return total
        }, 0)
    })

    // 格式化总时长为可读格式
    const formattedTotalDuration = computed(() => {
        const minutes = Math.floor(totalDuration.value / (1000 * 60))
        const hours = Math.floor(minutes / 60)
        const remainingMinutes = minutes % 60

        if (hours > 0) {
            return `${hours}小时${remainingMinutes}分钟`
        } else {
            return `${minutes}分钟`
        }
    })

    // 格式化持续时间
    const formatDuration = (duration: number) => {
        if (!duration) return '0分钟'
        
        const minutes = Math.floor(duration / (1000 * 60))
        const hours = Math.floor(minutes / 60)
        const remainingMinutes = minutes % 60

        if (hours > 0) {
            return `${hours}小时${remainingMinutes}分钟`
        } else {
            return `${minutes}分钟`
        }
    }

    // 开始记录活动
    const startActivity = async (taskId: number, notes?: string) => {
        try {
            // 如果有正在进行的任务，先结束它
            if (currentActivity.value) {
                await endActivity(currentActivity.value.taskId)
            }

            const activity: ActivityRecord = await apiStartActivity(taskId, notes)
            currentActivity.value = activity
            activities.value.unshift(activity)
            return activity
        } catch (error) {
            console.error('开始活动失败:', error)
            throw error
        }
    }

    // 结束活动
    const endActivity = async (taskId: number) => {
        try {
            const updatedActivity = await apiEndActivity(taskId)

            // 更新当前活动
            if (currentActivity.value && currentActivity.value.taskId === taskId) {
                currentActivity.value = updatedActivity
            }

            // 更新活动列表中的对应项
            const activityIndex = activities.value.findIndex(a => a.taskId === taskId && a.status === 'running')
            if (activityIndex !== -1) {
                activities.value[activityIndex] = updatedActivity
            }

            // 如果没有正在进行的任务，清除当前活动
            if (updatedActivity.status === 'completed') {
                currentActivity.value = null
            }

            return updatedActivity
        } catch (error) {
            console.error('结束活动失败:', error)
            throw error
        }
    }

    // 添加手动活动
    const addManualActivity = async (data: {
        taskId: number
        taskTitle?: string
        startTime: string
        endTime?: string
        duration?: number
        notes?: string
    }) => {
        try {
            const activity = await apiAddManualActivity(<ActivityRecord>data)
            activities.value.unshift(activity)
            return activity
        } catch (error) {
            console.error('添加手动活动失败:', error)
            throw error
        }
    }

    // 获取任务相关活动
    const getTaskActivities = async (taskId: number) => {
        try {
            return await apiGetActivityByTask(taskId)
        } catch (error) {
            console.error('获取任务活动失败:', error)
            throw error
        }
    }

    // 获取所有活动
    const getAllActivities = async () => {
        try {
            const fetchedActivities = await apiGetAllActivities()
            activities.value = fetchedActivities
            return fetchedActivities
        } catch (error) {
            console.error('获取所有活动失败:', error)
            throw error
        }
    }

    // 获取当前活动
    const getCurrentActivity = async (taskId: number) => {
        try {
            const activity = await apiGetCurrentActivity(taskId)
            if (activity) {
                currentActivity.value = activity
            }
            return activity
        } catch (error) {
            console.error('获取当前活动失败:', error)
            throw error
        }
    }

    // 获取每日报告
    const getDailyReport = async (date?: string) => {
        try {
            const report = await apiGetDailyReport(date)
            dailyReport.value = report
            return report
        } catch (error) {
            console.error('获取每日报告失败:', error)
            throw error
        }
    }

    // 获取每周报告
    const getWeeklyReport = async (weekStart?: string, weekEnd?: string) => {
        try {
            const report = await apiGetWeeklyReport(weekStart, weekEnd)
            weeklyReport.value = report
            return report
        } catch (error) {
            console.error('获取每周报告失败:', error)
            throw error
        }
    }

    return {
        // 状态
        activities,
        currentActivity,
        dailyReport,
        weeklyReport,

        // 计算属性
        totalDuration,
        formattedTotalDuration,

        // 方法
        startActivity,
        endActivity,
        addManualActivity,
        getTaskActivities,
        getAllActivities,
        getCurrentActivity,
        getDailyReport,
        getWeeklyReport,
        formatDuration
    }
})