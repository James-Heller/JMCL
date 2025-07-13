import { createRouter, createWebHashHistory, RouteRecordRaw } from "vue-router";
import MainView from "../views/MainView.vue";
import VersionsView from "../components/VersionsView.vue";
import StartGameView from "../components/StartGameView.vue";
import JVMVersion from "../components/JVMVersion.vue";

const routes: RouteRecordRaw[] = [
    {
        name: 'Index',
        path: '/',
        component: MainView,
        children: [
            {
                name: 'Start',
                path: '/start',
                component: StartGameView
            },
            {
                name: 'Versions',
                path: '/versions',
                component: VersionsView
            },
            {
                name: 'JVM-Version',
                path: '/jvm-version',
                component: JVMVersion
            }
        ]
    }
]

const router = createRouter({
    history: createWebHashHistory(),
    routes
})

export default router;