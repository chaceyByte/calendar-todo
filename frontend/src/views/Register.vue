<template>
  <div class="register-container">
    <div class="register-card">
      <div class="register-header">
        <h1>任务日历</h1>
        <p>创建您的账户</p>
      </div>

      <el-form 
        :model="registerForm" 
        :rules="registerRules" 
        ref="registerFormRef"
        class="register-form"
        @submit.prevent="handleRegister"
      >
        <el-form-item prop="username">
          <el-input
            v-model="registerForm.username"
            placeholder="请输入用户名"
            size="large"
            prefix-icon="User"
          />
        </el-form-item>

        <el-form-item prop="password">
          <el-input
            v-model="registerForm.password"
            type="password"
            placeholder="请输入密码"
            size="large"
            prefix-icon="Lock"
            show-password
          />
        </el-form-item>

        <el-form-item prop="confirmPassword">
          <el-input
            v-model="registerForm.confirmPassword"
            type="password"
            placeholder="请确认密码"
            size="large"
            prefix-icon="Lock"
            show-password
            @keyup.enter="handleRegister"
          />
        </el-form-item>

        <el-form-item prop="nickname">
          <el-input
            v-model="registerForm.nickname"
            placeholder="请输入昵称"
            size="large"
            prefix-icon="Avatar"
          />
        </el-form-item>

        <el-form-item prop="email">
          <el-input
            v-model="registerForm.email"
            placeholder="请输入邮箱"
            size="large"
            prefix-icon="Message"
          />
        </el-form-item>

        <el-form-item prop="captcha">
          <div class="captcha-container">
            <el-input
              v-model="registerForm.captcha"
              placeholder="请输入验证码"
              size="large"
              style="flex: 1; margin-right: 10px;"
              @keyup.enter="handleRegister"
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
            class="register-button"
            :loading="loading"
            @click="handleRegister"
          >
            {{ loading ? '注册中...' : '注册' }}
          </el-button>
        </el-form-item>
      </el-form>

      <div class="register-footer">
        <p>已有账户？<el-link type="primary" @click="goToLogin">立即登录</el-link></p>
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

const registerFormRef = ref<any>()
const loading = ref(false)
const captchaImage = ref('')

const registerForm = reactive({
  username: '',
  password: '',
  confirmPassword: '',
  nickname: '',
  email: '',
  captcha: ''
})

// 页面加载时获取验证码
onMounted(() => {
  refreshCaptcha()
})

const validateConfirmPassword = (_rule: any, value: string, callback: any) => {
  if (value === '') {
    callback(new Error('请确认密码'))
  } else if (value !== registerForm.password) {
    callback(new Error('两次输入密码不一致'))
  } else {
    callback()
  }
}

const registerRules: any = {
  username: [
    { required: true, message: '请输入用户名', trigger: 'blur' },
    { min: 3, max: 20, message: '用户名长度在 3 到 20 个字符', trigger: 'blur' }
  ],
  password: [
    { required: true, message: '请输入密码', trigger: 'blur' },
    { min: 6, max: 20, message: '密码长度在 6 到 20 个字符', trigger: 'blur' }
  ],
  confirmPassword: [
    { required: true, validator: validateConfirmPassword, trigger: 'blur' }
  ],
  nickname: [
    { required: true, message: '请输入昵称', trigger: 'blur' },
    { min: 2, max: 20, message: '昵称长度在 2 到 20 个字符', trigger: 'blur' }
  ],
  email: [
    { required: true, message: '请输入邮箱', trigger: 'blur' },
    { type: 'email', message: '邮箱格式不正确', trigger: 'blur' }
  ],
  captcha: [
    { required: true, message: '请输入验证码', trigger: 'blur' }
  ]
}

const handleRegister = async () => {
  console.log("......")
  if (!registerFormRef.value) return
console.log("......1")
  const valid = await registerFormRef.value.validate()
  if (!valid) return

  console.log("......2")
  loading.value = true
console.log("......3")
  try {
    await userStore.register(
      registerForm.username,
      registerForm.password,
      registerForm.nickname,
      registerForm.email,
      registerForm.captcha
    )
    console.log("......4")
    ElMessage.success('注册成功，已为您登录, 祝您使用愉快!')
    await router.push('/login')
  } catch (error) {
    console.log("....5",error)

    // 错误消息已经在request拦截器中显示，这里只需刷新验证码
    await refreshCaptcha() // 刷新验证码
  } finally {
    loading.value = false
  }
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

const goToLogin = () => {
  router.push('/login')
}
</script>

<style scoped>
.register-container {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 100vh;
  background: linear-gradient(135deg, var(--color-primary) 0%, var(--color-primary-dark) 100%);
  font-family: var(--font-sans);
  position: relative;
  overflow: hidden;
}

.register-container::before {
  content: '';
  position: absolute;
  top: -50%;
  left: -50%;
  width: 200%;
  height: 200%;
  background: radial-gradient(circle, rgba(255,255,255,0.1) 0%, transparent 70%);
  animation: float 20s ease-in-out infinite;
}

@keyframes float {
  0%, 100% { transform: translate(0, 0) rotate(0deg); }
  33% { transform: translate(30px, -30px) rotate(120deg); }
  66% { transform: translate(-20px, 20px) rotate(240deg); }
}

.register-card {
  width: 420px;
  padding: 48px;
  background: rgba(255, 255, 255, 0.85);
  backdrop-filter: blur(12px);
  border: 1px solid rgba(203, 213, 225, 0.3);
  border-radius: var(--radius);
  box-shadow: var(--shadow-lg);
  position: relative;
  z-index: 1;
}

.register-header {
  text-align: center;
  margin-bottom: 32px;
}

.register-header h1 {
  margin: 0 0 12px 0;
  font-size: 32px;
  font-weight: 800;
  color: var(--color-text);
  letter-spacing: -0.01em;
}

.register-header p {
  margin: 0;
  color: var(--color-text-muted);
  font-size: 14px;
  font-weight: 500;
}

.register-form {
  margin-bottom: 24px;
}

.register-button {
  width: 100%;
  height: 48px;
  background: var(--color-primary);
  border-radius: var(--radius);
  border: none;
  font-size: 16px;
  font-weight: 600;
  transition: var(--transition);
  box-shadow: 0 4px 6px rgba(13, 148, 136, 0.3);
  margin-top: 12px;
}

.register-button:hover {
  background: var(--color-primary-dark);
  transform: translateY(-2px);
  box-shadow: 0 6px 12px rgba(13, 148, 136, 0.4);
}

.register-footer {
  text-align: center;
  border-top: 1px solid var(--color-border-light);
  padding-top: 24px;
}

.register-footer p {
  margin: 0;
  color: var(--color-text-muted);
  font-size: 14px;
}

.captcha-container {
  display: flex;
  align-items: center;
  gap: 12px;
}

.captcha-image {
  width: 120px;
  height: 48px;
  cursor: pointer;
  border: 1px solid var(--color-border);
  border-radius: var(--radius);
  display: flex;
  justify-content: center;
  align-items: center;
  background: rgba(255, 255, 255, 0.8);
  transition: var(--transition);
  overflow: hidden;
}

.captcha-image:hover {
  border-color: var(--color-primary);
  box-shadow: 0 0 0 3px rgba(13, 148, 136, 0.1);
}

.captcha-image img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}
</style>