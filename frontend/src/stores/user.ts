import {defineStore} from 'pinia'
import {ref} from 'vue'
import {getUserInfo, type UserInfo} from '@/api/user'

// interface LoginResponse {
//   token: string
//   user: UserInfo
// }

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
    const register = async (username: string, password: string, nickname: string, email: string, captcha?: string) => {
        const response = await request.post('/api/auth/register', {
            username,
            password,
            nickname,
            email,
            captcha
        })
        console.log(response)
        const data = response.data || response;
        setUserInfo(data.user)
        setToken(data.token)

        // 保存到localStorage
        localStorage.setItem('userInfo', JSON.stringify(data.user))
        localStorage.setItem('token', data.token)

        return data.user
    }

    // 登录
    const login = async (username: string, password: string, captcha?: string) => {
        const response = await request.post('/api/auth/login', {
            username,
            password,
            captcha
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
    const initUser = async () => {
        const savedToken = localStorage.getItem('token')

        if (savedToken) {
            token.value = savedToken
            try {
                // 尝试从服务器获取最新的用户信息
                const latestUserInfo = await getUserInfo()
                setUserInfo(latestUserInfo)
                // 更新localStorage中的用户信息
                localStorage.setItem('userInfo', JSON.stringify(latestUserInfo))
            } catch (error) {
                // 如果获取失败，尝试使用localStorage中的用户信息
                const savedUser = localStorage.getItem('userInfo')
                if (savedUser) {
                    userInfo.value = JSON.parse(savedUser)
                }
                console.error('获取用户信息失败，使用本地缓存信息:', error)
            }
        }
    }
    
    // 强制刷新用户信息
    const refreshUserInfo = async () => {
        const token = localStorage.getItem('token')
        if (!token) {
            throw new Error('用户未登录')
        }
        
        try {
            const latestUserInfo = await getUserInfo()
            setUserInfo(latestUserInfo)
            localStorage.setItem('userInfo', JSON.stringify(latestUserInfo))
            return latestUserInfo
        } catch (error) {
            console.error('刷新用户信息失败:', error)
            throw error
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
        initUser,
        refreshUserInfo
    }
})