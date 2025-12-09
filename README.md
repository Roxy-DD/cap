# Cap 离线版 (Cap Offline)

<p align="center">
  <p align="center">
   <img width="150" height="150" src="https://github.com/CapSoftware/Cap/blob/main/apps/desktop/src-tauri/icons/Square310x310Logo.png" alt="Logo">
  </p>
	<h1 align="center"><b>Cap 离线版</b></h1>
	<p align="center">
		基于 Cap 的本地离线录屏工具，移除所有在线限制。
    <br />
    <br />
    <b>原项目：</b>
		<a href="https://github.com/CapSoftware/Cap">Cap (开源 Loom 替代)</a>
    <br />
  </p>
</p>
<br/>

## 🎯 离线版特性

本版本基于 [Cap v0.3.83](https://github.com/CapSoftware/Cap) 修改，解除了所有在线功能限制：

| 功能 | 原版 | 离线版 |
|------|------|--------|
| 即时模式 (Instant Mode) | 需要登录 | ✅ 无需登录 |
| 录制时长 | 限制 5 分钟 | ✅ 无限制 |
| Pro 功能 | 需要订阅 | ✅ 全部解锁 |
| PostHog 分析 | 开启 | ✅ 已禁用 |
| S3 云存储 | 需要认证 | ⚙️ 保留（可选） |

## 💻 下载

前往 [Releases](../../releases) 下载最新版本：
- **Windows**: `Cap_x.x.x_x64-setup.exe` (安装包)

## 🔧 修改的文件

<details>
<summary>查看修改详情</summary>

### Rust 后端
- `apps/desktop/src-tauri/src/auth.rs` - `is_upgraded()` 始终返回 `true`
- `apps/desktop/src-tauri/src/lib.rs` - 移除时长限制和升级检查
- `apps/desktop/src-tauri/src/posthog.rs` - 禁用 PostHog 分析
- `apps/desktop/src-tauri/src/recording.rs` - 允许无登录即时录制

### 前端 (SolidJS)
- `apps/desktop/src/utils/queries.ts` - 返回 Pro 状态
- `apps/desktop/src/routes/(window-chrome)/(main).tsx` - 移除登录要求和升级提示
- `apps/desktop/src/routes/in-progress-recording.tsx` - 禁用 5 分钟自动停止
- `apps/desktop/src/routes/target-select-overlay.tsx` - 移除登录检查

</details>

## 🏗️ 本地构建

### 前置要求
- Node.js 20+
- pnpm 9+
- Rust (stable)
- Windows: Visual Studio Build Tools

### 构建步骤

```bash
# 克隆仓库
git clone https://github.com/Roxy-DD/cap.git
cd cap

# 安装依赖
pnpm install

# 开发模式运行
pnpm dev:desktop

# 构建生产版本
cd apps/desktop
pnpm build:tauri
```

构建产物位于：`apps/desktop/src-tauri/target/release/bundle/`

## ⚖️ 许可证

本项目基于 [AGPLv3](LICENSE) 许可证开源。原项目版权归 [Cap Software](https://github.com/CapSoftware) 所有。

---

> ⚠️ **声明**：本项目仅供学习研究使用，请支持正版 [Cap](https://cap.so)。
