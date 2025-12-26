<template>
  <div class="settings-container">
    <div class="settings-card">
      <h1 class="settings-title">账户设置</h1>
      
      <!-- 用户信息概览 -->
      <div class="user-info-overview">
        <el-avatar :size="64" :src="userStore.userInfo?.avatar" />
        <div class="user-details">
          <h3>{{ userStore.userInfo?.nickname || '用户' }}</h3>
          <p>用户名: {{ userStore.userInfo?.username }}</p>
          <p>邮箱: {{ userStore.userInfo?.email || '未设置' }}</p>
        </div>
      </div>
      
      <!-- Tab标签页 -->
      <el-tabs v-model="activeTab" class="settings-tabs">
        <!-- 更改密码标签页 -->
        <el-tab-pane label="更改密码" name="changePassword">
          <div class="tab-content">
            <h3 class="tab-title">更改密码</h3>
            <p class="tab-description">修改您的登录密码</p>
            
            <el-form 
              ref="changePasswordFormRef" 
              :model="changePasswordForm" 
              :rules="changePasswordRules" 
              label-width="120px"
              class="settings-form"
            >
              <el-form-item label="原密码" prop="oldPassword">
                <el-input 
                  v-model="changePasswordForm.oldPassword" 
                  type="password" 
                  placeholder="请输入原密码"
                  show-password
                />
              </el-form-item>
              
              <el-form-item label="新密码" prop="newPassword">
                <el-input 
                  v-model="changePasswordForm.newPassword" 
                  type="password" 
                  placeholder="请输入新密码"
                  show-password
                />
                <div class="password-strength">
                  <span :class="passwordStrength.class">{{ passwordStrength.text }}</span>
                </div>
              </el-form-item>
              
              <el-form-item label="确认密码" prop="confirmPassword">
                <el-input 
                  v-model="changePasswordForm.confirmPassword" 
                  type="password" 
                  placeholder="请再次输入新密码"
                  show-password
                />
              </el-form-item>
              
              <el-form-item>
                <el-button 
                  type="primary" 
                  :loading="changePasswordLoading"
                  @click="handleChangePassword"
                  size="large"
                >
                  确认修改
                </el-button>
              </el-form-item>
            </el-form>
          </div>
        </el-tab-pane>
        
        <!-- 重置密码标签页 -->
        <el-tab-pane label="重置密码" name="resetPassword">
          <div class="tab-content">
            <h3 class="tab-title">重置密码</h3>
            <p class="tab-description">通过邮箱验证码重置您的密码</p>
            
            <el-form 
              ref="resetPasswordFormRef" 
              :model="resetPasswordForm" 
              :rules="resetPasswordRules" 
              label-width="120px"
              class="settings-form"
            >
              <el-form-item label="绑定邮箱">
                <div class="current-email-info">
                  <span class="email-label">{{ userStore.userInfo?.email || '未设置邮箱' }}</span>
                  <span v-if="userStore.userInfo?.email" class="email-tip">验证码将发送到此邮箱</span>
                  <span v-else class="email-warning">请先绑定邮箱才能使用此功能</span>
                </div>
              </el-form-item>
              
              <el-form-item label="验证码" prop="code">
                <div class="code-input-group">
                  <el-input 
                    v-model="resetPasswordForm.code" 
                    placeholder="请输入验证码"
                    style="width: 200px; margin-right: 10px;"
                  />
                  <el-button 
                    :disabled="codeCountdown > 0"
                    @click="sendResetPasswordCode"
                  >
                    {{ codeCountdown > 0 ? `${codeCountdown}秒后重试` : '获取验证码' }}
                  </el-button>
                </div>
              </el-form-item>
              
              <el-form-item label="新密码" prop="newPassword">
                <el-input 
                  v-model="resetPasswordForm.newPassword" 
                  type="password" 
                  placeholder="请输入新密码"
                  show-password
                />
              </el-form-item>
              
              <el-form-item label="确认密码" prop="confirmPassword">
                <el-input 
                  v-model="resetPasswordForm.confirmPassword" 
                  type="password" 
                  placeholder="请再次输入新密码"
                  show-password
                />
              </el-form-item>
              
              <el-form-item>
                <el-button 
                  type="primary" 
                  :loading="resetPasswordLoading"
                  @click="handleResetPassword"
                  size="large"
                >
                  重置密码
                </el-button>
              </el-form-item>
            </el-form>
          </div>
        </el-tab-pane>
        
        <!-- 更换邮箱标签页 -->
        <el-tab-pane label="更换邮箱" name="changeEmail">
          <div class="tab-content">
            <h3 class="tab-title">更换邮箱</h3>
            <p class="tab-description">更换您绑定的邮箱地址</p>
            
            <div class="current-email">
              <span>当前邮箱：</span>
              <strong>{{ userStore.userInfo?.email || '未设置' }}</strong>
            </div>
            
            <el-form 
              ref="changeEmailFormRef" 
              :model="changeEmailForm" 
              :rules="changeEmailRules" 
              label-width="120px"
              class="settings-form"
            >
              <el-form-item label="新邮箱" prop="newEmail">
                <el-input 
                  v-model="changeEmailForm.newEmail" 
                  placeholder="请输入新的邮箱地址"
                />
              </el-form-item>
              
              <el-form-item label="验证码" prop="code">
                <div class="code-input-group">
                  <el-input 
                    v-model="changeEmailForm.code" 
                    placeholder="请输入验证码"
                    style="width: 200px; margin-right: 10px;"
                  />
                  <el-button 
                    :disabled="emailCodeCountdown > 0"
                    @click="sendChangeEmailCode"
                  >
                    {{ emailCodeCountdown > 0 ? `${emailCodeCountdown}秒后重试` : '获取验证码' }}
                  </el-button>
                </div>
              </el-form-item>
              
              <el-form-item>
                <el-button 
                  type="primary" 
                  :loading="changeEmailLoading"
                  @click="handleChangeEmail"
                  size="large"
                >
                  更换邮箱
                </el-button>
              </el-form-item>
            </el-form>
          </div>
        </el-tab-pane>
      </el-tabs>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage } from 'element-plus'
import { useUserStore } from '@/stores/user'
import { sendEmailCode, changePassword, resetPassword, changeEmail } from '@/api/user'

type FormInstance = any
type FormRules = any

const router = useRouter()
const userStore = useUserStore()

// 当前激活的Tab
const activeTab = ref('changePassword')

// 表单引用
const changePasswordFormRef = ref<FormInstance>()
const resetPasswordFormRef = ref<FormInstance>()
const changeEmailFormRef = ref<FormInstance>()

// 加载状态
const changePasswordLoading = ref(false)
const resetPasswordLoading = ref(false)
const changeEmailLoading = ref(false)

// 验证码倒计时
const codeCountdown = ref(0)
const emailCodeCountdown = ref(0)

// 更改密码表单 - 使用reactive直接定义
const changePasswordForm = reactive({
  oldPassword: '',
  newPassword: '',
  confirmPassword: ''
})

// 重置密码表单
const resetPasswordForm = reactive({
  code: '',
  newPassword: '',
  confirmPassword: ''
})

// 更换邮箱表单
const changeEmailForm = reactive({
  newEmail: '',
  code: ''
})

// 密码强度计算
const passwordStrength = computed(() => {
  const password = changePasswordForm.newPassword
  if (!password) return { class: '', text: '' }
  
  let strength = 0
  if (password.length >= 8) strength++
  if (/[a-z]/.test(password)) strength++
  if (/[A-Z]/.test(password)) strength++
  if (/\d/.test(password)) strength++
  if (/[^a-zA-Z\d]/.test(password)) strength++
  
  const levels = [
    { class: 'weak', text: '弱' },
    { class: 'weak', text: '弱' },
    { class: 'medium', text: '中' },
    { class: 'medium', text: '中' },
    { class: 'strong', text: '强' },
    { class: 'strong', text: '强' }
  ]
  
  return levels[Math.min(strength, 5)]
})

// 表单验证规则
const changePasswordRules: FormRules = {
  oldPassword: [
    { required: true, message: '请输入原密码', trigger: 'blur' }
  ],
  newPassword: [
    { required: true, message: '请输入新密码', trigger: 'blur' },
    { min: 6, message: '密码长度不能少于6位', trigger: 'blur' }
  ],
  confirmPassword: [
    { required: true, message: '请确认新密码', trigger: 'blur' },
    {
      validator: (_rule: any, value: string, callback: any) => {
        if (value !== changePasswordForm.newPassword) {
          callback(new Error('两次输入的密码不一致'))
        } else {
          callback()
        }
      },
      trigger: 'blur'
    }
  ]
}

const resetPasswordRules: FormRules = {
  code: [
    { required: true, message: '请输入验证码', trigger: 'blur' },
    { len: 6, message: '验证码必须为6位', trigger: 'blur' }
  ],
  newPassword: [
    { required: true, message: '请输入新密码', trigger: 'blur' },
    { min: 6, message: '密码长度不能少于6位', trigger: 'blur' }
  ],
  confirmPassword: [
    { required: true, message: '请确认新密码', trigger: 'blur' },
    {
      validator: (_rule: any, value: string, callback: any) => {
        if (value !== resetPasswordForm.newPassword) {
          callback(new Error('两次输入的密码不一致'))
        } else {
          callback()
        }
      },
      trigger: 'blur'
    }
  ]
}

const changeEmailRules: FormRules = {
  newEmail: [
    { required: true, message: '请输入新邮箱地址', trigger: 'blur' },
    { type: 'email', message: '请输入正确的邮箱格式', trigger: 'blur' }
  ],
  code: [
    { required: true, message: '请输入验证码', trigger: 'blur' },
    { len: 6, message: '验证码必须为6位', trigger: 'blur' }
  ]
}

// 发送重置密码验证码
const sendResetPasswordCode = async () => {
  try {
    // 检查用户是否已绑定邮箱
    if (!userStore.userInfo?.email) {
      ElMessage.error('请先绑定邮箱才能使用此功能')
      return
    }
    
    await sendEmailCode({
      email: userStore.userInfo.email,
      type: 'RESET_PASSWORD'
    })
    
    ElMessage.success(`验证码已发送至 ${userStore.userInfo.email}`)
    startCountdown('reset')
  } catch (error) {
    ElMessage.error('验证码发送失败')
  }
}

// 发送更换邮箱验证码
const sendChangeEmailCode = async () => {
  try {
    await changeEmailFormRef.value?.validateField('newEmail')
    
    await sendEmailCode({
      email: changeEmailForm.newEmail,
      type: 'CHANGE_EMAIL'
    })
    
    ElMessage.success('验证码发送成功')
    startCountdown('email')
  } catch (error) {
    ElMessage.error('验证码发送失败')
  }
}

// 开始倒计时
const startCountdown = (type: 'reset' | 'email') => {
  const countdown = type === 'reset' ? codeCountdown : emailCodeCountdown
  countdown.value = 60
  
  const timer = setInterval(() => {
    countdown.value--
    if (countdown.value <= 0) {
      clearInterval(timer)
    }
  }, 1000)
}

// 更改密码
const handleChangePassword = async () => {
  try {
    await changePasswordFormRef.value?.validate()
    
    changePasswordLoading.value = true
    await changePassword({
      oldPassword: changePasswordForm.oldPassword,
      newPassword: changePasswordForm.newPassword,
      confirmPassword: changePasswordForm.confirmPassword
    })
    
    ElMessage.success('密码修改成功')
    
    // 清空表单
    changePasswordForm.oldPassword = ''
    changePasswordForm.newPassword = ''
    changePasswordForm.confirmPassword = ''
  } catch (error) {
    ElMessage.error('密码修改失败')
  } finally {
    changePasswordLoading.value = false
  }
}

// 重置密码
const handleResetPassword = async () => {
  try {
    await resetPasswordFormRef.value?.validate()
    
    resetPasswordLoading.value = true
    await resetPassword({
      code: resetPasswordForm.code,
      newPassword: resetPasswordForm.newPassword,
      confirmPassword: resetPasswordForm.confirmPassword
    })
    
    ElMessage.success('密码重置成功')
    
    // 清空表单
    resetPasswordForm.code = ''
    resetPasswordForm.newPassword = ''
    resetPasswordForm.confirmPassword = ''
  } catch (error) {
    ElMessage.error('密码重置失败')
  } finally {
    resetPasswordLoading.value = false
  }
}

// 更换邮箱
const handleChangeEmail = async () => {
  try {
    await changeEmailFormRef.value?.validate()
    
    changeEmailLoading.value = true
    await changeEmail({
      newEmail: changeEmailForm.newEmail,
      code: changeEmailForm.code
    })
    
    ElMessage.success('邮箱更换成功')
    
    // 更新用户信息
    await userStore.initUser()
    
    // 清空表单
    changeEmailForm.newEmail = ''
    changeEmailForm.code = ''
  } catch (error) {
    ElMessage.error('邮箱更换失败')
  } finally {
    changeEmailLoading.value = false
  }
}

onMounted(() => {
  // 页面加载时初始化数据
})
</script>

<style scoped>
.settings-container {
  max-width: 800px;
  margin: 0 auto;
  padding: 20px;
}

.settings-card {
  background: #fff;
  border-radius: 12px;
  padding: 40px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
}

.settings-title {
  font-size: 28px;
  font-weight: 600;
  color: #303133;
  margin-bottom: 30px;
  text-align: center;
}

/* 用户信息概览 */
.user-info-overview {
  display: flex;
  align-items: center;
  margin-bottom: 40px;
  padding: 24px;
  background: linear-gradient(135deg, #f5f7fa 0%, #e4e7ed 100%);
  border-radius: 8px;
  gap: 20px;
}

.user-details h3 {
  font-size: 20px;
  font-weight: 600;
  color: #303133;
  margin-bottom: 8px;
}

.user-details p {
  font-size: 14px;
  color: #606266;
  margin-bottom: 4px;
}

/* Tab标签页样式 */
.settings-tabs {
  margin-top: 20px;
}

:deep(.el-tabs__header) {
  margin-bottom: 30px;
}

:deep(.el-tabs__nav-wrap::after) {
  background-color: #ebeef5;
}

:deep(.el-tabs__item) {
  font-size: 16px;
  font-weight: 500;
  padding: 0 24px;
  height: 48px;
  line-height: 48px;
}

:deep(.el-tabs__item.is-active) {
  color: #409eff;
}

:deep(.el-tabs__active-bar) {
  background-color: #409eff;
  height: 3px;
}

.tab-content {
  padding: 0 20px;
}

.tab-title {
  font-size: 20px;
  font-weight: 600;
  color: #303133;
  margin-bottom: 8px;
}

.tab-description {
  font-size: 14px;
  color: #606266;
  margin-bottom: 30px;
}

.settings-form {
  max-width: 500px;
}

.code-input-group {
  display: flex;
  align-items: center;
}

.current-email {
  margin-bottom: 20px;
  padding: 12px 16px;
  background-color: #f5f7fa;
  border-radius: 4px;
  font-size: 14px;
  border-left: 4px solid #409eff;
}

.current-email-info {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.email-label {
  font-weight: 500;
  color: #303133;
  font-size: 14px;
}

.email-tip {
  color: #67c23a;
  font-size: 12px;
}

.email-warning {
  color: #f56c6c;
  font-size: 12px;
}

.password-strength {
  margin-top: 8px;
  font-size: 12px;
}

.password-strength .weak {
  color: #f56c6c;
}

.password-strength .medium {
  color: #e6a23c;
}

.password-strength .strong {
  color: #67c23a;
}

:deep(.el-form-item__label) {
  font-weight: 500;
}

:deep(.el-button--large) {
  padding: 12px 24px;
  font-size: 14px;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .settings-container {
    padding: 10px;
  }
  
  .settings-card {
    padding: 20px;
  }
  
  .user-info-overview {
    flex-direction: column;
    text-align: center;
    gap: 15px;
  }
  
  :deep(.el-form-item__label) {
    width: 100px !important;
  }
  
  .code-input-group {
    flex-direction: column;
    gap: 10px;
  }
  
  .code-input-group .el-input {
    width: 100% !important;
  }
}
</style>