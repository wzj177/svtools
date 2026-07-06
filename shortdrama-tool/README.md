# 短剧智剪 - ShortDrama Tool

合规短剧双语自动化生产与版权风险自检一体化工具

## 项目简介

面向抖音短剧分销、国产短剧跨境出海创作者的轻量化桌面端工具，**全程合规、无外网素材抓取、本地离线渲染**。

### 核心特性

- ✅ **双语字幕自动化**：短剧台词智能断句，中英时间轴批量对齐
- ✅ **智能高能切片**：基于画面、音频、台词冲突点自动分割 15-60 秒钩子片段
- ✅ **本地版权自检**：离线提取视频特征，比对外网侵权素材特征库
- ✅ **双平台适配**：一键批量输出抖音 9:16、TikTok 海外分发标准规格
- ✅ **三层合规隔离**：技术隔离 + 流程隔离 + 协议隔离

### 技术栈

| 层级 | 技术选型 |
|------|----------|
| 前端框架 | Vue 3 + TypeScript + Vite |
| UI 组件库 | Naive UI（深色类微信/钉钉风格） |
| 桌面框架 | Tauri 2.0 + Rust |
| 音视频处理 | FFmpeg (ffmpeg-next) |
| 数据存储 | SQLite (rusqlite) |
| 状态管理 | Pinia |
| 路由 | Vue Router 4 |

## 快速开始

### 环境要求

- Node.js 18+ 
- Rust 1.70+
- FFmpeg 6.0+（系统需预装）
- Windows 10/11, macOS 11+, or Linux

### 安装依赖

```bash
cd shortdrama-tool

# 安装前端依赖
npm install

# 确认 Rust 已安装
rustc --version

# 安装 FFmpeg（根据系统选择）
# Ubuntu/Debian
sudo apt-get install ffmpeg
# macOS
brew install ffmpeg
# Windows
# 从 https://ffmpeg.org/download.html 下载并添加到 PATH
```

### 开发模式运行

```bash
npm run tauri dev
```

### 构建发布版本

```bash
npm run tauri build
```

构建产物位于 `src-tauri/target/release/bundle/`

## 功能模块详解

### 1. 双语字幕处理

**目标**：短剧单集 30-60 分钟，批量拆分数十条短视频，自动生成中英双语字幕

**功能**：
- 视频音频提取（FFmpeg）
- 语音识别转写（Whisper 离线模型）
- 短剧专用分句规则（区分对话、旁白、画外音）
- AI 翻译微调（对接字节/DeepL API，仅文本传输）
- 时序自动补偿校准
- 批量导出 SRT/ASS 字幕

**技术指标**：中英时间轴批量对齐准确率 ≥96%

### 2. 智能高能切片

**目标**：自动识别短剧反转、高能片段，分割 15-60 秒短视频钩子

**功能**：
- 多模态特征提取：
  - 画面突变检测（场景切换、动作幅度）
  - 音频能量分析（情绪高点、对话间隔）
  - 台词冲突点识别
- 轻量化离线推理（本地 CPU 运行）
- 片段评分排序（输出 Top-K 高评分片段）
- 自动生成缩略图

**技术指标**：分割有效率 ≥90%

### 3. 本地版权自检

**目标**：提前识别视频中混入的无授权外网片段，避免发布后限流封号

**功能**：
- 轻量化视频指纹提取（pHash 关键帧特征）
- 音频指纹提取（频谱特征）
- 本地侵权特征库比对（仅存储特征码，不存完整视频）
- 风险分级提示（high/medium/low）
- 生成合规检测报告

**技术指标**：无授权片段识别准确率 ≥93%

### 4. 双平台合规导出

**目标**：一套工具同时适配抖音国内分销、TikTok 出海两套规格

**功能**：
- 尺寸自适应裁剪（抖音 9:16 竖屏 / TikTok 标准画幅）
- 批量封面生成
- 热门标题、标签模板
- 适配抖音开放平台投稿接口（可选）

### 5. 三层合规隔离体系

**技术隔离**：
- 全素材本地渲染，服务器仅提供翻译 API 接口（仅文本）
- 不存储任何用户音视频素材

**流程隔离**：
- 每次素材导入强制弹出合规声明弹窗
- 记录弹窗触发日志，留存操作证据

**协议隔离**：
- 用户注册协议加粗标注禁止使用无授权外网素材
- 明确侵权责任归属用户
- 内置侵权举报通道

## 项目结构

```
shortdrama-tool/
├── src/                          # 前端源码
│   ├── components/               # 公共组件
│   │   └── Layout.vue           # 主布局（侧边栏导航）
│   ├── views/                    # 页面视图
│   │   ├── WorkspaceView.vue    # 工作台
│   │   ├── SubtitleView.vue     # 双语字幕
│   │   ├── ClipView.vue         # 智能切片
│   │   ├── CopyrightView.vue    # 版权自检
│   │   └── ExportView.vue       # 批量导出
│   ├── router/                   # 路由配置
│   ├── stores/                   # Pinia 状态管理
│   │   └── app.ts               # 应用状态（含合规状态）
│   ├── assets/                   # 静态资源
│   ├── App.vue                   # 根组件
│   └── main.ts                   # 入口文件
├── src-tauri/                    # Rust 后端
│   ├── src/
│   │   └── main.rs              # Tauri Commands + 业务逻辑
│   ├── Cargo.toml               # Rust 依赖配置
│   └── tauri.conf.json          # Tauri 配置
├── package.json                  # 前端依赖
├── vite.config.ts               # Vite 配置
├── tsconfig.json                # TypeScript 配置
└── README.md                     # 项目说明
```

## 开发计划

### Phase 1: 基础框架搭建（已完成 ✅）

- [x] Vue 3 + Tauri 项目初始化
- [x] 5 个核心页面组件开发
- [x] 主布局和路由配置
- [x] 合规声明弹窗系统
- [x] Rust 后端基础架构
- [x] 前后端通信框架

### Phase 2: 核心功能真实化（进行中 🚧）

- [ ] FFmpeg 音频提取集成
- [ ] Tauri 文件选择器实现
- [ ] Whisper 语音识别集成
- [ ] 真实字幕生成流程
- [ ] 前后端 IPC 通信完善

**关键代码任务**：
```rust
// src-tauri/src/main.rs
// TODO: 实现真实的 FFmpeg 音频提取
fn extract_audio(video_path: &str, output_path: &str) -> Result<(), AppError> {
    // 使用 ffmpeg-next 提取音频
}

// TODO: 集成 Whisper 语音识别
fn transcribe_audio(audio_path: &str) -> Result<Vec<SubtitleSegment>, AppError> {
    // 调用 whisper.cpp 或 faster-whisper
}
```

### Phase 3: 智能切片与版权自检

- [ ] 多模态特征提取算法
- [ ] 画面运动强度分析
- [ ] 音频能量变化检测
- [ ] 视频 pHash 特征提取
- [ ] 本地侵权特征库构建
- [ ] 相似度比对算法

### Phase 4: 批量导出与合规加固

- [ ] 双平台规格适配
- [ ] FFmpeg 滤镜链裁剪
- [ ] 批量封面生成
- [ ] 合规日志 SQLite 存储
- [ ] 日志导出功能
- [ ] 用户协议文档完善

## 合规边界

### 绝对不研发的功能

❌ 各短视频平台视频解析、下载、去水印接口  
❌ 专门用于规避平台查重的洗白工具  
❌ 外网短剧素材资源库、搬运培训  
❌ 云端存储用户完整音视频的 SaaS 服务  

### 仅服务的场景

✅ 已取得商用授权的正版短剧素材二次创作  
✅ 抖音官方分销片源的二创剪辑  
✅ 国产短剧合规出海 TikTok  

## 常见问题

### Q: 为什么选择 Tauri 而不是 Electron？

A: 
- **轻量化**：安装包 <15MB vs Electron 300MB+
- **性能**：Rust 编译为原生二进制，FFmpeg 集成更高效
- **安全**：Rust 内存安全，二进制保护核心算法
- **打包简单**：单一可执行文件，无需 Python 环境部署

### Q: 如何保证合规性？

A: 
1. 技术层面：全程本地处理，无素材上传
2. 流程层面：强制合规声明弹窗，记录操作日志
3. 协议层面：用户协议明确侵权责任归属
4. 处置机制：收到投诉 24 小时内限制功能

### Q: 支持哪些操作系统？

A: Windows 10/11（优先），macOS 11+，Linux（Ubuntu/Debian）

## 许可证

MIT License

## 联系方式

项目负责人：独立全栈开发工程师  
适用场景：抖音短剧分销、国产短剧 TikTok 出海、影视合规解说

---

**声明**：本工具仅服务于合法授权的短剧内容创作，不支持任何形式的侵权行为。使用者需自行承担内容合规责任。
