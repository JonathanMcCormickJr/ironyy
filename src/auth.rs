use std::any;

use argon2::{Algorithm, Argon2, Params, PasswordHasher, Version, password_hash};
use base64::{Engine, prelude::BASE64_STANDARD_NO_PAD};
use easy_totp::EasyTotp;
use rand::{TryRngCore, rngs::OsRng};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::constants::APP_NAME;

pub struct User {
    username: String,
    uuid: Uuid,
    password_hash: String,
    password_number: u32,
    totp: Option<EasyTotp>,
}

impl User {
    // Hashing-related stuff
    const HASH_ALGO: Algorithm = Algorithm::Argon2id;
    const HASH_VERSION: Version = Version::V0x13;

    #[cfg(test)]
    const HASH_MEMORY_KIB: u32 = 1945;
    #[cfg(test)]
    const HASH_ITERATIONS: u32 = 1;

    #[cfg(not(test))]
    const HASH_MEMORY_KIB: u32 = 19_456;
    #[cfg(not(test))]
    const HASH_ITERATIONS: u32 = 8;

    fn hash_params() -> Params {
        Params::new(
            Self::HASH_MEMORY_KIB, // Memory size, expressed in kibibytes.
            Self::HASH_ITERATIONS, // Number of iterations/passes.
            1u32,                  // Degree of parallelism.
            Some(32usize),         // Length of the output (in bytes).
        )
        .expect("Failed to generate app-wide User hashing params properly")
    }
    fn hash(uuid: Uuid, pass_num: u32, password: String) -> Result<String, anyhow::Error> {
        let salt_result = password_hash::SaltString::from_b64(
            &BASE64_STANDARD_NO_PAD.encode((format!("{uuid}{pass_num}")).as_bytes()),
        );
        let salt;
        match salt_result {
            Ok(s) => salt = s,
            Err(e) => {
                return Err(anyhow::anyhow!(
                    "Failed to generate salt for password hashing: {}",
                    e
                ))
            }
        }

        let hash  = Argon2::new(Self::HASH_ALGO, Self::HASH_VERSION, Self::hash_params())
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow::anyhow!("Failed to hash password: {}", e))?;

        Ok(hash.to_string())
    }

    pub fn new(username: String, password: String) -> Result<Self, anyhow::Error> {
        let uuid = Uuid::new_v4();
        if let Err(e) = is_password_compliant(&password) {
            return Err(e);
        }

        let password_number = 0;
        let password_hash = Self::hash(uuid, password_number, password)?;

        Ok(Self {
            username,
            uuid,
            password_hash,
            password_number,
            totp: None,
        })
    }

    pub fn enable_2fa(&mut self) -> Result<(), anyhow::Error> {
        let issuer = Some(String::from(APP_NAME));
        let account_name = self.username.clone();

        let et = EasyTotp::new(issuer, account_name)?;
        let my_qr_code = EasyTotp::create_qr_png(et.clone())
            .map_err(|e| anyhow::anyhow!("{}", e))?;
        self.totp = Some(et);
        println!(
            "Scan this QR code with your authenticator app:\n{:#?}",
            my_qr_code
        );
        Ok(())
    }

    pub fn verify_password(&self, password_attempt: String) -> Result<bool, anyhow::Error> {
        let reference_hash = self.password_hash.clone();
        let attempt_hash = Self::hash(self.uuid, self.password_number, password_attempt)?;
        Ok(reference_hash == attempt_hash)
    }

    pub fn verify_totp(&self, code: &str) -> Result<bool, anyhow::Error> {
        match &self.totp {
            Some(secret) => {
                let reference_code = EasyTotp::generate_token(secret.clone())
                    .map_err(|e| anyhow::anyhow!("{}", e))?;
                Ok(reference_code == code)
            }
            None => Err(anyhow::anyhow!("2FA is not enabled for this user.")),
        }
    }

    pub fn change_password(&mut self, new_password: String) -> Result<(), anyhow::Error> {
        if let Err(e) = is_password_compliant(&new_password) {
            return Err(e);
        }
        self.password_number += 1;
        self.password_hash = Self::hash(self.uuid, self.password_number, new_password)?;
        Ok(())
    }

    pub fn disable_2fa(&mut self) {
        self.totp = None;
    }
    
}

fn is_password_compliant(password: &str) -> Result<(), anyhow::Error> {
    let is_acceptable_len = password.len() >= 16 && password.len() <= 128;
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_digit(10));
    let has_special = password.chars().any(|c| !c.is_alphanumeric());
    let compliance_summary = (
        is_acceptable_len,
        has_uppercase,
        has_lowercase,
        has_digit,
        has_special,
    );
    match compliance_summary {
        (true, true, true, true, true) => Ok(()),
        (false, _, _, _, _) => {
            return Err(anyhow::anyhow!(
                "Password must be between 16 and 128 characters long."
            ))
        }
        (_, false, _, _, _) => {
            return Err(anyhow::anyhow!(
                "Password must contain at least one uppercase letter."
            ))
        }
        (_, _, false, _, _) => {
            return Err(anyhow::anyhow!(
                "Password must contain at least one lowercase letter."
            ))
        }
        (_, _, _, false, _) => {
            return Err(anyhow::anyhow!(
                "Password must contain at least one digit."
            ))
        }
        (_, _, _, _, false) => {
            return Err(anyhow::anyhow!(
                "Password must contain at least one special character."
            ))
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing_and_verification() {
        let username = String::from("testuser");
        let password = String::from("StrongPassword!123");

        let user = User::new(username, password.clone()).expect("Failed to create user");

        assert!(user.verify_password(password).unwrap());
        assert!(!user.verify_password(String::from("WrongPassword")).unwrap());
    }

    #[test]
    fn test_totp_enable_and_verify() {
        let username = String::from("testuser2");
        let password = String::from("AnotherStrongPass!456");

        let mut user = User::new(username, password).expect("Failed to create user");
        user.enable_2fa().expect("Failed to enable 2FA");

        let secret = user.totp.clone().expect("TOTP secret should be set");
        let code = EasyTotp::generate_token(secret)
            .expect("Failed to generate TOTP code");

        assert!(user.verify_totp(&code).unwrap());
        assert!(!user.verify_totp("000000").unwrap());
    }

    #[test]
    fn test_password_compliance() {
        let compliant_password = "ValidPass3045713y5t31ght!";
        let short_password = "Short1!";
        let no_uppercase = "nouppercase123!";
        let no_lowercase = "NOLOWERCASE123!";
        let no_digit = "NoDigitPassword!";
        let no_special = "NoSpecialChar1234";
        let too_long_password = "A".repeat(129) + "1a!";
        assert!(is_password_compliant(compliant_password).is_ok());
        assert!(is_password_compliant(short_password).is_err());
        assert!(is_password_compliant(no_uppercase).is_err());
        assert!(is_password_compliant(no_lowercase).is_err());
        assert!(is_password_compliant(no_digit).is_err());
        assert!(is_password_compliant(no_special).is_err());
        assert!(is_password_compliant(&too_long_password).is_err());
    }

    #[test]
    fn test_change_password() {
        let username = String::from("testuser3");
        let old_password = String::from("OldStrongPass!789");
        let new_password = String::from("NewStrongPass!012");
        let mut user = User::new(username, old_password.clone()).expect("Failed to create user");
        assert!(user.verify_password(old_password).unwrap());
        user.change_password(new_password.clone()).expect("Failed to change password");
        assert!(user.verify_password(new_password).unwrap());
    }

    #[test]
    fn test_disable_2fa() {
        let username = String::from("testuser4");
        let password = String::from("Disable2FAPass!345");
        let mut user = User::new(username, password).expect("Failed to create user");
        user.enable_2fa().expect("Failed to enable 2FA");
        assert!(user.totp.is_some());
        user.disable_2fa();
        assert!(user.totp.is_none());

    }

}