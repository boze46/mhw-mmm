<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import "@/style.css"
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import ModTable from '@/components/ModTable.vue'
import ModInstallDialog from '@/components/ModInstallDialog.vue'
import { useModManager } from './composables/useModManager'
import type { Mod } from '@/types/mod'

const { config, mods, loading, loadConfig, saveConfig, loadAllMods, selectGameDirectory, installMod, enableMod, disableMod, deleteMod } = useModManager()
const showSetupDialog = ref(false)
const showInstallDialog = ref(false)
const selectedPath = ref('')
const searchQuery = ref('')
const selectedCategory = ref<string>('')

// 将 ModInfo 转换为 Mod 对象（用于表格显示）
const displayMods = computed<Mod[]>(() => {
  if (!config.value) return []

  let filtered = mods.value.map((modInfo, index) => {
    const configItem = config.value!.mods.find(m => m.name === modInfo.name)
    return {
      name: modInfo.name,
      nexusId: modInfo.nexusId,
      categories: modInfo.categories,
      enabled: modInfo.enabled,
      fileSize: modInfo.fileSize,
      installDate: modInfo.installDate,
      order: configItem?.order ?? index + 1,
      hasConflict: false, // TODO: 实现冲突检测
      conflictWith: [],
    }
  })

  // 按名称搜索
  if (searchQuery.value.trim()) {
    const query = searchQuery.value.toLowerCase()
    filtered = filtered.filter(mod =>
      mod.name.toLowerCase().includes(query),
    )
  }

  // 按分类过滤
  if (selectedCategory.value && selectedCategory.value !== '') {
    filtered = filtered.filter(mod =>
      mod.categories.includes(selectedCategory.value),
    )
  }

  // 按 order 排序
  return filtered.sort((a, b) => a.order - b.order)
})

onMounted(async () => {
  await loadConfig()
  // 如果没有设置游戏目录，显示设置对话框
  if (!config.value?.gameDirectory) {
    showSetupDialog.value = true
  } else {
    // 加载 MOD 列表
    await loadAllMods()
  }
})

async function handleSelectDirectory() {
  try {
    const path = await selectGameDirectory()
    selectedPath.value = path
  }
  catch (e) {
    console.error('选择目录失败:', e)
  }
}

async function handleSaveDirectory() {
  if (!config.value || !selectedPath.value)
    return

  try {
    const newConfig = { ...config.value, gameDirectory: selectedPath.value }
    await saveConfig(newConfig)
    showSetupDialog.value = false
    // 加载 MOD 列表
    await loadAllMods()
  }
  catch (e) {
    console.error('保存配置失败:', e)
  }
}

// MOD 操作处理
async function handleToggleEnable(mod: Mod) {
  try {
    if (mod.enabled) {
      await disableMod(mod.name)
    }
    else {
      await enableMod(mod.name)
    }
    // 重新加载 MOD 列表
    await loadAllMods()
  }
  catch (e) {
    console.error('切换启用状态失败:', e)
    alert(`操作失败: ${e}`)
  }
}

function handleEdit(mod: Mod) {
  console.log('编辑 MOD:', mod.name)
  // TODO: 实现编辑功能
  alert('编辑功能待实现')
}

async function handleUninstall(mod: Mod) {
  try {
    if (!confirm(`确定要卸载 MOD "${mod.name}" 吗？\n文件将从游戏目录移除，但保留在数据目录中。`)) {
      return
    }

    await disableMod(mod.name)
    await loadAllMods()
    alert('MOD 已卸载')
  }
  catch (e) {
    console.error('卸载失败:', e)
    alert(`卸载失败: ${e}`)
  }
}

async function handleDelete(mod: Mod) {
  try {
    if (!confirm(`确定要删除 MOD "${mod.name}" 吗？\n此操作不可撤销！`)) {
      return
    }

    await deleteMod(mod.name)
    await loadAllMods()
    alert('MOD 已删除')
  }
  catch (e) {
    console.error('删除失败:', e)
    alert(`删除失败: ${e}`)
  }
}

// MOD 拖拽排序处理
async function handleReorder(reorderedMods: Mod[]) {
  if (!config.value) return

  try {
    // 更新配置中的 MOD 顺序
    const newConfig = {
      ...config.value,
      mods: reorderedMods.map(mod => ({
        name: mod.name,
        order: mod.order,
        enabled: mod.enabled,
      })),
    }

    await saveConfig(newConfig)
    // 重新加载 MOD 列表以反映新的排序
    await loadAllMods()
  }
  catch (e) {
    console.error('更新排序失败:', e)
    alert(`更新排序失败: ${e}`)
  }
}

function handleInstallMod() {
  showInstallDialog.value = true
}

async function handleInstall(data: {
  archivePath: string
  modName: string
  nexusId: string | undefined
  categories: string[]
}) {
  try {
    await installMod(data.archivePath, data.modName, data.nexusId, data.categories)
    showInstallDialog.value = false
    await loadAllMods()
    alert('MOD 安装成功！')
  }
  catch (e) {
    console.error('安装失败:', e)
    alert(`安装失败: ${e}`)
  }
}
</script>

<template>
  <div class="min-h-screen bg-background text-foreground">
    <!-- 顶部标题栏 -->
    <header class="border-b">
      <div class="container mx-auto px-4 py-4">
        <h1 class="text-2xl font-bold">
          Monster Hunter World Mod Manager
        </h1>
      </div>
    </header>

    <!-- 主要内容区域 -->
    <main class="container mx-auto px-4 py-6">
      <div v-if="!config?.gameDirectory" class="flex items-center justify-center h-96">
        <div class="text-center space-y-4">
          <h2 class="text-xl font-semibold">
            欢迎使用 MOD 管理器
          </h2>
          <p class="text-muted-foreground">
            请先设置游戏目录以开始使用
          </p>
          <Button @click="showSetupDialog = true">
            设置游戏目录
          </Button>
        </div>
      </div>

      <div v-else class="space-y-6">
        <!-- 操作栏 -->
        <div class="flex items-center justify-between gap-4">
          <div class="flex gap-2">
            <Button @click="handleInstallMod">
              安装 MOD
            </Button>
            <Button variant="outline">
              设置
            </Button>
          </div>
          <div class="text-sm text-muted-foreground">
            游戏目录: {{ config.gameDirectory }}
          </div>
        </div>

        <!-- 搜索和过滤栏 -->
        <div class="flex items-center gap-4">
          <div class="flex-1">
            <Input
              v-model="searchQuery"
              placeholder="搜索 MOD 名称..."
              class="max-w-md"
            />
          </div>
          <div class="flex items-center gap-2">
            <span class="text-sm text-muted-foreground">分类:</span>
            <select
              v-model="selectedCategory"
              class="px-3 py-2 border rounded-md bg-background text-sm"
            >
              <option value="">
                全部
              </option>
              <option
                v-for="category in config.categories"
                :key="category.name"
                :value="category.name"
              >
                {{ category.name }}
              </option>
            </select>
          </div>
        </div>

        <!-- MOD 列表表格 -->
        <ModTable
          :mods="displayMods"
          :loading="loading"
          @toggle-enable="handleToggleEnable"
          @edit="handleEdit"
          @uninstall="handleUninstall"
          @delete="handleDelete"
          @reorder="handleReorder"
        />
      </div>
    </main>

    <!-- MOD 安装对话框 -->
    <ModInstallDialog
      :show="showInstallDialog"
      :categories="config?.categories || []"
      @close="showInstallDialog = false"
      @install="handleInstall"
    />

    <!-- 设置游戏目录对话框 -->
    <div
      v-if="showSetupDialog"
      class="fixed inset-0 bg-black/50 flex items-center justify-center z-50"
      @click.self="showSetupDialog = false"
    >
      <div class="bg-background rounded-lg shadow-lg p-6 w-[500px] max-w-[90vw]">
        <h2 class="text-lg font-semibold mb-4">
          设置游戏目录
        </h2>
        <p class="text-sm text-muted-foreground mb-4">
          请选择 Monster Hunter World 的安装目录
        </p>

        <div class="space-y-4">
          <div class="flex gap-2">
            <input
              v-model="selectedPath"
              type="text"
              readonly
              placeholder="点击按钮选择目录..."
              class="flex-1 px-3 py-2 border rounded-md bg-muted"
            >
            <Button @click="handleSelectDirectory">
              选择目录
            </Button>
          </div>

          <div class="flex justify-end gap-2">
            <Button
              variant="outline"
              @click="showSetupDialog = false"
            >
              取消
            </Button>
            <Button
              :disabled="!selectedPath"
              @click="handleSaveDirectory"
            >
              确认
            </Button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style>
</style>