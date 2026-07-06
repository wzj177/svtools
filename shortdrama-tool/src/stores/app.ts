import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export const useAppStore = defineStore('app', () => {
  const isComplianceAccepted = ref(false)
  const currentProject = ref<any>(null)
  const complianceLogs = ref<any[]>([])
  
  // 检查合规状态（从 Rust 后端获取）
  async function checkComplianceFromBackend(): Promise<boolean> {
    try {
      const accepted = await invoke<boolean>('check_compliance_status')
      isComplianceAccepted.value = accepted
      if (accepted) {
        localStorage.setItem('compliance_accepted', 'true')
      }
      return accepted
    } catch (error) {
      console.error('Failed to check compliance status:', error)
      // Fallback to localStorage
      const accepted = localStorage.getItem('compliance_accepted')
      isComplianceAccepted.value = accepted === 'true'
      return isComplianceAccepted.value
    }
  }
  
  // 同意合规声明（同步到 Rust 后端）
  async function acceptCompliance(): Promise<void> {
    try {
      await invoke('agree_compliance_terms')
      isComplianceAccepted.value = true
      localStorage.setItem('compliance_accepted', 'true')
    } catch (error) {
      console.error('Failed to agree compliance terms:', error)
      // Still set locally even if backend fails
      isComplianceAccepted.value = true
      localStorage.setItem('compliance_accepted', 'true')
    }
  }
  
  // 检查本地缓存（用于首次加载）
  function checkComplianceLocal(): boolean {
    const accepted = localStorage.getItem('compliance_accepted')
    isComplianceAccepted.value = accepted === 'true'
    return isComplianceAccepted.value
  }
  
  function setProject(project: any) {
    currentProject.value = project
  }
  
  // 添加合规日志
  function addComplianceLog(log: any) {
    complianceLogs.value.push(log)
  }
  
  // 导出合规日志
  async function exportComplianceLogs(outputPath: string): Promise<void> {
    await invoke('export_compliance_logs', { outputPath })
  }
  
  return {
    isComplianceAccepted,
    currentProject,
    complianceLogs,
    checkComplianceFromBackend,
    checkComplianceLocal,
    acceptCompliance,
    setProject,
    addComplianceLog,
    exportComplianceLogs
  }
})
