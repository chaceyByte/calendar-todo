<template>
  <div class="login-page">
    <!-- 左侧品牌区 -->
    <section class="brand-panel">
      <h1 class="brand-title">任务日历</h1>
      <p class="brand-desc">高效管理每一天</p>
    </section>

    <!-- 右侧登录区 -->
    <section class="login-panel">
      <div class="login-card">
        <h2 class="login-title">欢迎回来</h2>
        <p class="login-subtitle">请登录您的账户</p>

        <el-form
          ref="loginFormRef"
          :model="loginForm"
          :rules="loginRules"
          class="login-form"
          @submit.prevent="handleLogin"
        >
          <el-form-item prop="username">
            <el-input
              v-model="loginForm.username"
              placeholder="请输入用户名"
              size="large"
              :prefix-icon="User"
            />
          </el-form-item>

          <el-form-item prop="password">
            <el-input
              v-model="loginForm.password"
              type="password"
              placeholder="请输入密码"
              size="large"
              :prefix-icon="Lock"
              show-password
              @keyup.enter="handleLogin"
            />
          </el-form-item>

          <el-form-item prop="captcha">
            <div class="captcha-row">
              <el-input
                v-model="loginForm.captcha"
                placeholder="请输入验证码"
                size="large"
                @keyup.enter="handleLogin"
              />
              <div class="captcha-img" @click="refreshCaptcha">
                <img v-if="captchaImage" :src="captchaImage" alt="验证码" />
                <el-button v-else link>获取验证码</el-button>
              </div>
            </div>
          </el-form-item>

          <el-form-item>
            <el-button
              type="primary"
              size="large"
              class="login-btn"
              :loading="loading"
              @click="handleLogin"
            >
              {{ loading ? '登录中...' : '登录' }}
            </el-button>
          </el-form-item>

          <div class="form-footer">
            <el-link type="info" :underline="false">忘记密码？</el-link>
            <span class="split">|</span>
            <el-link type="primary" :underline="false" @click="goToRegister">
              立即注册
            </el-link>
          </div>
        </el-form>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useUserStore } from '@/stores/user'
import { getCaptcha } from '@/api/captcha'
import { ElMessage } from 'element-plus/es'
import { User, Lock } from '@element-plus/icons-vue'

const router = useRouter()
const userStore = useUserStore()

const loginFormRef = ref()
const loading = ref(false)
const captchaImage = ref('')

const loginForm = reactive({
  username: '',
  password: '',
  captcha: ''
})

const loginRules = {
  username: [
    { required: true, message: '请输入用户名', trigger: 'blur' },
    { min: 3, max: 20, message: '长度 3-20 个字符', trigger: 'blur' }
  ],
  password: [
    { required: true, message: '请输入密码', trigger: 'blur' },
    { min: 6, max: 20, message: '长度 6-20 个字符', trigger: 'blur' }
  ],
  captcha: [{ required: true, message: '请输入验证码', trigger: 'blur' }]
}

const refreshCaptcha = async () => {
  try {
    captchaImage.value = await getCaptcha()
  } catch {
    ElMessage.error('获取验证码失败')
  }
}

const handleLogin = async () => {
  const valid = await loginFormRef.value.validate()
  if (!valid) return
  loading.value = true
  try {
    await userStore.login(loginForm.username, loginForm.password, loginForm.captcha)
    ElMessage.success('登录成功')
    router.replace('/')
  } catch {
    refreshCaptcha()
  } finally {
    loading.value = false
  }
}

const goToRegister = () => router.push('/register')

onMounted(() => refreshCaptcha())
</script>

<style scoped>
.login-page {
  display: flex;
  height: 100vh;
  font-family: var(--font-sans);
  background: var(--color-bg);
}

/* 左侧品牌 - 创意前卫渐变 */
.brand-panel {
  flex: 1;
  background: linear-gradient(135deg, var(--color-primary) 0%, var(--color-primary-dark) 100%);
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  color: white;
  position: relative;
  overflow: hidden;
}

.brand-panel::before {
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

.brand-title {
  font-size: 48px;
  font-weight: 800;
  margin: 0 0 12px;
  letter-spacing: -0.02em;
  text-shadow: 0 4px 12px rgba(0,0,0,0.2);
  position: relative;
  z-index: 1;
}

.brand-desc {
  font-size: 18px;
  font-weight: 500;
  opacity: 0.9;
  margin: 0;
  position: relative;
  z-index: 1;
}

/* 右侧登录 - Glassmorphism卡片 */
.login-panel {
  width: 480px;
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 40px;
}

.login-card {
  width: 400px;
  background: rgba(255, 255, 255, 0.85);
  backdrop-filter: blur(12px);
  border: 1px solid rgba(203, 213, 225, 0.3);
  border-radius: var(--radius);
  padding: 40px;
  box-shadow: var(--shadow-lg);
}

.login-title {
  font-size: 28px;
  font-weight: 700;
  color: var(--color-text);
  margin: 0 0 8px;
  letter-spacing: -0.01em;
}

.login-subtitle {
  font-size: 14px;
  color: var(--color-text-muted);
  margin: 0 0 32px;
  font-weight: 500;
}

/* 输入框 - 现代设计 */
:deep(.el-input__inner) {
  height: 48px;
  line-height: 48px;
  border-radius: var(--radius);
  border: 1px solid var(--color-border);
  background: rgba(255, 255, 255, 0.8);
  font-weight: 500;
  transition: var(--transition);
}

:deep(.el-input__inner:focus) {
  border-color: var(--color-primary);
  box-shadow: 0 0 0 3px rgba(13, 148, 136, 0.1);
}

:deep(.el-input__wrapper) {
  box-shadow: none;
  background: transparent;
}

.captcha-row {
  display: flex;
  gap: 12px;
}

.captcha-img {
  height: 48px;
  width: 120px;
  border: 1px solid var(--color-border);
  border-radius: var(--radius);
  overflow: hidden;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(255, 255, 255, 0.8);
  transition: var(--transition);
}

.captcha-img:hover {
  border-color: var(--color-primary);
  box-shadow: 0 0 0 3px rgba(13, 148, 136, 0.1);
}

.captcha-img img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.login-btn {
  width: 100%;
  height: 48px;
  border-radius: var(--radius);
  background: var(--color-primary);
  border: none;
  font-size: 16px;
  font-weight: 600;
  transition: var(--transition);
  box-shadow: 0 4px 6px rgba(13, 148, 136, 0.3);
}

.login-btn:hover {
  background: var(--color-primary-dark);
  transform: translateY(-2px);
  box-shadow: 0 6px 12px rgba(13, 148, 136, 0.4);
}

.form-footer {
  text-align: center;
  font-size: 14px;
  color: var(--color-text-muted);
}

.split {
  margin: 0 12px;
  color: var(--color-border);
}

/* 响应式 */
@media (max-width: 768px) {
  .login-page {
    flex-direction: column;
  }
  .brand-panel {
    height: 30vh;
  }
  .login-panel {
    width: 100%;
    height: 70vh;
  }
}
</style>