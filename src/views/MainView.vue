<template>
    <el-container id="container">
        <el-header id="main-header">
            <h1>JMCL</h1>
            <el-avatar id="user-avatar" shape="square" :size="40"></el-avatar>
        </el-header>

        <el-container id="main-container">
            <el-aside width="150px">
                <el-menu class="menu-bg" default-active="/start" @open="handleOpenMenu" @close="handleCloseMenu" router>
                    <el-menu-item index="/start">
                        <template #title>
                            <el-icon>
                                <Play />
                            </el-icon>
                            <span>启动</span>
                        </template>
                    </el-menu-item>
                    <el-menu-item index="/versions">
                        <template #title>
                            <el-icon>
                                <Version />
                            </el-icon>
                            <span>版本列表</span>
                        </template>
                    </el-menu-item>
                    <el-menu-item index="/jvm-version">
                        <template #title>
                            <el-icon></el-icon>
                            <span>JVM 版本</span>
                        </template>
                    </el-menu-item>
                </el-menu>
            </el-aside>
            <el-main>
                <RouterView />
            </el-main>
        </el-container>
    </el-container>
</template>
<script setup lang="ts">
import { Play, Version } from '@vicons/carbon';


const handleOpenMenu = (key: string, path: string[]) => {
    console.log('Menu opened:', key, path);
}

const handleCloseMenu = (key: string, path: string[]) => {
    console.log('Menu closed:', key, path);
}
</script>
<style scoped>
#container {
    display: flex;
    flex-direction: column;
    height: 100vh;
    /* 父容器占满视口高度 */
    overflow: hidden;
    /* 防止滚动条 */
    margin: 0;
    /* 确保没有外边距 */
    padding: 0;
    /* 确保没有内边距 */
    box-sizing: border-box;
    /* 包括边框和内边距在内 */
}

#main-container {
    display: flex;
    flex: 1;
    /* 占满剩余空间 */
    margin: 0;
    /* 移除外边距 */
    margin-top: 5px;
    background: rgba(255, 255, 255, 0.8);
    border: 1px solid rgba(0, 0, 0, 0.1);
    border-radius: 1rem;
    flex-direction: column;
    overflow: hidden;
    /* 防止滚动条 */
    box-sizing: border-box;
    /* 包括边框和内边距在内 */
}

#main-header {
    display: flex;
    align-items: center;
    justify-content: left;
    padding-left: 10px;
    height: 60px;
    /* 确保高度为固定值 */
    margin: 0;
    background-color: rgba(64, 158, 255, 0.8);
    border: 1px solid rgba(0, 0, 0, 0.1);
    border-radius: 1rem;
}

::v-deep(.menu-bg) {
    width: 100%;
    height: 100%;
    background-color: rgba(255, 255, 255, 0.1) !important;
    border-radius: 1rem;
}

#user-avatar {
    margin-left: auto;
    margin-right: 10px;
}
</style>