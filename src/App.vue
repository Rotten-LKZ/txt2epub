<script setup lang="ts">
import { useI18n } from 'vue-i18n'

const { t } = useI18n()
</script>

<template>
  <div class="container">
    <span>{{ t('action.selected') }}</span>
    <el-button type="primary" plain>
      {{ t('action.select_file_button') }}
    </el-button>
  </div>
</template>

<style lang="scss">
body {
  margin: 0;
}

.container {
  padding: 14px;
}
</style>
