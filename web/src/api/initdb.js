import service from '@/utils/request'

// @Tags InitDB
// @Summary 初始化用户数据库
// @Produce  application/json
// @Param data body request.InitDB true "初始化数据库参数"
// @Success 200 {string} string "{"code":0,"data":{},"msg":"自动创建数据库成功"}"
// @Router /init/initdb [post]
export const initDB = (data) => {
  // 将前端 camelCase 字段转换为后端 snake_case 字段
  const payload = {
    admin_password: data.adminPassword,
    db_type: data.dbType,
    host: data.host || '',
    port: data.port || '',
    user_name: data.userName || '',
    password: data.password || '',
    db_name: data.dbName,
    db_path: data.dbPath || ''
  }
  return service({
    url: '/init/initdb',
    method: 'post',
    data: payload,
    donNotShowLoading: true
  })
}

// @Tags CheckDB
// @Summary 检查数据库是否已初始化
// @Produce  application/json
// @Success 200 {string} string "{"code":0,"data":{},"msg":"探测完成"}"
// @Router /init/checkdb [get]
export const checkDB = () => {
  return service({
    url: '/init/checkdb',
    method: 'get',
    donNotShowLoading: true
  })
}

