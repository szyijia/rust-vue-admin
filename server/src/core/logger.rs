use std::fs;
use std::path::Path;

use tracing_appender::rolling;
use tracing_subscriber::{
    fmt::{self, time::ChronoLocal},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};

use crate::config::LogConfig;

/// 初始化日志系统，对应 Gin-Vue-Admin 的 core.Zap()
///
/// 支持：
/// - 控制台彩色输出
/// - 按天滚动写入文件（JSON 格式）
/// - 通过 RUST_LOG 环境变量覆盖日志级别
pub fn init_logger(cfg: &LogConfig) {
    // 确保日志目录存在
    if !cfg.director.is_empty() {
        let log_dir = Path::new(&cfg.director);
        if !log_dir.exists() {
            fs::create_dir_all(log_dir).expect("创建日志目录失败");
        }
    }

    // 构建 EnvFilter（优先使用 RUST_LOG 环境变量）
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(cfg.level.clone()));

    let show_line = cfg.show_line;
    let log_in_console = cfg.log_in_console;
    let director = cfg.director.clone();

    // 控制台 layer（可选）
    let console_layer = if log_in_console || director.is_empty() {
        Some(
            fmt::layer()
                .with_target(show_line)
                .with_line_number(show_line)
                .with_timer(ChronoLocal::new("%Y-%m-%d %H:%M:%S%.3f".to_string()))
                .with_ansi(true),
        )
    } else {
        None
    };

    // 文件 layer（可选）
    let file_layer = if !director.is_empty() {
        let file_appender = rolling::daily(&director, "rust-vue-admin.log");
        let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
        // 将 guard 泄漏以保持文件写入器在进程生命周期内存活
        std::mem::forget(guard);

        Some(
            fmt::layer()
                .with_target(true)
                .with_line_number(true)
                .with_timer(ChronoLocal::new("%Y-%m-%d %H:%M:%S%.3f".to_string()))
                .with_ansi(false)
                .json()
                .with_writer(non_blocking),
        )
    } else {
        None
    };

    // 注册所有 layer（Option<Layer> 实现了 Layer trait，None 时为 no-op）
    tracing_subscriber::registry()
        .with(env_filter)
        .with(console_layer)
        .with(file_layer)
        .init();

    tracing::info!(
        prefix = %cfg.prefix,
        level = %cfg.level,
        "日志系统初始化完成"
    );
}
