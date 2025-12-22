import request from '@/utils/request'

export interface HolidayConfig {
  id?: number
  year: number
  date: string
  type: 'REST' | 'WORK'
  description?: string
}

export interface HolidayResponse {
  data: HolidayConfig[]
  success: boolean
  message?: string
}

// 节假日相关的API函数
export const getHolidaysByYear = async (year: number): Promise<HolidayConfig[]> => {
  try {
    const response = await request.get(`/holidays/${year}`)
    return response.data || []
  } catch (error) {
    console.error('获取节假日配置失败:', error)
    // 如果后端接口失败，返回空数组而不是mock数据
    return []
  }
}

export const createHoliday = async (holidayData: HolidayConfig) => {
  try {
    const response = await request.post('/holidays', holidayData)
    return response.data || response
  } catch (error) {
    console.error('创建节假日配置失败:', error)
    throw error
  }
}

export const updateHoliday = async (id: number, updatedHoliday: HolidayConfig) => {
  try {
    const response = await request.put(`/holidays/${id}`, updatedHoliday)
    return response.data || response
  } catch (error) {
    console.error('更新节假日配置失败:', error)
    throw error
  }
}

export const deleteHoliday = async (id: number) => {
  try {
    await request.delete(`/holidays/${id}`)
  } catch (error) {
    console.error('删除节假日配置失败:', error)
    throw error
  }
}