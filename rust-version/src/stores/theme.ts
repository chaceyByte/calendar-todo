import { defineStore } from 'pinia'
import { ref, computed, watch } from 'vue'
import { useConfigStore, type ThemeMode } from './config'

/**
 * 主题 Store
 * 
 * 这个 store 现在作为 config store 的包装器，
 * 专注于 UI 层的主题响应和本地状态管理。
 * 
 * 所有配置持久化都通过 config store 进行。
 */
export const useThemeStore = defineStore('theme', () => {
  // 使用 config store
  const configStore = useConfigStore()

  // ========== State ==========
  // 本地状态：系统主题偏好
  const systemPrefersDark = ref(false)

  // ========== Getters ==========
  
  /**
   * 当前主题模式（用户设置）
   */
  const themeMode = computed<ThemeMode>(() => {
    return configStore.theme
  })

  /**
   * 实际应用的主题（考虑 system 模式）
   */
  const actualTheme = computed<Exclude<ThemeMode, 'system'>>(() => {
    if (configStore.theme === 'system') {
      return systemPrefersDark.value ? 'dark' : 'light'
    }
    return configStore.theme
  })

  /**
   * 是否为暗色模式
   */
  const isDark = computed(() => actualTheme.value === 'dark')

  // ========== Actions ==========

  /**
   * 初始化主题
   * 
   * 从 config store 加载主题设置，并监听系统主题变化
   */
  function initTheme() {
    // 监听系统主题变化
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
    systemPrefersDark.value = mediaQuery.matches

    // 添加监听器
    const handleChange = (e: MediaQueryListEvent) => {
      systemPrefersDark.value = e.matches
      // 如果是 system 模式，主题会自动通过 actualTheme 计算更新
    }

    if (mediaQuery.addEventListener) {
      mediaQuery.addEventListener('change', handleChange)
    } else {
      // 兼容旧版浏览器
      mediaQuery.addListener(handleChange)
    }

    // 应用当前主题
    applyTheme()
  }

  /**
   * 设置主题模式
   * 
   * 通过 config store 持久化设置
   */
  async function setTheme(mode: ThemeMode) {
    try {
      await configStore.updateTheme(mode)
      // 主题应用由 config store 完成
    } catch (error) {
      console.error('设置主题失败:', error)
      throw error
    }
  }

  /**
   * 应用主题到 DOM
   * 
   * 这个函数现在主要是同步 DOM 状态
   */
  function applyTheme() {
    const root = document.documentElement
    const dark = isDark.value

    if (dark) {
      root.setAttribute('data-theme', 'dark')
      root.classList.add('dark')
      root.classList.remove('light')
    } else {
      root.setAttribute('data-theme', 'light')
      root.classList.add('light')
      root.classList.remove('dark')
    }

    // 更新 Element Plus 的主题
    const html = document.querySelector('html')
    if (html) {
      html.className = dark ? 'dark' : ''
    }
  }

  /**
   * 切换主题（在 light/dark 之间切换，忽略 system）
   */
  async function toggleTheme() {
    const newTheme = isDark.value ? 'light' : 'dark'
    await setTheme(newTheme)
  }

  // ========== Watchers ==========

  // 监听 actualTheme 变化，自动应用主题
  watch(actualTheme, () => {
    applyTheme()
  })

  return {
    // State
    themeMode,
    actualTheme,
    systemPrefersDark,
    isDark,

    // Actions
    initTheme,
    setTheme,
    applyTheme,
    toggleTheme,
  }
})
