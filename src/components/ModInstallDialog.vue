<script setup lang="ts">
import { ref, computed } from 'vue'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Checkbox } from '@/components/ui/checkbox'
import type { ArchivePreview, Category } from '@/types/mod'

interface Props {
  show: boolean
  categories: Category[]
}

interface Emits {
  (e: 'close'): void
  (e: 'install', data: {
    archivePath: string
    modName: string
    nexusId: string | undefined
    categories: string[]
  }): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

// 表单数据
const archivePath = ref('')
const modName = ref('')
const nexusId = ref('')
const selectedCategories = ref<string[]>([])
const archivePreview = ref<ArchivePreview | null>(null)
const loading = ref(false)
const error = ref<string | null>(null)

// 选择压缩包文件
async function handleSelectArchive() {
  try {
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

    if (selected) {
      archivePath.value = selected as string
      // 自动从文件名提取 MOD 名称（如果还没有输入）
      if (!modName.value) {
        const fileName = (selected as string).split(/[/\\]/).pop() || ''
        modName.value = fileName.replace(/\.(zip|rar|7z|tar\.gz)$/i, '')
      }
      // 预览压缩包内容
      await loadArchivePreview()
    }
  }
  catch (e) {
    error.value = `选择文件失败: ${e}`
  }
}

// 加载压缩包预览
async function loadArchivePreview() {
  if (!archivePath.value)
    return

  try {
    loading.value = true
    error.value = null
    const { invoke } = await import('@tauri-apps/api/core')
    archivePreview.value = await invoke<ArchivePreview>('preview_zip_archive', {
      archivePath: archivePath.value,
    })
  }
  catch (e) {
    error.value = `预览失败: ${e}`
    archivePreview.value = null
  }
  finally {
    loading.value = false
  }
}

// 切换分类选择
function toggleCategory(categoryName: string) {
  const index = selectedCategories.value.indexOf(categoryName)
  if (index > -1) {
    selectedCategories.value.splice(index, 1)
  }
  else {
    selectedCategories.value.push(categoryName)
  }
}

// 表单验证
const isValid = computed(() => {
  return archivePath.value && modName.value.trim() !== ''
})

// 安装 MOD
function handleInstall() {
  if (!isValid.value)
    return

  emit('install', {
    archivePath: archivePath.value,
    modName: modName.value.trim(),
    nexusId: nexusId.value.trim() || undefined,
    categories: selectedCategories.value,
  })

  // 重置表单
  resetForm()
}

// 重置表单
function resetForm() {
  archivePath.value = ''
  modName.value = ''
  nexusId.value = ''
  selectedCategories.value = []
  archivePreview.value = null
  error.value = null
}

// 关闭对话框
function handleClose() {
  resetForm()
  emit('close')
}

// 格式化文件大小
function formatSize(bytes: number): string {
  if (bytes < 1024)
    return `${bytes} B`
  if (bytes < 1024 * 1024)
    return `${(bytes / 1024).toFixed(2)} KB`
  return `${(bytes / (1024 * 1024)).toFixed(2)} MB`
}
</script>

<template>
  <div
    v-if="show"
    class="fixed inset-0 bg-black/50 flex items-center justify-center z-50"
    @click.self="handleClose"
  >
    <div class="bg-background rounded-lg shadow-lg w-[700px] max-w-[90vw] max-h-[90vh] flex flex-col">
      <!-- 标题栏 -->
      <div class="flex items-center justify-between px-6 py-4 border-b">
        <h2 class="text-xl font-semibold">
          安装 MOD
        </h2>
        <button
          type="button"
          class="text-muted-foreground hover:text-foreground"
          @click="handleClose"
        >
          ✕
        </button>
      </div>

      <!-- 内容区域 -->
      <div class="flex-1 overflow-y-auto px-6 py-4 space-y-6">
        <!-- 错误提示 -->
        <div
          v-if="error"
          class="bg-destructive/10 text-destructive px-4 py-3 rounded-md text-sm"
        >
          {{ error }}
        </div>

        <!-- 选择压缩包 -->
        <div class="space-y-2">
          <Label for="archive">压缩包文件 *</Label>
          <div class="flex gap-2">
            <Input
              id="archive"
              v-model="archivePath"
              readonly
              placeholder="点击按钮选择压缩包..."
              class="flex-1"
            />
            <Button @click="handleSelectArchive">
              选择文件
            </Button>
          </div>
        </div>

        <!-- MOD 名称 -->
        <div class="space-y-2">
          <Label for="modName">MOD 名称 *</Label>
          <Input
            id="modName"
            v-model="modName"
            placeholder="输入 MOD 名称..."
          />
        </div>

        <!-- Nexus ID -->
        <div class="space-y-2">
          <Label for="nexusId">Nexus ID（可选）</Label>
          <Input
            id="nexusId"
            v-model="nexusId"
            placeholder="例如: 12345"
          />
        </div>

        <!-- 分类选择 -->
        <div class="space-y-2">
          <Label>分类（可选）</Label>
          <div class="flex flex-wrap gap-3">
            <label
              v-for="category in categories"
              :key="category.name"
              class="flex items-center gap-2 cursor-pointer"
            >
              <Checkbox
                :checked="selectedCategories.includes(category.name)"
                @update:checked="toggleCategory(category.name)"
              />
              <span
                class="inline-block px-2 py-1 rounded text-xs"
                :style="{ backgroundColor: category.color + '20', color: category.color }"
              >
                {{ category.name }}
              </span>
            </label>
          </div>
        </div>

        <!-- 压缩包预览 -->
        <div
          v-if="archivePreview"
          class="space-y-2"
        >
          <Label>压缩包内容预览</Label>
          <div class="border rounded-md p-4 bg-muted/30">
            <div class="text-sm space-y-2">
              <div class="flex justify-between">
                <span class="text-muted-foreground">文件数量:</span>
                <span class="font-medium">{{ archivePreview.fileCount }}</span>
              </div>
              <div class="flex justify-between">
                <span class="text-muted-foreground">总大小:</span>
                <span class="font-medium">{{ formatSize(archivePreview.totalSize) }}</span>
              </div>
              <div class="flex justify-between">
                <span class="text-muted-foreground">包含 nativepc:</span>
                <span
                  class="font-medium"
                  :class="archivePreview.hasNativepc ? 'text-green-600' : 'text-yellow-600'"
                >
                  {{ archivePreview.hasNativepc ? '是' : '否' }}
                </span>
              </div>
            </div>

            <!-- 文件列表 -->
            <div
              v-if="archivePreview.files.length > 0"
              class="mt-4"
            >
              <div class="text-xs text-muted-foreground mb-2">
                文件列表（前 20 个）:
              </div>
              <div class="max-h-40 overflow-y-auto space-y-1 text-xs font-mono">
                <div
                  v-for="file in archivePreview.files.slice(0, 20)"
                  :key="file"
                  class="text-muted-foreground"
                >
                  {{ file }}
                </div>
                <div
                  v-if="archivePreview.files.length > 20"
                  class="text-muted-foreground italic"
                >
                  ... 还有 {{ archivePreview.files.length - 20 }} 个文件
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- 加载中提示 -->
        <div
          v-if="loading"
          class="text-center py-8 text-muted-foreground"
        >
          正在加载预览...
        </div>
      </div>

      <!-- 底部按钮 -->
      <div class="flex justify-end gap-2 px-6 py-4 border-t">
        <Button
          variant="outline"
          @click="handleClose"
        >
          取消
        </Button>
        <Button
          :disabled="!isValid || loading"
          @click="handleInstall"
        >
          安装
        </Button>
      </div>
    </div>
  </div>
</template>
