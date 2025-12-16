<template>
  <div class="auth-container">
    <!-- 背景图片 -->
    <div class="background"></div>
    
    <!-- 动态球体 -->
    <div class="floating-balls">
      <div class="ball ball-1"></div>
      <div class="ball ball-2"></div>
      <div class="ball ball-3"></div>
      <div class="ball ball-4"></div>
      <div class="ball ball-5"></div>
      <div class="ball ball-6"></div>
      <div class="ball ball-7"></div>
      <div class="ball ball-8"></div>
    </div>
    
    <!-- 统一认证卡片 -->
    <transition name="flip" mode="out-in">
      <div v-if="!isRegister" key="login" class="auth-card">
        <div class="auth-content">
          <div class="auth-form-container">
            <div class="brand-info">
              <h1 class="brand-title">任务日历</h1>
              <p class="brand-desc">高效管理每一天</p>
            </div>

            <el-form
              ref="loginFormRef"
              :model="loginForm"
              :rules="loginRules"
              class="auth-form"
              @submit.prevent="handleLogin"
            >
              <div class="form-title">
                <h2>欢迎回来</h2>
                <p>请登录您的账户</p>
              </div>

              <el-form-item prop="username">
                <el-input
                  v-model="loginForm.username"
                  placeholder="请输入用户名"
                  size="large"
                  :prefix-icon="User"
                  class="form-input"
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
                  class="form-input"
                  @keyup.enter="handleLogin"
                />
              </el-form-item>

              <el-form-item prop="captcha">
                <div class="captcha-container">
                  <el-input
                    v-model="loginForm.captcha"
                    placeholder="请输入验证码"
                    size="large"
                    class="form-input captcha-input"
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
                  class="auth-button"
                  :loading="loading"
                  @click="handleLogin"
                >
                  {{ loading ? '登录中...' : '登录' }}
                </el-button>
              </el-form-item>

              <div class="form-footer">
                <el-link type="info" :underline="false">忘记密码？</el-link>
                <span class="split">|</span>
                <el-link type="primary" :underline="false" @click="flipToRegister">
                  立即注册
                </el-link>
              </div>
            </el-form>
          </div>

          <!-- 右侧图片区域 -->
          <div class="auth-image">
            <img src="@/assets/images/todo_background.jpeg" alt="任务日历" />
          </div>
        </div>
      </div>

      <!-- 注册卡片 -->
      <div v-else key="register" class="auth-card">
        <div class="auth-content">
          <div class="auth-form-container">
            <div class="brand-info">
              <h1 class="brand-title">任务日历</h1>
              <p class="brand-desc">高效管理每一天</p>
            </div>

            <el-form
              ref="registerFormRef"
              :model="registerForm"
              :rules="registerRules"
              class="auth-form"
              @submit.prevent="handleRegister"
            >
              <div class="form-title">
                <h2>创建新账户</h2>
                <p>请填写以下信息完成注册</p>
              </div>

              <el-form-item prop="username">
                <el-input
                  v-model="registerForm.username"
                  placeholder="请输入用户名"
                  size="large"
                  prefix-icon="User"
                  class="form-input"
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
                  class="form-input"
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
                  class="form-input"
                />
              </el-form-item>

              <el-form-item prop="nickname">
                <el-input
                  v-model="registerForm.nickname"
                  placeholder="请输入昵称"
                  size="large"
                  prefix-icon="Avatar"
                  class="form-input"
                />
              </el-form-item>

              <el-form-item prop="email">
                <el-input
                  v-model="registerForm.email"
                  placeholder="请输入邮箱"
                  size="large"
                  prefix-icon="Message"
                  class="form-input"
                />
              </el-form-item>

              <el-form-item prop="captcha">
                <div class="captcha-container">
                  <el-input
                    v-model="registerForm.captcha"
                    placeholder="请输入验证码"
                    size="large"
                    class="form-input captcha-input"
                    @keyup.enter="handleRegister"
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
                  class="auth-button"
                  :loading="loading"
                  @click="handleRegister"
                >
                  {{ loading ? '注册中...' : '注册' }}
                </el-button>
              </el-form-item>

              <div class="form-footer">
                <span>已有账户？</span>
                <el-link type="primary" :underline="false" @click="flipToLogin">
                  立即登录
                </el-link>
              </div>
            </el-form>
          </div>

          <!-- 右侧图片区域 -->
          <div class="auth-image">
            <img src="@/assets/images/register.jpeg" alt="注册" />
          </div>
        </div>
      </div>
    </transition>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, watch } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useUserStore } from '@/stores/user'
import { getCaptcha } from '@/api/captcha'
import { ElMessage } from 'element-plus/es'
import { User, Lock } from '@element-plus/icons-vue'

const router = useRouter()
const route = useRoute()
const userStore = useUserStore()

const loginFormRef = ref()
const registerFormRef = ref()
const loading = ref(false)
const captchaImage = ref('')
const isRegister = ref(route.path === '/register')

// 登录表单
const loginForm = reactive({
  username: 'admin',
  password: '123456',
  captcha: ''
})

// 注册表单
const registerForm = reactive({
  username: '',
  password: '',
  confirmPassword: '',
  nickname: '',
  email: '',
  captcha: ''
})

// 登录表单验证规则
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

// 注册表单验证规则
const validateConfirmPassword = (_rule: any, value: string, callback: any) => {
  if (value === '') {
    callback(new Error('请确认密码'))
  } else if (value !== registerForm.password) {
    callback(new Error('两次输入密码不一致'))
  } else {
    callback()
  }
}

const registerRules = {
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

// 刷新验证码
const refreshCaptcha = async () => {
  try {
    captchaImage.value = await getCaptcha()
  } catch {
    ElMessage.error('获取验证码失败')
  }
}

// 处理登录
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

// 处理注册
const handleRegister = async () => {
  if (!registerFormRef.value) return
  const valid = await registerFormRef.value.validate()
  if (!valid) return

  loading.value = true
  try {
    await userStore.register(
      registerForm.username,
      registerForm.password,
      registerForm.nickname,
      registerForm.email,
      registerForm.captcha
    )
    ElMessage.success('注册成功，已为您登录，祝您使用愉快！')
    router.replace('/')
  } catch (error) {
    await refreshCaptcha()
  } finally {
    loading.value = false
  }
}

// 翻转到注册页
const flipToRegister = () => {
  isRegister.value = true
  router.push('/register')
}

// 翻转到登录页
const flipToLogin = () => {
  isRegister.value = false
  router.push('/login')
}

// 监听路由变化，切换登录/注册模式
watch(() => route.path, (newPath) => {
  isRegister.value = newPath === '/register'
})

// 初始化获取验证码
onMounted(() => refreshCaptcha())
</script>

<style scoped>
.auth-container {
  position: relative;
  width: 100%;
  height: 100vh;
  min-height: 700px;
  display: flex;
  justify-content: center;
  align-items: center;
  overflow: hidden;
}

/* 背景图设置 */
.background {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: url('@/assets/images/todo_background.jpeg') center/cover no-repeat fixed;
  z-index: -2;
}

/* 添加半透明遮罩层 */
.background::after {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(0, 0, 0, 0.4);
  backdrop-filter: blur(5px);
  z-index: -1;
}

/* 毛玻璃卡片 */
.auth-card {
  width: 1000px;
  height: 680px;
  background: rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.18);
  border-radius: 24px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
  padding: 32px;
  display: flex;
  flex-direction: column;
}

/* 内容容器 - 左右布局 */
.auth-content {
  display: flex;
  height: 100%;
  gap: 32px;
}

/* 表单容器 */
.auth-form-container {
  flex: 1;
  display: flex;
  flex-direction: column;
}

/* 表单包装器 */
.form-wrapper {
  flex: 1;
  display: flex;
  flex-direction: column;
  position: relative;
}

/* 右侧图片区域 */
.auth-image {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
}

.auth-image img {
  width: 100%;
  height: 100%;
  object-fit: cover;
  border-radius: 16px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
}

/* 品牌信息 */
.brand-info {
  text-align: center;
  margin-bottom: 32px;
}

.brand-title {
  font-size: 32px;
  font-weight: 700;
  margin: 0 0 8px;
  background: linear-gradient(120deg, #a8edea 0%, #fed6e3 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

.brand-desc {
  font-size: 14px;
  font-weight: 400;
  opacity: 0.8;
  margin: 0;
  color: #1e1b47;
}

/* 表单标题 */
.form-title {
  text-align: center;
  margin-bottom: 32px;
}

.form-title h2 {
  font-size: 24px;
  font-weight: 600;
  margin: 0 0 8px;
  color: #1e1b47;
}

.form-title p {
  font-size: 14px;
  margin: 0;
  opacity: 0.8;
  color: #1e1b47;
}

/* 表单样式 */
.auth-form {
  flex: 1;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
}

.form-input {
  border-radius: 12px;
}

.form-input :deep(.el-input__wrapper) {
  border-radius: 12px;
  background-color: rgba(255, 255, 255, 0.1);
  border: 1px solid rgba(255, 255, 255, 0.2);
  box-shadow: none;
}

.form-input :deep(.el-input__inner) {
  color: #1e1b47;
}

.form-input :deep(.el-input__inner::placeholder) {
  color: rgba(30, 27, 71, 0.6);
}

.form-input :deep(.el-input__prefix) {
  color: rgba(30, 27, 71, 0.6);
}

/* 验证码容器 */
.captcha-container {
  display: flex;
  align-items: center;
  gap: 12px;
}

.captcha-input {
  flex: 1;
}

.captcha-img {
  width: 120px;
  height: 40px;
  cursor: pointer;
  border-radius: 8px;
  overflow: hidden;
  display: flex;
  justify-content: center;
  align-items: center;
  background: rgba(255, 255, 255, 0.1);
  border: 1px solid rgba(255, 255, 255, 0.2);
}

.captcha-img img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

/* 按钮样式 */
.auth-button {
  width: 100%;
  height: 48px;
  border-radius: 12px;
  font-weight: 600;
  margin-top: 12px;
  background: linear-gradient(45deg, #a8edea 0%, #fed6e3 100%);
  border: none;
  color: #333;
}

.auth-button:hover {
  opacity: 0.9;
}

/* 表单底部 */
.form-footer {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  margin-top: 16px;
  font-size: 14px;
  color: #1e1b47;
}

.split {
  color: #1e1b47;
}

/* 表单项样式优化 */
.el-form-item {
  margin-bottom: 20px;
}

/* 链接样式 */
:deep(.el-link) {
  font-weight: 500;
}

/* 动态球体 */
.floating-balls {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  pointer-events: none;
  z-index: 0;
  overflow: hidden;
}

.ball {
  position: absolute;
  border-radius: 50%;
  filter: blur(1px);
  pointer-events: auto;
  transition: all 0.6s cubic-bezier(0.4, 0, 0.2, 1);
  animation: float 8s ease-in-out infinite;
  opacity: 0.7;
}

.ball:hover {
  transform: scale(1.5);
  filter: blur(0.5px);
  opacity: 0.9;
  z-index: 1;
}

.ball-1 {
  width: 120px;
  height: 120px;
  top: 8%;
  left: 5%;
  animation-delay: 0s;
  background: linear-gradient(135deg, rgba(76, 175, 80, 0.3), rgba(139, 195, 74, 0.3));
}

.ball-2 {
  width: 140px;
  height: 140px;
  top: 70%;
  left: 80%;
  animation-delay: 1.5s;
  background: linear-gradient(135deg, rgba(67, 160, 71, 0.3), rgba(255, 152, 0, 0.3));
}

.ball-3 {
  width: 110px;
  height: 110px;
  top: 25%;
  left: 75%;
  animation-delay: 3s;
  background: linear-gradient(135deg, rgba(56, 142, 60, 0.3), rgba(255, 87, 34, 0.3));
}

.ball-4 {
  width: 130px;
  height: 130px;
  top: 60%;
  left: 10%;
  animation-delay: 4.5s;
  background: linear-gradient(135deg, rgba(46, 125, 50, 0.3), rgba(233, 30, 99, 0.3));
}

.ball-5 {
  width: 150px;
  height: 150px;
  top: 15%;
  left: 85%;
  animation-delay: 6s;
  background: linear-gradient(135deg, rgba(27, 94, 32, 0.3), rgba(156, 39, 176, 0.3));
}

.ball-6 {
  width: 100px;
  height: 100px;
  top: 75%;
  left: 65%;
  animation-delay: 7.5s;
  background: linear-gradient(135deg, rgba(104, 159, 56, 0.3), rgba(255, 193, 7, 0.3));
}

.ball-7 {
  width: 125px;
  height: 125px;
  top: 40%;
  left: 20%;
  animation-delay: 9s;
  background: linear-gradient(135deg, rgba(85, 139, 47, 0.3), rgba(255, 235, 59, 0.3));
}

.ball-8 {
  width: 135px;
  height: 135px;
  top: 5%;
  left: 55%;
  animation-delay: 10.5s;
  background: linear-gradient(135deg, rgba(124, 179, 66, 0.3), rgba(255, 112, 67, 0.3));
}

.ball-9 {
  width: 115px;
  height: 115px;
  top: 85%;
  left: 35%;
  animation-delay: 12s;
  background: linear-gradient(135deg, rgba(0, 150, 136, 0.3), rgba(255, 64, 129, 0.3));
}

.ball-10 {
  width: 145px;
  height: 145px;
  top: 30%;
  left: 40%;
  animation-delay: 13.5s;
  background: linear-gradient(135deg, rgba(0, 188, 212, 0.3), rgba(255, 23, 68, 0.3));
}

@keyframes float {
  0%, 100% {
    transform: translateY(0) translateX(0) scale(1);
  }
  25% {
    transform: translateY(-25px) translateX(20px) scale(1.1);
  }
  50% {
    transform: translateY(15px) translateX(-15px) scale(0.95);
  }
  75% {
    transform: translateY(-20px) translateX(-20px) scale(1.05);
  }
}

/* 翻转动画 - 整个卡片翻转 */
.flip-enter-active,
.flip-leave-active {
  transition: all 0.6s ease;
}

.flip-enter-from {
  opacity: 0;
  transform: rotateY(90deg);
}

.flip-leave-to {
  opacity: 0;
  transform: rotateY(-90deg);
}

.flip-enter-to,
.flip-leave-from {
  opacity: 1;
  transform: rotateY(0deg);
}
</style>