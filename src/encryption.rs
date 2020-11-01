use aes::Aes128;
use block_modes::block_padding::ZeroPadding;
use block_modes::{BlockMode, Ecb};
use rand::rngs::OsRng;
use rand::RngCore;
use rsa::{pem, PaddingScheme, PublicKey, RSAPublicKey};
use std::convert::TryFrom;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

#[derive(Debug, Clone)]
pub enum EncryptionFlag {
    NoEncrypt = 0x00,
    Encrypt = 0xF0,
}

impl EncryptionFlag {
    pub fn encrypt(&self, data: Vec<u8>, file: &Path) -> crate::result::Result<(Vec<u8>, Vec<u8>)> {
        // Basic Idea, Generate random aes key, encrypt data with aes key via ecb, encrypt aes key with pubkey, send encrypted data & encrypted eas key

        let mut random_aes_key = [0u8; 16];

        OsRng.fill_bytes(&mut random_aes_key);

        let pub_key_file = File::open(file)?;
        let mut buf_reader = BufReader::new(pub_key_file);
        let mut pub_key_str = String::new();

        buf_reader.read_to_string(&mut pub_key_str)?;

        let public_key = RSAPublicKey::try_from(pem::parse(pub_key_str)?)?;

        let iv = Default::default();

        let ecb = Ecb::<Aes128, ZeroPadding>::new_var(random_aes_key.as_ref(), iv)?;

        let encrypted_data = ecb.encrypt_vec(data.as_slice());

        let mut rng = OsRng;

        let encrypted_aes_key = public_key.encrypt(
            &mut rng,
            PaddingScheme::new_oaep::<sha2::Sha256>(),
            random_aes_key.as_ref(),
        )?;

        Ok((encrypted_data, encrypted_aes_key))
    }
}
