use sha256::digest;

pub fn hash_password(password: impl Into<String>, salt: impl Into<String>) -> String {
    let salt = salt.into();
    digest(password.into() + salt.as_str())
}
