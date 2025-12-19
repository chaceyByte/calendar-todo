import axios from 'axios'
import {ElMessage} from 'element-plus/es'
import router from "@/router";

// 创建axios实例
const service = axios.create({
    baseURL: import.meta.env.MODE === 'development' ? '/api' : '/', // 开发环境使用/api前缀，生产环境使用根路径
    timeout: 5000 // 请求超时时间
})

// 请求拦截器
service.interceptors.request.use(
    config => {
        const token = localStorage.getItem('token')
        if (token) {
            config.headers.Authorization = `Bearer ${token}`
        }
        return config
    },
    error => {
        console.error('请求错误:', error)
        return Promise.reject(error)
    }
)

// 响应拦截器
service.interceptors.response.use(
    response => {
        const res = response.data
        if (res.success === false) {
            if (res.code === 401) {
                ElMessage.error('未授权，请登录')
                router.replace('/login')
            }
            ElMessage.error(res.message || '请求失败');
            return Promise.reject(new Error(res.message || '请求失败'))
        }

        return res
    },
    error => {
        if (error.response) {
            switch (error.response.status) {   // ✅ 这里用 error.response.status
                case 401:
                    ElMessage.error('未授权，请登录')
                    router.replace('/login')
                    break
                case 404:
                    ElMessage.error('请求的资源不存在')
                    break
                case 500:
                    ElMessage.error('服务器内部错误')
                    break
                default:
                    ElMessage.error(`请求失败: ${error.response.status}`)
            }
        } else {
            ElMessage.error('网络错误')
        }

        return Promise.reject(error)
    }
)

export default service