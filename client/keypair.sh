#!/bin/bash

export GNUPGHOME="$(mktemp -d)"
cat >foo <<EOF
     Key-Type: RSA
     Key-Length: 2048
     Subkey-Type: ELG-E
     Subkey-Length: 1024
     Name-Real: Testing Key
     Name-Comment: Not to be used for real keys.
     Name-Email: test@radical-yadql.io
     Expire-Date: 0
     Passphrase: 
     %commit
EOF
gpg --batch --generate-key foo
gpg --list-secret-keys
