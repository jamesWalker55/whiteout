<script setup lang="ts">
import { greet } from "@/lib/ffi";

const log = console.log;
</script>

<template>
  <div class="text-lg font-bold" @click="async () => log(await greet('Bob'))">
    Hello, world!
    <!-- Import icons with the following syntax: -->
    <i-ph-smiley />
    <!-- Icon list here: https://icones.js.org/ -->
  </div>
</template>
