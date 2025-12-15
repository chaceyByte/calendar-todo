import request from '@/utils/request'

export interface UserInfo {
  id: number
  username: string
  nickname: string
  avatar: string
  email?: string
}

export interface LoginResponse {
  token: string
  user: UserInfo
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

/**
 * 用户注册
 * @param username 用户名
 * @param password 密码
 * @param nickname 昵称
 * @param email 邮箱
 * @param captcha 验证码
 * @returns 登录响应，包含token和用户信息
 */
export const register = async (username: string, password: string, nickname: string, email: string, captcha: string) => {
  try {
    const response = await request.post('/api/auth/register', {
      username,
      password,
      nickname,
      email,
      captcha
    })
    
    return response.data || response
  } catch (error) {
    console.error('注册失败:', error)
    throw error
  }
}

/**
 * 用户登录
 * @param username 用户名
 * @param password 密码
 * @param captcha 验证码
 * @returns 登录响应，包含token和用户信息
 */
export const login = async (username: string, password: string, captcha?: string) => {
  try {
    const response = await request.post('/api/auth/login', {
      username,
      password,
      captcha
    })
    
    return response.data || response
  } catch (error) {
    console.error('登录失败:', error)
    throw error
  }
}