<template>
  <div>
    <el-table :data="tableData" stripe style="width: 100%">
      <el-table-column prop="ranking" label="排名" width="80" align="center" />
      <el-table-column prop="message" label="更新内容" show-overflow-tooltip />
      <el-table-column prop="author" label="提交人" width="140" />
      <el-table-column prop="date" label="时间" width="180" />
    </el-table>
  </div>
</template>

<script setup>
  import { ref } from 'vue'

  // 静态数据（已移除对 GitHub API 的依赖）
  const tableData = ref([
    { ranking: 1, message: '项目初始化', author: 'admin', date: '2024-01-01 00:00:00' },
  ])
</script>

<style scoped lang="scss"></style>
