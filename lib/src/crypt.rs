extern crate gpgme;

use gpgme::{Context, Protocol, SignatureSummary};

mod crypt {
    fn sign(clear: str) -> str {
        let proto = Protocol:OpenPgp;

        let mode = gpgme::SignMode::Clear;

        let mut ctx = Context::from_protocol(proto).unwrap();
        ctx.set_armor(true);

        if let Some(key) = matches.opt_str("key") {
            if proto != Protocol::UiServer {
                let key = ctx.find_secret_key(key).unwrap();
                ctx.add_signer(&key).unwrap();
            } else {
                eprintln!("ignoring --key in UI-server mode");
            }
        }

        let mut input = File::open(&matches.free[0]).unwrap();
        let mut output = Vec::new();
        ctx.sign(mode, &mut input, &mut output).expect("signing failed")
    }

    fn verify(sig: str) -> str {
        let proto = Protocol::OpenPgp;
        let mut ctx = Context::from_protocol(proto).unwrap();
        ctx.set_armor(true);
        ctx.verify_opaque(signature, &mut Vec::new())
    }

    fn encrypt(clear: str) -> str {
        let proto = Protocol::OpenPgp;

        let mut ctx = Context::from_protocol(proto).unwrap();
        ctx.set_armor(true);

        // TODO This should be changed to encrypt for ourselves.
        let recipients = matches.opt_strs("r");
        let keys = if !recipients.is_empty() {
            ctx.find_keys(recipients)
                .unwrap()
                .filter_map(Result::ok)
                .filter(|k| k.can_encrypt())
                .collect()
        } else {
            Vec::new()
        };

        let mut input = File::open(&matches.free[0]).unwrap();
        let mut output = Vec::new();
        ctx.encrypt(&keys, &mut input, &mut output).expect("encrypting failed")
    }

    fn decrypt(cypher: str) -> str {
        let proto = Protocol::OpenPgp;
        let mut ctx = Context::from_protocol(proto).unwrap();
        let mut input = File::open(&matches.free[0]).unwrap();
        let mut output = Vec::new();
        ctx.decrypt(&mut input, &mut output).expect("decrypting failed")
    }
}
