<template>
    <el-card>
        <template #header>
            <el-button type="primary"><el-icon><Add/></el-icon>添加</el-button>
        </template>
        <el-table :border="true" :data="mockData" :stripe="true" >
            <el-table-column prop="name" label="名称"/>
            <el-table-column prop="version" label="版本"/>
            <el-table-column prop="modLoader" label="Mod加载器"/>
            <el-table-column prop="location" label="位置"/>
            <el-table-column label="操作">
                <el-button type="primary" plain size="small">启动</el-button>
                <el-button type="warning" plain size="small">设置</el-button>
                <el-button type="danger" plain size="small">删除</el-button>
            </el-table-column>
        </el-table>
    </el-card>
</template>
<script setup lang="ts">
import {Add} from '@vicons/carbon'


const mockData = [
    {
        name: '纯净1.7.10',
        version: '1.7.10',
        modLoader: 'Forge',
        location: './.minecraft'
    }
]

</script>
<style scoped></style>