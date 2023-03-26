# Goal

- Encrypt text and embed it in pdf
- Scan printed pdf and decrypt the text

# Summary

## Library

- encryption/decryption as outlined below
- generation of qrcodes

## Cli

- interface to the library functionality

## Web App

- client side encryption/decryption in browser
- open qrcode payload in app
- MAYBE: scanning qrcodes (why only maybe: see problem below)

## Mobile Apps

- package the webapp with tauri

## Pdf Generation

- write a pandoc filter
- write a word plugin





# Problem

### Easy Part

- Encryption / Decryption (ChatGPT implemented both with very little help)

### Hard Part

#### QR-Code Detection

Only AI-based SDK's work really well:

- Google MLkit
- IOS CIDetector

Using those would make it possible to point the camera at a printed PDF and
scan the QR-Codes. However an already scanned document could not be processed
on Desktop, as both libraries are mobile only.

The most supported open source alternative is ZXing and its ports to from 
java to other languages. Compared to the capability of above mentioned 
SDK's it performs much worse.

##### Possible solution

Leave the QR-Code scanning to the dedicated apps, take the encoded payload
and decrypt it in another.

#### PDF Generation

##### Goal

- Latex
- Markdown
- Word

The first two could be implemented with a pandoc filter (see filter.py, test.md
,test.tex). Word support could probably be achieved by writing a word add-in
(https://learn.microsoft.com/en-us/office/dev/add-ins/tutorials/word-tutorial)


# Crypto

## Idea

The payload is always encrypted with either aes-siv or chacha20poly1305.
The key for these algorithms is generated randomly and then wrapped for one
or more recipients.
A recipient is either asymmetric with x25519 or symmetric with argon2id.

### Symmetric

- aes-siv?
- chacha20poly1305

### Key Hashing

- argon2id

### Asymmetric

- X25519

### Importable keys

- ssh
- ed25519 (convertable to x25519)
- age
- gpg


### Recipients

#### X25519
#### Argon2


## Format

json draft:

{ 
    "version": "cryptlinks/v1",
    "recipients": [
        { 
            "type": "X25519",
            "ephemeral": "base64encodedrandom",
            "key": "base64encodedpayloadkey"
        },
        {
            "type": "argon2id",
            "params": {},
            "salt": "base64encodedrandom",
            "key": "base64encodedpayloadkey"
        }
    ],
    "nonce", "base64",
    "mac", "base64headermac",
    "payload": "base64"
}










































# Possible libraries

## Rust

no video

- https://github.com/mkazutaka/wasm-barcode-reader not updated since 04.12.2020
- https://github.com/rxing-core/rxing much testdata, port from established zxing, works good, api is not amazing
- https://github.com/piderman314/bardecoder slow, fails often
- https://github.com/WanzenBug/rqrr fails often
- https://github.com/jackyef/rqrr-wasm same base as rqrr
- https://github.com/wg/quirc-rs 2017 last update

## Javascript

video

- https://github.com/cozmo/jsQR
- https://github.com/mebjas/html5-qrcode
- https://github.com/nimiq/qr-scanner


# Mobile App

## Tauri
- https://github.com/tauri-apps/tauri-mobile

### Frontent libs
- https://github.com/JonasKruckenberg/tauri-sycamore-template
- https://github.com/yewstack/yew
- https://github.com/silkenweb/silkenweb
- https://github.com/michalvavra/tauri-leptos-example
- https://github.com/emilk/egui

## Dioxus
- https://github.com/DioxusLabs/dioxus
