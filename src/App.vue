<script setup lang="ts">
import * as process from "@tauri-apps/api/process";
import { ref, onMounted } from "vue";

const main = ref<HTMLDivElement | null>(null);

onMounted(() => {
  const element = main.value!;
  element.addEventListener("click", () => process.exit());
});
</script>

<template>
  <div id="main" ref="main"></div>
</template>

<style scoped>
#main {
  height: 100%;
}
</style>
