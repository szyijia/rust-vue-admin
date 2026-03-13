// 系统配置服务 - 对应 Gin-Vue-Admin server/service/system/sys_system.go
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sysinfo::{CpuExt, DiskExt, SystemExt};

/// 系统服务器信息（对应 Go utils.Server）
#[derive(Debug, Serialize, Deserialize)]
pub struct ServerInfo {
    pub os: OsInfo,
    pub cpu: CpuInfo,
    pub ram: RamInfo,
    pub disk: Vec<DiskInfo>,
}

/// 操作系统信息（对应 Go utils.Os）
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsInfo {
    pub goos: String,
    pub num_cpu: usize,
    pub compiler: String,
    pub go_version: String,
    pub num_goroutine: usize,
}

/// CPU 信息（对应 Go utils.Cpu）
#[derive(Debug, Serialize, Deserialize)]
pub struct CpuInfo {
    pub cpus: Vec<f64>,
    pub cores: usize,
}

/// 内存信息（对应 Go utils.Ram）
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RamInfo {
    pub used_mb: u64,
    pub total_mb: u64,
    pub used_percent: u64,
}

/// 磁盘信息（对应 Go utils.Disk）
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiskInfo {
    pub mount_point: String,
    pub used_mb: u64,
    pub used_gb: u64,
    pub total_mb: u64,
    pub total_gb: u64,
    pub used_percent: u64,
}

const MB: u64 = 1024 * 1024;
const GB: u64 = 1024 * 1024 * 1024;

/// 获取服务器信息（对应 Go GetServerInfo）
pub fn get_server_info() -> Result<ServerInfo> {
    let mut sys = sysinfo::System::new_all();
    sys.refresh_all();
    // 等待一小段时间让 CPU 使用率更准确
    std::thread::sleep(std::time::Duration::from_millis(200));
    sys.refresh_cpu();

    // OS 信息（Rust 版本替代 Go 版本信息）
    let os_info = OsInfo {
        goos: std::env::consts::OS.to_string(),
        num_cpu: num_cpus::get(),
        compiler: "rustc".to_string(),
        go_version: format!("Rust {}", env!("CARGO_PKG_VERSION")),
        num_goroutine: tokio::runtime::Handle::try_current()
            .map(|_| 0usize) // tokio 无法直接获取 task 数
            .unwrap_or(0),
    };

    // CPU 信息
    let cpus: Vec<f64> = sys.cpus().iter().map(|c| c.cpu_usage() as f64).collect();
    let cores = sys.physical_core_count().unwrap_or(0);
    let cpu_info = CpuInfo { cpus, cores };

    // 内存信息
    let total_mem = sys.total_memory();
    let used_mem = sys.used_memory();
    let ram_info = RamInfo {
        total_mb: total_mem / MB,
        used_mb: used_mem / MB,
        used_percent: if total_mem > 0 {
            used_mem * 100 / total_mem
        } else {
            0
        },
    };

    // 磁盘信息
    let mut disk_infos = Vec::new();
    for disk in sys.disks() {
        let total = disk.total_space();
        let available = disk.available_space();
        let used = total.saturating_sub(available);
        disk_infos.push(DiskInfo {
            mount_point: disk.mount_point().to_string_lossy().to_string(),
            total_mb: total / MB,
            total_gb: total / GB,
            used_mb: used / MB,
            used_gb: used / GB,
            used_percent: if total > 0 { used * 100 / total } else { 0 },
        });
    }

    Ok(ServerInfo {
        os: os_info,
        cpu: cpu_info,
        ram: ram_info,
        disk: disk_infos,
    })
}
