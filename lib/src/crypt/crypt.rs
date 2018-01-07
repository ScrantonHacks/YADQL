use gpgme;
use gpgme::{Context, Protocol, SignatureSummary, SigningResult, EncryptionResult, DecryptionResult, VerificationResult};

/// # mod crypt
/// Allows specialized access to RSA functions.
fn sign(clear: &str, key: &str) {
    //! ## sign(clear: str, key: str) -> str
    //! Takes plaintext and our public key, and signs the plaintext.
    //! Returns the produced signature.
    let proto = Protocol::OpenPgp;
    let mode = gpgme::SignMode::Clear;
    let mut ctx = Context::from_protocol(proto).unwrap();
    ctx.set_armor(true);
    let key = ctx.find_secret_key(key).unwrap();
    ctx.add_signer(&key).unwrap();
    let mut output = Vec::new();
    ctx.sign(mode, clear, &mut output).expect("signing failed")
}

fn verify(sig: &str) {
    //! ## verify(sig: str) -> str
    //! Takes a signature string and verifies it against our known public keys.
    //! Returns the verified status.
    let proto = Protocol::OpenPgp;
    let mut ctx = Context::from_protocol(proto).unwrap();
    ctx.verify_opaque(sig, &mut Vec::new())
}

fn encrypt(clear: &str, us: &str) {
    //! ## encrypt(clear: str, us: str) -> str
    //! Takes a plaintext string and our private key fingerprint, then encrypts the plaintext with our public key.
    //! Returns the encrypted string.
    let proto = Protocol::OpenPgp;
    let mut ctx = Context::from_protocol(proto).unwrap();
    ctx.set_armor(true);
    let mut recipients = Vec::new();
    recipients.push(us);
    let keys = if !recipients.is_empty() {
        ctx.find_keys(recipients)
            .unwrap()
            .filter_map(Result::ok)
            .filter(|k| k.can_encrypt())
            .collect()
    } else {
        Vec::new()
    };
    let mut output = Vec::new();
    ctx.encrypt(&keys, clear, &mut output).expect("encrypting failed")
}

fn decrypt(cipher: &str) {
    //! ## decrypt(cipher: str) -> str
    //! Takes cipher text and decrypts it using our private key.
    //! Returns the decrypted text.
    let proto = Protocol::OpenPgp;
    let mut ctx = Context::from_protocol(proto).unwrap();
    let mut output = Vec::new();
    ctx.decrypt(cipher, &mut output).expect("decrypting failed")
}
