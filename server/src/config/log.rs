use serde::Deserialize;

/// 日志配置，对应 config.yaml log 节（参考 Gin-Vue-Admin zap 配置）
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct LogConfig {
    /// 日志级别: trace/debug/info/warn/error
    pub level: String,
    /// 输出格式: console/json
    pub format: String,
    /// 日志前缀
    pub prefix: String,
    /// 日志文件目录
    pub director: String,
    /// 是否显示代码行号
    pub show_line: bool,
    /// 是否同时输出到控制台
    pub log_in_console: bool,
    /// 日志保留天数，-1 表示永久
    pub retention_day: i32,
}
