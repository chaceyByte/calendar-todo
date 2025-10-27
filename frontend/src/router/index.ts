import { createRouter, createWebHistory } from 'vue-router'
import Layout from '@/layout/Layout.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      component: Layout,
      redirect: '/home',
      children: [
        {
          path: 'home',
          name: 'Home',
          component: () => import('@/views/Home.vue'),
          meta: { title: '首页' }
        },
        {
          path: 'calendar',
          name: 'Calendar',
          component: () => import('@/views/Calendar.vue'),
          meta: { title: '日历' }
        },
        {
          path: 'tasks',
          name: 'Tasks',
          component: () => import('@/views/Tasks.vue'),
          meta: { title: '任务' }
        },
        {
          path: 'tags',
          name: 'Tags',
          component: () => import('@/views/Tags.vue'),
          meta: { title: '标签' }
        }
      ]
    },
    {
      path: '/login',
      name: 'Login',
      component: () => import('@/views/Login.vue'),
      meta: { title: '登录' }
    }
  ]
})

export default router