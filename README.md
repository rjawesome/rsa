# Notes
- The releases for all OSes are avaliable in the releases page (each OS has its own release)

# Usage (End-to-end encrypted messaging using RSA/AES):
```
rsa_tool srv <tcp port> # server
rsa_tool cli <server ip> <tcp port> # client
```

# Usage (RSA):
```
rsa_tool gen <pubkey filename> <privkey filename> # generate keys
rsa_tool enc <pubkey filename> <plaintext/input filename> <ciphertext/output filename> # encode text
rsa_tool dec <privkey filename> <ciphertext/input filename> <plaintext/output filename> # decode text
```