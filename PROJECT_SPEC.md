# Monster Hunter World Mod Manager - 项目开发说明

## 1. 项目概述

### 1.1 项目目标
开发一个跨平台（Windows/Linux）的 Monster Hunter World Mod 管理器，提供直观的 UI 界面来管理、安装、启用/禁用游戏 Mod。

### 1.2 技术栈
- **前端框架**: Vue 3 + TypeScript
- **UI 组件**: shadcn-vue + Tailwind CSS
- **工具库**: VueUse
- **桌面框架**: Tauri 2
- **后端语言**: Rust (Tauri)

---

## 2. 核心功能

### 2.1 MOD 管理表格（核心功能）

#### 2.1.1 表格显示字段
| 字段 | 说明 | 可编辑 |
|------|------|--------|
| 排序拖拽 | 拖拽手柄，用于调整 MOD 优先级 | ✓ |
| MOD 名称 | 用户自定义名称，限制为文件系统允许的字符 | ✓ |
| Nexus ID | 链接到 Nexus Mods 网站的 ID（可选） | ✓ |
| 分类 | 多个分类标签（可多选） | ✓ |
| 文件大小 | MOD 的磁盘占用 | ✗ |
| 安装日期 | MOD 的添加时间 | ✗ |
| 冲突状态 | 黄色警告标签，提示文件冲突 | ✗ |
| 启用状态 | 开关按钮，控制 MOD 的启用/禁用 | ✓ |
| 操作 | 编辑、卸载、删除按钮 | ✓ |

#### 2.1.2 表格功能
- **拖拽排序**: 使用拖拽改变 MOD 优先级（排序越靠前，文件冲突时优先级越高）
- **搜索/过滤**: 按名称、分类搜索 MOD
- **批量操作**: 支持批量启用/禁用、删除

---

### 2.2 MOD 的文件机制

#### 2.2.1 MOD 压缩包结构
```
mod-archive.zip
├── nativepc/          # 主要 MOD 文件（大小写不敏感）
│   ├── assets/
│   └── pl/
├── some.dll           # 其他附加文件
└── readme.txt         # 可选的说明文件
```

**关键点**:
- `nativepc` 文件夹可能有大小写变化（`nativePC`, `NATIVEPC` 等）
- 解压后统一转换为小写 `nativepc`
- 其他文件直接复制到游戏根目录

#### 2.2.2 游戏目录结构
```
游戏主目录/
├── nativepc/          # MOD 文件安装位置
│   ├── assets/
│   └── pl/
├── some.dll           # MOD 的其他文件
└── MonsterHunterWorld.exe
```

**游戏目录路径示例**:
- Windows: `E:\steam\steamapps\common\Monster Hunter World`
- Linux: `/home/test/.steam/steam/common/Monster Hunter World`

---

### 2.3 MOD 安装流程

#### 2.3.1 触发安装
- **方式 1**: 点击"安装 MOD"按钮，选择压缩包
- **方式 2**: 直接拖拽压缩包到窗口

#### 2.3.2 安装对话框
弹出模态对话框，包含以下内容：

**表单字段**:
- **MOD 名称** (必填): 默认使用压缩包名称（去除扩展名）
- **Nexus ID** (可选): 空文本框
- **分类** (可选): 多选下拉框/标签选择器

**文件预览区**:
- 以树形结构展示压缩包内的文件
- 高亮显示 `nativepc` 文件夹及其他根级文件

**操作按钮**:
- **确认**: 开始解压和安装
- **取消**: 放弃安装

#### 2.3.3 安装过程
1. 验证压缩包格式（支持 `.zip`, `.rar`, `.7z`, `.tar.gz`）
2. 检测 `nativepc` 文件夹（大小写不敏感）
3. 解压到 `data/<MOD名称>/`
4. 将 `nativepc` 文件夹统一重命名为小写
5. 创建 `mod-info.json` 文件
6. 更新全局配置（MOD 列表和排序）
7. 刷新表格显示

---

### 2.4 MOD 启用/禁用机制

#### 2.4.1 启用 MOD
**操作**: 切换表格中的启用开关到 ON
**行为**:
1. 显示进度条（复制文件可能耗时）
2. 将 `data/<MOD名称>/nativepc/` 下的文件复制到游戏目录 `nativepc/`
3. 将其他根级文件（如 `.dll`）复制到游戏根目录
4. 按照 MOD 排序处理文件冲突（优先级高的覆盖低的）
5. 更新 `mod-info.json` 中的 `enabled` 字段
6. 完成后关闭进度条

#### 2.4.2 禁用 MOD
**操作**: 切换表格中的启用开关到 OFF
**行为**:
1. 显示进度条
2. 从游戏目录中删除该 MOD 的所有文件
3. 保留 `data/<MOD名称>/` 中的文件
4. 更新 `mod-info.json` 中的 `enabled` 字段
5. 完成后关闭进度条

---

### 2.5 MOD 删除/卸载

#### 2.5.1 卸载 MOD
**功能**: 从游戏目录移除 MOD 文件，但保留在 `data/` 中
**操作**: 点击"卸载"按钮
**行为**:
- 禁用 MOD（如果当前启用）
- 文件仍保留在 `data/<MOD名称>/`
- 表格中 MOD 状态显示为"已卸载"

#### 2.5.2 删除 MOD
**功能**: 完全删除 MOD
**操作**: 点击"删除"按钮
**确认对话框**: "确定要删除 [MOD名称] 吗？此操作不可撤销。"
**行为**:
1. 从游戏目录删除 MOD 文件
2. 从 `data/` 目录删除 MOD 文件夹
3. 从全局配置中移除 MOD 记录
4. 刷新表格

---

### 2.6 文件冲突处理

#### 2.6.1 冲突检测
**时机**:
- 启用新 MOD 时
- 调整 MOD 排序后

**检测逻辑**:
- 扫描所有启用的 MOD
- 比较文件路径，找出重复文件
- 按排序确定优先级

#### 2.6.2 冲突显示
- **表格中**: 显示黄色 `⚠️ 冲突` 标签
- **鼠标悬停**: 显示冲突详情（与哪个 MOD 冲突）
- **冲突警告弹窗**: 首次检测到冲突时提示用户

**示例提示**:
```
检测到文件冲突：
- MOD "高清材质包" 与 "武器重制" 有 3 个文件冲突
- 当前 "高清材质包" 优先级更高，其文件将覆盖 "武器重制"

是否允许此冲突？
[确认] [查看详情] [取消]
```

---

### 2.7 分类管理

#### 2.7.1 预定义分类
- 武器
- 装备
- 美化

#### 2.7.2 分类管理功能
**设置界面**:
- 显示所有分类列表
- 添加新分类按钮
- 删除分类按钮（包括预定义分类）
- 重命名分类

**分类应用**:
- MOD 可以属于多个分类
- 表格按分类筛选
- 分类标签显示（彩色标签）

---

## 3. 数据结构设计

### 3.1 mod-info.json (单个 MOD 的元数据)

```json
{
  "name": "高清材质包",
  "nexusId": "12345",
  "categories": ["美化"],
  "enabled": true,
  "installDate": "2025-12-05T10:30:00Z",
  "fileSize": 524288000,
  "files": {
    "nativepc": [
      "nativepc/assets/texture.tex",
      "nativepc/pl/player.model"
    ],
    "root": [
      "dinput8.dll",
      "readme.txt"
    ]
  }
}
```

### 3.2 config.json (全局配置)

```json
{
  "version": "0.1.0",
  "gameDirectory": "E:\\steam\\steamapps\\common\\Monster Hunter World",
  "dataDirectory": "./data",
  "mods": [
    {
      "name": "高清材质包",
      "order": 1,
      "enabled": true
    },
    {
      "name": "武器重制",
      "order": 2,
      "enabled": false
    }
  ],
  "categories": [
    {
      "name": "武器",
      "color": "#FF5733"
    },
    {
      "name": "装备",
      "color": "#33FF57"
    },
    {
      "name": "美化",
      "color": "#3357FF"
    }
  ],
  "settings": {
    "autoDetectConflicts": true,
    "showConflictWarnings": true
  }
}
```

---

## 4. 文件结构

```
项目根目录/
├── data/                          # MOD 数据目录
│   ├── 高清材质包/
│   │   ├── nativepc/              # MOD 文件
│   │   └── mod-info.json          # MOD 元数据
│   ├── 武器重制/
│   │   ├── nativepc/
│   │   └── mod-info.json
│   └── config.json                # 全局配置
├── src/
│   ├── components/
│   │   ├── ModTable.vue           # MOD 表格组件
│   │   ├── ModInstallDialog.vue   # 安装对话框
│   │   ├── CategoryManager.vue    # 分类管理
│   │   └── ConflictWarning.vue    # 冲突警告
│   ├── composables/
│   │   ├── useModManager.ts       # MOD 管理逻辑
│   │   ├── useFileSystem.ts       # 文件操作
│   │   └── useConflictDetector.ts # 冲突检测
│   ├── types/
│   │   └── mod.ts                 # TypeScript 类型定义
│   └── App.vue
├── src-tauri/
│   ├── src/
│   │   ├── commands/
│   │   │   ├── mod_manager.rs     # MOD 管理命令
│   │   │   ├── file_ops.rs        # 文件操作命令
│   │   │   └── archive.rs         # 压缩包处理
│   │   └── main.rs
│   └── Cargo.toml
└── package.json
```

---

## 5. 技术实现要点

### 5.1 压缩包处理（Rust 侧）
**推荐库**:
- `zip` - 处理 .zip
- `sevenz-rust` 或 `compress-tools` - 处理 .7z, .rar
- `tar` + `flate2` - 处理 .tar.gz

**功能**:
- 解压到指定目录
- 读取压缩包内容列表（用于预览）
- 大小写不敏感的 `nativepc` 检测

### 5.2 文件操作（Rust 侧）
**功能**:
- 复制文件夹（支持进度回调）
- 删除文件夹
- 计算目录大小
- 跨平台路径处理

### 5.3 拖拽排序（Vue 侧）
**推荐库**: `@vueuse/core` 的 `useSortable` 或集成 `sortablejs`

### 5.4 文件拖放
**实现**: 使用 `useDropZone` (VueUse)

### 5.5 进度显示
**方案**: Tauri 的事件系统 + Vue 响应式状态
- Rust 后端发送进度事件
- Vue 前端监听并更新进度条

---

## 6. UI/UX 设计要点

### 6.1 主界面布局
```
┌─────────────────────────────────────────────┐
│  Monster Hunter World Mod Manager           │
├─────────────────────────────────────────────┤
│  [安装 MOD]  [设置]  [分类管理]             │
│  搜索: [_____________]  分类: [全部 ▼]     │
├─────────────────────────────────────────────┤
│  拖拽  │ 名称      │ Nexus │ 分类 │ 大小  │ 状态 │ 操作 │
│  ═══  │ 高清材质   │ 12345 │ 美化 │ 500M │ ON  │ ...  │
│  ═══  │ 武器重制   │       │ 武器 │ 200M │ OFF │ ...  │
│  ═══  │ UI美化 ⚠️ │ 67890 │ 美化 │ 50M  │ ON  │ ...  │
└─────────────────────────────────────────────┘
```

### 6.2 配色方案
- **主色调**: 使用 Tailwind 的 slate/gray
- **强调色**:
  - 成功/启用: green-500
  - 警告/冲突: yellow-500
  - 危险/删除: red-500
- **分类标签**: 彩色标签（可自定义）

### 6.3 交互反馈
- 加载状态: 骨架屏 + 加载动画
- 操作反馈: Toast 通知
- 确认操作: 模态对话框
- 进度显示: 线性进度条

---

## 7. 开发优先级

### Phase 1: 基础框架（MVP）
- [ ] 设置游戏目录
- [ ] 基础表格显示（静态数据）
- [ ] MOD 安装对话框（UI）
- [ ] 解压缩功能（仅支持 .zip）
- [ ] 数据持久化（config.json, mod-info.json）

### Phase 2: 核心功能
- [ ] MOD 启用/禁用（文件复制/删除）
- [ ] 拖拽排序
- [ ] 删除/卸载功能
- [ ] 搜索/过滤

### Phase 3: 高级功能
- [ ] 文件冲突检测
- [ ] 冲突警告 UI
- [ ] 分类管理
- [ ] 支持更多压缩格式（.rar, .7z, .tar.gz）

### Phase 4: 优化体验
- [ ] 进度条优化
- [ ] 批量操作
- [ ] 设置页面
- [ ] 快捷键支持
- [ ] 国际化（中文/英文）

---

## 8. 注意事项

### 8.1 跨平台考虑
- 路径分隔符使用 Rust 的 `std::path::PathBuf`
- 文件名大小写敏感性（Linux vs Windows）
- 权限问题（可能需要管理员权限复制到 Steam 目录）

### 8.2 错误处理
- 压缩包损坏
- 磁盘空间不足
- 游戏目录不存在
- 文件权限错误
- 重复 MOD 名称冲突

### 8.3 性能优化
- 大文件复制使用流式处理
- 冲突检测使用缓存
- 虚拟滚动（如果 MOD 数量很多）

---

## 9. 测试计划

### 9.1 单元测试
- 压缩包解析逻辑
- 文件冲突检测算法
- 路径处理函数

### 9.2 集成测试
- 完整的安装流程
- 启用/禁用流程
- 冲突处理流程

### 9.3 手动测试
- 不同压缩格式
- 大小写不同的 nativepc
- 极大文件（>1GB）
- 边界情况（空压缩包、无 nativepc 的压缩包）

---

## 10. 未来扩展

- 自动更新 MOD（集成 Nexus Mods API）
- MOD 配置文件编辑（.ini, .json）
- 备份/恢复功能
- MOD 推荐系统
- 社区 MOD 分享
