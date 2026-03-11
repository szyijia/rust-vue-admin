use std::net::SocketAddr;

use axum::Router;
use tokio::net::TcpListener;
use tracing::info;

use crate::config::SystemConfig;

/// 启动 HTTP 服务器，对应 Gin-Vue-Admin 的 core.RunServer()
pub async fn run_server(router: Router, cfg: &SystemConfig) -> anyhow::Result<()> {
    let addr: SocketAddr = format!("0.0.0.0:{}", cfg.addr)
        .parse()
        .expect("无效的服务器地址");

    let listener = TcpListener::bind(addr).await?;

    info!(
        addr = %addr,
        env = %cfg.env,
        "🚀 rust-vue-admin 服务器启动成功"
    );
    info!("📖 API 文档: http://{}:{}/swagger-ui", "127.0.0.1", cfg.addr);
    info!("❤️  健康检查: http://{}:{}/health", "127.0.0.1", cfg.addr);

    axum::serve(listener, router.into_make_service_with_connect_info::<SocketAddr>())
        .await?;

    Ok(())
}
