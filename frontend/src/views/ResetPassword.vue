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
    
    <!-- 重置密码卡片 -->
    <div class="auth-card">
      <div class="auth-content">
        <div class="auth-form-container">
          <div class="brand-info">
            <h1 class="brand-title">任务日历</h1>
            <p class="brand-desc">重置密码</p>
          </div>

          <el-form
            ref="resetFormRef"
            :model="resetForm"
            :rules="resetRules"
            class="auth-form"
            @submit.prevent="handleReset"
          >
            <div class="form-title">
              <h2>重置密码</h2>
              <p>请输入用户名，系统将向您的注册邮箱发送验证码</p>
            </div>

            <el-form-item prop="username">
              <el-input
                v-model="resetForm.username"
                placeholder="请输入用户名"
                size="large"
                prefix-icon="User"
                class="form-input"
                @blur="handleUsernameBlur"
              />
            </el-form-item>

            <!-- 邮箱显示区域 -->
            <el-form-item v-if="userEmail" class="email-display">
              <div class="email-info">
                <span class="email-label">注册邮箱：</span>
                <span class="email-value">{{ userEmail }}</span>
              </div>
            </el-form-item>

            <el-form-item prop="captcha">
              <div class="captcha-container">
                <el-input
                  v-model="resetForm.captcha"
                  placeholder="请输入验证码"
                  size="large"
                  class="form-input captcha-input"
                />
                <el-button 
                  type="primary" 
                  size="large" 
                  :loading="sendingCaptcha"
                  :disabled="!resetForm.username"
                  @click="sendCaptcha"
                  class="captcha-btn"
                >
                  {{ sendingCaptcha ? '发送中...' : '发送验证码' }}
                </el-button>
              </div>
            </el-form-item>

            <el-form-item>
              <el-button
                type="primary"
                size="large"
                class="auth-button"
                :loading="loading"
                @click="handleReset"
              >
                {{ loading ? '验证中...' : '验证并重置密码' }}
              </el-button>
            </el-form-item>

            <!-- 随机密码显示区域（验证成功后显示） -->
            <el-form-item v-if="generatedPassword" class="password-display">
              <div class="password-info">
                <span class="password-label">新密码：</span>
                <span class="password-value">{{ generatedPassword }}</span>
                <el-button 
                  type="success" 
                  size="small" 
                  @click="copyToClipboard"
                  class="copy-btn"
                >
                  复制
                </el-button>
              </div>
            </el-form-item>

            <!-- 跳转登录按钮（验证成功后显示） -->
            <el-form-item v-if="generatedPassword">
              <el-button
                type="success"
                size="large"
                class="auth-button"
                @click="goToLogin"
              >
                前往登录
              </el-button>
            </el-form-item>

            <div class="form-footer">
              <el-link type="primary" :underline="false" @click="goToLogin">
                返回登录
              </el-link>
            </div>
          </el-form>
        </div>

        <!-- 右侧图片区域 -->
        <div class="auth-image">
          <img src="@/assets/images/todo_background.jpeg" alt="重置密码" />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue'
import { useRouter } from 'vue-router'
import { sendEmailCode, resetPasswordByUsername, getUserByUsername } from '@/api/user'
import { ElMessage } from 'element-plus'

const router = useRouter()

const resetFormRef = ref()
const loading = ref(false)
const sendingCaptcha = ref(false)
const userEmail = ref('')
const generatedPassword = ref('')

// 重置密码表单
const resetForm = reactive({
  username: '',
  captcha: ''
})

// 生成12位随机密码
const generateRandomPassword = (): string => {
  const chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*'
  let password = ''
  for (let i = 0; i < 12; i++) {
    password += chars.charAt(Math.floor(Math.random() * chars.length))
  }
  return password
}

// 重置密码表单验证规则
const resetRules = {
  username: [
    { required: true, message: '请输入用户名', trigger: 'blur' },
    { min: 3, max: 20, message: '用户名长度在 3 到 20 个字符', trigger: 'blur' }
  ],
  captcha: [
    { required: true, message: '请输入验证码', trigger: 'blur' }
  ]
}

// 用户名失焦时获取用户邮箱
const handleUsernameBlur = async () => {
  if (!resetForm.username) return
  
  try {
    const userInfo = await getUserByUsername(resetForm.username)
    if (userInfo && userInfo.email) {
      userEmail.value = userInfo.email
      // 清空之前可能生成的密码
      generatedPassword.value = ''
    } else {
      userEmail.value = ''
      generatedPassword.value = ''
      ElMessage.warning('未找到该用户名对应的用户信息')
    }
  } catch (error) {
    userEmail.value = ''
    generatedPassword.value = ''
    ElMessage.error('获取用户信息失败，请检查用户名是否正确')
  }
}

// 发送验证码
const sendCaptcha = async () => {
  if (!resetForm.username) {
    ElMessage.warning('请输入用户名')
    return
  }
  
  if (!userEmail.value) {
    ElMessage.warning('请先输入用户名获取用户信息')
    return
  }
  
  sendingCaptcha.value = true
  try {
    // 调用sendEmailCode接口发送验证码
    await sendEmailCode({
      email: userEmail.value,
      type: 'RESET_PASSWORD'
    })
    
    ElMessage.success(`验证码已发送到您的邮箱：${userEmail.value}`)
  } catch (error) {
    ElMessage.error('发送验证码失败，请检查用户信息是否正确')
  } finally {
    sendingCaptcha.value = false
  }
}

// 复制密码到剪贴板
const copyToClipboard = async () => {
  try {
    await navigator.clipboard.writeText(generatedPassword.value)
    ElMessage.success('密码已复制到剪贴板')
  } catch (error) {
    // 降级方案：使用document.execCommand
    const textArea = document.createElement('textarea')
    textArea.value = generatedPassword.value
    document.body.appendChild(textArea)
    textArea.select()
    document.execCommand('copy')
    document.body.removeChild(textArea)
    ElMessage.success('密码已复制到剪贴板')
  }
}

// 验证验证码
const verifyCaptcha = async () => {
  if (!resetFormRef.value) return false
  
  const valid = await resetFormRef.value.validate()
  if (!valid) return false
  
  if (!userEmail.value) {
    ElMessage.warning('请先输入用户名获取用户信息')
    return false
  }
  
  return true
}

// 处理密码重置
const handleReset = async () => {
  if (!await verifyCaptcha()) return
  
  loading.value = true
  try {
    // 调用新的resetPasswordByUsername API
    const response = await resetPasswordByUsername({
      username: resetForm.username,
      code: resetForm.captcha
    })
    
    // 使用后端生成的随机密码
    const newPassword = response.newPassword || generateRandomPassword()
    
    // 显示新密码
    generatedPassword.value = newPassword
    
    // 自动复制密码到剪贴板
    await copyToClipboard()
    
    ElMessage.success(`密码重置成功！新密码已自动复制到剪贴板：${newPassword}`)
    
    // 重置成功后跳转到登录页
    setTimeout(() => {
      router.push('/login')
    }, 3000)
  } catch (error) {
    ElMessage.error('重置密码失败，请检查验证码是否正确')
  } finally {
    loading.value = false
  }
}

// 返回登录页
const goToLogin = () => {
  router.push('/login')
}
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

.captcha-btn {
  width: 120px;
  height: 40px;
  border-radius: 8px;
}

/* 邮箱显示样式 */
.email-display {
  margin-bottom: 0 !important;
}

.email-info {
  display: flex;
  align-items: center;
  padding: 8px 12px;
  background: rgba(255, 255, 255, 0.1);
  border-radius: 8px;
  border: 1px solid rgba(255, 255, 255, 0.2);
}

.email-label {
  font-size: 14px;
  color: #1e1b47;
  font-weight: 500;
  margin-right: 8px;
}

.email-value {
  font-size: 14px;
  color: #1e1b47;
  font-weight: 600;
}

/* 密码显示样式 */
.password-display {
  margin-bottom: 0 !important;
}

.password-info {
  display: flex;
  align-items: center;
  padding: 8px 12px;
  background: rgba(255, 255, 255, 0.1);
  border-radius: 8px;
  border: 1px solid rgba(255, 255, 255, 0.2);
}

.password-label {
  font-size: 14px;
  color: #1e1b47;
  font-weight: 500;
  margin-right: 8px;
}

.password-value {
  font-size: 14px;
  color: #1e1b47;
  font-weight: 600;
  flex: 1;
  font-family: monospace;
}

.copy-btn {
  margin-left: 8px;
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
</style>