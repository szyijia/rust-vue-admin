use serde::Deserialize;

/// 验证码配置，对应 config.yaml captcha 节
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct CaptchaConfig {
    /// 验证码长度
    pub key_long: u32,
    /// 图片宽度
    pub img_width: u32,
    /// 图片高度
    pub img_height: u32,
    /// 0=始终开启，>0=限制次数后开启
    pub open_captcha: u32,
    /// 验证码超时时间（秒）
    pub open_captcha_timeout: u32,
}
