<script setup lang="ts">
import { ref, onMounted } from 'vue'
import "@/style.css"
import { Button } from '@/components/ui/button'
import { useModManager } from './composables/useModManager'

const { config, loadConfig, saveConfig, selectGameDirectory } = useModManager()
const showSetupDialog = ref(false)
const selectedPath = ref('')

onMounted(async () => {
  await loadConfig()
  // 如果没有设置游戏目录，显示设置对话框
  if (!config.value?.gameDirectory) {
    showSetupDialog.value = true
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
  }
  catch (e) {
    console.error('保存配置失败:', e)
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
        <div class="flex items-center justify-between">
          <div class="flex gap-2">
            <Button>安装 MOD</Button>
            <Button variant="outline">
              设置
            </Button>
          </div>
          <div class="text-sm text-muted-foreground">
            游戏目录: {{ config.gameDirectory }}
          </div>
        </div>

        <!-- MOD 列表表格 -->
        <div class="rounded-md border">
          <div class="p-8 text-center text-muted-foreground">
            暂无 MOD，点击"安装 MOD"开始添加
          </div>
        </div>
      </div>
    </main>

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