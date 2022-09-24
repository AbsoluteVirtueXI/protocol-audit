use crypto::aead::AeadEncryptor;
use crypto::aes_gcm::AesGcm;
use k256::ecdsa::SigningKey;
use rustc_serialize::hex::FromHex;
use std::iter;

type Digest = String;
type Key = String;

#[derive(Debug)]
pub struct Keystore {
    digest: Digest,
    sk: Key,
    pk: Key,
}

impl Keystore {
    pub fn new(password: &str) -> Keystore {
        let digest = md5::compute(password);

        let rnd: Vec<u8> = iter::repeat_with(|| fastrand::i8(..))
            .take(32)
            .map(|v| v as u8)
            .collect();
        let signing_key = SigningKey::from_bytes(&rnd);
        let sk = signing_key.unwrap().to_bytes();
        let key = digest.to_ascii_lowercase();
        let data = sk.to_ascii_lowercase();
        let data_add = sk;
        let iv = "000000000000000000000000";

        let key_size = crypto::aes::KeySize::KeySize128;
        let mut aes = AesGcm::new(key_size, &key, &iv.from_hex().unwrap(), &data_add);
        let mut output: Vec<u8> = iter::repeat(0).take(data.len()).collect();
        let mut output_tag: Vec<u8> = iter::repeat(0).take(16).collect();
        aes.encrypt(&data, &mut output[..], &mut output_tag[..]);

        let k = Keystore {
            digest: format!("0x{}", hex::encode(digest.to_vec())),
            sk: format!("0x{}", hex::encode(sk)),
            pk: hex::encode(output),
        };
        k
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_keystore() {
        /* code goes here */
    }
}
