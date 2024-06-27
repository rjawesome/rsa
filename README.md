# Notes
- The releases for all OSes are avaliable in the releases page (each OS has its own release)

# Usage (End-to-end encrypted client-to-client [via server] messaging using RSA/AES):
```
rsa_tool newsrv <tcp port> # external server
rsa_tool newcli <server ip> <tcp port> <your username> <other username> # when connection is established, keys can be verified if server is sus
# Example of a valid client connection
# external server runs: rsa_tool newsrv 3000
# alice runs: rsa_tool newcli localhost 3000 alice bob
# bob runs: rsa_tool newcli localhost 3000 bob alice
```

# Usage (End-to-end encrypted server-to-client messaging using RSA/AES):
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