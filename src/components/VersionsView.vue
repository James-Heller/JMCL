<template>
    <el-card>
        <template #header>
            <el-button type="primary" @click="addDialogActiveFlag = true"><el-icon><Add/></el-icon>添加</el-button>
        </template>
        <el-table :border="true" :data="data" :stripe="true" >
          <el-table-column prop="id" label="ID" width="40"/>
            <el-table-column prop="name" label="名称"/>
            <el-table-column prop="version" label="版本"/>
            <el-table-column prop="mod_loader" label="Mod加载器"/>
            <el-table-column prop="location" label="位置"/>
            <el-table-column label="操作">
                <el-button type="primary" plain size="small">启动</el-button>
                <el-button type="warning" plain size="small">设置</el-button>
                <el-button type="danger" plain size="small">删除</el-button>
            </el-table-column>
        </el-table>
    </el-card>


  <el-dialog v-model="addDialogActiveFlag" title="添加" :before-close="onAddDialogClose">
    <AddGameDiag/>
  </el-dialog>
</template>
<script setup lang="ts">
import {Add} from '@vicons/carbon'
import {onMounted, ref} from "vue";
import {GameVersion} from "../types/GameVersion";
import {invoke} from "@tauri-apps/api/core";
import AddGameDiag from "../dialogs/AddGameDiag.vue";


const data = ref<GameVersion[]>([])
const addDialogActiveFlag = ref(false)


const load_data = () => {
  invoke<string>("get_installed_game").then(res => {

    data.value = JSON.parse(res)
  })
}

const onAddDialogClose = () => {
  addDialogActiveFlag.value = false
}


onMounted(() => {
  load_data()
})
</script>
<style scoped></style>