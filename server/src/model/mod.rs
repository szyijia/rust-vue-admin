// 数据模型模块，后续阶段实现
// 对应 Gin-Vue-Admin server/model/
pub mod common;
pub mod system;

#[allow(unused_imports)]
pub use system::{JwtBlacklist, SysRole, SysUser};
