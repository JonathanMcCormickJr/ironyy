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
    totp_secret: Option<String>,
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
        let password_number = 0;
        let password_hash = Self::hash(uuid, password_number, password)?;

        Ok(Self {
            username,
            uuid,
            password_hash,
            password_number,
            totp_secret: None,
        })
    }

    pub fn enable_2fa(&mut self) -> Result<(), anyhow::Error> {
        let raw_secret = generate_random_secret()?;
        let issuer = Some(String::from(APP_NAME));
        let account_name = self.username.clone();

        let my_qr_code = EasyTotp::create_qr_png(raw_secret.clone(), issuer, account_name)
            .map_err(|e| anyhow::anyhow!("{}", e))?;
        self.totp_secret = Some(raw_secret);
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
}

fn generate_random_secret() -> Result<String, anyhow::Error> {
    let mut bytes = [0u8; 32]; // 32 bytes = 256 bits
    OsRng.try_fill_bytes(&mut bytes)?;
    Ok(BASE64_STANDARD_NO_PAD.encode(&bytes))
}
