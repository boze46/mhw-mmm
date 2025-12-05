import { invoke } from '@tauri-apps/api/core'
import { ref } from 'vue'
import type { AppConfig, ModInfo, OperationResult, ArchivePreview } from '@/types/mod'

/**
 * MOD 管理器 Composable
 */
export function useModManager() {
  const config = ref<AppConfig | null>(null)
  const mods = ref<ModInfo[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  /**
   * 加载配置
   */
  async function loadConfig() {
    try {
      loading.value = true
      error.value = null
      config.value = await invoke<AppConfig>('load_config')
      return config.value
    }
    catch (e) {
      error.value = String(e)
      throw e
    }
    finally {
      loading.value = false
    }
  }

  /**
   * 保存配置
   */
  async function saveConfig(newConfig: AppConfig) {
    try {
      loading.value = true
      error.value = null
      const result = await invoke<OperationResult>('save_config', { config: newConfig })
      if (result.success) {
        config.value = newConfig
      }
      return result
    }
    catch (e) {
      error.value = String(e)
      throw e
    }
    finally {
      loading.value = false
    }
  }

  /**
   * 加载所有 MOD
   */
  async function loadAllMods() {
    try {
      loading.value = true
      error.value = null
      mods.value = await invoke<ModInfo[]>('load_all_mods')
      return mods.value
    }
    catch (e) {
      error.value = String(e)
      throw e
    }
    finally {
      loading.value = false
    }
  }

  /**
   * 加载单个 MOD 信息
   */
  async function loadModInfo(modName: string) {
    try {
      loading.value = true
      error.value = null
      return await invoke<ModInfo>('load_mod_info', { modName })
    }
    catch (e) {
      error.value = String(e)
      throw e
    }
    finally {
      loading.value = false
    }
  }

  /**
   * 保存单个 MOD 信息
   */
  async function saveModInfo(modName: string, modInfo: ModInfo) {
    try {
      loading.value = true
      error.value = null
      return await invoke<OperationResult>('save_mod_info', { modName, modInfo })
    }
    catch (e) {
      error.value = String(e)
      throw e
    }
    finally {
      loading.value = false
    }
  }

  /**
   * 选择游戏目录
   */
  async function selectGameDirectory() {
    try {
      loading.value = true
      error.value = null
      const { open } = await import('@tauri-apps/plugin-dialog')
      const selected = await open({
        title: '选择 Monster Hunter World 游戏目录',
        directory: true,
        multiple: false,
      })
      if (!selected) {
        throw new Error('未选择目录')
      }
      return selected as string
    }
    catch (e) {
      error.value = String(e)
      throw e
    }
    finally {
      loading.value = false
    }
  }

  /**
   * 选择压缩包文件
   */
  async function selectArchiveFile() {
    try {
      loading.value = true
      error.value = null
      const { open } = await import('@tauri-apps/plugin-dialog')
      const selected = await open({
        title: '选择 MOD 压缩包',
        directory: false,
        multiple: false,
        filters: [{
          name: '压缩包',
          extensions: ['zip', 'rar', '7z', 'tar.gz'],
        }],
      })
      if (!selected) {
        throw new Error('未选择文件')
      }
      return selected as string
    }
    catch (e) {
      error.value = String(e)
      throw e
    }
    finally {
      loading.value = false
    }
  }

  /**
   * 预览压缩包内容
   */
  async function previewArchive(archivePath: string) {
    try {
      loading.value = true
      error.value = null
      return await invoke<ArchivePreview>('preview_zip_archive', { archivePath })
    }
    catch (e) {
      error.value = String(e)
      throw e
    }
    finally {
      loading.value = false
    }
  }

  /**
   * 安装 MOD
   */
  async function installMod(
    archivePath: string,
    modName: string,
    nexusId: string | undefined,
    categories: string[],
  ) {
    try {
      loading.value = true
      error.value = null
      return await invoke<OperationResult>('install_mod', {
        archivePath,
        modName,
        nexusId: nexusId || undefined,
        categories,
      })
    }
    catch (e) {
      error.value = String(e)
      throw e
    }
    finally {
      loading.value = false
    }
  }

  /**
   * 启用 MOD
   */
  async function enableMod(modName: string) {
    try {
      loading.value = true
      error.value = null
      return await invoke<OperationResult>('enable_mod', { modName })
    }
    catch (e) {
      error.value = String(e)
      throw e
    }
    finally {
      loading.value = false
    }
  }

  /**
   * 禁用 MOD
   */
  async function disableMod(modName: string) {
    try {
      loading.value = true
      error.value = null
      return await invoke<OperationResult>('disable_mod', { modName })
    }
    catch (e) {
      error.value = String(e)
      throw e
    }
    finally {
      loading.value = false
    }
  }

  /**
   * 删除 MOD
   */
  async function deleteMod(modName: string) {
    try {
      loading.value = true
      error.value = null
      return await invoke<OperationResult>('delete_mod', { modName })
    }
    catch (e) {
      error.value = String(e)
      throw e
    }
    finally {
      loading.value = false
    }
  }

  return {
    config,
    mods,
    loading,
    error,
    loadConfig,
    saveConfig,
    loadAllMods,
    loadModInfo,
    saveModInfo,
    selectGameDirectory,
    selectArchiveFile,
    previewArchive,
    installMod,
    enableMod,
    disableMod,
    deleteMod,
  }
}
