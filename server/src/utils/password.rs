use anyhow::Result;

/// 对密码进行 bcrypt 哈希，对应 Gin-Vue-Admin 的 utils.BcryptHash()
pub fn hash_password(password: &str) -> Result<String> {
    let hashed = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;
    Ok(hashed)
}

/// 验证密码是否匹配，对应 Gin-Vue-Admin 的 utils.BcryptCheck()
pub fn verify_password(password: &str, hash: &str) -> bool {
    bcrypt::verify(password, hash).unwrap_or(false)
}

/// 密码强度校验：至少8位，包含大小写字母和数字
pub fn validate_password_strength(password: &str) -> bool {
    if password.len() < 8 {
        return false;
    }
    let has_upper = password.chars().any(|c| c.is_uppercase());
    let has_lower = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    has_upper && has_lower && has_digit
}
