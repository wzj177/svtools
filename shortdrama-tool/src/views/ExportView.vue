<template>
  <div class="export-view">
    <n-card title="批量导出" class="main-card">
      <n-space vertical size="large">
        <n-alert type="info" title="双平台规格一键输出">
          支持抖音 9:16 竖屏短剧分销、TikTok 海外分发标准画幅一键切换
        </n-alert>

        <!-- 平台选择 -->
        <n-space>
          <n-radio-group v-model:value="targetPlatform">
            <n-radio-button value="douyin">抖音 (9:16)</n-radio-button>
            <n-radio-button value="tiktok">TikTok (9:16)</n-radio-button>
            <n-radio-button value="both">双平台同时导出</n-radio-button>
          </n-radio-group>
        </n-space>

        <!-- 导出设置 -->
        <n-grid :cols="2" :x-gap="16">
          <n-grid-item>
            <n-form label-placement="top">
              <n-form-item label="分辨率">
                <n-select 
                  v-model:value="resolution"
                  :options="[
                    { label: '1080x1920 (推荐)', value: '1080x1920' },
                    { label: '720x1280', value: '720x1280' }
                  ]"
                />
              </n-form-item>
              <n-form-item label="帧率">
                <n-select 
                  v-model:value="fps"
                  :options="[
                    { label: '30fps', value: 30 },
                    { label: '60fps', value: 60 }
                  ]"
                />
              </n-form-item>
            </n-form>
          </n-grid-item>
          
          <n-grid-item>
            <n-form label-placement="top">
              <n-form-item label="视频编码">
                <n-select 
                  v-model:value="codec"
                  :options="[
                    { label: 'H.264 (兼容性好)', value: 'h264' },
                    { label: 'H.265 (体积小)', value: 'h265' }
                  ]"
                />
              </n-form-item>
              <n-form-item label="码率">
                <n-select 
                  v-model:value="bitrate"
                  :options="[
                    { label: '6 Mbps', value: 6 },
                    { label: '8 Mbps', value: 8 },
                    { label: '10 Mbps', value: 10 }
                  ]"
                />
              </n-form-item>
            </n-form>
          </n-grid-item>
        </n-grid>

        <!-- 附加选项 -->
        <n-space>
          <n-checkbox v-model:checked="generateCover">自动生成封面</n-checkbox>
          <n-checkbox v-model:checked="generateTags">生成热门标签</n-checkbox>
          <n-checkbox v-model:checked="autoUpload">完成后自动上传 (需配置 API)</n-checkbox>
        </n-space>

        <!-- 导出列表 -->
        <div v-if="exportJobs.length > 0">
          <h3>导出任务 ({{ exportJobs.length }})</h3>
          <n-list>
            <n-list-item v-for="(job, index) in exportJobs" :key="index">
              <n-space justify="space-between">
                <div>
                  <n-tag>{{ job.sourceName }}</n-tag>
                  <span style="margin-left: 8px">→ {{ job.outputFormat }}</span>
                </div>
                <n-space>
                  <n-tag :type="job.status === 'done' ? 'success' : 'warning'">
                    {{ job.status === 'done' ? '已完成' : '待导出' }}
                  </n-tag>
                  <n-button size="small" @click="openFolder(job.outputPath)">打开文件夹</n-button>
                </n-space>
              </n-space>
            </n-list-item>
          </n-list>
        </div>

        <!-- 操作按钮 -->
        <n-space>
          <n-button type="primary" size="large" :loading="exporting" @click="startExport">
            开始批量导出
          </n-button>
          <n-button @click="clearJobs">清空任务</n-button>
        </n-space>

        <!-- 导出进度 -->
        <div v-if="exporting">
          <n-progress 
            type="line" 
            :percentage="exportProgress"
            :status="exportProgress === 100 ? 'success' : 'default'"
          />
          <p>正在处理：{{ currentProcessingFile }}</p>
        </div>
      </n-space>
    </n-card>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { NCard, NSpace, NAlert, NRadioGroup, NRadioButton, NGrid, NGridItem, NForm, NFormItem, NSelect, NCheckbox, NList, NListItem, NTag, NButton, NProgress } from 'naive-ui'

interface ExportJob {
  sourceName: string
  outputFormat: string
  outputPath: string
  status: 'pending' | 'processing' | 'done'
}

const targetPlatform = ref('douyin')
const resolution = ref('1080x1920')
const fps = ref(30)
const codec = ref('h264')
const bitrate = ref(8)
const generateCover = ref(true)
const generateTags = ref(true)
const autoUpload = ref(false)

const exporting = ref(false)
const exportProgress = ref(0)
const currentProcessingFile = ref('')
const exportJobs = ref<ExportJob[]>([
  { sourceName: '短剧_01.mp4', outputFormat: 'MP4 (1080x1920)', outputPath: '/output/douyin/', status: 'pending' },
  { sourceName: '短剧_02.mp4', outputFormat: 'MP4 (1080x1920)', outputPath: '/output/douyin/', status: 'pending' }
])

const startExport = async () => {
  exporting.value = true
  exportProgress.value = 0
  
  for (let i = 0; i <= 100; i += 10) {
    await new Promise(resolve => setTimeout(resolve, 500))
    exportProgress.value = i
    currentProcessingFile.value = `处理中... ${i}%`
  }
  
  exportJobs.value.forEach(job => job.status = 'done')
  exporting.value = false
}

const clearJobs = () => {
  exportJobs.value = []
  exportProgress.value = 0
}

const openFolder = (path: string) => {
  console.log('打开文件夹:', path)
}
</script>

<style scoped>
.export-view {
  padding: 24px;
  height: 100%;
  overflow-y: auto;
}

.main-card {
  max-width: 1000px;
  margin: 0 auto;
}

h3 {
  margin: 16px 0 12px;
  font-size: 16px;
  color: #fff;
}
</style>
