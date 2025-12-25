<template>
  <div class="layout-container">
    <!-- 顶部导航栏 -->
    <header class="header">
      <div class="header-left">
        <h1 class="logo">任务日历</h1>
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
            <el-avatar :size="32" :src="userStore.userInfo?.avatar" />
            <span class="username">{{ userStore.userInfo?.nickname || '用户' }}</span>
            <el-icon><arrow-down /></el-icon>
          </div>
          <template #dropdown>
            <el-dropdown-menu>
              <el-dropdown-item @click="handleLogout">
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
      <router-view v-slot="{ Component }">
        <transition name="fade" mode="out-in">
          <component :is="Component" />
        </transition>
      </router-view>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useUserStore } from '@/stores/user'
import { ArrowDown, SwitchButton } from '@element-plus/icons-vue'

const router = useRouter()
const userStore = useUserStore()

const tabs = ref([
  { name: 'Home', title: '首页' },
  { name: 'Calendar', title: '日历' },
  { name: 'Tasks', title: '任务' },
  { name: 'Tags', title: '标签' },
  { name: 'Archived', title: '归档任务' }
  // { name: 'Reports', title: '报告' }
])

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

.header {
  height: 64px;
  background: #fff;
  border-bottom: 1px solid #e4e7ed;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 24px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.header-left {
  display: flex;
  align-items: center;
  gap: 40px;
}

.logo {
  font-size: 20px;
  font-weight: 600;
  color: #409eff;
}

.nav-tabs {
  display: flex;
  gap: 8px;
}

.tab-item {
  padding: 8px 16px;
  border-radius: 6px;
  text-decoration: none;
  color: #606266;
  font-size: 14px;
  transition: all 0.3s;
  position: relative;
}

.tab-item:hover {
  color: #409eff;
  background-color: #f5f7fa;
}

.tab-item.active {
  color: #409eff;
  background-color: #ecf5ff;
  font-weight: 500;
}

.tab-item.active::after {
  content: '';
  position: absolute;
  bottom: -1px;
  left: 16px;
  right: 16px;
  height: 2px;
  background-color: #409eff;
  border-radius: 1px;
}

.user-info {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  border-radius: 6px;
  cursor: pointer;
  transition: background-color 0.3s;
}

.user-info:hover {
  background-color: #f5f7fa;
}

.username {
  font-size: 14px;
  color: #303133;
}

.main-content {
  flex: 1;
  overflow: auto;
  padding: 24px;
}

/* 页面切换动画 */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>