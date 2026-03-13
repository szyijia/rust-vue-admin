use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20240101_000002_seed_data"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // ========== 1. 角色数据（sys_authorities，对齐 ginvueadmin 数据库） ==========
        db.execute_unprepared(
            "INSERT INTO sys_authorities (authority_id, authority_name, parent_id, default_router, created_at, updated_at) VALUES
             (888,  '超级管理员', 0,   'dashboard', NOW(), NOW()),
             (8881, '管理员',     888, 'dashboard', NOW(), NOW()),
             (9528, '测试角色',   0,   'dashboard', NOW(), NOW())"
        ).await?;

        // ========== 2. 用户数据（sys_users，admin密码将由initDb接口更新） ==========
        db.execute_unprepared(
            "INSERT INTO sys_users (id, uuid, username, password, nick_name, header_img, authority_id, phone, email, enable, created_at, updated_at) VALUES
             (1, '02c80c06-ed32-45ab-8130-279427e1b074', 'admin', '', 'admin', 'https://qmplusimg.henrongyi.top/gva_header.jpg', 888, '17611111111', '333333333@qq.com', 1, NOW(), NOW())"
        ).await?;

        // ========== 3. 用户-角色关联（sys_user_authority） ==========
        db.execute_unprepared(
            "INSERT INTO sys_user_authority (sys_user_id, sys_authority_authority_id) VALUES
             (1, 888), (1, 8881), (1, 9528)"
        ).await?;

        // ========== 4. 角色数据权限（sys_data_authority_id） ==========
        db.execute_unprepared(
            "INSERT INTO sys_data_authority_id (sys_authority_authority_id, data_authority_id_authority_id) VALUES
             (888, 888), (888, 8881), (888, 9528), (9528, 8881), (9528, 9528)"
        ).await?;

        // ========== 5. 菜单数据（sys_base_menus，对齐 ginvueadmin 数据库） ==========
        db.execute_unprepared(
            "INSERT INTO sys_base_menus (id, menu_level, parent_id, path, name, hidden, component, sort, active_name, keep_alive, default_menu, title, icon, close_tab, transition_type, created_at, updated_at) VALUES
             (1,  0, 0, 'dashboard',   'dashboard',   0, 'view/dashboard/index.vue',        1, '', 0, 0, '仪表盘',     'odometer',       0, '', NOW(), NOW()),
             (2,  0, 0, 'about',       'about',       0, 'view/about/index.vue',             9, '', 0, 0, '关于我们',   'info-filled',    0, '', NOW(), NOW()),
             (3,  0, 0, 'admin',       'superAdmin',  0, 'view/superAdmin/index.vue',        3, '', 0, 0, '超级管理员', 'user',           0, '', NOW(), NOW()),
             (4,  0, 0, 'person',      'person',      1, 'view/person/person.vue',           4, '', 0, 0, '个人信息',   'message',        0, '', NOW(), NOW()),
             (5,  0, 0, 'example',     'example',     0, 'view/example/index.vue',           7, '', 0, 0, '示例文件',   'management',     0, '', NOW(), NOW()),
             (6,  0, 0, 'systemTools', 'systemTools', 0, 'view/systemTools/index.vue',       5, '', 0, 0, '系统工具',   'tools',          0, '', NOW(), NOW()),
             (7,  0, 0, 'https://www.rust-vue-admin.com', 'https://www.rust-vue-admin.com', 0, '/', 0, '', 0, 0, '官方网站', 'customer-gva', 0, '', NOW(), NOW()),
             (8,  0, 0, 'state',       'state',       0, 'view/system/state.vue',            8, '', 0, 0, '服务器状态', 'cloudy',         0, '', NOW(), NOW()),
             (9,  0, 0, 'plugin',      'plugin',      0, 'view/routerHolder.vue',            6, '', 0, 0, '插件系统',   'cherry',         0, '', NOW(), NOW())"
        ).await?;

        db.execute_unprepared(
            "INSERT INTO sys_base_menus (id, menu_level, parent_id, path, name, hidden, component, sort, active_name, keep_alive, default_menu, title, icon, close_tab, transition_type, created_at, updated_at) VALUES
             (10, 1, 3, 'authority',   'authority',   0, 'view/superAdmin/authority/authority.vue',    1, '', 0, 0, '角色管理',   'avatar',        0, '', NOW(), NOW()),
             (11, 1, 3, 'menu',        'menu',        0, 'view/superAdmin/menu/menu.vue',             2, '', 1, 0, '菜单管理',   'tickets',       0, '', NOW(), NOW()),
             (12, 1, 3, 'api',         'api',         0, 'view/superAdmin/api/api.vue',               3, '', 1, 0, 'api管理',    'platform',      0, '', NOW(), NOW()),
             (13, 1, 3, 'user',        'user',        0, 'view/superAdmin/user/user.vue',             4, '', 0, 0, '用户管理',   'coordinate',    0, '', NOW(), NOW()),
             (14, 1, 3, 'dictionary',  'dictionary',  0, 'view/superAdmin/dictionary/sysDictionary.vue', 5, '', 0, 0, '字典管理', 'notebook',     0, '', NOW(), NOW()),
             (15, 1, 3, 'operation',   'operation',   0, 'view/superAdmin/operation/sysOperationRecord.vue', 6, '', 0, 0, '操作历史', 'pie-chart', 0, '', NOW(), NOW()),
             (16, 1, 3, 'sysParams',   'sysParams',   0, 'view/superAdmin/params/sysParams.vue',     7, '', 0, 0, '参数管理',   'compass',       0, '', NOW(), NOW()),
             (17, 1, 5, 'upload',      'upload',      0, 'view/example/upload/upload.vue',            5, '', 0, 0, '媒体库（上传下载）', 'upload', 0, '', NOW(), NOW()),
             (18, 1, 5, 'breakpoint',  'breakpoint',  0, 'view/example/breakpoint/breakpoint.vue',    6, '', 0, 0, '断点续传', 'upload-filled',   0, '', NOW(), NOW()),
             (19, 1, 5, 'customer',    'customer',    0, 'view/example/customer/customer.vue',        7, '', 0, 0, '客户列表（资源示例）', 'avatar', 0, '', NOW(), NOW())"
        ).await?;

        db.execute_unprepared(
            "INSERT INTO sys_base_menus (id, menu_level, parent_id, path, name, hidden, component, sort, active_name, keep_alive, default_menu, title, icon, close_tab, transition_type, created_at, updated_at) VALUES
             (20, 1, 6, 'autoCode',         'autoCode',         0, 'view/systemTools/autoCode/index.vue',         1, '', 1, 0, '代码生成器',         'cpu',            0, '', NOW(), NOW()),
             (21, 1, 6, 'formCreate',       'formCreate',       0, 'view/systemTools/formCreate/index.vue',       3, '', 1, 0, '表单生成器',         'magic-stick',    0, '', NOW(), NOW()),
             (22, 1, 6, 'system',           'system',           0, 'view/systemTools/system/system.vue',          4, '', 0, 0, '系统配置',           'operation',      0, '', NOW(), NOW()),
             (23, 1, 6, 'autoCodeAdmin',    'autoCodeAdmin',    0, 'view/systemTools/autoCodeAdmin/index.vue',    2, '', 0, 0, '自动化代码管理',     'magic-stick',    0, '', NOW(), NOW()),
             (24, 1, 6, 'loginLog',         'loginLog',         0, 'view/systemTools/loginLog/index.vue',         5, '', 0, 0, '登录日志',           'monitor',        0, '', NOW(), NOW()),
             (25, 1, 6, 'apiToken',         'apiToken',         0, 'view/systemTools/apiToken/index.vue',         6, '', 0, 0, 'API Token',          'key',            0, '', NOW(), NOW()),
             (26, 1, 6, 'autoCodeEdit/:id', 'autoCodeEdit',     1, 'view/systemTools/autoCode/index.vue',         0, '', 0, 0, '自动化代码-${id}',   'magic-stick',    0, '', NOW(), NOW()),
             (27, 1, 6, 'autoPkg',          'autoPkg',          0, 'view/systemTools/autoPkg/autoPkg.vue',        0, '', 0, 0, '模板配置',           'folder',         0, '', NOW(), NOW()),
             (28, 1, 6, 'exportTemplate',   'exportTemplate',   0, 'view/systemTools/exportTemplate/exportTemplate.vue', 5, '', 0, 0, '导出模板', 'reading',          0, '', NOW(), NOW()),
             (29, 1, 6, 'skills',           'skills',           0, 'view/systemTools/skills/index.vue',           6, '', 0, 0, 'Skills管理',         'document',       0, '', NOW(), NOW()),
             (30, 1, 6, 'picture',          'picture',          0, 'view/systemTools/autoCode/picture.vue',       6, '', 0, 0, 'AI页面绘制',         'picture-filled', 0, '', NOW(), NOW()),
             (31, 1, 6, 'mcpTool',          'mcpTool',          0, 'view/systemTools/autoCode/mcp.vue',           7, '', 0, 0, 'Mcp Tools模板',      'magnet',         0, '', NOW(), NOW()),
             (32, 1, 6, 'mcpTest',          'mcpTest',          0, 'view/systemTools/autoCode/mcpTest.vue',       7, '', 0, 0, 'Mcp Tools测试',      'partly-cloudy',  0, '', NOW(), NOW()),
             (33, 1, 6, 'sysVersion',       'sysVersion',       0, 'view/systemTools/version/version.vue',        8, '', 0, 0, '版本管理',           'server',         0, '', NOW(), NOW()),
             (34, 1, 6, 'sysError',         'sysError',         0, 'view/systemTools/sysError/sysError.vue',      9, '', 0, 0, '错误日志',           'warn',           0, '', NOW(), NOW())"
        ).await?;

        db.execute_unprepared(
            "INSERT INTO sys_base_menus (id, menu_level, parent_id, path, name, hidden, component, sort, active_name, keep_alive, default_menu, title, icon, close_tab, transition_type, created_at, updated_at) VALUES
             (35, 1, 9, 'https://plugin.rust-vue-admin.com/', 'https://plugin.rust-vue-admin.com/', 0, 'https://plugin.rust-vue-admin.com/', 0, '', 0, 0, '插件市场',     'shop',             0, '', NOW(), NOW()),
             (36, 1, 9, 'installPlugin', 'installPlugin', 0, 'view/systemTools/installPlugin/index.vue', 1, '', 0, 0, '插件安装',     'box',              0, '', NOW(), NOW()),
             (37, 1, 9, 'pubPlug',       'pubPlug',       0, 'view/systemTools/pubPlug/pubPlug.vue',     3, '', 0, 0, '打包插件',     'files',            0, '', NOW(), NOW()),
             (38, 1, 9, 'plugin-email',  'plugin-email',  0, 'plugin/email/view/index.vue',              4, '', 0, 0, '邮件插件',     'message',          0, '', NOW(), NOW()),
             (39, 1, 9, 'anInfo',        'anInfo',        0, 'plugin/announcement/view/info.vue',        5, '', 0, 0, '公告管理[示例]', 'scaleToOriginal', 0, '', NOW(), NOW())"
        ).await?;

        // ========== 6. 角色-菜单绑定（sys_authority_menus，对齐 ginvueadmin 数据库） ==========
        // 888（超级管理员）：精简菜单
        db.execute_unprepared(
            "INSERT INTO sys_authority_menus (sys_base_menu_id, sys_authority_authority_id) VALUES
             (1,888),(3,888),(4,888),(6,888),(7,888),
             (10,888),(11,888),(12,888),(13,888),(14,888),(22,888)"
        ).await?;

        // 8881（管理员）：完整菜单
        db.execute_unprepared(
            "INSERT INTO sys_authority_menus (sys_base_menu_id, sys_authority_authority_id) VALUES
             (1,8881),(2,8881),(3,8881),(4,8881),(5,8881),(6,8881),(7,8881),(8,8881),(9,8881),
             (17,8881),(18,8881),(19,8881),(20,8881),(21,8881),(22,8881),(23,8881),
             (24,8881),(25,8881),(26,8881),(27,8881),(28,8881),
             (29,8881),(30,8881),(31,8881),(32,8881),(33,8881),(34,8881)"
        ).await?;

        // 9528（测试角色）：最少菜单
        db.execute_unprepared(
            "INSERT INTO sys_authority_menus (sys_base_menu_id, sys_authority_authority_id) VALUES
             (1,9528),(2,9528),(4,9528),(8,9528)"
        ).await?;

        // ========== 7. 字典数据（sys_dictionaries） ==========
        db.execute_unprepared(
            "INSERT INTO sys_dictionaries (id, name, type, status, `desc`, created_at, updated_at) VALUES
             (1, '性别',             'gender',    1, '性别字典',             NOW(), NOW()),
             (2, '数据库int类型',    'int',       1, 'int类型对应的数据库类型', NOW(), NOW()),
             (3, '数据库时间日期类型','time.Time', 1, '数据库时间日期类型',     NOW(), NOW()),
             (4, '数据库浮点型',     'float64',   1, '数据库浮点型',          NOW(), NOW()),
             (5, '数据库字符串',     'string',    1, '数据库字符串',          NOW(), NOW()),
             (6, '数据库bool类型',   'bool',      1, '数据库bool类型',       NOW(), NOW())"
        ).await?;

        // ========== 8. 字典详情数据（sys_dictionary_details，对齐 ginvueadmin 数据库） ==========
        db.execute_unprepared(
            "INSERT INTO sys_dictionary_details (id, label, value, extend, status, sort, sys_dictionary_id, parent_id, level, path, created_at, updated_at) VALUES
             (1,'男','1','',1,1,1,NULL,0,'',NOW(),NOW()),(2,'女','2','',1,2,1,NULL,0,'',NOW(),NOW()),
             (3,'smallint','1','mysql',1,1,2,NULL,0,'',NOW(),NOW()),(4,'mediumint','2','mysql',1,2,2,NULL,0,'',NOW(),NOW()),
             (5,'int','3','mysql',1,3,2,NULL,0,'',NOW(),NOW()),(6,'bigint','4','mysql',1,4,2,NULL,0,'',NOW(),NOW()),
             (7,'int2','5','pgsql',1,5,2,NULL,0,'',NOW(),NOW()),(8,'int4','6','pgsql',1,6,2,NULL,0,'',NOW(),NOW()),
             (9,'int6','7','pgsql',1,7,2,NULL,0,'',NOW(),NOW()),(10,'int8','8','pgsql',1,8,2,NULL,0,'',NOW(),NOW()),
             (11,'date','0','mysql',1,0,3,NULL,0,'',NOW(),NOW()),(12,'time','1','mysql',1,1,3,NULL,0,'',NOW(),NOW()),
             (13,'year','2','mysql',1,2,3,NULL,0,'',NOW(),NOW()),(14,'datetime','3','mysql',1,3,3,NULL,0,'',NOW(),NOW()),
             (15,'timestamp','5','mysql',1,5,3,NULL,0,'',NOW(),NOW()),(16,'timestamptz','6','pgsql',1,5,3,NULL,0,'',NOW(),NOW()),
             (17,'float','0','mysql',1,0,4,NULL,0,'',NOW(),NOW()),(18,'double','1','mysql',1,1,4,NULL,0,'',NOW(),NOW()),
             (19,'decimal','2','mysql',1,2,4,NULL,0,'',NOW(),NOW()),(20,'numeric','3','pgsql',1,3,4,NULL,0,'',NOW(),NOW()),
             (21,'smallserial','4','pgsql',1,4,4,NULL,0,'',NOW(),NOW()),
             (22,'char','0','mysql',1,0,5,NULL,0,'',NOW(),NOW()),(23,'varchar','1','mysql',1,1,5,NULL,0,'',NOW(),NOW()),
             (24,'tinyblob','2','mysql',1,2,5,NULL,0,'',NOW(),NOW()),(25,'tinytext','3','mysql',1,3,5,NULL,0,'',NOW(),NOW()),
             (26,'text','4','mysql',1,4,5,NULL,0,'',NOW(),NOW()),(27,'blob','5','mysql',1,5,5,NULL,0,'',NOW(),NOW()),
             (28,'mediumblob','6','mysql',1,6,5,NULL,0,'',NOW(),NOW()),(29,'mediumtext','7','mysql',1,7,5,NULL,0,'',NOW(),NOW()),
             (30,'longblob','8','mysql',1,8,5,NULL,0,'',NOW(),NOW()),(31,'longtext','9','mysql',1,9,5,NULL,0,'',NOW(),NOW()),
             (32,'tinyint','1','mysql',1,0,6,NULL,0,'',NOW(),NOW()),(33,'bool','2','pgsql',1,0,6,NULL,0,'',NOW(),NOW())"
        ).await?;

        // ========== 9. 导出模板数据（sys_export_templates） ==========
        db.execute_unprepared(
            r#"INSERT INTO sys_export_templates (id, db_name, name, table_name, template_id, template_info, created_at, updated_at) VALUES
             (1, '', 'api', 'sys_apis', 'api', '{\n"path":"路径",\n"method":"方法（大写）",\n"description":"方法介绍",\n"api_group":"方法分组"\n}', NOW(), NOW())"#
        ).await?;

        // ========== 10. 文件上传示例数据（exa_file_upload_and_downloads） ==========
        db.execute_unprepared(
            "INSERT INTO exa_file_upload_and_downloads (id, name, class_id, url, tag, `key`, created_at, updated_at) VALUES
             (1, '10.png',   0, 'https://qmplusimg.henrongyi.top/gvalogo.png',              'png', '158787308910.png',  NOW(), NOW()),
             (2, 'logo.png', 0, 'https://qmplusimg.henrongyi.top/1576554439myAvatar.png',   'png', '1587973709logo.png', NOW(), NOW())"
        ).await?;

        // ========== 11. 忽略的API数据（sys_ignore_apis） ==========
        db.execute_unprepared(
            "INSERT INTO sys_ignore_apis (path, method, created_at, updated_at) VALUES
             ('/swagger/*any',                  'GET',  NOW(), NOW()),
             ('/api/freshCasbin',               'GET',  NOW(), NOW()),
             ('/uploads/file/*filepath',        'GET',  NOW(), NOW()),
             ('/health',                        'GET',  NOW(), NOW()),
             ('/uploads/file/*filepath',        'HEAD', NOW(), NOW()),
             ('/autoCode/llmAuto',              'POST', NOW(), NOW()),
             ('/system/reloadSystem',           'POST', NOW(), NOW()),
             ('/base/login',                    'POST', NOW(), NOW()),
             ('/base/captcha',                  'POST', NOW(), NOW()),
             ('/init/initdb',                   'POST', NOW(), NOW()),
             ('/init/checkdb',                  'POST', NOW(), NOW()),
             ('/info/getInfoDataSource',        'GET',  NOW(), NOW()),
             ('/info/getInfoPublic',            'GET',  NOW(), NOW())"
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("DELETE FROM sys_ignore_apis").await?;
        db.execute_unprepared("DELETE FROM exa_file_upload_and_downloads").await?;
        db.execute_unprepared("DELETE FROM sys_export_templates").await?;
        db.execute_unprepared("DELETE FROM sys_dictionary_details").await?;
        db.execute_unprepared("DELETE FROM sys_dictionaries").await?;
        db.execute_unprepared("DELETE FROM sys_authority_menus").await?;
        db.execute_unprepared("DELETE FROM sys_base_menus").await?;
        db.execute_unprepared("DELETE FROM sys_data_authority_id").await?;
        db.execute_unprepared("DELETE FROM sys_user_authority").await?;
        db.execute_unprepared("DELETE FROM sys_users").await?;
        db.execute_unprepared("DELETE FROM sys_authorities").await?;
        Ok(())
    }
}
