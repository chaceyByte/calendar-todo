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

export interface SendEmailCodeRequest {
  email: string
  type: 'REGISTER' | 'RESET_PASSWORD' | 'CHANGE_EMAIL'
}

export interface ChangePasswordRequest {
  oldPassword: string
  newPassword: string
  confirmPassword: string
}

export interface ResetPasswordRequest {
  email: string
  code: string
  newPassword: string
  confirmPassword: string
}

export interface ChangeEmailRequest {
  newEmail: string
  code: string
}

/**
 * 获取用户信息
 * @returns 用户信息
 */
export const getUserInfo = async () => {
  try {
    const response = await request.get('/auth/profile')
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
    const response = await request.post('/auth/register', {
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
    const response = await request.post('/auth/login', {
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

/**
 * 发送邮箱验证码
 * @param data 发送验证码请求数据
 */
export const sendEmailCode = async (data: SendEmailCodeRequest) => {
  try {
    const response = await request.post('/auth/send-email-code', data)
    return response.data || response
  } catch (error) {
    console.error('发送验证码失败:', error)
    throw error
  }
}

/**
 * 更改密码
 * @param data 更改密码请求数据
 */
export const changePassword = async (data: ChangePasswordRequest) => {
  try {
    const response = await request.post('/auth/change-password', data)
    return response.data || response
  } catch (error) {
    console.error('更改密码失败:', error)
    throw error
  }
}

/**
 * 重置密码
 * @param data 重置密码请求数据
 */
export const resetPassword = async (data: ResetPasswordRequest) => {
  try {
    const response = await request.post('/auth/reset-password', data)
    return response.data || response
  } catch (error) {
    console.error('重置密码失败:', error)
    throw error
  }
}

/**
 * 更换邮箱
 * @param data 更换邮箱请求数据
 */
export const changeEmail = async (data: ChangeEmailRequest) => {
  try {
    const response = await request.post('/auth/change-email', data)
    return response.data || response
  } catch (error) {
    console.error('更换邮箱失败:', error)
    throw error
  }
}