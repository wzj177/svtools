import { createRouter, createWebHistory } from 'vue-router'
import Layout from '../components/Layout.vue'

const routes = [
  {
    path: '/',
    component: Layout,
    redirect: '/workspace',
    children: [
      {
        path: '/workspace',
        name: 'Workspace',
        component: () => import('../views/WorkspaceView.vue'),
        meta: { title: '工作台', icon: 'workbench' }
      },
      {
        path: '/subtitle',
        name: 'Subtitle',
        component: () => import('../views/SubtitleView.vue'),
        meta: { title: '双语字幕', icon: 'subtitle' }
      },
      {
        path: '/clip',
        name: 'Clip',
        component: () => import('../views/ClipView.vue'),
        meta: { title: '智能切片', icon: 'clip' }
      },
      {
        path: '/copyright',
        name: 'Copyright',
        component: () => import('../views/CopyrightView.vue'),
        meta: { title: '版权自检', icon: 'copyright' }
      },
      {
        path: '/export',
        name: 'Export',
        component: () => import('../views/ExportView.vue'),
        meta: { title: '批量导出', icon: 'export' }
      }
    ]
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router
