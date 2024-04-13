import { createRouter, createWebHashHistory } from 'vue-router'

import Main from './pages/Main.vue'
import Settings from './pages/Settings.vue'

const routes = [
  {
    path: '/',
    component: Main,
  },
  {
    path: '/settings',
    component: () => Settings,
  },
]

const router = createRouter({
  history: createWebHashHistory(),
  routes,
})

export default router
