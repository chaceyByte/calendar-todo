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
  font-family: 'Alibaba PuHuiTi', 'Source Han Sans CN', 'Noto Sans SC', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
}

/* 左侧品牌 */
.brand-panel {
  flex: 1;
  background: linear-gradient(135deg, #6a11cb 0%, #2575fc 100%);
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  color: #fff;
}
.brand-title {
  font-size: 32px;
  font-weight: 700;
  margin: 0 0 8px;
}
.brand-desc {
  font-size: 14px;
  font-weight: 400;
  opacity: 0.9;
  margin: 0;
}

/* 右侧登录 */
.login-panel {
  width: 480px;
  background: #fff;
  display: flex;
  justify-content: center;
  align-items: center;
}
.login-card {
  width: 360px;
}
.login-title {
  font-size: 24px;
  font-weight: 600;
  color: #111827;
  margin: 0 0 4px;
}
.login-subtitle {
  font-size: 14px;
  color: #6b7280;
  margin: 0 0 32px;
}

/* 输入框统一高度 & 聚焦发光 */
:deep(.el-input__inner) {
  height: 48px;
  line-height: 48px;
  border-radius: 8px;
  border: 1px solid #e5e7eb;
}
:deep(.el-input__wrapper) {
  box-shadow: 0 0 0 1px #e5e7eb;
}
:deep(.el-input__wrapper.is-focus) {
  box-shadow: 0 0 0 2px rgba(124, 58, 237, 0.2);
}

.captcha-row {
  display: flex;
  gap: 12px;
}
.captcha-img {
  height: 48px;
  width: 120px;
  border: 1px solid #e5e7eb;
  border-radius: 8px;
  overflow: hidden;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
}
.captcha-img img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.login-btn {
  width: 100%;
  height: 48px;
  border-radius: 8px;
  background: linear-gradient(135deg, #6a11cb 0%, #2575fc 100%);
  border: none;
  font-size: 16px;
}
.login-btn:hover {
  opacity: 0.95;
}

.form-footer {
  text-align: center;
  font-size: 14px;
  color: #9ca3af;
}
.split {
  margin: 0 8px;
}

.tips {
  margin-top: 24px;
  font-size: 12px;
  color: #9ca3af;
  text-align: center;
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