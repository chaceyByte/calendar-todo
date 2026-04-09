<template>
  <div class="general-settings-content">
    <!-- 页面标题 -->
    <div class="page-header">
      <h2 class="page-title">General Settings</h2>
      <p class="page-desc">Manage your application preferences and sanctuary environment.</p>
    </div>

    <div class="settings-grid">
      <!-- 左侧设置区域 -->
      <div class="settings-left">
        <!-- 外观主题设置 -->
        <section class="settings-section theme-section">
          <h3 class="section-label">Appearance Theme</h3>
          <div class="theme-selector">
            <button
              v-for="theme in themeOptions"
              :key="theme.value"
              :class="['theme-card', { active: configStore.theme === theme.value }]"
              @click="handleThemeChange(theme.value)"
              :disabled="configStore.isLoading"
            >
              <div class="theme-icon" :class="theme.value">
                <el-icon v-if="theme.value === 'light'" :size="24"><Sunny /></el-icon>
                <el-icon v-else-if="theme.value === 'dark'" :size="24"><Moon /></el-icon>
                <el-icon v-else :size="24"><Monitor /></el-icon>
              </div>
              <span class="theme-name">{{ theme.label }}</span>
              <div v-if="configStore.theme === theme.value" class="check-mark">
                <el-icon><Check /></el-icon>
              </div>
            </button>
          </div>
        </section>

        <!-- 数据库存储路径设置 -->
        <section class="settings-section path-section">
          <h3 class="section-label">Database Storage Path</h3>
          <div class="path-input-group">
            <div class="path-display">
              <span class="path-text">{{ displayDbPath }}</span>
              <el-tag v-if="dbPathStatus" :type="dbPathStatus.type" size="small" class="path-status">
                {{ dbPathStatus.text }}
              </el-tag>
            </div>
            <el-button 
              class="browse-btn" 
              @click="handleBrowsePath"
              :loading="isValidatingPath"
              :disabled="configStore.isDbSwitching"
            >
              <el-icon><FolderOpened /></el-icon>
              <span>Browse Path</span>
            </el-button>
          </div>
          <p class="path-hint">
            {{ pathHint }}
          </p>
          
          <!-- 数据库状态指示器 -->
          <div class="db-status-indicator">
            <el-tag v-if="configStore.isDbSwitching" type="warning" effect="dark">
              <el-icon class="is-loading"><Loading /></el-icon>
              正在切换数据库...
            </el-tag>
            <el-tag v-else-if="configStore.isDbReady" type="success" effect="plain">
              <el-icon><CircleCheck /></el-icon>
              数据库连接正常
            </el-tag>
            <el-tag v-else-if="configStore.dbStatus.error" type="danger" effect="plain">
              <el-icon><CircleClose /></el-icon>
              {{ configStore.dbStatus.error }}
            </el-tag>
          </div>
        </section>

        <!-- 自动备份设置 -->
        <section class="settings-section backup-section">
          <div class="backup-header">
            <div class="backup-title-group">
              <div class="backup-icon">
                <el-icon><Upload /></el-icon>
              </div>
              <h3 class="section-label">Auto-Backup</h3>
            </div>
            <el-switch
              v-model="autoBackupEnabled"
              :loading="configStore.isLoading"
              @change="handleAutoBackupChange"
            />
          </div>
          
          <div v-if="autoBackupEnabled" class="backup-config">
            <div class="config-row">
              <span class="config-label">Frequency</span>
              <el-select 
                v-model="backupFrequency" 
                size="small" 
                style="width: 120px"
                @change="handleBackupConfigChange"
              >
                <el-option label="Every 1 Hour" value="1h" />
                <el-option label="Every 6 Hours" value="6h" />
                <el-option label="Every 12 Hours" value="12h" />
                <el-option label="Every Day" value="24h" />
                <el-option label="Every Week" value="7d" />
              </el-select>
            </div>
            <div class="config-row">
              <span class="config-label">Max Snapshots</span>
              <el-input-number 
                v-model="maxSnapshots" 
                :min="1" 
                :max="100" 
                size="small"
                @change="handleBackupConfigChange"
              />
            </div>
            <div class="config-row">
              <span class="config-label">Cloud Sync</span>
              <span class="config-value status-disconnected">
                <span class="status-dot"></span>
                Disconnected
              </span>
            </div>
            <el-button 
              type="primary" 
              class="backup-btn"
              :loading="configStore.isLoading"
            >
              Run Backup Now
            </el-button>
          </div>
        </section>
      </div>
    </div>

    <!-- 全局 Loading 遮罩 -->
    <el-dialog
      v-model="showLoadingDialog"
      :show-close="false"
      :close-on-click-modal="false"
      :close-on-press-escape="false"
      width="300px"
      align-center
      class="loading-dialog"
    >
      <div class="loading-content">
        <el-icon :size="48" class="is-loading"><Loading /></el-icon>
        <p class="loading-text">{{ configStore.loadingMessage }}</p>
      </div>
    </el-dialog>

    <!-- 数据库切换确认对话框 -->
    <el-dialog
      v-model="showSwitchConfirm"
      title="确认切换数据库"
      width="400px"
      align-center
    >
      <div class="confirm-content">
        <el-icon :size="48" color="#E6A23C"><Warning /></el-icon>
        <p>确定要切换到新的数据库吗？</p>
        <p class="confirm-path">{{ pendingDbPath }}</p>
        <el-alert
          title="切换后应用将重新加载"
          type="warning"
          :closable="false"
          show-icon
        />
      </div>
      <template #footer>
        <el-button @click="showSwitchConfirm = false">取消</el-button>
        <el-button type="primary" @click="confirmSwitchDatabase" :loading="configStore.isLoading">
          确认切换
        </el-button>
      </template>
    </el-dialog>

    <!-- 错误提示 -->
    <el-alert
      v-if="configStore.lastError"
      :title="configStore.lastError"
      type="error"
      closable
      @close="configStore.clearError"
      class="error-alert"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { useConfigStore, type ThemeMode } from '../stores/config'
import { open } from '@tauri-apps/plugin-dialog'
import {
  Sunny,
  Moon,
  Monitor,
  Check,
  FolderOpened,
  Upload,
  Picture,
  InfoFilled,
  Loading,
  CircleCheck,
  CircleClose,
  Warning
} from '@element-plus/icons-vue'
import { ElMessage } from 'element-plus'

// 配置 store
const configStore = useConfigStore()

// 主题选项
const themeOptions = [
  { value: 'light' as ThemeMode, label: 'Light' },
  { value: 'dark' as ThemeMode, label: 'Dark' },
  { value: 'system' as ThemeMode, label: 'System' }
]

// 自动备份本地状态
const autoBackupEnabled = ref(false)
const backupFrequency = ref<'1h' | '6h' | '12h' | '24h' | '7d'>('12h')
const maxSnapshots = ref(15)

// 路径相关状态
const isValidatingPath = ref(false)
const pathValidationResult = ref<{ valid: boolean; message: string } | null>(null)
const pendingDbPath = ref('')
const showSwitchConfirm = ref(false)

// 显示的数据库路径
const displayDbPath = computed(() => {
  if (configStore.dbPath) {
    return configStore.dbPath
  }
  return 'Using default location'
})

// 路径状态标签
const dbPathStatus = computed(() => {
  if (pathValidationResult.value) {
    return {
      type: (pathValidationResult.value.valid ? 'success' : 'danger') as 'success' | 'danger',
      text: pathValidationResult.value.message
    }
  }
  if (configStore.dbPath) {
    return {
      type: 'info' as const,
      text: 'Custom path'
    }
  }
  return null
})

// 路径提示
const pathHint = computed(() => {
  if (configStore.isDbSwitching) {
    return '正在切换数据库，请稍候...'
  }
  if (pathValidationResult.value) {
    return pathValidationResult.value.message
  }
  return 'Select a custom location for your database file. Changes will take effect after restart.'
})

// Loading 对话框显示控制
const showLoadingDialog = computed(() => {
  return configStore.isLoading && configStore.loadingMessage.includes('切换')
})

// 初始化
onMounted(async () => {
  // 从 store 同步备份配置
  syncBackupConfigFromStore()
})

// 监听 store 配置变化
watch(() => configStore.autoBackup, () => {
  syncBackupConfigFromStore()
}, { deep: true })

// 同步备份配置
function syncBackupConfigFromStore() {
  const backup = configStore.autoBackup
  autoBackupEnabled.value = backup.enabled
  backupFrequency.value = backup.frequency
  maxSnapshots.value = backup.max_snapshots
}

// 处理主题变更
async function handleThemeChange(theme: ThemeMode) {
  try {
    await configStore.updateTheme(theme)
    ElMessage.success('主题已更新')
  } catch (error) {
    ElMessage.error('主题更新失败')
  }
}

// 处理浏览路径
async function handleBrowsePath() {
  try {
    isValidatingPath.value = true
    pathValidationResult.value = null

    // 使用 Tauri 的文件对话框
    const selected = await open({
      directory: false,
      multiple: false,
      filters: [
        {
          name: 'SQLite Database',
          extensions: ['db', 'sqlite', 'sqlite3']
        },
        {
          name: 'All Files',
          extensions: ['*']
        }
      ]
    })

    if (selected && typeof selected === 'string') {
      // 验证路径
      const validation = await configStore.validateDbPath(selected)
      pathValidationResult.value = validation

      if (validation.valid) {
        // 如果路径有效，询问是否切换
        pendingDbPath.value = selected
        showSwitchConfirm.value = true
      } else {
        ElMessage.warning(validation.message)
      }
    }
  } catch (error) {
    console.error('选择路径失败:', error)
    ElMessage.error('选择路径失败')
  } finally {
    isValidatingPath.value = false
  }
}

// 确认切换数据库
async function confirmSwitchDatabase() {
  showSwitchConfirm.value = false
  
  try {
    await configStore.switchDatabase(pendingDbPath.value)
    ElMessage.success('数据库切换成功，应用将重新加载')
    // 页面会在事件监听中自动刷新
  } catch (error) {
    console.error('切换数据库失败:', error)
    ElMessage.error(`切换失败: ${error}`)
    pathValidationResult.value = {
      valid: false,
      message: String(error)
    }
  }
}

// 处理自动备份开关
async function handleAutoBackupChange(val: string | number | boolean) {
  const enabled = Boolean(val)
  try {
    await configStore.updateAutoBackup({
      enabled,
      frequency: backupFrequency.value,
      max_snapshots: maxSnapshots.value,
      cloud_sync: false
    })
    ElMessage.success('自动备份设置已更新')
  } catch (error) {
    ElMessage.error('设置更新失败')
    // 回滚 UI 状态
    autoBackupEnabled.value = !enabled
  }
}

// 处理备份配置变更
async function handleBackupConfigChange() {
  try {
    await configStore.updateAutoBackup({
      enabled: autoBackupEnabled.value,
      frequency: backupFrequency.value,
      max_snapshots: maxSnapshots.value,
      cloud_sync: false
    })
  } catch (error) {
    ElMessage.error('备份配置更新失败')
  }
}
</script>

<style scoped lang="scss">
.general-settings-content {
  width: 100%;
  max-width: 1200px;
  margin: 0 auto;
  position: relative;
}

// 页面标题
.page-header {
  margin-bottom: 32px;
}

.page-title {
  font-size: 32px;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0 0 8px 0;
  letter-spacing: -0.02em;
}

.page-desc {
  font-size: 16px;
  color: var(--text-secondary);
  margin: 0;
}

// 设置网格布局
.settings-grid {
  display: grid;
  grid-template-columns: 1fr 380px;
  gap: 32px;
}

// 左侧设置区域
.settings-left {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

// 通用设置区块样式
.settings-section {
  background: var(--bg-card);
  border-radius: 20px;
  padding: 24px;
  box-shadow: var(--shadow-sm);
  border: 1px solid var(--border-light);
}

.section-label {
  font-size: 12px;
  font-weight: 600;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.08em;
  margin: 0 0 16px 0;
}

// 主题选择器
.theme-selector {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 16px;
}

.theme-card {
  position: relative;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
  padding: 24px 16px;
  border: 2px solid var(--border-color);
  border-radius: 16px;
  background: var(--bg-card);
  cursor: pointer;
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);

  &:hover:not(:disabled) {
    border-color: var(--color-primary);
    transform: translateY(-2px);
    box-shadow: var(--shadow-md);
  }

  &.active {
    border-color: var(--color-primary);
    background: var(--color-primary-light);
  }

  &:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }
}

.theme-icon {
  width: 48px;
  height: 48px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 24px;

  &.light {
    background: #fff4e0;
    color: #f5a623;
  }

  &.dark {
    background: #1c1c1e;
    color: #ffffff;
  }

  &.system {
    background: #e8f4fd;
    color: #5ac8fa;
  }
}

.theme-name {
  font-size: 15px;
  font-weight: 600;
  color: var(--text-primary);
}

.check-mark {
  position: absolute;
  top: 8px;
  right: 8px;
  width: 20px;
  height: 20px;
  border-radius: 50%;
  background: var(--color-primary);
  color: white;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 12px;
}

// 路径输入组
.path-input-group {
  display: flex;
  gap: 12px;
  align-items: center;
}

.path-display {
  flex: 1;
  padding: 12px 16px;
  background: var(--bg-input);
  border-radius: 12px;
  border: 1px solid var(--border-light);
  min-width: 0;
  display: flex;
  align-items: center;
  gap: 8px;
}

.path-text {
  font-size: 14px;
  color: var(--text-primary);
  font-family: 'SF Mono', Monaco, monospace;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  flex: 1;
}

.path-status {
  flex-shrink: 0;
}

.browse-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 20px;
  border: none;
  border-radius: 12px;
  background: var(--bg-input);
  color: var(--text-primary);
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s ease;
  white-space: nowrap;

  &:hover:not(:disabled) {
    background: var(--bg-card-hover);
  }

  &:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .el-icon {
    font-size: 16px;
  }
}

.path-hint {
  font-size: 13px;
  color: var(--text-secondary);
  margin: 12px 0 0 0;
}

// 数据库状态指示器
.db-status-indicator {
  margin-top: 12px;
  
  .el-icon {
    margin-right: 4px;
  }
}

// 备份设置
.backup-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.backup-title-group {
  display: flex;
  align-items: center;
  gap: 12px;
}

.backup-icon {
  width: 40px;
  height: 40px;
  border-radius: 10px;
  background: var(--color-primary-light);
  color: var(--color-primary);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 20px;
}

.backup-config {
  display: flex;
  flex-direction: column;
  gap: 16px;
  padding-top: 16px;
  border-top: 1px solid var(--divider-color);
}

.config-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.config-label {
  font-size: 14px;
  color: var(--text-secondary);
}

.config-value {
  font-size: 14px;
  font-weight: 600;
  color: var(--color-primary);

  &.status-disconnected {
    display: flex;
    align-items: center;
    gap: 6px;
    color: var(--color-error);
    font-size: 13px;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }
}

.status-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: var(--color-error);
}

.backup-btn {
  width: 100%;
  margin-top: 8px;
}

// 右侧占位卡片
.settings-right {
  position: sticky;
  top: 24px;
  height: fit-content;
}

.placeholder-card {
  position: relative;
  background: var(--bg-card);
  border-radius: 20px;
  overflow: hidden;
  box-shadow: var(--shadow-md);
  border: 1px solid var(--border-light);
}

.placeholder-image {
  height: 280px;
  background: linear-gradient(135deg, #1c1c1e 0%, #2c2c2e 100%);
  display: flex;
  align-items: center;
  justify-content: center;
}

.placeholder-icon {
  color: var(--text-tertiary);
  opacity: 0.5;
}

.placeholder-content {
  padding: 24px;
}

.placeholder-label {
  font-size: 11px;
  font-weight: 700;
  color: var(--text-tertiary);
  text-transform: uppercase;
  letter-spacing: 0.1em;
  margin-bottom: 8px;
  display: block;
}

.placeholder-title {
  font-size: 20px;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0 0 12px 0;
  line-height: 1.3;
}

.placeholder-desc {
  font-size: 14px;
  color: var(--text-secondary);
  margin: 0;
  line-height: 1.5;
}

.placeholder-badge {
  position: absolute;
  top: 16px;
  right: 16px;
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 12px;
  background: rgba(0, 0, 0, 0.6);
  backdrop-filter: blur(10px);
  border-radius: 20px;
  color: white;
  font-size: 12px;
  font-weight: 600;

  .el-icon {
    font-size: 14px;
  }
}

// Loading 对话框
.loading-dialog {
  :deep(.el-dialog__body) {
    padding: 30px;
  }
}

.loading-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
  
  .loading-text {
    margin: 0;
    color: var(--text-secondary);
    font-size: 14px;
  }
}

// 确认对话框
.confirm-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
  text-align: center;
  
  p {
    margin: 0;
    color: var(--text-primary);
  }
  
  .confirm-path {
    font-family: 'SF Mono', Monaco, monospace;
    font-size: 12px;
    color: var(--text-secondary);
    background: var(--bg-input);
    padding: 8px 12px;
    border-radius: 6px;
    max-width: 100%;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
}

// 错误提示
.error-alert {
  position: fixed;
  bottom: 24px;
  left: 50%;
  transform: translateX(-50%);
  z-index: 2000;
  max-width: 500px;
}

// 响应式适配
@media (max-width: 1024px) {
  .settings-grid {
    grid-template-columns: 1fr;
  }

  .settings-right {
    position: static;
  }

  .theme-selector {
    grid-template-columns: repeat(3, 1fr);
  }
}

@media (max-width: 640px) {
  .theme-selector {
    grid-template-columns: 1fr;
  }

  .path-input-group {
    flex-direction: column;
    align-items: stretch;
  }

  .browse-btn {
    justify-content: center;
  }
}
</style>