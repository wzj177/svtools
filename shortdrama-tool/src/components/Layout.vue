<template>
  <n-layout has-sider class="layout">
    <n-layout-sider
      bordered
      collapse-mode="width"
      :collapsed-width="64"
      :width="220"
      show-trigger
      @collapse="collapsed = true"
      @expand="collapsed = false"
      class="sidebar"
    >
      <div class="logo">
        <n-icon :component="VideocamOutline" :size="28" color="#18a058" />
        <span v-show="!collapsed" class="logo-text">短剧智剪</span>
      </div>
      
      <n-menu
        v-model:value="activeKey"
        mode="vertical"
        :options="menuOptions"
        @update:value="handleMenuClick"
      />
      
      <div class="compliance-badge">
        <n-tag type="success" size="small" :bordered="false">
          <template #icon>
            <n-icon :component="ShieldCheckmarkOutline" />
          </template>
          合规模式
        </n-tag>
      </div>
    </n-layout-sider>
    
    <n-layout-content class="content">
      <router-view />
    </n-layout-content>
  </n-layout>
</template>

<script setup lang="ts">
import { ref, h } from 'vue'
import { NIcon, NTag } from 'naive-ui'
import { 
  VideocamOutline, 
  DocumentTextOutline, 
  CutOutline, 
  ShieldCheckmarkOutline, 
  DownloadOutline,
  HomeOutline
} from '@vicons/ionicons5'
import { useRoute, useRouter } from 'vue-router'

const route = useRoute()
const router = useRouter()
const collapsed = ref(false)
const activeKey = ref(route.path)

const renderIcon = (icon: any) => {
  return () => h(NIcon, null, { default: () => h(icon) })
}

const menuOptions = [
  {
    label: '工作台',
    key: '/workspace',
    icon: renderIcon(HomeOutline)
  },
  {
    label: '双语字幕',
    key: '/subtitle',
    icon: renderIcon(DocumentTextOutline)
  },
  {
    label: '智能切片',
    key: '/clip',
    icon: renderIcon(CutOutline)
  },
  {
    label: '版权自检',
    key: '/copyright',
    icon: renderIcon(ShieldCheckmarkOutline)
  },
  {
    label: '批量导出',
    key: '/export',
    icon: renderIcon(DownloadOutline)
  }
]

const handleMenuClick = (key: string) => {
  router.push(key)
}
</script>

<style scoped>
.layout {
  height: 100vh;
}

.sidebar {
  background: #1a1a1a;
}

.logo {
  height: 64px;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 12px;
  border-bottom: 1px solid #2a2a2a;
}

.logo-text {
  font-size: 18px;
  font-weight: 600;
  color: #fff;
}

.compliance-badge {
  position: absolute;
  bottom: 20px;
  left: 0;
  right: 0;
  padding: 0 16px;
  display: flex;
  justify-content: center;
}

.content {
  background: #1a1a1a;
}
</style>
