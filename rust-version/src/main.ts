/// <reference path="./shims-vue.d.ts" />
import { createApp } from 'vue'
import { createPinia } from 'pinia'
import ElementPlus from 'element-plus'
import * as ElementPlusIconsVue from '@element-plus/icons-vue'
import 'element-plus/dist/index.css'
import './style.css'
import './styles/theme.css'
import App from '@/App.vue'
import router from './router'
import { useConfigStore } from './stores/config'

async function initApp() {
  const app = createApp(App)

  // 注册所有Element Plus图标
  for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
    app.component(key, component)
  }

  app.use(createPinia())
  app.use(ElementPlus)
  app.use(router)

  // 初始化配置（在挂载前）
  const configStore = useConfigStore()
  await configStore.initConfig()

  app.mount('#app')
}

initApp().catch(console.error)
