use std::path::Path;
use std::time::Duration;

use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use tokio::sync::mpsc;
use tokio::time::sleep;
use tracing::{info, warn, error};

use crate::config::{AppConfig, SystemConfig};
use crate::global::AppState;
use super::config::{get_config_path, load_config};

/// 防抖延迟（毫秒）——文件变更后等待一段时间再重载，避免编辑器保存时多次触发
const DEBOUNCE_MS: u64 = 500;

/// 启动配置文件监听器，使用 notify crate 监听 config.yaml 文件变化并热重载安全的配置项
///
/// 对应 Gin-Vue-Admin 中 viper.WatchConfig() 的功能
/// 使用 notify crate 的 RecommendedWatcher 监听文件系统事件
/// 内置 500ms 防抖，避免编辑器保存时的多次触发
/// 不可热重载的配置（数据库/Redis/端口/日志）变更时只记录日志提示重启
pub fn start_config_watcher(state: AppState) {
    // 确定配置文件路径（使用统一的 get_config_path 函数）
    let config_path = get_config_path();
    let path = Path::new(&config_path).to_path_buf();

    // 检查文件是否存在
    if !path.exists() {
        warn!("⚠️  配置文件 {} 不存在，跳过热重载监听", config_path);
        return;
    }

    // 获取要监听的目录（notify 监听目录比监听单文件更可靠）
    // 注意：当 config_path 是纯文件名（如 "config.yaml"）时，parent() 返回 Some("") 而非 None，
    // 所以需要额外检查路径是否为空字符串
    let watch_dir = path.parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."))
        .to_path_buf();
    let file_name = path.file_name().map(|n| n.to_os_string());

    info!("👀 启动配置文件监听: {}（使用 notify 文件系统事件 + {}ms 防抖）", config_path, DEBOUNCE_MS);
    info!("📋 以下配置项变更时将自动热重载：jwt, captcha, cors, email, system（部分字段）");
    info!("📋 以下配置项变更需要重启服务才能生效：");
    for reason in AppConfig::SKIP_REASONS.iter().chain(SystemConfig::SKIP_REASONS.iter()) {
        info!("   ⏭️  {}", reason);
    }

    // 创建 tokio channel 用于在 notify 回调和异步任务之间通信
    let (tx, mut rx) = mpsc::channel::<()>(16);

    // 启动 notify watcher（在 tokio spawn 内部，确保 watcher 生命周期与任务绑定）
    tokio::spawn(async move {
        // 创建 notify watcher，回调中通过 channel 发送通知
        let tx_clone = tx.clone();
        let file_name_clone = file_name.clone();
        let mut watcher = match RecommendedWatcher::new(
            move |result: Result<Event, notify::Error>| {
                match result {
                    Ok(event) => {
                        // 只关注写入/创建/修改事件
                        match event.kind {
                            EventKind::Modify(_) | EventKind::Create(_) => {
                                // 如果有文件名过滤，检查事件是否与目标文件相关
                                let is_target = file_name_clone.as_ref().map_or(true, |name| {
                                    event.paths.iter().any(|p| {
                                        p.file_name().map_or(false, |n| n == name)
                                    })
                                });
                                if is_target {
                                    let _ = tx_clone.try_send(());
                                }
                            }
                            _ => {}
                        }
                    }
                    Err(e) => {
                        error!("❌ 文件监听错误: {}", e);
                    }
                }
            },
            Config::default(),
        ) {
            Ok(w) => w,
            Err(e) => {
                error!("❌ 无法创建文件监听器: {}，配置热重载功能不可用", e);
                return;
            }
        };

        // 监听配置文件所在目录
        if let Err(e) = watcher.watch(&watch_dir, RecursiveMode::NonRecursive) {
            error!("❌ 无法监听目录 {}: {}，配置热重载功能不可用", watch_dir.display(), e);
            return;
        }

        info!("✅ 文件监听器已启动，正在监听 {} 目录", watch_dir.display());

        // 事件处理循环（带防抖）
        loop {
            // 等待第一个事件
            if rx.recv().await.is_none() {
                warn!("⚠️  文件监听通道已关闭，停止配置热重载");
                break;
            }

            // 防抖：等待一段时间，消耗掉这段时间内的所有后续事件
            sleep(Duration::from_millis(DEBOUNCE_MS)).await;
            while rx.try_recv().is_ok() {
                // 消耗掉防抖窗口内的所有事件
            }

            info!("🔄 检测到配置文件变更，正在重新加载...");

            // 重新解析配置文件
            let new_config = match load_config() {
                Ok(cfg) => cfg,
                Err(e) => {
                    error!("❌ 配置文件解析失败: {}，保持原有配置不变", e);
                    continue;
                }
            };

            // 获取当前配置做对比
            let old_config = state.get_config();

            // 检测并记录不可热重载的配置变更
            old_config.log_skipped_changes(&new_config);

            // 构建合并后的配置：安全字段用新值，不安全字段保留旧值
            let merged = old_config.merge_from_new(&new_config);

            // 检查合并后是否有实际变化
            if *old_config == merged {
                info!("ℹ️  配置文件已变更但可热重载的配置项无变化，跳过更新");
                continue;
            }

            // 更新全局配置
            state.set_config(merged).await;
            info!("✅ 配置热重载完成");
        }

        // 注意: watcher 在这里被 drop，监听自动停止
        // 这行代码确保 watcher 不会被提前 drop
        drop(watcher);
    });
}


