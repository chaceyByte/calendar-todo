import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

// 配置类型定义
export type ThemeMode = 'light' | 'dark' | 'system'

export interface AutoBackupConfig {
  enabled: boolean
  frequency: '1h' | '6h' | '12h' | '24h' | '7d'
  max_snapshots: number
  cloud_sync: boolean
}

export interface AppSettings {
  theme: ThemeMode
  db_path: string
  auto_backup: AutoBackupConfig
}

export interface AppConfig {
  version: string
  app: AppSettings
}

export interface DbStatus {
  ready: boolean
  switching: boolean
  error: string | null
}

export interface PathValidationResult {
  valid: boolean
  message: string
}

// 默认配置
const defaultConfig: AppConfig = {
  version: '1.0.0',
  app: {
    theme: 'system',
    db_path: '',
    auto_backup: {
      enabled: false,
      frequency: '12h',
      max_snapshots: 15,
      cloud_sync: false
    }
  }
}

export const useConfigStore = defineStore('config', () => {
  // ========== State ==========
  const config = ref<AppConfig>(defaultConfig)
  const dbStatus = ref<DbStatus>({
    ready: false,
    switching: false,
    error: null
  })
  const isLoading = ref(false)
  const loadingMessage = ref('')
  const lastError = ref<string | null>(null)

  // ========== Getters ==========
  const theme = computed(() => config.value.app.theme)
  const dbPath = computed(() => config.value.app.db_path)
  const autoBackup = computed(() => config.value.app.auto_backup)
  const isDbReady = computed(() => dbStatus.value.ready)
  const isDbSwitching = computed(() => dbStatus.value.switching)

  // ========== Actions ==========

  /**
   * 初始化配置
   * 从后端加载配置并初始化主题
   */
  async function initConfig() {
    try {
      isLoading.value = true
      loadingMessage.value = '加载配置...'
      lastError.value = null

      // 加载配置
      const loadedConfig = await invoke<AppConfig>('get_config')
      config.value = loadedConfig

      // 加载数据库状态
      await refreshDbStatus()

      // 初始化主题
      await initTheme()

      // 监听数据库切换事件
      await listen('database-switched', (event) => {
        console.log('数据库已切换:', event.payload)
        refreshDbStatus()
        // 可以在这里触发页面刷新或数据重载
        window.location.reload()
      })

      console.log('✅ 配置初始化完成')
    } catch (error) {
      console.error('❌ 配置初始化失败:', error)
      lastError.value = String(error)
      // 使用默认配置继续
    } finally {
      isLoading.value = false
      loadingMessage.value = ''
    }
  }

  /**
   * 刷新数据库状态
   */
  async function refreshDbStatus() {
    try {
      const status = await invoke<DbStatus>('get_db_status')
      dbStatus.value = status
    } catch (error) {
      console.error('获取数据库状态失败:', error)
      dbStatus.value = {
        ready: false,
        switching: false,
        error: String(error)
      }
    }
  }

  /**
   * 更新主题
   */
  async function updateTheme(newTheme: ThemeMode) {
    try {
      isLoading.value = true
      loadingMessage.value = '更新主题...'
      lastError.value = null

      const updatedConfig = await invoke<AppConfig>('update_theme', {
        theme: newTheme
      })

      config.value = updatedConfig

      // 应用主题
      applyTheme(newTheme)

      console.log('✅ 主题更新成功:', newTheme)
    } catch (error) {
      console.error('❌ 主题更新失败:', error)
      lastError.value = String(error)
      throw error
    } finally {
      isLoading.value = false
      loadingMessage.value = ''
    }
  }

  /**
   * 更新数据库路径
   */
  async function updateDbPath(newPath: string) {
    try {
      isLoading.value = true
      loadingMessage.value = '更新数据库路径...'
      lastError.value = null

      const updatedConfig = await invoke<AppConfig>('update_db_path', {
        path: newPath
      })

      config.value = updatedConfig

      console.log('✅ 数据库路径更新成功:', newPath)
    } catch (error) {
      console.error('❌ 数据库路径更新失败:', error)
      lastError.value = String(error)
      throw error
    } finally {
      isLoading.value = false
      loadingMessage.value = ''
    }
  }

  /**
   * 验证数据库路径
   */
  async function validateDbPath(path: string): Promise<PathValidationResult> {
    try {
      const result = await invoke<PathValidationResult>('validate_db_path', {
        path
      })
      return result
    } catch (error) {
      console.error('路径验证失败:', error)
      return {
        valid: false,
        message: String(error)
      }
    }
  }

  /**
   * 切换数据库
   * 这是一个重量级操作，会显示 loading 状态
   */
  async function switchDatabase(newPath: string) {
    try {
      isLoading.value = true
      loadingMessage.value = '正在切换数据库，请稍候...'
      lastError.value = null

      // 先验证路径
      const validation = await validateDbPath(newPath)
      if (!validation.valid) {
        throw new Error(validation.message)
      }

      // 执行切换并获取更新后的配置
      const updatedConfig = await invoke<AppConfig>('switch_database', { path: newPath })

      // 更新整个配置对象（确保响应式）
      config.value = updatedConfig

      console.log('✅ 数据库切换成功')
    } catch (error) {
      console.error('❌ 数据库切换失败:', error)
      lastError.value = String(error)
      throw error
    } finally {
      isLoading.value = false
      loadingMessage.value = ''
    }
  }

  /**
   * 更新自动备份配置
   */
  async function updateAutoBackup(newConfig: AutoBackupConfig) {
    try {
      isLoading.value = true
      loadingMessage.value = '更新自动备份配置...'
      lastError.value = null

      const updatedConfig = await invoke<AppConfig>('update_auto_backup', {
        backupConfig: newConfig
      })

      config.value = updatedConfig

      console.log('✅ 自动备份配置更新成功')
    } catch (error) {
      console.error('❌ 自动备份配置更新失败:', error)
      lastError.value = String(error)
      throw error
    } finally {
      isLoading.value = false
      loadingMessage.value = ''
    }
  }

  // ========== 主题相关 ==========

  /**
   * 初始化主题
   */
  function initTheme() {
    const savedTheme = config.value.app.theme
    applyTheme(savedTheme)

    // 监听系统主题变化
    if (savedTheme === 'system') {
      const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
      mediaQuery.addEventListener('change', (e) => {
        applySystemTheme(e.matches)
      })
    }
  }

  /**
   * 应用主题到 DOM
   * 
   * 这个函数现在主要是同步 DOM 状态
   */
  function applyTheme(themeMode: ThemeMode) {
    if (themeMode === 'system') {
      const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches
      applySystemTheme(prefersDark)
    } else {
      const isDark = themeMode === 'dark'
      setThemeAttributes(isDark)
    }
  }

  /**
   * 应用系统主题
   */
  function applySystemTheme(isDark: boolean) {
    setThemeAttributes(isDark)
  }

  /**
   * 设置主题属性
   */
  function setThemeAttributes(isDark: boolean) {
    const root = document.documentElement
    const html = document.querySelector('html')

    if (isDark) {
      root.setAttribute('data-theme', 'dark')
      root.classList.add('dark')
      root.classList.remove('light')
      if (html) html.className = 'dark'
    } else {
      root.setAttribute('data-theme', 'light')
      root.classList.add('light')
      root.classList.remove('dark')
      if (html) html.className = ''
    }
  }

  /**
   * 清除错误
   */
  function clearError() {
    lastError.value = null
  }

  return {
    // State
    config,
    dbStatus,
    isLoading,
    loadingMessage,
    lastError,

    // Getters
    theme,
    dbPath,
    autoBackup,
    isDbReady,
    isDbSwitching,

    // Actions
    initConfig,
    refreshDbStatus,
    updateTheme,
    updateDbPath,
    validateDbPath,
    switchDatabase,
    updateAutoBackup,
    initTheme,
    applyTheme,
    clearError
  }
})