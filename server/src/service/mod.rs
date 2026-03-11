// 业务服务模块，后续阶段实现
// 对应 Gin-Vue-Admin server/service/

pub mod system;
pub mod user;

pub use user::UserService;
