use std::str;
use gpgme;
use gpgme::{Context, Protocol, SigningResult, EncryptionResult, DecryptionResult, VerificationResult, Error};

/// # mod crypt
/// Allows specialized access to RSA functions.
pub fn sign(clear: &str, key: &str) -> String {
    //! ## sign(clear: &str, key: &str) -> str
    //! Takes plaintext and our public key, and signs the plaintext.
    //! Returns the produced signature.
    let proto = Protocol::OpenPgp;
    let mode = gpgme::SignMode::Clear;
    let mut ctx = Context::from_protocol(proto).unwrap();
    ctx.set_armor(true);
    let key = ctx.find_secret_key(key).unwrap();
    ctx.add_signer(&key).unwrap();
    let mut output = Vec::new();
    ctx.sign(mode, clear, &mut output).expect("signing failed");
    String::from_utf8(output).unwrap()
}

pub fn verify(sig: String) -> String {
    //! ## verify(sig: &str) -> str
    //! Takes a signature string and verifies it against our known public keys.
    //! Returns the verified status.
    let proto = Protocol::OpenPgp;
    let mut ctx = Context::from_protocol(proto).unwrap();
    let mut output = Vec::new();
    ctx.verify_opaque(sig, &mut output);
    String::from_utf8(output).unwrap()
}

pub fn encrypt(clear: &str, us: &str) -> String {
    //! ## encrypt(clear: &str, us: &str) -> str
    //! Takes a plaintext string and our private key fingerprint, then encrypts the plaintext with our public key.
    //! Returns the encrypted string.
    let proto = Protocol::OpenPgp;
    let mut ctx = Context::from_protocol(proto).unwrap();
    ctx.set_armor(true);
    let mut recipients = Vec::new();
    recipients.push(us);
    let keys: Vec<gpgme::Key> = ctx.find_keys(recipients)
                  .unwrap()
                  .filter_map(Result::ok)
                  .filter(|k| k.can_encrypt())
                  .collect();
    let mut output = Vec::new();
    ctx.encrypt(&keys, clear, &mut output).expect("encrypting failed");
    String::from_utf8(output).unwrap()
}

pub fn decrypt(cipher: &str) -> String {
    //! ## decrypt(cipher: &str) -> str
    //! Takes cipher text and decrypts it using our private key.
    //! Returns the decrypted text.
    let proto = Protocol::OpenPgp;
    let mut ctx = Context::from_protocol(proto).unwrap();
    let mut output = Vec::new();
    ctx.decrypt(cipher, &mut output).expect("decrypting failed");
    String::from_utf8(output).unwrap()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn sign_and_verify() {
        let x = sign("Test.", "test@radical-yadql.io");
        verify(x);
    }

    #[test]
    fn encrypt_and_decrypt() {
        let x = "Test.";
        let y = encrypt(x, "test@radical-yadql.io");
        let z = decrypt(&y);
        assert_eq!(x, z);
    }
}
