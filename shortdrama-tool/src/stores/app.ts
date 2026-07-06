import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useAppStore = defineStore('app', () => {
  const isComplianceAccepted = ref(false)
  const currentProject = ref<any>(null)
  
  function acceptCompliance() {
    isComplianceAccepted.value = true
    localStorage.setItem('compliance_accepted', 'true')
  }
  
  function checkCompliance(): boolean {
    const accepted = localStorage.getItem('compliance_accepted')
    isComplianceAccepted.value = accepted === 'true'
    return isComplianceAccepted.value
  }
  
  function setProject(project: any) {
    currentProject.value = project
  }
  
  return {
    isComplianceAccepted,
    currentProject,
    acceptCompliance,
    checkCompliance,
    setProject
  }
})
