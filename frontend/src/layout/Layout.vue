<template>
  <div class="layout-container">
    <!-- 顶部导航栏 - Glassmorphism风格 -->
    <header class="header glass-card">
      <div class="header-left">
        <h1 class="logo">
          <span class="logo-icon">📊</span>
          任务日历
        </h1>
        <nav class="nav-tabs">
          <router-link 
            v-for="tab in tabs" 
            :key="tab.name"
            :to="{ name: tab.name }"
            class="tab-item"
            :class="{ active: $route.name === tab.name }"
          >
            {{ tab.title }}
          </router-link>
        </nav>
      </div>
      
      <div class="header-right">
        <el-dropdown trigger="click">
          <div class="user-info">
            <el-avatar :size="36" :src="userStore.userInfo?.avatar" class="user-avatar" />
            <span class="username">{{ userStore.userInfo?.nickname || '用户' }}</span>
            <el-icon class="dropdown-icon"><arrow-down /></el-icon>
          </div>
          <template #dropdown>
            <el-dropdown-menu class="dropdown-menu">
              <el-dropdown-item @click="handleSettings" class="dropdown-item">
                <el-icon><setting /></el-icon>
                账户设置
              </el-dropdown-item>
              <el-dropdown-item @click="handleLogout" class="dropdown-item" divided>
                <el-icon><switch-button /></el-icon>
                退出登录
              </el-dropdown-item>
            </el-dropdown-menu>
          </template>
        </el-dropdown>
      </div>
    </header>

    <!-- 主要内容区域 -->
    <main class="main-content">
      <router-view />
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useUserStore } from '@/stores/user'
import { ArrowDown, SwitchButton, Setting } from '@element-plus/icons-vue'

const router = useRouter()
const userStore = useUserStore()

const tabs = ref([
  { name: 'Home', title: '四象限' },
  { name: 'Calendar', title: '日历' },
  { name: 'Tasks', title: '任务' },
  { name: 'Tags', title: '标签' },
  { name: 'Archived', title: '归档任务' }
])

const handleSettings = () => {
  router.push('/settings')
}

const handleLogout = () => {
  userStore.logout()
  router.push('/login')
}

// 组件挂载时初始化用户信息
onMounted(async () => {
  await userStore.initUser()
})
</script>

<style scoped>
.layout-container {
  height: 100vh;
  display: flex;
  flex-direction: column;
}

/* Glassmorphism导航栏 - 创意前卫风格 */
.header {
  height: 72px;
  padding: 0 32px;
  position: relative;
  z-index: 2000;
  margin: 16px;
  border-radius: var(--radius);
  background: rgba(255, 255, 255, 0.85);
  backdrop-filter: blur(12px);
  border: 1px solid rgba(203, 213, 225, 0.2);
  box-shadow: var(--shadow-md);
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 48px;
}

/* Logo - 前卫设计 */
.logo {
  font-size: 24px;
  font-weight: 700;
  color: var(--color-primary);
  display: flex;
  align-items: center;
  gap: 12px;
  letter-spacing: -0.02em;
}

.logo-icon {
  font-size: 28px;
  filter: drop-shadow(0 2px 4px rgba(13, 148, 136, 0.3));
}

/* 导航标签 - 锐利边角 */
.nav-tabs {
  display: flex;
  gap: 4px;
}

.tab-item {
  padding: 10px 20px;
  border-radius: var(--radius);
  text-decoration: none;
  color: var(--color-text-muted);
  font-size: 14px;
  font-weight: 500;
  transition: var(--transition);
  position: relative;
  cursor: pointer;
}

.tab-item:hover {
  color: var(--color-primary);
  background: rgba(13, 148, 136, 0.05);
}

.tab-item.active {
  color: white;
  background: var(--color-primary);
  font-weight: 600;
  box-shadow: 0 2px 8px rgba(13, 148, 136, 0.3);
}

/* 用户信息 - Glassmorphism效果 */
.user-info {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 8px 16px;
  border-radius: var(--radius);
  cursor: pointer;
  transition: var(--transition);
  background: rgba(255, 255, 255, 0.6);
  border: 1px solid rgba(203, 213, 225, 0.3);
}

.user-info:hover {
  background: rgba(255, 255, 255, 0.9);
  box-shadow: var(--shadow);
}

.user-avatar {
  border: 2px solid rgba(13, 148, 136, 0.1);
  box-shadow: var(--shadow-sm);
}

.username {
  font-size: 14px;
  font-weight: 500;
  color: var(--color-text);
}

.dropdown-icon {
  color: var(--color-text-muted);
  transition: var(--transition);
}

.user-info:hover .dropdown-icon {
  color: var(--color-primary);
  transform: translateY(2px);
}

/* 下拉菜单样式 */
.dropdown-menu {
  border-radius: var(--radius);
  border: 1px solid var(--color-border);
  box-shadow: var(--shadow-lg);
  overflow: hidden;
}

.dropdown-item {
  padding: 12px 20px;
  font-size: 14px;
  transition: var(--transition);
}

.dropdown-item:hover {
  background: rgba(13, 148, 136, 0.05);
  color: var(--color-primary);
}

/* 主要内容区域 */
.main-content {
  flex: 1;
  overflow: auto;
  padding: 24px 32px;
}

/* 页面切换动画 */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>