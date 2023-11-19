use std::path::PathBuf;

use aes::cipher::KeyInit;
use aes::Aes128;
use ecb::cipher::block_padding::ZeroPadding;
use ecb::cipher::BlockEncryptMut;
use ecb::Encryptor;
use rand::rngs::OsRng;
use rand::RngCore;
use rsa::pkcs8::DecodePublicKey;
use rsa::RsaPublicKey;
use sha2::Sha256;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, BufReader};

#[derive(Debug, Clone, Copy)]
pub enum TinfoilEncryption {
    NoEncrypt = 0x00,
    Encrypt = 0xF0,
}

impl TinfoilEncryption {
    pub async fn encrypt(
        &self,
        data: Vec<u8>,
        file: &PathBuf,
    ) -> anyhow::Result<(Vec<u8>, Vec<u8>)> {
        // Basic Idea, Generate random aes key, encrypt data with aes key via ecb, encrypt aes key with pubkey, send encrypted data & encrypted eas key

        let mut random_aes_key = [0u8; 16];

        OsRng.fill_bytes(&mut random_aes_key);

        let pub_key_file = File::open(file).await?;

        let mut buf_reader = BufReader::new(pub_key_file);
        let mut pub_key_str = String::new();

        buf_reader.read_to_string(&mut pub_key_str).await?;

        let public_key = RsaPublicKey::from_public_key_pem(pub_key_str.as_str())?;

        let ecb = Encryptor::<Aes128>::new(random_aes_key.as_ref().into());

        let encrypted_data = ecb.encrypt_padded_vec_mut::<ZeroPadding>(&data);

        let mut rng = OsRng;

        let encrypted_aes_key = public_key.encrypt(
            &mut rng,
            rsa::Oaep::new::<Sha256>(),
            random_aes_key.as_ref(),
        )?;

        Ok((encrypted_data, encrypted_aes_key))
    }
}
