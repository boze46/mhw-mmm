<script setup lang="ts">
import { computed } from 'vue'
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'
import { Button } from '@/components/ui/button'
import type { Mod } from '@/types/mod'

interface Props {
  mods: Mod[]
  loading?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  loading: false,
})

const emit = defineEmits<{
  toggleEnable: [mod: Mod]
  edit: [mod: Mod]
  uninstall: [mod: Mod]
  delete: [mod: Mod]
}>()

// 格式化文件大小
function formatFileSize(bytes: number): string {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return `${(bytes / Math.pow(k, i)).toFixed(2)} ${sizes[i]}`
}

// 格式化日期
function formatDate(dateString: string): string {
  const date = new Date(dateString)
  return date.toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
  })
}

// 获取分类的颜色（使用预定义的颜色映射）
const categoryColors: Record<string, string> = {
  '武器': 'bg-red-100 text-red-800',
  '装备': 'bg-green-100 text-green-800',
  '美化': 'bg-blue-100 text-blue-800',
}

function getCategoryColor(category: string): string {
  return categoryColors[category] || 'bg-gray-100 text-gray-800'
}
</script>

<template>
  <div class="space-y-4">
    <!-- 加载状态 -->
    <div v-if="loading" class="flex items-center justify-center p-8">
      <div class="text-muted-foreground">
        加载中...
      </div>
    </div>

    <!-- 空状态 -->
    <div v-else-if="mods.length === 0" class="rounded-md border">
      <div class="p-8 text-center text-muted-foreground">
        暂无 MOD，点击"安装 MOD"开始添加
      </div>
    </div>

    <!-- MOD 列表表格 -->
    <div v-else class="rounded-md border">
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead class="w-[50px]">
              排序
            </TableHead>
            <TableHead class="min-w-[200px]">
              名称
            </TableHead>
            <TableHead class="w-[120px]">
              Nexus ID
            </TableHead>
            <TableHead class="w-[200px]">
              分类
            </TableHead>
            <TableHead class="w-[100px]">
              大小
            </TableHead>
            <TableHead class="w-[120px]">
              安装日期
            </TableHead>
            <TableHead class="w-[80px]">
              状态
            </TableHead>
            <TableHead class="w-[200px] text-right">
              操作
            </TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          <TableRow v-for="mod in mods" :key="mod.name">
            <!-- 排序拖拽手柄 -->
            <TableCell>
              <div class="flex items-center justify-center cursor-move">
                <svg
                  class="w-5 h-5 text-muted-foreground"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M4 8h16M4 16h16"
                  />
                </svg>
              </div>
            </TableCell>

            <!-- 名称 -->
            <TableCell class="font-medium">
              <div class="flex items-center gap-2">
                <span>{{ mod.name }}</span>
                <span
                  v-if="mod.hasConflict"
                  class="inline-flex items-center px-2 py-0.5 rounded text-xs bg-yellow-100 text-yellow-800"
                  title="存在文件冲突"
                >
                  ⚠️ 冲突
                </span>
              </div>
            </TableCell>

            <!-- Nexus ID -->
            <TableCell>
              <span v-if="mod.nexusId" class="text-sm text-muted-foreground">
                {{ mod.nexusId }}
              </span>
              <span v-else class="text-sm text-muted-foreground">-</span>
            </TableCell>

            <!-- 分类 -->
            <TableCell>
              <div class="flex flex-wrap gap-1">
                <span
                  v-for="category in mod.categories"
                  :key="category"
                  :class="getCategoryColor(category)"
                  class="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium"
                >
                  {{ category }}
                </span>
                <span v-if="mod.categories.length === 0" class="text-sm text-muted-foreground">
                  -
                </span>
              </div>
            </TableCell>

            <!-- 文件大小 -->
            <TableCell class="text-sm text-muted-foreground">
              {{ formatFileSize(mod.fileSize) }}
            </TableCell>

            <!-- 安装日期 -->
            <TableCell class="text-sm text-muted-foreground">
              {{ formatDate(mod.installDate) }}
            </TableCell>

            <!-- 启用状态 -->
            <TableCell>
              <button
                type="button"
                :class="[
                  'relative inline-flex h-6 w-11 items-center rounded-full transition-colors',
                  mod.enabled ? 'bg-green-600' : 'bg-gray-200',
                ]"
                @click="emit('toggleEnable', mod)"
              >
                <span
                  :class="[
                    'inline-block h-4 w-4 transform rounded-full bg-white transition-transform',
                    mod.enabled ? 'translate-x-6' : 'translate-x-1',
                  ]"
                />
              </button>
            </TableCell>

            <!-- 操作按钮 -->
            <TableCell class="text-right">
              <div class="flex justify-end gap-2">
                <Button
                  variant="ghost"
                  size="sm"
                  @click="emit('edit', mod)"
                >
                  编辑
                </Button>
                <Button
                  variant="ghost"
                  size="sm"
                  @click="emit('uninstall', mod)"
                >
                  卸载
                </Button>
                <Button
                  variant="ghost"
                  size="sm"
                  class="text-destructive hover:text-destructive"
                  @click="emit('delete', mod)"
                >
                  删除
                </Button>
              </div>
            </TableCell>
          </TableRow>
        </TableBody>
      </Table>
    </div>
  </div>
</template>
