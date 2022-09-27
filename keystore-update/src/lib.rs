use aes_gcm::{
    aead::{Aead, KeyInit, OsRng, Payload},
    AeadCore, Aes256Gcm,
};
use chrono::prelude::*;
use rand::{self, RngCore};

type Key = String;

#[derive(Debug)]
pub struct Keystore {
    pk: Key,
    nonce: String,
    aad: String,
}

impl Keystore {
    pub fn create() -> (Key, Keystore) {
        let key = Aes256Gcm::generate_key(&mut OsRng);

        let cipher = Aes256Gcm::new_from_slice(&key).unwrap();

        let mut rnd = vec![0u8; 32];
        rand::thread_rng().fill_bytes(&mut rnd);

        let data_add = Utc::now().to_string();

        let payload = Payload {
            msg: &rnd,
            aad: data_add.as_bytes(),
        };

        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

        let ciphertext = cipher.encrypt(&nonce, payload).unwrap();

        (
            hex::encode(key),
            Keystore {
                pk: hex::encode(ciphertext),
                nonce: hex::encode(nonce),
                aad: data_add,
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_encrypt_and_decrypt_secret() {
        /* test here */
    }
}
