<template>
  <div class="grid grid-cols-2 gap-2">
    <a v-for="item in wikis" :key="item.url" :href="item.url"
      class="text-sm text-black/70 dark:text-white/70 no-underline hover:text-[var(--el-color-primary)] dark:hover:text-white"
      target="_blank">
      {{ item.title }}
    </a>
  </div>
</template>

<script setup>
const wikis = [
  {
    title: 'Vue3',
    url: 'https://v3.cn.vuejs.org/guide/introduction.html'
  },
  {
    title: 'Rust 文档',
    url: 'https://doc.rust-lang.org/'
  },
  {
    title: 'Axum 文档',
    url: 'https://docs.rs/axum/latest/axum/'
  },
  {
    title: 'RVA 文档',
    url: 'https://github.com/szyijia/rust-vue-admin'
  },
  {
    title: 'GitHub 仓库',
    url: 'https://github.com/szyijia/rust-vue-admin'
  }
]
</script>

<style scoped lang="scss"></style>
