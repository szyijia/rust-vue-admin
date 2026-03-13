use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20240101_000001_create_tables"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // ========== 1. sys_authorities ==========
        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS `sys_authorities` (
                `created_at` datetime(3) DEFAULT NULL,
                `updated_at` datetime(3) DEFAULT NULL,
                `deleted_at` datetime(3) DEFAULT NULL,
                `authority_id` bigint(20) unsigned NOT NULL AUTO_INCREMENT COMMENT '角色ID',
                `authority_name` varchar(191) DEFAULT NULL COMMENT '角色名',
                `parent_id` bigint(20) unsigned DEFAULT NULL COMMENT '父角色ID',
                `default_router` varchar(191) DEFAULT 'dashboard' COMMENT '默认菜单',
                PRIMARY KEY (`authority_id`),
                UNIQUE KEY `uni_sys_authorities_authority_id` (`authority_id`)
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4"
        ).await?;

        // ========== 2. sys_users ==========
        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS `sys_users` (
                `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
                `created_at` datetime(3) DEFAULT NULL,
                `updated_at` datetime(3) DEFAULT NULL,
                `deleted_at` datetime(3) DEFAULT NULL,
                `uuid` varchar(191) DEFAULT NULL COMMENT '用户UUID',
                `username` varchar(191) DEFAULT NULL COMMENT '用户登录名',
                `password` varchar(191) DEFAULT NULL COMMENT '用户登录密码',
                `nick_name` varchar(191) DEFAULT '系统用户' COMMENT '用户昵称',
                `header_img` varchar(191) DEFAULT 'https://qmplusimg.henrongyi.top/gva_header.jpg' COMMENT '用户头像',
                `authority_id` bigint(20) unsigned DEFAULT '888' COMMENT '用户角色ID',
                `phone` varchar(191) DEFAULT NULL COMMENT '用户手机号',
                `email` varchar(191) DEFAULT NULL COMMENT '用户邮箱',
                `enable` bigint(20) DEFAULT '1' COMMENT '用户是否被冻结 1正常 2冻结',
                `origin_setting` text COMMENT '配置',
                PRIMARY KEY (`id`),
                KEY `idx_sys_users_deleted_at` (`deleted_at`),
                KEY `idx_sys_users_uuid` (`uuid`),
                KEY `idx_sys_users_username` (`username`)
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4"
        ).await?;

        // ========== 3. jwt_blacklists ==========
        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS `jwt_blacklists` (
                `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
                `created_at` datetime(3) DEFAULT NULL,
                `updated_at` datetime(3) DEFAULT NULL,
                `deleted_at` datetime(3) DEFAULT NULL,
                `jwt` text COMMENT 'jwt',
                PRIMARY KEY (`id`),
                KEY `idx_jwt_blacklists_deleted_at` (`deleted_at`)
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4"
        ).await?;

        // ========== 4. sys_base_menus ==========
        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS `sys_base_menus` (
                `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
                `created_at` datetime(3) DEFAULT NULL,
                `updated_at` datetime(3) DEFAULT NULL,
                `deleted_at` datetime(3) DEFAULT NULL,
                `menu_level` bigint(20) unsigned DEFAULT NULL,
                `parent_id` bigint(20) unsigned DEFAULT NULL COMMENT '父菜单ID',
                `path` varchar(191) DEFAULT NULL COMMENT '路由path',
                `name` varchar(191) DEFAULT NULL COMMENT '路由name',
                `hidden` tinyint(1) DEFAULT NULL COMMENT '是否在列表隐藏',
                `component` varchar(191) DEFAULT NULL COMMENT '对应前端文件路径',
                `sort` bigint(20) DEFAULT NULL COMMENT '排序标记',
                `active_name` varchar(191) DEFAULT NULL COMMENT '高亮菜单',
                `keep_alive` tinyint(1) DEFAULT NULL COMMENT '是否缓存',
                `default_menu` tinyint(1) DEFAULT NULL COMMENT '是否是基础路由（开发中）',
                `title` varchar(191) DEFAULT NULL COMMENT '菜单名',
                `icon` varchar(191) DEFAULT NULL COMMENT '菜单图标',
                `close_tab` tinyint(1) DEFAULT NULL COMMENT '自动关闭tab',
                `transition_type` varchar(191) DEFAULT NULL COMMENT '路由切换动画',
                PRIMARY KEY (`id`),
                KEY `idx_sys_base_menus_deleted_at` (`deleted_at`)
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4"
        ).await?;

        // ========== 5. sys_authority_menus ==========
        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS `sys_authority_menus` (
                `sys_base_menu_id` bigint(20) unsigned NOT NULL,
                `sys_authority_authority_id` bigint(20) unsigned NOT NULL COMMENT '角色ID',
                PRIMARY KEY (`sys_base_menu_id`,`sys_authority_authority_id`)
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4"
        ).await?;

        // ========== 6. sys_apis ==========
        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS `sys_apis` (
                `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
                `created_at` datetime(3) DEFAULT NULL,
                `updated_at` datetime(3) DEFAULT NULL,
                `deleted_at` datetime(3) DEFAULT NULL,
                `path` varchar(191) DEFAULT NULL COMMENT 'api路径',
                `description` varchar(191) DEFAULT NULL COMMENT 'api中文描述',
                `api_group` varchar(191) DEFAULT NULL COMMENT 'api组',
                `method` varchar(191) DEFAULT 'POST' COMMENT '方法',
                PRIMARY KEY (`id`),
                KEY `idx_sys_apis_deleted_at` (`deleted_at`)
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4"
        ).await?;

        // ========== 7. casbin_rule（gin-vue-admin 原生 casbin 适配器表） ==========
        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS `casbin_rule` (
                `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
                `ptype` varchar(100) DEFAULT NULL,
                `v0` varchar(100) DEFAULT NULL,
                `v1` varchar(100) DEFAULT NULL,
                `v2` varchar(100) DEFAULT NULL,
                `v3` varchar(100) DEFAULT NULL,
                `v4` varchar(100) DEFAULT NULL,
                `v5` varchar(100) DEFAULT NULL,
                PRIMARY KEY (`id`),
                UNIQUE KEY `idx_casbin_rule` (`ptype`,`v0`,`v1`,`v2`,`v3`,`v4`,`v5`)
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4"
        ).await?;

        // ========== 8. casbin_rules（rust casbin 适配器表） ==========
        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS `casbin_rules` (
                `id` int(11) NOT NULL AUTO_INCREMENT,
                `ptype` varchar(100) NOT NULL DEFAULT '',
                `v0` varchar(100) NOT NULL DEFAULT '',
                `v1` varchar(100) NOT NULL DEFAULT '',
                `v2` varchar(100) NOT NULL DEFAULT '',
                `v3` varchar(100) NOT NULL DEFAULT '',
                `v4` varchar(100) NOT NULL DEFAULT '',
                `v5` varchar(100) NOT NULL DEFAULT '',
                PRIMARY KEY (`id`)
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4"
        ).await?;

        // ========== 9. sys_user_authority ==========
        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS `sys_user_authority` (
                `sys_user_id` bigint(20) unsigned NOT NULL,
                `sys_authority_authority_id` bigint(20) unsigned NOT NULL COMMENT '角色ID',
                PRIMARY KEY (`sys_user_id`,`sys_authority_authority_id`)
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4"
        ).await?;

        // ========== 10. sys_data_authority_id ==========
        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS `sys_data_authority_id` (
                `sys_authority_authority_id` bigint(20) unsigned NOT NULL COMMENT '角色ID',
                `data_authority_id_authority_id` bigint(20) unsigned NOT NULL COMMENT '角色ID',
                PRIMARY KEY (`sys_authority_authority_id`,`data_authority_id_authority_id`)
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4"
        ).await?;

        // ========== 11. sys_dictionaries ==========
        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS `sys_dictionaries` (
                `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
                `created_at` datetime(3) DEFAULT NULL,
                `updated_at` datetime(3) DEFAULT NULL,
                `deleted_at` datetime(3) DEFAULT NULL,
                `name` varchar(191) DEFAULT NULL COMMENT '字典名（中）',
                `type` varchar(191) DEFAULT NULL COMMENT '字典名（英）',
                `status` tinyint(1) DEFAULT NULL COMMENT '状态',
                `desc` varchar(191) DEFAULT NULL COMMENT '描述',
                `parent_id` bigint(20) unsigned DEFAULT NULL COMMENT '父级字典ID',
                PRIMARY KEY (`id`),
                KEY `idx_sys_dictionaries_deleted_at` (`deleted_at`)
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4"
        ).await?;

        // ========== 12. sys_dictionary_details ==========
        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS `sys_dictionary_details` (
                `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
                `created_at` datetime(3) DEFAULT NULL,
                `updated_at` datetime(3) DEFAULT NULL,
                `deleted_at` datetime(3) DEFAULT NULL,
                `label` varchar(191) DEFAULT NULL COMMENT '展示值',
                `value` varchar(191) DEFAULT NULL COMMENT '字典值',
                `extend` varchar(191) DEFAULT NULL COMMENT '扩展值',
                `status` tinyint(1) DEFAULT NULL COMMENT '启用状态',
                `sort` bigint(20) DEFAULT NULL COMMENT '排序标记',
                `sys_dictionary_id` bigint(20) unsigned DEFAULT NULL COMMENT '关联标记',
                `parent_id` bigint(20) unsigned DEFAULT NULL COMMENT '父级字典详情ID',
                `level` bigint(20) DEFAULT NULL COMMENT '层级深度',
                `path` varchar(191) DEFAULT NULL COMMENT '层级路径',
                PRIMARY KEY (`id`),
                KEY `idx_sys_dictionary_details_deleted_at` (`deleted_at`)
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4"
        ).await?;

        // ========== 13. sys_operation_records ==========
        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS `sys_operation_records` (
                `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
                `created_at` datetime(3) DEFAULT NULL,
                `updated_at` datetime(3) DEFAULT NULL,
                `deleted_at` datetime(3) DEFAULT NULL,
                `ip` varchar(191) DEFAULT NULL COMMENT '请求ip',
                `method` varchar(191) DEFAULT NULL COMMENT '请求方法',
                `path` varchar(191) DEFAULT NULL COMMENT '请求路径',
                `status` bigint(20) DEFAULT NULL COMMENT '请求状态',
                `latency` bigint(20) DEFAULT NULL COMMENT '延迟',
                `agent` text COMMENT '代理',
                `error_message` varchar(191) DEFAULT NULL COMMENT '错误信息',
                `body` text COMMENT '请求Body',
                `resp` text COMMENT '响应Body',
                `user_id` bigint(20) unsigned DEFAULT NULL COMMENT '用户id',
                PRIMARY KEY (`id`),
                KEY `idx_sys_operation_records_deleted_at` (`deleted_at`)
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4"
        ).await?;

        // ========== 14. sys_params ==========
        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS `sys_params` (
                `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
                `created_at` datetime(3) DEFAULT NULL,
                `updated_at` datetime(3) DEFAULT NULL,
                `deleted_at` datetime(3) DEFAULT NULL,
                `name` varchar(191) DEFAULT NULL COMMENT '参数名称',
                `key` varchar(191) DEFAULT NULL COMMENT '参数键',
                `value` varchar(191) DEFAULT NULL COMMENT '参数值',
                `desc` varchar(191) DEFAULT NULL COMMENT '参数说明',
                PRIMARY KEY (`id`),
                KEY `idx_sys_params_deleted_at` (`deleted_at`)
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4"
        ).await?;

        // ========== 15. sys_base_menu_btns ==========
        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS `sys_base_menu_btns` (
                `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
                `created_at` datetime(3) DEFAULT NULL,
                `updated_at` datetime(3) DEFAULT NULL,
                `deleted_at` datetime(3) DEFAULT NULL,
                `name` varchar(191) DEFAULT NULL COMMENT '按钮关键key',
                `desc` varchar(191) DEFAULT NULL,
                `sys_base_menu_id` bigint(20) unsigned DEFAULT NULL COMMENT '菜单ID',
                PRIMARY KEY (`id`),
                KEY `idx_sys_base_menu_btns_deleted_at` (`deleted_at`)
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4"
        ).await?;

        // ========== 16. sys_authority_btns ==========
        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS `sys_authority_btns` (
                `authority_id` bigint(20) unsigned DEFAULT NULL COMMENT '角色ID',
                `sys_menu_id` bigint(20) unsigned DEFAULT NULL COMMENT '菜单ID',
                `sys_base_menu_btn_id` bigint(20) unsigned DEFAULT NULL COMMENT '菜单按钮ID'
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4"
        ).await?;

        // ========== 17. sys_base_menu_parameters ==========
        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS `sys_base_menu_parameters` (
                `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
                `created_at` datetime(3) DEFAULT NULL,
                `updated_at` datetime(3) DEFAULT NULL,
                `deleted_at` datetime(3) DEFAULT NULL,
                `sys_base_menu_id` bigint(20) unsigned DEFAULT NULL,
                `type` varchar(191) DEFAULT NULL COMMENT '地址栏携带参数为params还是query',
                `key` varchar(191) DEFAULT NULL COMMENT '地址栏携带参数的key',
                `value` varchar(191) DEFAULT NULL COMMENT '地址栏携带参数的值',
                PRIMARY KEY (`id`),
                KEY `idx_sys_base_menu_parameters_deleted_at` (`deleted_at`)
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4"
        ).await?;

        // ========== 18. sys_ignore_apis ==========
        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS `sys_ignore_apis` (
                `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
                `created_at` datetime(3) DEFAULT NULL,
                `updated_at` datetime(3) DEFAULT NULL,
                `deleted_at` datetime(3) DEFAULT NULL,
                `path` varchar(191) DEFAULT NULL COMMENT 'api路径',
                `method` varchar(191) DEFAULT 'POST' COMMENT '方法',
                PRIMARY KEY (`id`),
                KEY `idx_sys_ignore_apis_deleted_at` (`deleted_at`)
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4"
        ).await?;

        // ========== 19. sys_export_templates ==========
        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS `sys_export_templates` (
                `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
                `created_at` datetime(3) DEFAULT NULL,
                `updated_at` datetime(3) DEFAULT NULL,
                `deleted_at` datetime(3) DEFAULT NULL,
                `db_name` varchar(191) DEFAULT NULL COMMENT '数据库名称',
                `name` varchar(191) DEFAULT NULL COMMENT '模板名称',
                `table_name` varchar(191) DEFAULT NULL COMMENT '表名称',
                `template_id` varchar(191) DEFAULT NULL COMMENT '模板标识',
                `template_info` text,
                `sql` text COMMENT '自定义导出SQL',
                `import_sql` text COMMENT '自定义导入SQL',
                `limit` bigint(20) DEFAULT NULL COMMENT '导出限制',
                `order` varchar(191) DEFAULT NULL COMMENT '排序',
                PRIMARY KEY (`id`),
                KEY `idx_sys_export_templates_deleted_at` (`deleted_at`)
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4"
        ).await?;

        // ========== 20. sys_export_template_condition ==========
        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS `sys_export_template_condition` (
                `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
                `created_at` datetime(3) DEFAULT NULL,
                `updated_at` datetime(3) DEFAULT NULL,
                `deleted_at` datetime(3) DEFAULT NULL,
                `template_id` varchar(191) DEFAULT NULL COMMENT '模板标识',
                `from` varchar(191) DEFAULT NULL COMMENT '条件取的key',
                `column` varchar(191) DEFAULT NULL COMMENT '作为查询条件的字段',
                `operator` varchar(191) DEFAULT NULL COMMENT '操作符',
                PRIMARY KEY (`id`),
                KEY `idx_sys_export_template_condition_deleted_at` (`deleted_at`)
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4"
        ).await?;

        // ========== 21. sys_export_template_join ==========
        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS `sys_export_template_join` (
                `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
                `created_at` datetime(3) DEFAULT NULL,
                `updated_at` datetime(3) DEFAULT NULL,
                `deleted_at` datetime(3) DEFAULT NULL,
                `template_id` varchar(191) DEFAULT NULL COMMENT '模板标识',
                `joins` varchar(191) DEFAULT NULL COMMENT '关联',
                `table` varchar(191) DEFAULT NULL COMMENT '关联表',
                `on` varchar(191) DEFAULT NULL COMMENT '关联条件',
                PRIMARY KEY (`id`),
                KEY `idx_sys_export_template_join_deleted_at` (`deleted_at`)
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4"
        ).await?;

        // ========== 22. exa_file_upload_and_downloads ==========
        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS `exa_file_upload_and_downloads` (
                `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
                `created_at` datetime(3) DEFAULT NULL,
                `updated_at` datetime(3) DEFAULT NULL,
                `deleted_at` datetime(3) DEFAULT NULL,
                `name` varchar(191) DEFAULT NULL COMMENT '文件名',
                `class_id` bigint(20) DEFAULT '0' COMMENT '分类id',
                `url` varchar(191) DEFAULT NULL COMMENT '文件地址',
                `tag` varchar(191) DEFAULT NULL COMMENT '文件标签',
                `key` varchar(191) DEFAULT NULL COMMENT '编号',
                PRIMARY KEY (`id`),
                KEY `idx_exa_file_upload_and_downloads_deleted_at` (`deleted_at`)
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4"
        ).await?;

        // ========== 23. exa_customers ==========
        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS `exa_customers` (
                `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
                `created_at` datetime(3) DEFAULT NULL,
                `updated_at` datetime(3) DEFAULT NULL,
                `deleted_at` datetime(3) DEFAULT NULL,
                `customer_name` varchar(191) DEFAULT NULL COMMENT '客户名',
                `customer_phone_data` varchar(191) DEFAULT NULL COMMENT '客户手机号',
                `sys_user_id` bigint(20) unsigned DEFAULT NULL COMMENT '管理ID',
                `sys_user_authority_id` bigint(20) unsigned DEFAULT NULL COMMENT '管理角色ID',
                PRIMARY KEY (`id`),
                KEY `idx_exa_customers_deleted_at` (`deleted_at`)
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4"
        ).await?;

        // ========== 24. exa_files ==========
        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS `exa_files` (
                `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
                `created_at` datetime(3) DEFAULT NULL,
                `updated_at` datetime(3) DEFAULT NULL,
                `deleted_at` datetime(3) DEFAULT NULL,
                `file_name` varchar(191) DEFAULT NULL,
                `file_md5` varchar(191) DEFAULT NULL,
                `file_path` varchar(191) DEFAULT NULL,
                `chunk_total` bigint(20) DEFAULT NULL,
                `is_finish` tinyint(1) DEFAULT NULL,
                PRIMARY KEY (`id`),
                KEY `idx_exa_files_deleted_at` (`deleted_at`)
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4"
        ).await?;

        // ========== 25. exa_file_chunks ==========
        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS `exa_file_chunks` (
                `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
                `created_at` datetime(3) DEFAULT NULL,
                `updated_at` datetime(3) DEFAULT NULL,
                `deleted_at` datetime(3) DEFAULT NULL,
                `exa_file_id` bigint(20) unsigned DEFAULT NULL,
                `file_chunk_number` bigint(20) DEFAULT NULL,
                `file_chunk_path` varchar(191) DEFAULT NULL,
                PRIMARY KEY (`id`),
                KEY `idx_exa_file_chunks_deleted_at` (`deleted_at`)
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4"
        ).await?;

        // ========== 26. exa_attachment_category ==========
        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS `exa_attachment_category` (
                `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
                `created_at` datetime(3) DEFAULT NULL,
                `updated_at` datetime(3) DEFAULT NULL,
                `deleted_at` datetime(3) DEFAULT NULL,
                `name` varchar(255) DEFAULT NULL COMMENT '分类名称',
                `pid` bigint(20) DEFAULT '0' COMMENT '父节点ID',
                PRIMARY KEY (`id`),
                KEY `idx_exa_attachment_category_deleted_at` (`deleted_at`)
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4"
        ).await?;

        // ========== 27. sys_auto_code_histories ==========
        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS `sys_auto_code_histories` (
                `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
                `created_at` datetime(3) DEFAULT NULL,
                `updated_at` datetime(3) DEFAULT NULL,
                `deleted_at` datetime(3) DEFAULT NULL,
                `table_name` varchar(191) DEFAULT NULL COMMENT '表名',
                `package` varchar(191) DEFAULT NULL COMMENT '模块名/插件名',
                `request` text COMMENT '前端传入的结构化信息',
                `struct_name` varchar(191) DEFAULT NULL COMMENT '结构体名称',
                `abbreviation` varchar(191) DEFAULT NULL COMMENT '结构体名称缩写',
                `business_db` varchar(191) DEFAULT NULL COMMENT '业务库',
                `description` varchar(191) DEFAULT NULL COMMENT 'Struct中文名称',
                `templates` text COMMENT '模板信息',
                `Injections` text COMMENT '注入路径',
                `flag` bigint(20) DEFAULT NULL COMMENT '[0:创建,1:回滚]',
                `api_ids` varchar(191) DEFAULT NULL COMMENT 'api表注册内容',
                `menu_id` bigint(20) unsigned DEFAULT NULL COMMENT '菜单ID',
                `export_template_id` bigint(20) unsigned DEFAULT NULL COMMENT '导出模板ID',
                `package_id` bigint(20) unsigned DEFAULT NULL COMMENT '包ID',
                PRIMARY KEY (`id`),
                KEY `idx_sys_auto_code_histories_deleted_at` (`deleted_at`)
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4"
        ).await?;

        // ========== 28. sys_auto_code_packages ==========
        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS `sys_auto_code_packages` (
                `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
                `created_at` datetime(3) DEFAULT NULL,
                `updated_at` datetime(3) DEFAULT NULL,
                `deleted_at` datetime(3) DEFAULT NULL,
                `desc` varchar(191) DEFAULT NULL COMMENT '描述',
                `label` varchar(191) DEFAULT NULL COMMENT '展示名',
                `template` varchar(191) DEFAULT NULL COMMENT '模版',
                `package_name` varchar(191) DEFAULT NULL COMMENT '包名',
                `module` varchar(191) DEFAULT NULL,
                PRIMARY KEY (`id`),
                KEY `idx_sys_auto_code_packages_deleted_at` (`deleted_at`)
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4"
        ).await?;

        // ========== 29. sys_login_logs ==========
        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS `sys_login_logs` (
                `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
                `created_at` datetime(3) DEFAULT NULL,
                `updated_at` datetime(3) DEFAULT NULL,
                `deleted_at` datetime(3) DEFAULT NULL,
                `username` varchar(191) DEFAULT NULL COMMENT '用户名',
                `ip` varchar(191) DEFAULT NULL COMMENT '请求ip',
                `status` tinyint(1) DEFAULT NULL COMMENT '登录状态',
                `error_message` varchar(191) DEFAULT NULL COMMENT '错误信息',
                `agent` varchar(191) DEFAULT NULL COMMENT '代理',
                `user_id` bigint(20) unsigned DEFAULT NULL COMMENT '用户id',
                PRIMARY KEY (`id`),
                KEY `idx_sys_login_logs_deleted_at` (`deleted_at`)
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4"
        ).await?;

        // ========== 30. sys_api_tokens ==========
        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS `sys_api_tokens` (
                `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
                `created_at` datetime(3) DEFAULT NULL,
                `updated_at` datetime(3) DEFAULT NULL,
                `deleted_at` datetime(3) DEFAULT NULL,
                `user_id` bigint(20) unsigned DEFAULT NULL COMMENT '用户ID',
                `authority_id` bigint(20) unsigned DEFAULT NULL COMMENT '角色ID',
                `token` text COMMENT 'Token',
                `status` tinyint(1) DEFAULT '1' COMMENT '状态',
                `expires_at` datetime(3) DEFAULT NULL COMMENT '过期时间',
                `remark` varchar(191) DEFAULT NULL COMMENT '备注',
                PRIMARY KEY (`id`),
                KEY `idx_sys_api_tokens_deleted_at` (`deleted_at`)
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4"
        ).await?;

        // ========== 31. sys_error ==========
        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS `sys_error` (
                `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
                `created_at` datetime(3) DEFAULT NULL,
                `updated_at` datetime(3) DEFAULT NULL,
                `deleted_at` datetime(3) DEFAULT NULL,
                `form` text COMMENT '错误来源',
                `info` text COMMENT '错误内容',
                `level` varchar(191) DEFAULT NULL COMMENT '日志等级',
                `solution` text COMMENT '解决方案',
                `status` varchar(20) DEFAULT '未处理' COMMENT '处理状态',
                PRIMARY KEY (`id`),
                KEY `idx_sys_error_deleted_at` (`deleted_at`)
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4"
        ).await?;

        // ========== 32. sys_versions ==========
        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS `sys_versions` (
                `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
                `created_at` datetime(3) DEFAULT NULL,
                `updated_at` datetime(3) DEFAULT NULL,
                `deleted_at` datetime(3) DEFAULT NULL,
                `version_name` varchar(255) DEFAULT NULL COMMENT '版本名称',
                `version_code` varchar(100) DEFAULT NULL COMMENT '版本号',
                `description` varchar(500) DEFAULT NULL COMMENT '版本描述',
                `version_data` text COMMENT '版本数据JSON',
                PRIMARY KEY (`id`),
                KEY `idx_sys_versions_deleted_at` (`deleted_at`)
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4"
        ).await?;

        // ========== 33. gva_announcements_info ==========
        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS `gva_announcements_info` (
                `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
                `created_at` datetime(3) DEFAULT NULL,
                `updated_at` datetime(3) DEFAULT NULL,
                `deleted_at` datetime(3) DEFAULT NULL,
                `title` varchar(191) DEFAULT NULL COMMENT '公告标题',
                `content` text COMMENT '公告内容',
                `user_id` bigint(20) DEFAULT NULL COMMENT '发布者',
                `attachments` json DEFAULT NULL COMMENT '相关附件',
                PRIMARY KEY (`id`),
                KEY `idx_gva_announcements_info_deleted_at` (`deleted_at`)
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4"
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        // 按依赖逆序删除所有表
        let tables = vec![
            "gva_announcements_info",
            "sys_versions",
            "sys_error",
            "sys_api_tokens",
            "sys_login_logs",
            "sys_auto_code_packages",
            "sys_auto_code_histories",
            "exa_attachment_category",
            "exa_file_chunks",
            "exa_files",
            "exa_customers",
            "exa_file_upload_and_downloads",
            "sys_export_template_join",
            "sys_export_template_condition",
            "sys_export_templates",
            "sys_ignore_apis",
            "sys_base_menu_parameters",
            "sys_authority_btns",
            "sys_base_menu_btns",
            "sys_params",
            "sys_operation_records",
            "sys_dictionary_details",
            "sys_dictionaries",
            "sys_data_authority_id",
            "sys_user_authority",
            "casbin_rules",
            "casbin_rule",
            "sys_apis",
            "sys_authority_menus",
            "sys_base_menus",
            "jwt_blacklists",
            "sys_users",
            "sys_authorities",
        ];
        for table in tables {
            db.execute_unprepared(&format!("DROP TABLE IF EXISTS `{}`", table)).await?;
        }
        Ok(())
    }
}

// ========== 表名枚举定义 ==========

#[derive(Iden)]
enum SysAuthorities {
    Table,
    AuthorityId,
    AuthorityName,
    ParentId,
    DefaultRouter,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

#[derive(Iden)]
enum SysUsers {
    Table,
    Id,
    Uuid,
    Username,
    Password,
    NickName,
    HeaderImg,
    Phone,
    Email,
    Enable,
    AuthorityId,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

#[derive(Iden)]
enum JwtBlacklists {
    Table,
    Id,
    Jwt,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum SysBaseMenus {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
    ParentId,
    Path,
    Name,
    Hidden,
    Component,
    Sort,
    KeepAlive,
    DefaultMenu,
    Title,
    Icon,
    CloseTab,
}

#[derive(Iden)]
enum SysAuthorityMenus {
    Table,
    SysBaseMenuId,
    SysAuthorityAuthorityId,
}

#[derive(Iden)]
enum SysApis {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
    Path,
    Description,
    ApiGroup,
    Method,
}

#[derive(Iden)]
enum CasbinRules {
    Table,
    Id,
    Ptype,
    V0,
    V1,
    V2,
    V3,
    V4,
    V5,
}
