# signet - code signing tool

signet is a command line tool for signing source code changes and
arbitrary files such as build outputs. Signatures are generated in
[SSHSIG][sshsig] format and signing keys are standard SSH keys so
signet is compatible with OpenSSH's `ssh-keygen -Y sign | verify`.

signet stores encrypted signing keys in keychains located in
~/.config/signet on Unix systems including macOS and Linux, and
the user's AppData folder on Windows.

    signet init -s
    signet keys -c
    signet sign -k <id> -n file <FILE>

Configure git to use signet to sign commits and tags:

    git config user.signingkey <id>
    git config gpg.format      ssh
    git config gpg.ssh.program signet

    git config commit.gpgsign  true
    git config tag.gpgsign     true

Signing keys are encrypted with a password supplied by the user and
that password can be stored in the system keyring or requested via
interactive prompt when required.

[sshsig]: https://github.com/openssh/openssh-portable/blob/master/PROTOCOL.sshsig
