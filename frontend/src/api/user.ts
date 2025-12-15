import request from '@/utils/request'

export interface UserInfo {
  id: number
  username: string
  nickname: string
  avatar: string
  email?: string
}

/**
 * 获取用户信息
 * @returns 用户信息
 */
export const getUserInfo = async () => {
  try {
    const response = await request.get('/api/auth/profile')
    return response.data
  } catch (error) {
    console.error('获取用户信息失败:', error)
    throw error
  }
}