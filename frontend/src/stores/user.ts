import { defineStore } from 'pinia'
import { ref } from 'vue'

export interface UserInfo {
  id: number
  username: string
  nickname: string
  avatar: string
  email?: string
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

  // 登录
  const login = async (username: string, password: string) => {
    // 模拟登录，实际应该调用API
    const mockUser: UserInfo = {
      id: 1,
      username: username,
      nickname: username === 'admin' ? '管理员' : '用户',
      avatar: 'https://cube.elemecdn.com/3/7c/3ea6beec64369c2642b92c6726f1epng.png',
      email: `${username}@example.com`
    }
    
    setUserInfo(mockUser)
    setToken('mock-token-' + Date.now())
    
    // 保存到localStorage
    localStorage.setItem('userInfo', JSON.stringify(mockUser))
    localStorage.setItem('token', token.value)
    
    return Promise.resolve(mockUser)
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
    login,
    logout,
    initUser
  }
})