<script setup lang="ts">

import {onMounted, ref} from "vue";
import type {Manifest} from "../types/Manifest";
import requests from "../requests";

const version_manifest = ref<Manifest>()
const type = ref<"Online" | "Local">()

onMounted(async () => {
  version_manifest.value = (await requests.get("https://launchermeta.mojang.com/mc/game/version_manifest.json")).data
})
</script>

<template>
<el-card>
  <template #header>
    <el-radio-group v-model="type" text-color="#626aef" fill="rgb(239, 240, 253)">
      <el-radio-button label="本地" value="Local"/>
      <el-radio-button label="下载" value="Online"/>
    </el-radio-group>
    <div id="local-box" v-if="type=== 'Local'">

    </div>
  </template>
</el-card>
</template>

<style scoped>

</style>