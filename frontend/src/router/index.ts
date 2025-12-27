import { createRouter, createWebHistory } from 'vue-router'
import Layout from '@/layout/Layout.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      component: Layout,
      redirect: '/home',
      meta: { requiresAuth: true },
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
        },
        {
          path: 'reports',
          name: 'Reports',
          component: () => import('@/views/Reports.vue'),
          meta: { title: '报告' }
        },
        {
          path: 'archived',
          name: 'Archived',
          component: () => import('@/views/Archived.vue'),
          meta: { title: '归档任务' }
        },
        {
          path: 'settings',
          name: 'Settings',
          component: () => import('@/views/Settings.vue'),
          meta: { title: '账户设置' }
        }
      ]
    },
    {
      path: '/login',
      name: 'Login',
      component: () => import('@/views/Auth.vue'),
      meta: { title: '登录', requiresAuth: false }
    },
    {
      path: '/register',
      name: 'Register',
      component: () => import('@/views/Auth.vue'),
      meta: { title: '注册', requiresAuth: false }
    },
    {
      path: '/reset-password',
      name: 'ResetPassword',
      component: () => import('@/views/ResetPassword.vue'),
      meta: { title: '重置密码', requiresAuth: false }
    }
  ]
})

// 路由守卫
router.beforeEach((to, _from, next) => {
  // 从localStorage中直接检查token，避免Pinia store初始化问题
  const token = localStorage.getItem('token')
  
  // 检查是否需要登录
  if (to.meta.requiresAuth && !token) {
    next('/login')
  } else if ((to.path === '/login' || to.path === '/register') && token) {
    // 如果已登录，访问登录或注册页时跳转到首页
    next('/')
  } else {
    next()
  }
})

export default router