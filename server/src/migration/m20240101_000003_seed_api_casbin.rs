use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20240101_000003_seed_api_casbin"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // ========== 1. API 数据（sys_apis，完整对齐 ginvueadmin 数据库） ==========
        db.execute_unprepared(
            "INSERT INTO sys_apis (id, path, description, api_group, method, created_at, updated_at) VALUES
             (1,'/jwt/jsonInBlacklist','jwt加入黑名单(退出，必选)','jwt','POST',NOW(),NOW()),
             (2,'/sysLoginLog/deleteLoginLog','删除登录日志','登录日志','DELETE',NOW(),NOW()),
             (3,'/sysLoginLog/deleteLoginLogByIds','批量删除登录日志','登录日志','DELETE',NOW(),NOW()),
             (4,'/sysLoginLog/findLoginLog','根据ID获取登录日志','登录日志','GET',NOW(),NOW()),
             (5,'/sysLoginLog/getLoginLogList','获取登录日志列表','登录日志','GET',NOW(),NOW()),
             (6,'/sysApiToken/createApiToken','签发API Token','API Token','POST',NOW(),NOW()),
             (7,'/sysApiToken/getApiTokenList','获取API Token列表','API Token','POST',NOW(),NOW()),
             (8,'/sysApiToken/deleteApiToken','作废API Token','API Token','POST',NOW(),NOW()),
             (9,'/user/deleteUser','删除用户','系统用户','DELETE',NOW(),NOW()),
             (10,'/user/admin_register','用户注册','系统用户','POST',NOW(),NOW()),
             (11,'/user/getUserList','获取用户列表','系统用户','POST',NOW(),NOW()),
             (12,'/user/setUserInfo','设置用户信息','系统用户','PUT',NOW(),NOW()),
             (13,'/user/setSelfInfo','设置自身信息(必选)','系统用户','PUT',NOW(),NOW()),
             (14,'/user/getUserInfo','获取自身信息(必选)','系统用户','GET',NOW(),NOW()),
             (15,'/user/setUserAuthorities','设置权限组','系统用户','POST',NOW(),NOW()),
             (16,'/user/changePassword','修改密码（建议选择)','系统用户','POST',NOW(),NOW()),
             (17,'/user/setUserAuthority','修改用户角色(必选)','系统用户','POST',NOW(),NOW()),
             (18,'/user/resetPassword','重置用户密码','系统用户','POST',NOW(),NOW()),
             (19,'/user/setSelfSetting','用户界面配置','系统用户','PUT',NOW(),NOW())"
        ).await?;

        db.execute_unprepared(
            "INSERT INTO sys_apis (id, path, description, api_group, method, created_at, updated_at) VALUES
             (20,'/api/createApi','创建api','api','POST',NOW(),NOW()),
             (21,'/api/deleteApi','删除Api','api','POST',NOW(),NOW()),
             (22,'/api/updateApi','更新Api','api','POST',NOW(),NOW()),
             (23,'/api/getApiList','获取api列表','api','POST',NOW(),NOW()),
             (24,'/api/getAllApis','获取所有api','api','POST',NOW(),NOW()),
             (25,'/api/getApiById','获取api详细信息','api','POST',NOW(),NOW()),
             (26,'/api/deleteApisByIds','批量删除api','api','DELETE',NOW(),NOW()),
             (27,'/api/syncApi','获取待同步API','api','GET',NOW(),NOW()),
             (28,'/api/getApiGroups','获取路由组','api','GET',NOW(),NOW()),
             (29,'/api/enterSyncApi','确认同步API','api','POST',NOW(),NOW()),
             (30,'/api/ignoreApi','忽略API','api','POST',NOW(),NOW())"
        ).await?;

        db.execute_unprepared(
            "INSERT INTO sys_apis (id, path, description, api_group, method, created_at, updated_at) VALUES
             (31,'/authority/copyAuthority','拷贝角色','角色','POST',NOW(),NOW()),
             (32,'/authority/createAuthority','创建角色','角色','POST',NOW(),NOW()),
             (33,'/authority/deleteAuthority','删除角色','角色','POST',NOW(),NOW()),
             (34,'/authority/updateAuthority','更新角色信息','角色','PUT',NOW(),NOW()),
             (35,'/authority/getAuthorityList','获取角色列表','角色','POST',NOW(),NOW()),
             (36,'/authority/setDataAuthority','设置角色资源权限','角色','POST',NOW(),NOW()),
             (37,'/authority/getUsersByAuthority','获取角色关联用户ID列表','角色','GET',NOW(),NOW()),
             (38,'/authority/setRoleUsers','全量覆盖角色关联用户','角色','POST',NOW(),NOW())"
        ).await?;

        db.execute_unprepared(
            "INSERT INTO sys_apis (id, path, description, api_group, method, created_at, updated_at) VALUES
             (39,'/casbin/updateCasbin','更改角色api权限','casbin','POST',NOW(),NOW()),
             (40,'/casbin/getPolicyPathByAuthorityId','获取权限列表','casbin','POST',NOW(),NOW()),
             (41,'/menu/addBaseMenu','新增菜单','菜单','POST',NOW(),NOW()),
             (42,'/menu/getMenu','获取菜单树(必选)','菜单','POST',NOW(),NOW()),
             (43,'/menu/deleteBaseMenu','删除菜单','菜单','POST',NOW(),NOW()),
             (44,'/menu/updateBaseMenu','更新菜单','菜单','POST',NOW(),NOW()),
             (45,'/menu/getBaseMenuById','根据id获取菜单','菜单','POST',NOW(),NOW()),
             (46,'/menu/getMenuList','分页获取基础menu列表','菜单','POST',NOW(),NOW()),
             (47,'/menu/getBaseMenuTree','获取用户动态路由','菜单','POST',NOW(),NOW()),
             (48,'/menu/getMenuAuthority','获取指定角色menu','菜单','POST',NOW(),NOW()),
             (49,'/menu/addMenuAuthority','增加menu和角色关联关系','菜单','POST',NOW(),NOW())"
        ).await?;

        db.execute_unprepared(
            "INSERT INTO sys_apis (id, path, description, api_group, method, created_at, updated_at) VALUES
             (50,'/fileUploadAndDownload/findFile','寻找目标文件（秒传）','分片上传','GET',NOW(),NOW()),
             (51,'/fileUploadAndDownload/breakpointContinue','断点续传','分片上传','POST',NOW(),NOW()),
             (52,'/fileUploadAndDownload/breakpointContinueFinish','断点续传完成','分片上传','POST',NOW(),NOW()),
             (53,'/fileUploadAndDownload/removeChunk','上传完成移除文件','分片上传','POST',NOW(),NOW()),
             (54,'/fileUploadAndDownload/upload','文件上传（建议选择）','文件上传与下载','POST',NOW(),NOW()),
             (55,'/fileUploadAndDownload/deleteFile','删除文件','文件上传与下载','POST',NOW(),NOW()),
             (56,'/fileUploadAndDownload/editFileName','文件名或者备注编辑','文件上传与下载','POST',NOW(),NOW()),
             (57,'/fileUploadAndDownload/getFileList','获取上传文件列表','文件上传与下载','POST',NOW(),NOW()),
             (58,'/fileUploadAndDownload/importURL','导入URL','文件上传与下载','POST',NOW(),NOW()),
             (59,'/system/getServerInfo','获取服务器信息','系统服务','POST',NOW(),NOW()),
             (60,'/system/getSystemConfig','获取配置文件内容','系统服务','POST',NOW(),NOW()),
             (61,'/system/setSystemConfig','设置配置文件内容','系统服务','POST',NOW(),NOW())"
        ).await?;

        db.execute_unprepared(
            "INSERT INTO sys_apis (id, path, description, api_group, method, created_at, updated_at) VALUES
             (62,'/skills/getTools','获取技能工具列表','skills','GET',NOW(),NOW()),
             (63,'/skills/getSkillList','获取技能列表','skills','POST',NOW(),NOW()),
             (64,'/skills/getSkillDetail','获取技能详情','skills','POST',NOW(),NOW()),
             (65,'/skills/saveSkill','保存技能定义','skills','POST',NOW(),NOW()),
             (66,'/skills/deleteSkill','删除技能','skills','POST',NOW(),NOW()),
             (67,'/skills/createScript','创建技能脚本','skills','POST',NOW(),NOW()),
             (68,'/skills/getScript','读取技能脚本','skills','POST',NOW(),NOW()),
             (69,'/skills/saveScript','保存技能脚本','skills','POST',NOW(),NOW()),
             (70,'/skills/createResource','创建技能资源','skills','POST',NOW(),NOW()),
             (71,'/skills/getResource','读取技能资源','skills','POST',NOW(),NOW()),
             (72,'/skills/saveResource','保存技能资源','skills','POST',NOW(),NOW()),
             (73,'/skills/createReference','创建技能参考','skills','POST',NOW(),NOW()),
             (74,'/skills/getReference','读取技能参考','skills','POST',NOW(),NOW()),
             (75,'/skills/saveReference','保存技能参考','skills','POST',NOW(),NOW()),
             (76,'/skills/createTemplate','创建技能模板','skills','POST',NOW(),NOW()),
             (77,'/skills/getTemplate','读取技能模板','skills','POST',NOW(),NOW()),
             (78,'/skills/saveTemplate','保存技能模板','skills','POST',NOW(),NOW()),
             (79,'/skills/getGlobalConstraint','读取全局约束','skills','POST',NOW(),NOW()),
             (80,'/skills/saveGlobalConstraint','保存全局约束','skills','POST',NOW(),NOW()),
             (81,'/skills/packageSkill','打包技能','skills','POST',NOW(),NOW())"
        ).await?;

        db.execute_unprepared(
            "INSERT INTO sys_apis (id, path, description, api_group, method, created_at, updated_at) VALUES
             (82,'/customer/customer','更新客户','客户','PUT',NOW(),NOW()),
             (83,'/customer/customer','创建客户','客户','POST',NOW(),NOW()),
             (84,'/customer/customer','删除客户','客户','DELETE',NOW(),NOW()),
             (85,'/customer/customer','获取单一客户','客户','GET',NOW(),NOW()),
             (86,'/customer/customerList','获取客户列表','客户','GET',NOW(),NOW()),
             (87,'/autoCode/getDB','获取所有数据库','代码生成器','GET',NOW(),NOW()),
             (88,'/autoCode/getTables','获取数据库表','代码生成器','GET',NOW(),NOW()),
             (89,'/autoCode/createTemp','自动化代码','代码生成器','POST',NOW(),NOW()),
             (90,'/autoCode/preview','预览自动化代码','代码生成器','POST',NOW(),NOW()),
             (91,'/autoCode/getColumn','获取所选table的所有字段','代码生成器','GET',NOW(),NOW()),
             (92,'/autoCode/installPlugin','安装插件','代码生成器','POST',NOW(),NOW()),
             (93,'/autoCode/pubPlug','打包插件','代码生成器','POST',NOW(),NOW()),
             (94,'/autoCode/removePlugin','卸载插件','代码生成器','POST',NOW(),NOW()),
             (95,'/autoCode/getPluginList','获取已安装插件','代码生成器','GET',NOW(),NOW()),
             (96,'/autoCode/mcp','自动生成 MCP Tool 模板','代码生成器','POST',NOW(),NOW()),
             (97,'/autoCode/mcpTest','MCP Tool 测试','代码生成器','POST',NOW(),NOW()),
             (98,'/autoCode/mcpList','获取 MCP ToolList','代码生成器','POST',NOW(),NOW())"
        ).await?;

        db.execute_unprepared(
            "INSERT INTO sys_apis (id, path, description, api_group, method, created_at, updated_at) VALUES
             (99,'/autoCode/createPackage','配置模板','模板配置','POST',NOW(),NOW()),
             (100,'/autoCode/getTemplates','获取模板文件','模板配置','GET',NOW(),NOW()),
             (101,'/autoCode/getPackage','获取所有模板','模板配置','POST',NOW(),NOW()),
             (102,'/autoCode/delPackage','删除模板','模板配置','POST',NOW(),NOW()),
             (103,'/autoCode/getMeta','获取meta信息','代码生成器历史','POST',NOW(),NOW()),
             (104,'/autoCode/rollback','回滚自动生成代码','代码生成器历史','POST',NOW(),NOW()),
             (105,'/autoCode/getSysHistory','查询回滚记录','代码生成器历史','POST',NOW(),NOW()),
             (106,'/autoCode/delSysHistory','删除回滚记录','代码生成器历史','POST',NOW(),NOW()),
             (107,'/autoCode/addFunc','增加模板方法','代码生成器历史','POST',NOW(),NOW())"
        ).await?;

        db.execute_unprepared(
            "INSERT INTO sys_apis (id, path, description, api_group, method, created_at, updated_at) VALUES
             (108,'/sysDictionaryDetail/updateSysDictionaryDetail','更新字典内容','系统字典详情','PUT',NOW(),NOW()),
             (109,'/sysDictionaryDetail/createSysDictionaryDetail','新增字典内容','系统字典详情','POST',NOW(),NOW()),
             (110,'/sysDictionaryDetail/deleteSysDictionaryDetail','删除字典内容','系统字典详情','DELETE',NOW(),NOW()),
             (111,'/sysDictionaryDetail/findSysDictionaryDetail','根据ID获取字典内容','系统字典详情','GET',NOW(),NOW()),
             (112,'/sysDictionaryDetail/getSysDictionaryDetailList','获取字典内容列表','系统字典详情','GET',NOW(),NOW()),
             (113,'/sysDictionaryDetail/getDictionaryTreeList','获取字典数列表','系统字典详情','GET',NOW(),NOW()),
             (114,'/sysDictionaryDetail/getDictionaryTreeListByType','根据分类获取字典数列表','系统字典详情','GET',NOW(),NOW()),
             (115,'/sysDictionaryDetail/getDictionaryDetailsByParent','根据父级ID获取字典详情','系统字典详情','GET',NOW(),NOW()),
             (116,'/sysDictionaryDetail/getDictionaryPath','获取字典详情的完整路径','系统字典详情','GET',NOW(),NOW()),
             (117,'/sysDictionary/createSysDictionary','新增字典','系统字典','POST',NOW(),NOW()),
             (118,'/sysDictionary/deleteSysDictionary','删除字典','系统字典','DELETE',NOW(),NOW()),
             (119,'/sysDictionary/updateSysDictionary','更新字典','系统字典','PUT',NOW(),NOW()),
             (120,'/sysDictionary/findSysDictionary','根据ID获取字典（建议选择）','系统字典','GET',NOW(),NOW()),
             (121,'/sysDictionary/getSysDictionaryList','获取字典列表','系统字典','GET',NOW(),NOW()),
             (122,'/sysDictionary/importSysDictionary','导入字典JSON','系统字典','POST',NOW(),NOW()),
             (123,'/sysDictionary/exportSysDictionary','导出字典JSON','系统字典','GET',NOW(),NOW())"
        ).await?;

        db.execute_unprepared(
            "INSERT INTO sys_apis (id, path, description, api_group, method, created_at, updated_at) VALUES
             (124,'/sysOperationRecord/createSysOperationRecord','新增操作记录','操作记录','POST',NOW(),NOW()),
             (125,'/sysOperationRecord/findSysOperationRecord','根据ID获取操作记录','操作记录','GET',NOW(),NOW()),
             (126,'/sysOperationRecord/getSysOperationRecordList','获取操作记录列表','操作记录','GET',NOW(),NOW()),
             (127,'/sysOperationRecord/deleteSysOperationRecord','删除操作记录','操作记录','DELETE',NOW(),NOW()),
             (128,'/sysOperationRecord/deleteSysOperationRecordByIds','批量删除操作历史','操作记录','DELETE',NOW(),NOW()),
             (129,'/simpleUploader/upload','插件版分片上传','断点续传(插件版)','POST',NOW(),NOW()),
             (130,'/simpleUploader/checkFileMd5','文件完整度验证','断点续传(插件版)','GET',NOW(),NOW()),
             (131,'/simpleUploader/mergeFileMd5','上传完成合并文件','断点续传(插件版)','GET',NOW(),NOW()),
             (132,'/email/emailTest','发送测试邮件','email','POST',NOW(),NOW()),
             (133,'/email/sendEmail','发送邮件','email','POST',NOW(),NOW()),
             (134,'/authorityBtn/setAuthorityBtn','设置按钮权限','按钮权限','POST',NOW(),NOW()),
             (135,'/authorityBtn/getAuthorityBtn','获取已有按钮权限','按钮权限','POST',NOW(),NOW()),
             (136,'/authorityBtn/canRemoveAuthorityBtn','删除按钮','按钮权限','POST',NOW(),NOW())"
        ).await?;

        db.execute_unprepared(
            "INSERT INTO sys_apis (id, path, description, api_group, method, created_at, updated_at) VALUES
             (137,'/sysExportTemplate/createSysExportTemplate','新增导出模板','导出模板','POST',NOW(),NOW()),
             (138,'/sysExportTemplate/deleteSysExportTemplate','删除导出模板','导出模板','DELETE',NOW(),NOW()),
             (139,'/sysExportTemplate/deleteSysExportTemplateByIds','批量删除导出模板','导出模板','DELETE',NOW(),NOW()),
             (140,'/sysExportTemplate/updateSysExportTemplate','更新导出模板','导出模板','PUT',NOW(),NOW()),
             (141,'/sysExportTemplate/findSysExportTemplate','根据ID获取导出模板','导出模板','GET',NOW(),NOW()),
             (142,'/sysExportTemplate/getSysExportTemplateList','获取导出模板列表','导出模板','GET',NOW(),NOW()),
             (143,'/sysExportTemplate/exportExcel','导出Excel','导出模板','GET',NOW(),NOW()),
             (144,'/sysExportTemplate/exportTemplate','下载模板','导出模板','GET',NOW(),NOW()),
             (145,'/sysExportTemplate/previewSQL','预览SQL','导出模板','GET',NOW(),NOW()),
             (146,'/sysExportTemplate/importExcel','导入Excel','导出模板','POST',NOW(),NOW()),
             (147,'/sysError/createSysError','新建错误日志','错误日志','POST',NOW(),NOW()),
             (148,'/sysError/deleteSysError','删除错误日志','错误日志','DELETE',NOW(),NOW()),
             (149,'/sysError/deleteSysErrorByIds','批量删除错误日志','错误日志','DELETE',NOW(),NOW()),
             (150,'/sysError/updateSysError','更新错误日志','错误日志','PUT',NOW(),NOW()),
             (151,'/sysError/findSysError','根据ID获取错误日志','错误日志','GET',NOW(),NOW()),
             (152,'/sysError/getSysErrorList','获取错误日志列表','错误日志','GET',NOW(),NOW()),
             (153,'/sysError/getSysErrorSolution','触发错误处理(异步)','错误日志','GET',NOW(),NOW())"
        ).await?;

        db.execute_unprepared(
            "INSERT INTO sys_apis (id, path, description, api_group, method, created_at, updated_at) VALUES
             (154,'/info/createInfo','新建公告','公告','POST',NOW(),NOW()),
             (155,'/info/deleteInfo','删除公告','公告','DELETE',NOW(),NOW()),
             (156,'/info/deleteInfoByIds','批量删除公告','公告','DELETE',NOW(),NOW()),
             (157,'/info/updateInfo','更新公告','公告','PUT',NOW(),NOW()),
             (158,'/info/findInfo','根据ID获取公告','公告','GET',NOW(),NOW()),
             (159,'/info/getInfoList','获取公告列表','公告','GET',NOW(),NOW()),
             (160,'/sysParams/createSysParams','新建参数','参数管理','POST',NOW(),NOW()),
             (161,'/sysParams/deleteSysParams','删除参数','参数管理','DELETE',NOW(),NOW()),
             (162,'/sysParams/deleteSysParamsByIds','批量删除参数','参数管理','DELETE',NOW(),NOW()),
             (163,'/sysParams/updateSysParams','更新参数','参数管理','PUT',NOW(),NOW()),
             (164,'/sysParams/findSysParams','根据ID获取参数','参数管理','GET',NOW(),NOW()),
             (165,'/sysParams/getSysParamsList','获取参数列表','参数管理','GET',NOW(),NOW()),
             (166,'/sysParams/getSysParam','获取参数列表','参数管理','GET',NOW(),NOW()),
             (167,'/attachmentCategory/getCategoryList','分类列表','媒体库分类','GET',NOW(),NOW()),
             (168,'/attachmentCategory/addCategory','添加/编辑分类','媒体库分类','POST',NOW(),NOW()),
             (169,'/attachmentCategory/deleteCategory','删除分类','媒体库分类','POST',NOW(),NOW()),
             (170,'/sysVersion/findSysVersion','获取单一版本','版本控制','GET',NOW(),NOW()),
             (171,'/sysVersion/getSysVersionList','获取版本列表','版本控制','GET',NOW(),NOW()),
             (172,'/sysVersion/downloadVersionJson','下载版本json','版本控制','GET',NOW(),NOW()),
             (173,'/sysVersion/exportVersion','创建版本','版本控制','POST',NOW(),NOW()),
             (174,'/sysVersion/importVersion','同步版本','版本控制','POST',NOW(),NOW()),
             (175,'/sysVersion/deleteSysVersion','删除版本','版本控制','DELETE',NOW(),NOW()),
             (176,'/sysVersion/deleteSysVersionByIds','批量删除版本','版本控制','DELETE',NOW(),NOW())"
        ).await?;

        // ========== 2. Casbin 权限规则（casbin_rule，对齐 ginvueadmin 数据库 - 角色888 超级管理员） ==========
        db.execute_unprepared(
            "INSERT INTO casbin_rule (ptype, v0, v1, v2, v3, v4, v5) VALUES
             ('p','888','/api/createApi','POST','','',''),
             ('p','888','/api/deleteApi','POST','','',''),
             ('p','888','/api/deleteApisByIds','DELETE','','',''),
             ('p','888','/api/enterSyncApi','POST','','',''),
             ('p','888','/api/getAllApis','POST','','',''),
             ('p','888','/api/getApiById','POST','','',''),
             ('p','888','/api/getApiGroups','GET','','',''),
             ('p','888','/api/getApiList','POST','','',''),
             ('p','888','/api/ignoreApi','POST','','',''),
             ('p','888','/api/syncApi','GET','','',''),
             ('p','888','/api/updateApi','POST','','',''),
             ('p','888','/attachmentCategory/addCategory','POST','','',''),
             ('p','888','/attachmentCategory/deleteCategory','POST','','',''),
             ('p','888','/attachmentCategory/getCategoryList','GET','','',''),
             ('p','888','/authority/copyAuthority','POST','','',''),
             ('p','888','/authority/createAuthority','POST','','',''),
             ('p','888','/authority/deleteAuthority','POST','','',''),
             ('p','888','/authority/getAuthorityList','POST','','',''),
             ('p','888','/authority/getUsersByAuthority','GET','','',''),
             ('p','888','/authority/setDataAuthority','POST','','',''),
             ('p','888','/authority/setRoleUsers','POST','','',''),
             ('p','888','/authority/updateAuthority','PUT','','',''),
             ('p','888','/authorityBtn/canRemoveAuthorityBtn','POST','','',''),
             ('p','888','/authorityBtn/getAuthorityBtn','POST','','',''),
             ('p','888','/authorityBtn/setAuthorityBtn','POST','','','')"
        ).await?;

        db.execute_unprepared(
            "INSERT INTO casbin_rule (ptype, v0, v1, v2, v3, v4, v5) VALUES
             ('p','888','/autoCode/addFunc','POST','','',''),
             ('p','888','/autoCode/createPackage','POST','','',''),
             ('p','888','/autoCode/createPlug','POST','','',''),
             ('p','888','/autoCode/createTemp','POST','','',''),
             ('p','888','/autoCode/delPackage','POST','','',''),
             ('p','888','/autoCode/delSysHistory','POST','','',''),
             ('p','888','/autoCode/getColumn','GET','','',''),
             ('p','888','/autoCode/getDB','GET','','',''),
             ('p','888','/autoCode/getMeta','POST','','',''),
             ('p','888','/autoCode/getPackage','POST','','',''),
             ('p','888','/autoCode/getPluginList','GET','','',''),
             ('p','888','/autoCode/getSysHistory','POST','','',''),
             ('p','888','/autoCode/getTables','GET','','',''),
             ('p','888','/autoCode/getTemplates','GET','','',''),
             ('p','888','/autoCode/installPlugin','POST','','',''),
             ('p','888','/autoCode/mcp','POST','','',''),
             ('p','888','/autoCode/mcpList','POST','','',''),
             ('p','888','/autoCode/mcpTest','POST','','',''),
             ('p','888','/autoCode/preview','POST','','',''),
             ('p','888','/autoCode/pubPlug','POST','','',''),
             ('p','888','/autoCode/removePlugin','POST','','',''),
             ('p','888','/autoCode/rollback','POST','','','')"
        ).await?;

        db.execute_unprepared(
            "INSERT INTO casbin_rule (ptype, v0, v1, v2, v3, v4, v5) VALUES
             ('p','888','/casbin/getPolicyPathByAuthorityId','POST','','',''),
             ('p','888','/casbin/updateCasbin','POST','','',''),
             ('p','888','/customer/customer','DELETE','','',''),
             ('p','888','/customer/customer','GET','','',''),
             ('p','888','/customer/customer','POST','','',''),
             ('p','888','/customer/customer','PUT','','',''),
             ('p','888','/customer/customerList','GET','','',''),
             ('p','888','/email/emailTest','POST','','',''),
             ('p','888','/email/sendEmail','POST','','',''),
             ('p','888','/fileUploadAndDownload/breakpointContinue','POST','','',''),
             ('p','888','/fileUploadAndDownload/breakpointContinueFinish','POST','','',''),
             ('p','888','/fileUploadAndDownload/deleteFile','POST','','',''),
             ('p','888','/fileUploadAndDownload/editFileName','POST','','',''),
             ('p','888','/fileUploadAndDownload/findFile','GET','','',''),
             ('p','888','/fileUploadAndDownload/getFileList','POST','','',''),
             ('p','888','/fileUploadAndDownload/importURL','POST','','',''),
             ('p','888','/fileUploadAndDownload/removeChunk','POST','','',''),
             ('p','888','/fileUploadAndDownload/upload','POST','','','')"
        ).await?;

        db.execute_unprepared(
            "INSERT INTO casbin_rule (ptype, v0, v1, v2, v3, v4, v5) VALUES
             ('p','888','/info/createInfo','POST','','',''),
             ('p','888','/info/deleteInfo','DELETE','','',''),
             ('p','888','/info/deleteInfoByIds','DELETE','','',''),
             ('p','888','/info/findInfo','GET','','',''),
             ('p','888','/info/getInfoList','GET','','',''),
             ('p','888','/info/updateInfo','PUT','','',''),
             ('p','888','/jwt/jsonInBlacklist','POST','','',''),
             ('p','888','/menu/addBaseMenu','POST','','',''),
             ('p','888','/menu/addMenuAuthority','POST','','',''),
             ('p','888','/menu/deleteBaseMenu','POST','','',''),
             ('p','888','/menu/getBaseMenuById','POST','','',''),
             ('p','888','/menu/getBaseMenuTree','POST','','',''),
             ('p','888','/menu/getMenu','POST','','',''),
             ('p','888','/menu/getMenuAuthority','POST','','',''),
             ('p','888','/menu/getMenuList','POST','','',''),
             ('p','888','/menu/updateBaseMenu','POST','','','')"
        ).await?;

        db.execute_unprepared(
            "INSERT INTO casbin_rule (ptype, v0, v1, v2, v3, v4, v5) VALUES
             ('p','888','/simpleUploader/checkFileMd5','GET','','',''),
             ('p','888','/simpleUploader/mergeFileMd5','GET','','',''),
             ('p','888','/simpleUploader/upload','POST','','',''),
             ('p','888','/skills/createReference','POST','','',''),
             ('p','888','/skills/createResource','POST','','',''),
             ('p','888','/skills/createScript','POST','','',''),
             ('p','888','/skills/createTemplate','POST','','',''),
             ('p','888','/skills/deleteSkill','POST','','',''),
             ('p','888','/skills/getGlobalConstraint','POST','','',''),
             ('p','888','/skills/getReference','POST','','',''),
             ('p','888','/skills/getResource','POST','','',''),
             ('p','888','/skills/getScript','POST','','',''),
             ('p','888','/skills/getSkillDetail','POST','','',''),
             ('p','888','/skills/getSkillList','POST','','',''),
             ('p','888','/skills/getTemplate','POST','','',''),
             ('p','888','/skills/getTools','GET','','',''),
             ('p','888','/skills/packageSkill','POST','','',''),
             ('p','888','/skills/saveGlobalConstraint','POST','','',''),
             ('p','888','/skills/saveReference','POST','','',''),
             ('p','888','/skills/saveResource','POST','','',''),
             ('p','888','/skills/saveScript','POST','','',''),
             ('p','888','/skills/saveSkill','POST','','',''),
             ('p','888','/skills/saveTemplate','POST','','','')"
        ).await?;

        db.execute_unprepared(
            "INSERT INTO casbin_rule (ptype, v0, v1, v2, v3, v4, v5) VALUES
             ('p','888','/sysApiToken/createApiToken','POST','','',''),
             ('p','888','/sysApiToken/deleteApiToken','POST','','',''),
             ('p','888','/sysApiToken/getApiTokenList','POST','','',''),
             ('p','888','/sysDictionary/createSysDictionary','POST','','',''),
             ('p','888','/sysDictionary/deleteSysDictionary','DELETE','','',''),
             ('p','888','/sysDictionary/exportSysDictionary','GET','','',''),
             ('p','888','/sysDictionary/findSysDictionary','GET','','',''),
             ('p','888','/sysDictionary/getSysDictionaryList','GET','','',''),
             ('p','888','/sysDictionary/importSysDictionary','POST','','',''),
             ('p','888','/sysDictionary/updateSysDictionary','PUT','','',''),
             ('p','888','/sysDictionaryDetail/createSysDictionaryDetail','POST','','',''),
             ('p','888','/sysDictionaryDetail/deleteSysDictionaryDetail','DELETE','','',''),
             ('p','888','/sysDictionaryDetail/findSysDictionaryDetail','GET','','',''),
             ('p','888','/sysDictionaryDetail/getDictionaryDetailsByParent','GET','','',''),
             ('p','888','/sysDictionaryDetail/getDictionaryPath','GET','','',''),
             ('p','888','/sysDictionaryDetail/getDictionaryTreeList','GET','','',''),
             ('p','888','/sysDictionaryDetail/getDictionaryTreeListByType','GET','','',''),
             ('p','888','/sysDictionaryDetail/getSysDictionaryDetailList','GET','','',''),
             ('p','888','/sysDictionaryDetail/updateSysDictionaryDetail','PUT','','','')"
        ).await?;

        db.execute_unprepared(
            "INSERT INTO casbin_rule (ptype, v0, v1, v2, v3, v4, v5) VALUES
             ('p','888','/sysError/createSysError','POST','','',''),
             ('p','888','/sysError/deleteSysError','DELETE','','',''),
             ('p','888','/sysError/deleteSysErrorByIds','DELETE','','',''),
             ('p','888','/sysError/findSysError','GET','','',''),
             ('p','888','/sysError/getSysErrorList','GET','','',''),
             ('p','888','/sysError/getSysErrorSolution','GET','','',''),
             ('p','888','/sysError/updateSysError','PUT','','',''),
             ('p','888','/sysExportTemplate/createSysExportTemplate','POST','','',''),
             ('p','888','/sysExportTemplate/deleteSysExportTemplate','DELETE','','',''),
             ('p','888','/sysExportTemplate/deleteSysExportTemplateByIds','DELETE','','',''),
             ('p','888','/sysExportTemplate/exportExcel','GET','','',''),
             ('p','888','/sysExportTemplate/exportTemplate','GET','','',''),
             ('p','888','/sysExportTemplate/findSysExportTemplate','GET','','',''),
             ('p','888','/sysExportTemplate/getSysExportTemplateList','GET','','',''),
             ('p','888','/sysExportTemplate/importExcel','POST','','',''),
             ('p','888','/sysExportTemplate/previewSQL','GET','','',''),
             ('p','888','/sysExportTemplate/updateSysExportTemplate','PUT','','','')"
        ).await?;

        db.execute_unprepared(
            "INSERT INTO casbin_rule (ptype, v0, v1, v2, v3, v4, v5) VALUES
             ('p','888','/sysLoginLog/deleteLoginLog','DELETE','','',''),
             ('p','888','/sysLoginLog/deleteLoginLogByIds','DELETE','','',''),
             ('p','888','/sysLoginLog/findLoginLog','GET','','',''),
             ('p','888','/sysLoginLog/getLoginLogList','GET','','',''),
             ('p','888','/sysOperationRecord/createSysOperationRecord','POST','','',''),
             ('p','888','/sysOperationRecord/deleteSysOperationRecord','DELETE','','',''),
             ('p','888','/sysOperationRecord/deleteSysOperationRecordByIds','DELETE','','',''),
             ('p','888','/sysOperationRecord/findSysOperationRecord','GET','','',''),
             ('p','888','/sysOperationRecord/getSysOperationRecordList','GET','','',''),
             ('p','888','/sysOperationRecord/updateSysOperationRecord','PUT','','',''),
             ('p','888','/sysParams/createSysParams','POST','','',''),
             ('p','888','/sysParams/deleteSysParams','DELETE','','',''),
             ('p','888','/sysParams/deleteSysParamsByIds','DELETE','','',''),
             ('p','888','/sysParams/findSysParams','GET','','',''),
             ('p','888','/sysParams/getSysParam','GET','','',''),
             ('p','888','/sysParams/getSysParamsList','GET','','',''),
             ('p','888','/sysParams/updateSysParams','PUT','','','')"
        ).await?;

        db.execute_unprepared(
            "INSERT INTO casbin_rule (ptype, v0, v1, v2, v3, v4, v5) VALUES
             ('p','888','/system/getServerInfo','POST','','',''),
             ('p','888','/system/getSystemConfig','POST','','',''),
             ('p','888','/system/setSystemConfig','POST','','',''),
             ('p','888','/sysVersion/deleteSysVersion','DELETE','','',''),
             ('p','888','/sysVersion/deleteSysVersionByIds','DELETE','','',''),
             ('p','888','/sysVersion/downloadVersionJson','GET','','',''),
             ('p','888','/sysVersion/exportVersion','POST','','',''),
             ('p','888','/sysVersion/findSysVersion','GET','','',''),
             ('p','888','/sysVersion/getSysVersionList','GET','','',''),
             ('p','888','/sysVersion/importVersion','POST','','',''),
             ('p','888','/user/admin_register','POST','','',''),
             ('p','888','/user/changePassword','POST','','',''),
             ('p','888','/user/deleteUser','DELETE','','',''),
             ('p','888','/user/getUserInfo','GET','','',''),
             ('p','888','/user/getUserList','POST','','',''),
             ('p','888','/user/resetPassword','POST','','',''),
             ('p','888','/user/setSelfInfo','PUT','','',''),
             ('p','888','/user/setSelfSetting','PUT','','',''),
             ('p','888','/user/setUserAuthorities','POST','','',''),
             ('p','888','/user/setUserAuthority','POST','','',''),
             ('p','888','/user/setUserInfo','PUT','','','')"
        ).await?;

        // ========== 3. Casbin 权限规则 - 角色 8881（管理员） ==========
        db.execute_unprepared(
            "INSERT INTO casbin_rule (ptype, v0, v1, v2, v3, v4, v5) VALUES
             ('p','8881','/api/createApi','POST','','',''),
             ('p','8881','/api/deleteApi','POST','','',''),
             ('p','8881','/api/getAllApis','POST','','',''),
             ('p','8881','/api/getApiById','POST','','',''),
             ('p','8881','/api/getApiList','POST','','',''),
             ('p','8881','/api/updateApi','POST','','',''),
             ('p','8881','/authority/createAuthority','POST','','',''),
             ('p','8881','/authority/deleteAuthority','POST','','',''),
             ('p','8881','/authority/getAuthorityList','POST','','',''),
             ('p','8881','/authority/getUsersByAuthority','GET','','',''),
             ('p','8881','/authority/setDataAuthority','POST','','',''),
             ('p','8881','/authority/setRoleUsers','POST','','',''),
             ('p','8881','/casbin/getPolicyPathByAuthorityId','POST','','',''),
             ('p','8881','/casbin/updateCasbin','POST','','',''),
             ('p','8881','/customer/customer','DELETE','','',''),
             ('p','8881','/customer/customer','GET','','',''),
             ('p','8881','/customer/customer','POST','','',''),
             ('p','8881','/customer/customer','PUT','','',''),
             ('p','8881','/customer/customerList','GET','','',''),
             ('p','8881','/fileUploadAndDownload/deleteFile','POST','','',''),
             ('p','8881','/fileUploadAndDownload/editFileName','POST','','',''),
             ('p','8881','/fileUploadAndDownload/getFileList','POST','','',''),
             ('p','8881','/fileUploadAndDownload/importURL','POST','','',''),
             ('p','8881','/fileUploadAndDownload/upload','POST','','',''),
             ('p','8881','/jwt/jsonInBlacklist','POST','','','')"
        ).await?;

        db.execute_unprepared(
            "INSERT INTO casbin_rule (ptype, v0, v1, v2, v3, v4, v5) VALUES
             ('p','8881','/menu/addBaseMenu','POST','','',''),
             ('p','8881','/menu/addMenuAuthority','POST','','',''),
             ('p','8881','/menu/deleteBaseMenu','POST','','',''),
             ('p','8881','/menu/getBaseMenuById','POST','','',''),
             ('p','8881','/menu/getBaseMenuTree','POST','','',''),
             ('p','8881','/menu/getMenu','POST','','',''),
             ('p','8881','/menu/getMenuAuthority','POST','','',''),
             ('p','8881','/menu/getMenuList','POST','','',''),
             ('p','8881','/menu/updateBaseMenu','POST','','',''),
             ('p','8881','/system/getSystemConfig','POST','','',''),
             ('p','8881','/system/setSystemConfig','POST','','',''),
             ('p','8881','/user/admin_register','POST','','',''),
             ('p','8881','/user/changePassword','POST','','',''),
             ('p','8881','/user/getUserInfo','GET','','',''),
             ('p','8881','/user/getUserList','POST','','',''),
             ('p','8881','/user/setUserAuthority','POST','','','')"
        ).await?;

        // ========== 4. Casbin 权限规则 - 角色 9528（测试角色） ==========
        db.execute_unprepared(
            "INSERT INTO casbin_rule (ptype, v0, v1, v2, v3, v4, v5) VALUES
             ('p','9528','/api/createApi','POST','','',''),
             ('p','9528','/api/deleteApi','POST','','',''),
             ('p','9528','/api/getAllApis','POST','','',''),
             ('p','9528','/api/getApiById','POST','','',''),
             ('p','9528','/api/getApiList','POST','','',''),
             ('p','9528','/api/updateApi','POST','','',''),
             ('p','9528','/authority/createAuthority','POST','','',''),
             ('p','9528','/authority/deleteAuthority','POST','','',''),
             ('p','9528','/authority/getAuthorityList','POST','','',''),
             ('p','9528','/authority/getUsersByAuthority','GET','','',''),
             ('p','9528','/authority/setDataAuthority','POST','','',''),
             ('p','9528','/authority/setRoleUsers','POST','','',''),
             ('p','9528','/autoCode/createTemp','POST','','',''),
             ('p','9528','/casbin/getPolicyPathByAuthorityId','POST','','',''),
             ('p','9528','/casbin/updateCasbin','POST','','',''),
             ('p','9528','/customer/customer','DELETE','','',''),
             ('p','9528','/customer/customer','GET','','',''),
             ('p','9528','/customer/customer','POST','','',''),
             ('p','9528','/customer/customer','PUT','','',''),
             ('p','9528','/customer/customerList','GET','','',''),
             ('p','9528','/fileUploadAndDownload/deleteFile','POST','','',''),
             ('p','9528','/fileUploadAndDownload/editFileName','POST','','',''),
             ('p','9528','/fileUploadAndDownload/getFileList','POST','','',''),
             ('p','9528','/fileUploadAndDownload/importURL','POST','','',''),
             ('p','9528','/fileUploadAndDownload/upload','POST','','','')"
        ).await?;

        db.execute_unprepared(
            "INSERT INTO casbin_rule (ptype, v0, v1, v2, v3, v4, v5) VALUES
             ('p','9528','/jwt/jsonInBlacklist','POST','','',''),
             ('p','9528','/menu/addBaseMenu','POST','','',''),
             ('p','9528','/menu/addMenuAuthority','POST','','',''),
             ('p','9528','/menu/deleteBaseMenu','POST','','',''),
             ('p','9528','/menu/getBaseMenuById','POST','','',''),
             ('p','9528','/menu/getBaseMenuTree','POST','','',''),
             ('p','9528','/menu/getMenu','POST','','',''),
             ('p','9528','/menu/getMenuAuthority','POST','','',''),
             ('p','9528','/menu/getMenuList','POST','','',''),
             ('p','9528','/menu/updateBaseMenu','POST','','',''),
             ('p','9528','/system/getSystemConfig','POST','','',''),
             ('p','9528','/system/setSystemConfig','POST','','',''),
             ('p','9528','/user/admin_register','POST','','',''),
             ('p','9528','/user/changePassword','POST','','',''),
             ('p','9528','/user/getUserInfo','GET','','',''),
             ('p','9528','/user/getUserList','POST','','',''),
             ('p','9528','/user/setUserAuthority','POST','','','')"
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("DELETE FROM casbin_rule").await?;
        db.execute_unprepared("DELETE FROM sys_apis").await?;
        Ok(())
    }
}
