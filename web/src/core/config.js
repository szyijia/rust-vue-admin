/**
 * 网站配置文件
 */
import packageInfo from '../../package.json'

const greenText = (text) => `\x1b[32m${text}\x1b[0m`

export const config = {
  appName: 'Rust-Vue-Admin',
  showViteLogo: true,
  keepAliveTabs: false,
  logs: []
}

export const viteLogo = (env) => {
  if (config.showViteLogo) {
    console.log(greenText(`> 欢迎使用 Rust-Vue-Admin`))
    console.log(greenText(`> 当前版本: v${packageInfo.version}`))
    console.log(greenText(`> 默认后端地址: http://127.0.0.1:${env.VITE_SERVER_PORT}`))
    console.log(greenText(`> 默认前端地址: http://127.0.0.1:${env.VITE_CLI_PORT}`))
    console.log('\n')
  }
}

export default config
