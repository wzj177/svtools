<template>
  <div class="workspace">
    <div class="header">
      <h1 class="title">短剧智剪</h1>
      <p class="subtitle">合规短剧双语自动化生产与版权自检一体化工具</p>
    </div>

    <n-grid :cols="2" :x-gap="16" :y-gap="16" class="feature-grid">
      <n-grid-item v-for="feature in features" :key="feature.path">
        <n-card 
          class="feature-card" 
          hoverable
          @click="navigateTo(feature.path)"
        >
          <div class="feature-content">
            <div class="icon-wrapper" :style="{ backgroundColor: feature.color + '20' }">
              <n-icon :component="feature.icon" :size="32" :color="feature.color" />
            </div>
            <div class="feature-info">
              <h3 class="feature-title">{{ feature.title }}</h3>
              <p class="feature-desc">{{ feature.description }}</p>
            </div>
          </div>
        </n-card>
      </n-grid-item>
    </n-grid>

    <!-- 合规声明弹窗 -->
    <n-modal 
      v-model:show="isComplianceModalVisible"
      preset="dialog"
      title="⚠️ 合规使用声明"
      style="width: 600px"
      :closable="false"
      :close-on-esc="false"
      :mask-closable="false"
    >
      <div class="compliance-content">
        <n-alert type="warning" title="重要提示">
          本工具仅支持处理您拥有合法商用授权的正版短剧素材
        </n-alert>
        
        <div class="compliance-rules">
          <h4>使用规范：</h4>
          <ul>
            <li>✓ 仅限抖音官方分销正版片源二次创作</li>
            <li>✓ 仅限已取得商用授权的国产短剧素材</li>
            <li>✗ 禁止导入外网下载、无授权的视频素材</li>
            <li>✗ 禁止用于搬运、洗稿等侵权行为</li>
          </ul>
        </div>

        <n-checkbox v-model:checked="agreeTerms">
          我已阅读并同意上述使用规范，承诺仅使用合法授权素材
        </n-checkbox>
      </div>

      <template #action>
        <n-button 
          type="primary" 
          block 
          :disabled="!agreeTerms"
          @click="acceptCompliance"
        >
          同意并继续
        </n-button>
      </template>
    </n-modal>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { NCard, NGrid, NGridItem, NIcon, NModal, NAlert, NCheckbox, NButton } from 'naive-ui'
import { DocumentTextOutline, CutOutline, ShieldCheckmarkOutline, DownloadOutline } from '@vicons/ionicons5'
import { useRouter } from 'vue-router'

const router = useRouter()
const isComplianceModalVisible = ref(false)
const agreeTerms = ref(false)

const features = [
  {
    title: '双语字幕',
    description: '智能断句、中英翻译、时间轴对齐',
    icon: DocumentTextOutline,
    path: '/subtitle',
    color: '#18a058'
  },
  {
    title: '智能切片',
    description: '基于情绪和冲突点自动分割高能片段',
    icon: CutOutline,
    path: '/clip',
    color: '#2080f0'
  },
  {
    title: '版权自检',
    description: '本地特征比对，提前识别侵权风险',
    icon: ShieldCheckmarkOutline,
    path: '/copyright',
    color: '#f0a020'
  },
  {
    title: '批量导出',
    description: '一键输出抖音/TikTok 双平台规格',
    icon: DownloadOutline,
    path: '/export',
    color: '#d03050'
  }
]

const checkCompliance = () => {
  const accepted = localStorage.getItem('compliance_accepted')
  if (!accepted) {
    isComplianceModalVisible.value = true
  }
}

const acceptCompliance = () => {
  localStorage.setItem('compliance_accepted', 'true')
  isComplianceModalVisible.value = false
}

const navigateTo = (path: string) => {
  checkCompliance()
  router.push(path)
}

checkCompliance()
</script>

<style scoped>
.workspace {
  padding: 32px;
  height: 100%;
  overflow-y: auto;
}

.header {
  text-align: center;
  margin-bottom: 40px;
}

.title {
  font-size: 32px;
  font-weight: 600;
  margin: 0 0 8px 0;
  background: linear-gradient(90deg, #18a058, #2080f0);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

.subtitle {
  font-size: 14px;
  color: #888;
  margin: 0;
}

.feature-grid {
  max-width: 900px;
  margin: 0 auto;
}

.feature-card {
  cursor: pointer;
  transition: all 0.3s ease;
  border-radius: 12px;
}

.feature-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.3);
}

.feature-content {
  display: flex;
  align-items: center;
  gap: 16px;
}

.icon-wrapper {
  width: 64px;
  height: 64px;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.feature-info {
  flex: 1;
}

.feature-title {
  font-size: 18px;
  font-weight: 600;
  margin: 0 0 6px 0;
  color: #fff;
}

.feature-desc {
  font-size: 13px;
  color: #888;
  margin: 0;
}

.compliance-content {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.compliance-rules {
  background: #2a2a2a;
  padding: 16px;
  border-radius: 8px;
}

.compliance-rules h4 {
  margin: 0 0 12px 0;
  font-size: 15px;
  color: #fff;
}

.compliance-rules ul {
  margin: 0;
  padding-left: 20px;
}

.compliance-rules li {
  margin: 8px 0;
  font-size: 14px;
  line-height: 1.6;
  color: #ccc;
}
</style>
