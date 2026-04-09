import { createRouter, createWebHistory } from 'vue-router'
import Analytics from '../views/Analytics.vue'
import Calendar from '../views/Calendar.vue'
import Archive from '../views/Archive.vue'
import Tags from '../views/Tags.vue'
import Quadrant from '../views/Quadrant.vue'
import Tasks from '../views/Tasks.vue'
import Settings from '../views/Settings.vue'

const routes = [
  {
    path: '/',
    redirect: '/analytics'
  },
  {
    path: '/analytics',
    name: 'Analytics',
    component: Analytics
  },
  {
    path: '/calendar',
    name: 'Calendar',
    component: Calendar
  },
  {
    path: '/archive',
    name: 'Archive',
    component: Archive
  },
  {
    path: '/tags',
    name: 'Tags',
    component: Tags
  },
  {
    path: '/quadrant',
    name: 'Quadrant',
    component: Quadrant
  },
  {
    path: '/tasks',
    name: 'Tasks',
    component: Tasks
  },
  {
    path: '/settings',
    name: 'Settings',
    component: Settings
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router