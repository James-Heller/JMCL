<template>
    <el-button id="start-btn" @click="onStart" size="large" type="primary">启动</el-button>
    {{ hasMinecraftFloder }}

</template>
<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { ref } from 'vue';

const hasMinecraftFloder = ref(false);

const onStart = () => {

    invoke<boolean>('has_minecraft_folder').then((result: boolean) => {
        console.log(result);
        
        
        hasMinecraftFloder.value = result;
    })
}

</script>
<style scoped>
#start-btn {
    position: fixed;
    bottom: 20px;
    right: 40px;
    z-index: 1000;
    width: 120px;
    height: 50px;
}
</style>