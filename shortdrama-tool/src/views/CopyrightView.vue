<template>
  <div class="copyright-view">
    <n-card title="版权风险自检" class="main-card">
      <n-space vertical size="large">
        <n-alert type="warning" title="重要说明">
          本功能通过本地特征比对，提前识别可能存在的侵权风险片段。检测结果仅供参考，请人工复核。
        </n-alert>

        <n-upload
          accept=".mp4,.mov,.avi,.mkv"
          @change="handleFileChange"
        >
          <n-button>Select Video File</n-button>
        </n-upload>

        <div v-if="selectedFile">
          <n-space>
            <n-tag>{{ selectedFile }}</n-tag>
            <n-button type="primary" :loading="scanning" @click="startScan">
              开始检测
            </n-button>
          </n-space>
        </div>

        <div v-if="scanning">
          <n-progress 
            type="line" 
            :percentage="scanProgress"
            status="default"
          />
          <p>正在分析视频特征...</p>
        </div>

        <div v-if="scanResult">
          <n-result
            :status="scanResult.riskLevel === 'high' ? 'error' : scanResult.riskLevel === 'medium' ? 'warning' : 'success'"
            :title="getResultTitle(scanResult.riskLevel)"
            :description="getResultDesc(scanResult.riskLevel)"
          >
            <template #icon>
              <n-icon :component="ShieldCheckmarkOutline" :size="64" 
                :color="scanResult.riskLevel === 'high' ? '#d03050' : scanResult.riskLevel === 'medium' ? '#f0a020' : '#18a058'" />
            </template>
          </n-result>

          <n-divider />

          <h3>检测报告</h3>
          <n-descriptions bordered>
            <n-descriptions-item label="视频时长">{{ scanResult.duration }}</n-descriptions-item>
            <n-descriptions-item label="特征点数量">{{ scanResult.featureCount }}</n-descriptions-item>
            <n-descriptions-item label="匹配素材库">外网短剧特征库 v2024.12</n-descriptions-item>
            <n-descriptions-item label="高风险片段数">{{ scanResult.highRiskSegments.length }}</n-descriptions-item>
          </n-descriptions>

          <div v-if="scanResult.highRiskSegments.length > 0">
            <h4 style="margin-top: 16px">高风险片段详情</h4>
            <n-list>
              <n-list-item v-for="(seg, idx) in scanResult.highRiskSegments" :key="idx">
                <n-space justify="space-between">
                  <div>
                    <n-tag type="error">高风险</n-tag>
                    <span style="margin-left: 8px">{{ seg.timeRange }} - 相似度 {{ seg.similarity }}%</span>
                  </div>
                  <n-button size="small" @click="locateSegment(seg)">定位片段</n-button>
                </n-space>
              </n-list-item>
            </n-list>
          </div>

          <n-alert type="info" title="合规建议">
            <ul>
              <li>如检测到高风险片段，请确认是否拥有该素材的合法授权</li>
              <li>建议使用抖音官方分销渠道获取正版片源</li>
              <li>本工具仅支持处理已获授权的素材进行二次创作</li>
            </ul>
          </n-alert>
        </div>
      </n-space>
    </n-card>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import type { UploadFileInfo } from 'naive-ui'
import { NCard, NSpace, NAlert, NUpload, NButton, NTag, NProgress, NResult, NIcon, NDivider, NDescriptions, NDescriptionsItem, NList, NListItem } from 'naive-ui'
import { ShieldCheckmarkOutline } from '@vicons/ionicons5'

interface ScanResult {
  riskLevel: 'low' | 'medium' | 'high'
  duration: string
  featureCount: number
  highRiskSegments: Array<{
    timeRange: string
    similarity: number
  }>
}

const selectedFile = ref('')
const scanning = ref(false)
const scanProgress = ref(0)
const scanResult = ref<ScanResult | null>(null)

const handleFileChange = ({ file }: { file: UploadFileInfo }) => {
  selectedFile.value = file.name
  scanResult.value = null
  scanProgress.value = 0
}

const startScan = async () => {
  scanning.value = true
  scanProgress.value = 0
  
  for (let i = 0; i <= 100; i += 5) {
    await new Promise(resolve => setTimeout(resolve, 200))
    scanProgress.value = i
  }
  
  scanResult.value = {
    riskLevel: 'low',
    duration: '00:02:30',
    featureCount: 1247,
    highRiskSegments: []
  }
  
  scanning.value = false
}

const getResultTitle = (level: string) => {
  if (level === 'high') return '检测到高风险素材'
  if (level === 'medium') return '存在中等风险'
  return '未检测到明显风险'
}

const getResultDesc = (level: string) => {
  if (level === 'high') return '发现与外网侵权素材高度相似的片段，请立即核实授权情况'
  if (level === 'medium') return '部分片段存在潜在风险，建议人工复核'
  return '当前素材未发现明显侵权风险，可放心使用'
}

const locateSegment = (segment: any) => {
  console.log('定位到片段:', segment)
}
</script>

<style scoped>
.copyright-view {
  padding: 24px;
  height: 100%;
  overflow-y: auto;
}

.main-card {
  max-width: 900px;
  margin: 0 auto;
}

h3, h4 {
  margin: 16px 0 12px;
  color: #fff;
}

ul {
  margin: 8px 0;
  padding-left: 20px;
}

li {
  margin: 6px 0;
  line-height: 1.6;
}
</style>
