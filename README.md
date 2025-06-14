# README.md
# crypto_file_toolkit

A simple command-line tool to encrypt and decrypt files using AES-256-GCM encryption with a password-derived key.

## Usage

Build the project with Cargo:

```sh
cargo build --release
```

Run the tool with the following options:

```
target\debug\crypto_file_toolkit.exe [OPTIONS] --file <FILE> --key <KEY> --output <OUTPUT>
```

## Options

- `-e`, `--encrypt`  
  Encrypt the specified file.

- `-d`, `--decrypt`  
  Decrypt the specified file.

- `--file <FILE>`  
  Path to the input file (for encryption: plaintext file, for decryption: encrypted file).

- `--key <KEY>`  
  Password to use for encryption/decryption.

- `-o`, `--output <OUTPUT>`  
  Path to the output file (for encryption: encrypted file, for decryption: decrypted file).

**Note:** You must specify either `--encrypt` or `--decrypt`.

## Examples

### Encrypt a file

```sh
target\debug\crypto_file_toolkit --encrypt --file input.txt --key mypassword --output encrypted.bin
```

or using short options:

```sh
target\debug\crypto_file_toolkit -e --file input.txt --key mypassword -o encrypted.bin
```

### Decrypt a file

```sh
target\debug\crypto_file_toolkit --decrypt --file encrypted.bin --key mypassword --output decrypted.txt
```

or using short options:

```sh
target\debug\crypto_file_toolkit -d --file encrypted.bin --key mypassword -o decrypted.txt
```

## Help

You can display the help message with:

```sh
crypto_file_toolkit --help
```

Example output:

```
crypto_file_toolkit

Usage: crypto_file_toolkit [OPTIONS] --file <FILE> --key <KEY> --output <OUTPUT>

Options:
  -e, --encrypt           Encrypt the specified file
  -d, --decrypt           Decrypt the specified file
      --file <FILE>       Path to the input file
      --key <KEY>         Password to use for encryption/decryption
  -o, --output <OUTPUT>   Path to the output file
  -h, --help              Print help information
```

## License

MIT

---

See [`src/main.rs`](src/main.rs) for the main CLI logic and [`src/crypto.rs`](src/crypto.rs) for encryption/decryption implementation.