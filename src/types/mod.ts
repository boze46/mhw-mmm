/**
 * MOD 管理器类型定义
 */

/**
 * 分类定义
 */
export interface Category {
  name: string
  color: string
}

/**
 * MOD 文件信息
 */
export interface ModFiles {
  nativepc: string[]
  root: string[]
}

/**
 * MOD 元数据（存储在 mod-info.json 中）
 */
export interface ModInfo {
  name: string
  nexusId?: string
  categories: string[]
  enabled: boolean
  installDate: string
  fileSize: number
  files: ModFiles
}

/**
 * MOD 配置项（存储在 config.json 的 mods 数组中）
 */
export interface ModConfigItem {
  name: string
  order: number
  enabled: boolean
}

/**
 * 全局配置（config.json）
 */
export interface AppConfig {
  version: string
  gameDirectory: string
  dataDirectory: string
  mods: ModConfigItem[]
  categories: Category[]
  settings: {
    autoDetectConflicts: boolean
    showConflictWarnings: boolean
  }
}

/**
 * 表格中显示的 MOD 对象（组合 ModInfo 和 ModConfigItem）
 */
export interface Mod {
  name: string
  nexusId?: string
  categories: string[]
  enabled: boolean
  fileSize: number
  installDate: string
  order: number
  hasConflict?: boolean
  conflictWith?: string[]
}

/**
 * MOD 安装表单数据
 */
export interface ModInstallFormData {
  name: string
  nexusId: string
  categories: string[]
}

/**
 * 压缩包内容预览
 */
export interface ArchivePreview {
  hasNativePC: boolean
  nativePCPath: string
  files: ArchiveFileNode[]
}

/**
 * 压缩包文件树节点
 */
export interface ArchiveFileNode {
  name: string
  path: string
  isDirectory: boolean
  children?: ArchiveFileNode[]
}

/**
 * 文件操作进度
 */
export interface FileProgress {
  current: number
  total: number
  percentage: number
  currentFile?: string
}

/**
 * 操作结果
 */
export interface OperationResult {
  success: boolean
  message?: string
  error?: string
}
