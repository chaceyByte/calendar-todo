<template>
  <div class="app-layout">
    <Sidebar />
    <main class="main-content">
      <router-view />
    </main>
    <!-- 全局任务对话框 - 在所有页面都可用 -->
    <GlobalTaskDialog />
  </div>
</template>

<script setup lang="ts">
import Sidebar from './components/Sidebar.vue'
import GlobalTaskDialog from './components/GlobalTaskDialog.vue'
</script>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

.app-layout {
  display: flex;
  min-height: 100vh;
}

.main-content {
  flex: 1;
  margin-left: 240px;
  min-height: 100vh;
}
</style>