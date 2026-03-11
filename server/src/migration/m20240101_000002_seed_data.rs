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

        // ========== 1. 角色数据 ==========
        db.execute_unprepared(
            "INSERT INTO sys_authorities (authority_id, authority_name, parent_id, default_router, created_at, updated_at)
             VALUES (888, '超级管理员', 0, 'dashboard', NOW(), NOW())"
        ).await?;

        // ========== 2. 菜单数据 ==========
        db.execute_unprepared(
            "INSERT INTO sys_base_menus (id, parent_id, path, name, hidden, component, sort, keep_alive, default_menu, title, icon, close_tab, created_at, updated_at) VALUES
             (1, 0, 'dashboard',  'dashboard',  0, 'view/dashboard/index.vue',                 1, 1, 0, '仪表盘',     'odometer',   0, NOW(), NOW()),
             (2, 0, 'superAdmin', 'superAdmin', 0, 'view/superAdmin/index.vue',                2, 0, 0, '超级管理员', 'user',       0, NOW(), NOW()),
             (3, 2, 'authority',  'authority',  0, 'view/superAdmin/authority/authority.vue',    1, 0, 0, '角色管理',   'avatar',     0, NOW(), NOW()),
             (4, 2, 'menu',       'menu',       0, 'view/superAdmin/menu/menu.vue',             2, 0, 0, '菜单管理',   'tickets',    0, NOW(), NOW()),
             (5, 2, 'api',        'api',        0, 'view/superAdmin/api/api.vue',               3, 0, 0, 'API管理',    'platform',   0, NOW(), NOW()),
             (6, 2, 'user',       'user',       0, 'view/superAdmin/user/user.vue',             4, 0, 0, '用户管理',   'coordinate', 0, NOW(), NOW()),
             (7, 0, 'person',     'person',     1, 'view/person/person.vue',                    0, 0, 0, '个人信息',   'avatar',     0, NOW(), NOW())"
        ).await?;

        // ========== 3. 角色-菜单绑定 ==========
        db.execute_unprepared(
            "INSERT INTO sys_authority_menus (sys_base_menu_id, sys_authority_authority_id) VALUES
             (1, 888), (2, 888), (3, 888), (4, 888), (5, 888), (6, 888), (7, 888)"
        ).await?;

        // ========== 4. API 数据 ==========
        db.execute_unprepared(
            "INSERT INTO sys_apis (path, description, api_group, method, created_at, updated_at) VALUES
             ('/base/login',                             '用户登录',         'base',      'POST', NOW(), NOW()),
             ('/base/register',                          '用户注册',         'base',      'POST', NOW(), NOW()),
             ('/base/captcha',                           '获取验证码',       'base',      'GET',  NOW(), NOW()),
             ('/user/getUserInfo',                       '获取用户信息',     'user',      'GET',  NOW(), NOW()),
             ('/user/getUserList',                       '获取用户列表',     'user',      'POST', NOW(), NOW()),
             ('/user/setUserAuthority',                  '设置用户角色',     'user',      'POST', NOW(), NOW()),
             ('/user/deleteUser',                        '删除用户',         'user',      'DELETE', NOW(), NOW()),
             ('/user/setSelfInfo',                       '修改个人信息',     'user',      'PUT',  NOW(), NOW()),
             ('/user/setUserInfo',                       '修改用户信息',     'user',      'PUT',  NOW(), NOW()),
             ('/user/resetPassword',                     '重置密码',         'user',      'POST', NOW(), NOW()),
             ('/user/setUserEnable',                     '设置用户启用状态', 'user',      'POST', NOW(), NOW()),
             ('/user/changePassword',                    '修改密码',         'user',      'POST', NOW(), NOW()),
             ('/user/admin_register',                    '管理员注册用户',   'user',      'POST', NOW(), NOW()),
             ('/jwt/jsonInBlacklist',                    '拉黑JWT',          'jwt',       'POST', NOW(), NOW()),
             ('/authority/createAuthority',              '创建角色',         'authority', 'POST', NOW(), NOW()),
             ('/authority/deleteAuthority',              '删除角色',         'authority', 'POST', NOW(), NOW()),
             ('/authority/updateAuthority',              '更新角色',         'authority', 'PUT',  NOW(), NOW()),
             ('/authority/getAuthorityList',             '获取角色列表',     'authority', 'POST', NOW(), NOW()),
             ('/authority/setDataAuthority',             '设置数据权限',     'authority', 'POST', NOW(), NOW()),
             ('/authority/copyAuthority',                '拷贝角色',         'authority', 'POST', NOW(), NOW()),
             ('/menu/getMenu',                           '获取用户菜单',     'menu',      'POST', NOW(), NOW()),
             ('/menu/getMenuList',                       '获取菜单列表',     'menu',      'POST', NOW(), NOW()),
             ('/menu/addBaseMenu',                       '新增菜单',         'menu',      'POST', NOW(), NOW()),
             ('/menu/deleteBaseMenu',                    '删除菜单',         'menu',      'POST', NOW(), NOW()),
             ('/menu/updateBaseMenu',                    '更新菜单',         'menu',      'POST', NOW(), NOW()),
             ('/menu/getMenuAuthority',                  '获取角色菜单',     'menu',      'POST', NOW(), NOW()),
             ('/menu/addMenuAuthority',                  '设置角色菜单',     'menu',      'POST', NOW(), NOW()),
             ('/menu/getBaseMenuTree',                   '获取菜单树',       'menu',      'POST', NOW(), NOW()),
             ('/menu/getBaseMenuById',                   '根据ID获取菜单',   'menu',      'POST', NOW(), NOW()),
             ('/api/createApi',                          '创建API',          'api',       'POST', NOW(), NOW()),
             ('/api/deleteApi',                          '删除API',          'api',       'POST', NOW(), NOW()),
             ('/api/updateApi',                          '更新API',          'api',       'POST', NOW(), NOW()),
             ('/api/getApiList',                         '获取API列表',      'api',       'POST', NOW(), NOW()),
             ('/api/getAllApis',                          '获取所有API',      'api',       'POST', NOW(), NOW()),
             ('/api/deleteApisByIds',                    '批量删除API',      'api',       'DELETE', NOW(), NOW()),
             ('/api/getApiById',                         '根据ID获取API',    'api',       'POST', NOW(), NOW()),
             ('/api/getApiGroups',                       '获取API分组',      'api',       'GET',  NOW(), NOW()),
             ('/casbin/UpdateCasbin',                    '更新角色API权限',  'casbin',    'POST', NOW(), NOW()),
             ('/casbin/getPolicyPathByAuthorityId',      '获取角色权限列表', 'casbin',    'POST', NOW(), NOW())"
        ).await?;

        // ========== 5. Casbin 权限规则 ==========
        db.execute_unprepared(
            "INSERT INTO casbin_rules (ptype, v0, v1, v2, v3, v4, v5) VALUES
             ('p', '888', '/base/login',                             'POST',   '', '', ''),
             ('p', '888', '/base/register',                          'POST',   '', '', ''),
             ('p', '888', '/base/captcha',                           'GET',    '', '', ''),
             ('p', '888', '/user/getUserInfo',                       'GET',    '', '', ''),
             ('p', '888', '/user/getUserList',                       'POST',   '', '', ''),
             ('p', '888', '/user/setUserAuthority',                  'POST',   '', '', ''),
             ('p', '888', '/user/deleteUser',                        'DELETE', '', '', ''),
             ('p', '888', '/user/setSelfInfo',                       'PUT',    '', '', ''),
             ('p', '888', '/user/setUserInfo',                       'PUT',    '', '', ''),
             ('p', '888', '/user/resetPassword',                     'POST',   '', '', ''),
             ('p', '888', '/user/setUserEnable',                     'POST',   '', '', ''),
             ('p', '888', '/user/changePassword',                    'POST',   '', '', ''),
             ('p', '888', '/user/admin_register',                    'POST',   '', '', ''),
             ('p', '888', '/jwt/jsonInBlacklist',                    'POST',   '', '', ''),
             ('p', '888', '/authority/createAuthority',              'POST',   '', '', ''),
             ('p', '888', '/authority/deleteAuthority',              'POST',   '', '', ''),
             ('p', '888', '/authority/updateAuthority',              'PUT',    '', '', ''),
             ('p', '888', '/authority/getAuthorityList',             'POST',   '', '', ''),
             ('p', '888', '/authority/setDataAuthority',             'POST',   '', '', ''),
             ('p', '888', '/authority/copyAuthority',                'POST',   '', '', ''),
             ('p', '888', '/menu/getMenu',                           'POST',   '', '', ''),
             ('p', '888', '/menu/getMenuList',                       'POST',   '', '', ''),
             ('p', '888', '/menu/addBaseMenu',                       'POST',   '', '', ''),
             ('p', '888', '/menu/deleteBaseMenu',                    'POST',   '', '', ''),
             ('p', '888', '/menu/updateBaseMenu',                    'POST',   '', '', ''),
             ('p', '888', '/menu/getMenuAuthority',                  'POST',   '', '', ''),
             ('p', '888', '/menu/addMenuAuthority',                  'POST',   '', '', ''),
             ('p', '888', '/menu/getBaseMenuTree',                   'POST',   '', '', ''),
             ('p', '888', '/menu/getBaseMenuById',                   'POST',   '', '', ''),
             ('p', '888', '/api/createApi',                          'POST',   '', '', ''),
             ('p', '888', '/api/deleteApi',                          'POST',   '', '', ''),
             ('p', '888', '/api/updateApi',                          'POST',   '', '', ''),
             ('p', '888', '/api/getApiList',                         'POST',   '', '', ''),
             ('p', '888', '/api/getAllApis',                          'POST',   '', '', ''),
             ('p', '888', '/api/deleteApisByIds',                    'DELETE', '', '', ''),
             ('p', '888', '/api/getApiById',                         'POST',   '', '', ''),
             ('p', '888', '/api/getApiGroups',                       'GET',    '', '', ''),
             ('p', '888', '/casbin/UpdateCasbin',                    'POST',   '', '', ''),
             ('p', '888', '/casbin/getPolicyPathByAuthorityId',      'POST',   '', '', '')"
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("DELETE FROM sys_authority_menus").await?;
        db.execute_unprepared("DELETE FROM casbin_rules").await?;
        db.execute_unprepared("DELETE FROM sys_apis").await?;
        db.execute_unprepared("DELETE FROM sys_base_menus").await?;
        db.execute_unprepared("DELETE FROM sys_authorities").await?;
        Ok(())
    }
}
