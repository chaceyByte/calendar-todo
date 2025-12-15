<template>
  <div class="login-container">
    <div class="login-card">
      <div class="login-header">
        <h1>任务日历</h1>
        <p>欢迎回来，请登录您的账户</p>
      </div>

      <el-form 
        :model="loginForm" 
        :rules="loginRules" 
        ref="loginFormRef"
        class="login-form"
        @submit.prevent="handleLogin"
      >
        <el-form-item prop="username">
          <el-input
            v-model="loginForm.username"
            placeholder="请输入用户名"
            size="large"
            prefix-icon="User"
          />
        </el-form-item>

        <el-form-item prop="password">
          <el-input
            v-model="loginForm.password"
            type="password"
            placeholder="请输入密码"
            size="large"
            prefix-icon="Lock"
            show-password
            @keyup.enter="handleLogin"
          />
        </el-form-item>

        <el-form-item prop="captcha">
          <div class="captcha-container">
            <el-input
              v-model="loginForm.captcha"
              placeholder="请输入验证码"
              size="large"
              style="flex: 1; margin-right: 10px;"
              @keyup.enter="handleLogin"
            />
            <div class="captcha-image" @click="refreshCaptcha">
              <img :src="captchaImage" alt="验证码" v-if="captchaImage" />
              <el-button size="large" @click="refreshCaptcha" v-else>获取验证码</el-button>
            </div>
          </div>
        </el-form-item>

        <el-form-item>
          <el-button 
            type="primary" 
            size="large" 
            class="login-button"
            :loading="loading"
            @click="handleLogin"
          >
            {{ loading ? '登录中...' : '登录' }}
          </el-button>
        </el-form-item>
      </el-form>

      <div class="login-footer">
        <p>演示账号：admin / 123456</p>
        <p>没有账号？<el-link type="primary" @click="goToRegister">立即注册</el-link></p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useUserStore } from '@/stores/user'
import { getCaptcha } from '@/api/captcha'
import { ElMessage } from 'element-plus/es'

// import type { FormInstance, FormRules } from 'element-plus'

const router = useRouter()
const userStore = useUserStore()

const loginFormRef = ref<any>()
const loading = ref(false)

const loginForm = reactive({
  username: '',
  password: '',
  captcha: ''
})

const captchaImage = ref('')

// 页面加载时自动填充演示账号
onMounted(() => {
  loginForm.username = 'admin'
  loginForm.password = '123456'
  refreshCaptcha()
})

const loginRules: any = {
  username: [
    { required: true, message: '请输入用户名', trigger: 'blur' },
    { min: 3, max: 20, message: '用户名长度在 3 到 20 个字符', trigger: 'blur' }
  ],
  password: [
    { required: true, message: '请输入密码', trigger: 'blur' },
    { min: 6, max: 20, message: '密码长度在 6 到 20 个字符', trigger: 'blur' }
  ],
  captcha: [
    { required: true, message: '请输入验证码', trigger: 'blur' }
  ]
}

// 获取验证码
const refreshCaptcha = async () => {
  try {
    captchaImage.value = await getCaptcha()
  } catch (error) {
    // 错误已经在API函数中处理，这里可以添加额外的UI反馈
    ElMessage.error('获取验证码失败，请刷新页面重试')
  }
}

const handleLogin = async () => {
  if (!loginFormRef.value) return

  const valid = await loginFormRef.value.validate()
  if (!valid) return

  loading.value = true

  try {
    console.log('开始登录，用户名:', loginForm.username)
    await userStore.login(loginForm.username, loginForm.password, loginForm.captcha)
    console.log('登录成功，准备跳转')
    ElMessage.success('登录成功')
    
    // 确保token已保存到localStorage
    const token = localStorage.getItem('token')
    console.log('保存的token:', token)
    
    // 使用replace而不是push，避免浏览器历史记录问题
    router.replace('/')
  } catch (error) {
    console.error('登录错误:', error)
    // 错误消息已经在request拦截器中显示，这里只需刷新验证码
    refreshCaptcha() // 刷新验证码
  } finally {
    loading.value = false
  }
}

const goToRegister = () => {
  router.push('/register')
}
</script>

<style scoped>
.login-container {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 100vh;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

.login-card {
  width: 400px;
  padding: 40px;
  background: white;
  border-radius: 12px;
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.1);
}

.login-header {
  text-align: center;
  margin-bottom: 30px;
}

.login-header h1 {
  margin: 0 0 10px 0;
  font-size: 28px;
  font-weight: 600;
  color: #303133;
}

.login-header p {
  margin: 0;
  color: #909399;
  font-size: 14px;
}

.login-form {
  margin-bottom: 20px;
}

.login-button {
  width: 100%;
  margin-top: 10px;
}

.login-footer {
  text-align: center;
  border-top: 1px solid #e4e7ed;
  padding-top: 20px;
}

.captcha-container {
  display: flex;
  align-items: center;
}

.captcha-image {
  width: 120px;
  height: 40px;
  cursor: pointer;
  border: 1px solid #dcdfe6;
  border-radius: 4px;
  display: flex;
  justify-content: center;
  align-items: center;
}

.captcha-image img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.login-footer p {
  margin: 5px 0;
  color: #909399;
  font-size: 12px;
}
</style>