#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;
use chrono::{DateTime, Utc};
use thiserror::Error;
use log::{info, error};

// 错误类型定义
#[derive(Error, Debug)]
pub enum AppError {
    #[error("FFmpeg 错误：{0}")]
    FfmpegError(String),
    #[error("文件操作错误：{0}")]
    FileError(String),
    #[error("字幕生成错误：{0}")]
    SubtitleError(String),
    #[error("切片错误：{0}")]
    ClipError(String),
    #[error("版权检测错误：{0}")]
    CopyrightError(String),
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

// 数据结构定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtitleSegment {
    pub start: f64,
    pub end: f64,
    pub text_zh: String,
    pub text_en: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtitleRequest {
    pub video_path: String,
    pub output_dir: String,
    pub source_language: String,
    pub target_language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtitleResponse {
    pub success: bool,
    pub segments: Vec<SubtitleSegment>,
    pub srt_path: Option<String>,
    pub ass_path: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipSegment {
    pub start: f64,
    pub end: f64,
    pub score: f32,
    pub reason: String,
    pub thumbnail_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipRequest {
    pub video_path: String,
    pub output_dir: String,
    pub min_duration: f64,
    pub max_duration: f64,
    pub top_k: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipResponse {
    pub success: bool,
    pub segments: Vec<ClipSegment>,
    pub total_duration: f64,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyrightRisk {
    pub start: f64,
    pub end: f64,
    pub risk_level: String,
    pub risk_type: String,
    pub similarity: f32,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyrightRequest {
    pub video_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyrightResponse {
    pub success: bool,
    pub risks: Vec<CopyrightRisk>,
    pub overall_risk: String,
    pub report_path: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceLog {
    pub timestamp: DateTime<Utc>,
    pub action: String,
    pub details: String,
    pub user_confirmed: bool,
}

// 应用状态
pub struct AppState {
    pub compliance_logs: Mutex<Vec<ComplianceLog>>,
    pub agreed_to_terms: Mutex<bool>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            compliance_logs: Mutex::new(Vec::new()),
            agreed_to_terms: Mutex::new(false),
        }
    }
}

// Tauri Commands

/// 检查用户是否已同意合规声明
#[tauri::command]
pub fn check_compliance_status(state: State<AppState>) -> bool {
    *state.agreed_to_terms.lock().unwrap()
}

/// 记录用户同意合规声明
#[tauri::command]
pub fn agree_compliance_terms(state: State<AppState>) -> Result<(), String> {
    let mut agreed = state.agreed_to_terms.lock().unwrap();
    *agreed = true;
    
    let mut logs = state.compliance_logs.lock().unwrap();
    logs.push(ComplianceLog {
        timestamp: Utc::now(),
        action: "agree_terms".to_string(),
        details: "User agreed to compliance terms".to_string(),
        user_confirmed: true,
    });
    
    info!("User agreed to compliance terms");
    Ok(())
}

/// 生成双语字幕
#[tauri::command]
pub async fn generate_subtitle(
    request: SubtitleRequest,
    state: State<'_, AppState>,
) -> Result<SubtitleResponse, AppError> {
    if !*state.agreed_to_terms.lock().unwrap() {
        return Err(AppError::SubtitleError("请先同意合规声明".to_string()));
    }
    
    info!("Generating subtitles for: {}", request.video_path);
    
    {
        let mut logs = state.compliance_logs.lock().unwrap();
        logs.push(ComplianceLog {
            timestamp: Utc::now(),
            action: "generate_subtitle".to_string(),
            details: format!("Video: {}", request.video_path),
            user_confirmed: true,
        });
    }
    
    // Phase 2: 实现 FFmpeg + Whisper
    Ok(SubtitleResponse {
        success: false,
        segments: vec![],
        srt_path: None,
        ass_path: None,
        message: "Phase 2 将实现真实功能".to_string(),
    })
}

/// 智能切片
#[tauri::command]
pub async fn smart_clip(
    request: ClipRequest,
    state: State<'_, AppState>,
) -> Result<ClipResponse, AppError> {
    if !*state.agreed_to_terms.lock().unwrap() {
        return Err(AppError::ClipError("请先同意合规声明".to_string()));
    }
    
    info!("Smart clipping for: {}", request.video_path);
    
    {
        let mut logs = state.compliance_logs.lock().unwrap();
        logs.push(ComplianceLog {
            timestamp: Utc::now(),
            action: "smart_clip".to_string(),
            details: format!("Video: {}", request.video_path),
            user_confirmed: true,
        });
    }
    
    // Phase 3: 实现多模态切片
    Ok(ClipResponse {
        success: false,
        segments: vec![],
        total_duration: 0.0,
        message: "Phase 3 将实现真实功能".to_string(),
    })
}

/// 版权风险自检
#[tauri::command]
pub async fn copyright_check(
    request: CopyrightRequest,
    state: State<'_, AppState>,
) -> Result<CopyrightResponse, AppError> {
    if !*state.agreed_to_terms.lock().unwrap() {
        return Err(AppError::CopyrightError("请先同意合规声明".to_string()));
    }
    
    info!("Copyright check for: {}", request.video_path);
    
    {
        let mut logs = state.compliance_logs.lock().unwrap();
        logs.push(ComplianceLog {
            timestamp: Utc::now(),
            action: "copyright_check".to_string(),
            details: format!("Video: {}", request.video_path),
            user_confirmed: true,
        });
    }
    
    // Phase 3: 实现版权检测
    Ok(CopyrightResponse {
        success: false,
        risks: vec![],
        overall_risk: "unknown".to_string(),
        report_path: None,
        message: "Phase 3 将实现真实功能".to_string(),
    })
}

/// 导出合规日志
#[tauri::command]
pub fn export_compliance_logs(state: State<AppState>, output_path: String) -> Result<(), String> {
    use std::fs::File;
    use std::io::Write;
    
    let logs = state.compliance_logs.lock().unwrap();
    let mut file = File::create(&output_path)
        .map_err(|e| format!("无法创建文件：{}", e))?;
    
    for log in logs.iter() {
        let line = format!(
            "[{}] {} - {} (confirmed: {})\n",
            log.timestamp.format("%Y-%m-%d %H:%M:%S"),
            log.action,
            log.details,
            log.user_confirmed
        );
        file.write_all(line.as_bytes())
            .map_err(|e| format!("写入失败：{}", e))?;
    }
    
    info!("Exported {} compliance logs to {}", logs.len(), output_path);
    Ok(())
}

fn main() {
    env_logger::init();
    
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            check_compliance_status,
            agree_compliance_terms,
            generate_subtitle,
            smart_clip,
            copyright_check,
            export_compliance_logs
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
