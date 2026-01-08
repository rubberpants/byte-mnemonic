# byte-mnemonic (Rust)

Like the NATO alphabet but for binary data. Each byte gets a unique four-letter word.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
byte-mnemonic = "0.1"
```

## Usage

```rust
use byte_mnemonic::{encode, decode, encode_hex, decode_to_hex, byte_to_word, word_to_byte};

// Encode bytes to words
let words = encode(&[0x48, 0x65, 0x6C, 0x6C, 0x6F]);
assert_eq!(words, "FILM EXIT LOAF LOAF ONLY");

// Decode words back to bytes
let bytes = decode("FILM EXIT LOAF LOAF ONLY").unwrap();
assert_eq!(bytes, vec![0x48, 0x65, 0x6C, 0x6C, 0x6F]);

// Encode from hex string
let words = encode_hex("48656C6C6F").unwrap();
assert_eq!(words, "FILM EXIT LOAF LOAF ONLY");

// Decode to hex string
let hex = decode_to_hex("FILM EXIT LOAF LOAF ONLY").unwrap();
assert_eq!(hex, "48656c6c6f");

// Single byte/word conversion
assert_eq!(byte_to_word(0x00), "ABLE");
assert_eq!(word_to_byte("ZOOM"), Some(255));
```

## API

### `encode(bytes: &[u8]) -> String`

Encode a byte slice to space-separated mnemonic words.

### `decode(mnemonic: &str) -> Result<Vec<u8>, DecodeError>`

Decode mnemonic words back to bytes. Case-insensitive.

### `encode_hex(hex: &str) -> Result<String, DecodeError>`

Encode a hex string to mnemonic words. Accepts `0x` prefix.

### `decode_to_hex(mnemonic: &str) -> Result<String, DecodeError>`

Decode mnemonic words to a lowercase hex string.

### `byte_to_word(byte: u8) -> &'static str`

Get the mnemonic word for a single byte.

### `word_to_byte(word: &str) -> Option<u8>`

Get the byte value for a mnemonic word. Case-insensitive, trims whitespace.

### `WORDS: [&str; 256]`

The complete word list (256 words, indexed by byte value).

## Error Handling

The crate uses `DecodeError` for error cases:

```rust
pub enum DecodeError {
    UnknownWord(String),  // Unrecognized mnemonic word
    InvalidHex(String),   // Invalid hexadecimal input
}
```

## License

MIT

