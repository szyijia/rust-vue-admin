use base64::{engine::general_purpose, Engine};
use captcha::{
    filters::{Dots, Noise, Wave},
    Captcha,
};

/// 验证码生成结果
pub struct CaptchaResult {
    /// 验证码 ID（存入 Redis 的 key）
    pub captcha_id: String,
    /// 验证码图片（base64 编码的 PNG）
    pub pic_path: String,
    /// 验证码答案（存入 Redis，不返回给前端）
    pub answer: String,
}

/// 纯数字字符集（captcha crate 默认字体不包含 '0'，故排除）
const DIGITS: &[char] = &['1', '2', '3', '4', '5', '6', '7', '8', '9'];

/// 生成纯数字图形验证码
pub fn generate_captcha(width: u32, height: u32) -> anyhow::Result<CaptchaResult> {
    // 生成 4 位纯数字验证码
    // 注意：captcha 库内部画布为 400x300，add_chars 后需要先 view 裁剪到目标尺寸，
    // 再应用 Wave/Dots 等滤镜，否则 Wave 会将字符推出 text_area 导致 view 裁剪时字符被截断。
    let mut c = Captcha::new();
    c.set_chars(DIGITS)
        .add_chars(4)
        .view(width, height)
        .apply_filter(Noise::new(0.2))
        .apply_filter(Wave::new(1.0, 10.0).horizontal())
        .apply_filter(Dots::new(3));

    // 获取答案
    let answer = c.chars_as_string();

    // 转为 base64 PNG
    let img_bytes = c
        .as_png()
        .ok_or_else(|| anyhow::anyhow!("验证码图片生成失败"))?;

    let pic_path = format!(
        "data:image/png;base64,{}",
        general_purpose::STANDARD.encode(&img_bytes)
    );

    // 生成唯一 ID
    let captcha_id = uuid::Uuid::new_v4().to_string();

    Ok(CaptchaResult {
        captcha_id,
        pic_path,
        answer,
    })
}

/// 验证码 Redis key 前缀
pub fn captcha_key(captcha_id: &str) -> String {
    format!("captcha:{}", captcha_id)
}
