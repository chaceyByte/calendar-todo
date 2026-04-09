<template>
  <div class="settings-page">
    <!-- 顶部导航栏 -->
    <header class="header">
      <div class="header-left">
        <h1 class="page-title">Settings</h1>
        <nav class="nav-tabs">
          <button 
            v-for="tab in tabs" 
            :key="tab.key"
            :class="['nav-tab', { active: currentTab === tab.key }]"
            @click="currentTab = tab.key"
          >
            {{ tab.label }}
          </button>
        </nav>
      </div>
      <div class="header-right">
        <div class="search-box">
          <el-icon class="search-icon"><Search /></el-icon>
          <input type="text" placeholder="Search setting..." class="search-input" />
        </div>
        <button class="icon-btn">
          <el-icon><Bell /></el-icon>
        </button>
        <div class="user-avatar">
          <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Felix" alt="User" />
        </div>
      </div>
    </header>

    <!-- 设置内容区 -->
    <div class="settings-content">
      <!-- Calendar 设置 - 节假日和补班设置 -->
      <div v-if="currentTab === 'calendar'" class="tab-panel">
        <HolidaySettingsContent />
      </div>

      <!-- General 设置 -->
      <div v-else-if="currentTab === 'general'" class="tab-panel">
        <GeneralSettingsContent />
      </div>

      <!-- Security 设置 -->
      <!-- <div v-else-if="currentTab === 'security'" class="tab-panel">
        <div class="placeholder-section">
          <el-empty description="Security settings coming soon" />
        </div>
      </div> -->

      <!-- Notifications 设置 -->
      <!-- <div v-else-if="currentTab === 'notifications'" class="tab-panel">
        <div class="placeholder-section">
          <el-empty description="Notification settings coming soon" />
        </div>
      </div> -->
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { Search, Bell } from '@element-plus/icons-vue'
import HolidaySettingsContent from '../components/HolidaySettingsContent.vue'
import GeneralSettingsContent from '../components/GeneralSettingsContent.vue'
import { useThemeStore } from '../stores/theme'

// 主题 store
const themeStore = useThemeStore()

// 标签页配置
const tabs = [
  { key: 'general', label: 'General' },
  { key: 'calendar', label: 'Calendar' }
  // { key: 'security', label: 'Security' },
  // { key: 'notifications', label: 'Notifications' }
]

const currentTab = ref('general')

// 初始化主题
onMounted(() => {
  themeStore.initTheme()
})
</script>

<style scoped lang="scss">
.settings-page {
  min-height: 100vh;
  background: var(--bg-page);
  padding: 24px 48px;
}

// 顶部导航栏
.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 32px;
  padding-bottom: 24px;
  border-bottom: 1px solid var(--divider-color);
}

.header-left {
  display: flex;
  align-items: center;
  gap: 32px;
}

.page-title {
  font-size: 24px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
}

.nav-tabs {
  display: flex;
  gap: 4px;
  background: var(--bg-input);
  padding: 4px;
  border-radius: 48px;
}

.nav-tab {
  padding: 8px 24px;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  font-size: 14px;
  font-weight: 500;
  border-radius: 32px;
  cursor: pointer;
  transition: all 0.2s;

  &:hover {
    color: var(--text-primary);
  }

  &.active {
    background: var(--bg-card);
    color: var(--color-primary);
    box-shadow: var(--shadow-sm);
  }
}

.header-right {
  display: flex;
  align-items: center;
  gap: 16px;
}

.search-box {
  position: relative;
  display: flex;
  align-items: center;
}

.search-icon {
  position: absolute;
  left: 16px;
  color: var(--text-secondary);
  font-size: 18px;
}

.search-input {
  padding: 9px 16px 10px 48px;
  border: none;
  border-radius: 9999px;
  background: var(--bg-input);
  font-size: 14px;
  width: 200px;
  outline: none;
  color: var(--text-primary);

  &::placeholder {
    color: var(--text-tertiary);
  }
}

.icon-btn {
  width: 40px;
  height: 40px;
  border: none;
  border-radius: 50%;
  background: transparent;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-secondary);
  transition: all 0.2s;

  &:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }
}

.user-avatar {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  overflow: hidden;
  box-shadow: 0 0 0 2px var(--bg-card), 0 0 0 4px var(--bg-card);

  img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }
}

// 设置内容区
.settings-content {
  width: 100%;
}

.tab-panel {
  animation: fadeIn 0.3s ease;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.placeholder-section {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 400px;
  background: var(--bg-card);
  border-radius: 24px;
  box-shadow: var(--shadow-sm);
  border: 1px solid var(--border-light);
}
</style>
