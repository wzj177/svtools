<template>
  <div class="subtitle-view">
    <n-card title="双语字幕处理" class="main-card">
      <n-space vertical size="large">
        <n-upload
          multiple
          :max="10"
          accept=".mp4,.mov,.avi,.mkv"
          @change="handleFileChange"
        >
          <n-upload-dragger>
            <div style="margin-bottom: 12px">
              <n-icon :component="CloudUploadOutline" :size="48" color="#2080f0" />
            </div>
            <n-text style="font-size: 16px">
              点击或者拖动视频到该区域来上传
            </n-text>
            <n-text depth="3" style="margin-top: 8px">
              支持批量导入，仅处理合法授权素材
            </n-text>
          </n-upload-dragger>
        </n-upload>

        <div v-if="videoList.length > 0">
          <h3>已导入视频 ({{ videoList.length }})</h3>
          <n-list>
            <n-list-item v-for="(video, index) in videoList" :key="index">
              <n-space>
                <n-tag type="info">{{ index + 1 }}</n-tag>
                <span>{{ video.name }}</span>
                <n-tag :type="video.status === 'done' ? 'success' : 'warning'">
                  {{ video.status === 'done' ? '已完成' : '待处理' }}
                </n-tag>
              </n-space>
            </n-list-item>
          </n-list>
        </div>

        <n-space>
          <n-button 
            type="primary" 
            :loading="processing"
            :disabled="videoList.length === 0"
            @click="startProcessing"
          >
            开始处理
          </n-button>
          <n-button @click="clearAll">清空列表</n-button>
        </n-space>

        <n-progress 
          v-if="processing"
          type="line"
          :percentage="progress"
          :status="progress === 100 ? 'success' : 'default'"
        />

        <div v-if="subtitles.length > 0">
          <h3>生成的字幕文件</h3>
          <n-data-table
            :columns="columns"
            :data="subtitles"
            :bordered="false"
          />
        </div>
      </n-space>
    </n-card>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import type { UploadFileInfo } from 'naive-ui'
import { NCard, NUpload, NUploadDragger, NIcon, NText, NList, NListItem, NSpace, NTag, NButton, NProgress, NDataTable } from 'naive-ui'
import { CloudUploadOutline } from '@vicons/ionicons5'

interface VideoFile {
  name: string
  status: 'pending' | 'processing' | 'done'
}

interface SubtitleResult {
  videoName: string
  subtitlePath: string
  duration: string
}

const videoList = ref<VideoFile[]>([])
const processing = ref(false)
const progress = ref(0)
const subtitles = ref<SubtitleResult[]>([])

const handleFileChange = ({ fileList }: { fileList: UploadFileInfo[] }) => {
  videoList.value = fileList.map(item => ({
    name: item.name,
    status: 'pending'
  }))
}

const startProcessing = async () => {
  processing.value = true
  progress.value = 0
  
  for (let i = 0; i <= 100; i += 10) {
    await new Promise(resolve => setTimeout(resolve, 300))
    progress.value = i
  }
  
  subtitles.value = videoList.value.map(video => ({
    videoName: video.name,
    subtitlePath: `/output/${video.name.replace(/\.[^/.]+$/, '')}.srt`,
    duration: '00:01:30'
  }))
  
  videoList.value.forEach(v => v.status = 'done')
  processing.value = false
}

const clearAll = () => {
  videoList.value = []
  subtitles.value = []
  progress.value = 0
}

const columns = [
  { title: '视频名称', key: 'videoName' },
  { title: '字幕路径', key: 'subtitlePath' },
  { title: '时长', key: 'duration' }
]
</script>

<style scoped>
.subtitle-view {
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
