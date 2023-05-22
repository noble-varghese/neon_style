## Setting up GPG for Commit Signing

1.  Install GPG in your local machine using the following command:

```
$ brew install gnupg
$ gpg --version
```

2.  Generate a GPG key pair using the following command:

```
$ gpg --full-generate-key
```

Follow the prompts to generate the key pair. Choose "RSA and RSA" as the type of key, and "4096" as the key size.

3.  Retrieve the GPG key ID by running the following command:

```
$ gpg --list-secret-keys --keyid-format LONG
```

 The output will contain the key ID, which is the string of characters following "rsa4096/".

4.  Export the GPG public key using the following command, replacing "KEY_ID" with the actual key ID:

```
$ gpg --armor --export KEY_ID
```

Copy the output of this command and paste on the github for verifying your commits.

5.  Configure Git to use the GPG key for signing commits:

```
$ git config --global user.signingkey KEY_ID
$ git config --global commit.gpgsign true
```

Replace "KEY_ID" with the actual key ID.

You can now sign commits with the GPG key by using the `-S` option with the `git commit` command:

```
$ git commit -S -m "commit message"
```

The `-S` option tells Git to sign the commit with the GPG key.
