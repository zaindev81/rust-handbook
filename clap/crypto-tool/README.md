# crypto-tool

```sh
# Encrypt with AES and Base64 encoding
cargo run -- encrypt -i data.txt -o encrypted.dat --password mypass --base64

# Decrypt the file
cargo run -- decrypt -i encrypted.dat -o decrypted.txt --password mypass --base64

# Encrypt with DES to stdout
cargo run -- encrypt -i data.txt --method des --password mypass

# Use RSA encryption (for small files)
cargo run -- encrypt -i small.txt --method rsa --password mypass
```