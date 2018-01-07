use gpgme;
use gpgme::{Context, Protocol, SignatureSummary};

/// # mod crypt
/// Allows specialized access to RSA functions.
fn sign(clear: &str, key: &str) -> SigningResult {
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
    ctx.sign(mode, &mut clear, &mut output).expect("signing failed")
}

fn verify(sig: &str) -> VerificationResult {
    //! ## verify(sig: str) -> str
    //! Takes a signature string and verifies it against our known public keys.
    //! Returns the verified status.
    let proto = Protocol::OpenPgp;
    let mut ctx = Context::from_protocol(proto).unwrap();
    ctx.set_armor(true);
    ctx.verify_opaque(sig, &mut Vec::new()).unwrap()
}

fn encrypt(clear: &str, us: &str) -> EncryptionResult {
    //! ## encrypt(clear: str, us: str) -> str
    //! Takes a plaintext string and our private key fingerprint, then encrypts the plaintext with our public key.
    //! Returns the encrypted string.
    let proto = Protocol::OpenPgp;
    let mut ctx = Context::from_protocol(proto).unwrap();
    ctx.set_armor(true);
    let keys = ctx.find_keys(us).unwrap().filter_map(Result::ok).filter(|k| k.can_encrypt()).collect();
    let mut output = Vec::new();
    ctx.encrypt(&keys, &mut clear, &mut output).expect("encrypting failed")
}

fn decrypt(cipher: &str) -> DecryptionResult {
    //! ## decrypt(cipher: str) -> str
    //! Takes cipher text and decrypts it using our private key.
    //! Returns the decrypted text.
    let proto = Protocol::OpenPgp;
    let mut ctx = Context::from_protocol(proto).unwrap();
    let mut output = Vec::new();
    ctx.decrypt(&mut cipher, &mut output).expect("decrypting failed")
}
