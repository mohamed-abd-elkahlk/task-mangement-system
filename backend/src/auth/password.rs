use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};

pub fn verify_password(password: &str, password_hash: &str) -> bool {
    let password_hash = PasswordHash::new(password_hash).expect("faild to parse password hash");
    Argon2::default()
        .verify_password(password.as_bytes(), &password_hash)
        .is_ok()
}

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default().hash_password(password.as_bytes(), &salt)?;
    Ok(password_hash.to_string())
}
