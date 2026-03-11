<template>
  <div class="h-full gva-container2 overflow-auto bg-slate-50/60 dark:bg-slate-900">
    <div class="space-y-4 p-4 lg:p-6">
      <section
        class="relative overflow-hidden rounded-xl border border-slate-200/80 bg-white px-5 py-6 shadow-sm dark:border-slate-700 dark:from-slate-900 dark:via-slate-800 dark:to-slate-900">
        <div class="relative flex flex-col gap-4 lg:flex-row lg:items-end lg:justify-between">
          <div>
            <p class="text-xs tracking-[0.2em] text-slate-500 dark:text-slate-400">DASHBOARD</p>
            <h1 class="mt-2 text-xl font-semibold text-slate-900 dark:text-slate-100 lg:text-2xl">
              欢迎回来，开始今天的工作
            </h1>
            <p class="mt-2 text-sm text-slate-600 dark:text-slate-300">
              {{ today }}
            </p>
          </div>
        </div>
      </section>

      <div class="flex flex-col gap-4">
        <gva-card title="内容数据">
          <gva-chart :type="4" />
        </gva-card>
        <gva-card title="最新更新">
          <gva-table />
        </gva-card>
      </div>
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue'
import {
  GvaTable,
  GvaChart,
  GvaCard
} from './components'

const today = computed(() => {
  try {
    const d = new Date()
    return d.toLocaleDateString('zh-CN', {
      year: 'numeric',
      month: '2-digit',
      day: '2-digit'
    })
  } catch (e) {
    return new Date().toISOString().slice(0, 10)
  }
})

defineOptions({
  name: 'Dashboard'
})
</script>

<style lang="scss" scoped></style>
