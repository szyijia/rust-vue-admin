/*
 * rust-vue-admin web框架
 * */
import { register } from './global'
import packageInfo from '../../package.json'

export default {
    install: (app) => {
        register(app)
        console.log(`
       欢迎使用 Rust-Vue-Admin
       当前版本: v${packageInfo.version}
       后端地址: http://127.0.0.1:${import.meta.env.VITE_SERVER_PORT}
       前端地址: http://127.0.0.1:${import.meta.env.VITE_CLI_PORT}
    `)
    }
}
