import { defineStore } from 'pinia'
import { ref } from 'vue'
import request from '@/utils/request'

export interface UserInfo {
  id: number
  username: string
  nickname: string
  avatar: string
  email?: string
}

interface LoginResponse {
  token: string
  user: UserInfo
}

export const useUserStore = defineStore('user', () => {
  const userInfo = ref<UserInfo | null>(null)
  const token = ref<string>('')

  // 设置用户信息
  const setUserInfo = (info: UserInfo) => {
    userInfo.value = info
  }

  // 设置token
  const setToken = (newToken: string) => {
    token.value = newToken
  }

  // 注册
  const register = async (username: string, password: string, nickname: string, email: string) => {
    const response = await request.post('/api/auth/register', {
      username,
      password,
      nickname,
      email
    })
    
    const data = response.data || response
    setUserInfo(data.user)
    setToken(data.token)
    
    // 保存到localStorage
    localStorage.setItem('userInfo', JSON.stringify(data.user))
    localStorage.setItem('token', data.token)
    
    return data.user
  }

  // 登录
  const login = async (username: string, password: string) => {
    const response = await request.post('/api/auth/login', {
      username,
      password
    })
    
    const data = response.data || response
    setUserInfo(data.user)
    setToken(data.token)
    
    // 保存到localStorage
    localStorage.setItem('userInfo', JSON.stringify(data.user))
    localStorage.setItem('token', data.token)
    
    return data.user
  }

  // 退出登录
  const logout = () => {
    userInfo.value = null
    token.value = ''
    localStorage.removeItem('userInfo')
    localStorage.removeItem('token')
  }

  // 初始化用户信息
  const initUser = () => {
    const savedUser = localStorage.getItem('userInfo')
    const savedToken = localStorage.getItem('token')
    
    if (savedUser && savedToken) {
      userInfo.value = JSON.parse(savedUser)
      token.value = savedToken
    }
  }

  return {
    userInfo,
    token,
    setUserInfo,
    setToken,
    register,
    login,
    logout,
    initUser
  }
})