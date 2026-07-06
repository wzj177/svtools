<template>
  <div class="clip-view">
    <n-card title="智能切片" class="main-card">
      <n-space vertical size="large">
        <n-alert type="info" title="功能说明">
          基于视频画面、音频情绪、台词冲突点自动分割 15-60 秒短剧钩子片段
        </n-alert>

        <n-grid :cols="2" :x-gap="16">
          <n-grid-item>
            <n-form label-placement="top">
              <n-form-item label="最小片段时长 (秒)">
                <n-slider v-model:value="minDuration" :min="10" :max="30" />
              </n-form-item>
              <n-form-item label="最大片段时长 (秒)">
                <n-slider v-model:value="maxDuration" :min="30" :max="90" />
              </n-form-item>
              <n-form-item label="灵敏度">
                <n-select 
                  v-model:value="sensitivity"
                  :options="[
                    { label: '低', value: 'low' },
                    { label: '中', value: 'medium' },
                    { label: '高', value: 'high' }
                  ]"
                />
              </n-form-item>
            </n-form>
          </n-grid-item>
          
          <n-grid-item>
            <n-statistic label="预计生成片段数" :value="estimatedClips" />
            <n-space style="margin-top: 16px">
              <n-button type="primary" @click="analyzeVideo">分析视频</n-button>
              <n-button @click="exportClips" :disabled="clips.length === 0">导出片段</n-button>
            </n-space>
          </n-grid-item>
        </n-grid>

        <!-- 片段列表 -->
        <div v-if="clips.length > 0">
          <h3>识别到的精彩片段 ({{ clips.length }})</h3>
          <n-list>
            <n-list-item v-for="(clip, index) in clips" :key="index">
              <n-space justify="space-between">
                <div>
                  <n-tag type="info">片段 {{ index + 1 }}</n-tag>
                  <span style="margin-left: 8px">{{ clip.startTime }} - {{ clip.endTime }}</span>
                </div>
                <n-space>
                  <n-tag :type="clip.type === 'conflict' ? 'error' : 'success'">
                    {{ clip.type === 'conflict' ? '冲突点' : '情绪高点' }}
                  </n-tag>
                  <n-button size="small" @click="previewClip(clip)">预览</n-button>
                </n-space>
              </n-space>
            </n-list-item>
          </n-list>
        </div>
      </n-space>
    </n-card>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { NCard, NSpace, NAlert, NGrid, NGridItem, NForm, NFormItem, NSlider, NSelect, NStatistic, NButton, NList, NListItem, NTag } from 'naive-ui'

interface Clip {
  startTime: string
  endTime: string
  type: 'emotion' | 'conflict'
  score: number
}

const minDuration = ref(15)
const maxDuration = ref(45)
const sensitivity = ref('medium')
const estimatedClips = ref(0)
const clips = ref<Clip[]>([])

const analyzeVideo = () => {
  // 模拟分析结果
  clips.value = [
    { startTime: '00:00:15', endTime: '00:00:45', type: 'conflict', score: 0.92 },
    { startTime: '00:01:20', endTime: '00:01:55', type: 'emotion', score: 0.88 },
    { startTime: '00:02:30', endTime: '00:03:05', type: 'conflict', score: 0.95 }
  ]
  estimatedClips.value = clips.value.length
}

const exportClips = () => {
  console.log('导出片段:', clips.value)
}

const previewClip = (clip: Clip) => {
  console.log('预览片段:', clip)
}
</script>

<style scoped>
.clip-view {
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
