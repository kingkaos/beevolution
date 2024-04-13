import { createRouter, createWebHashHistory } from 'vue-router'

import Main from './pages/Main.vue'
import Settings from './pages/Settings.vue'

import SettingsDatabase from './views/SettingsDatabase.vue'
import SettingsAppearance from './views/SettingsAppearance.vue'

const routes = [
  {
    path: '/',
    component: Main,
  },
  {
    path: '/settings',
    component: () => Settings,
  },
  {
    path: '/settings/database',
    component: () => SettingsDatabase,
  },
  {
    path: '/settings/appearance',
    component: () => SettingsAppearance,
  },
]

const router = createRouter({
  history: createWebHashHistory(),
  routes,
})

export default router
