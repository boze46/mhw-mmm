import { invoke } from '@tauri-apps/api/core'
import { ref } from 'vue'
import type { AppConfig, ModInfo, OperationResult } from '@/types/mod'

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
      return await invoke<string>('select_game_directory')
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
      return await invoke<string>('select_archive_file')
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
  }
}
