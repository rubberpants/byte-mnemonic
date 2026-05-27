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
assert_eq!(words, "FILM HOLY ICON ICON IMPS");

// Decode words back to bytes
let bytes = decode("FILM HOLY ICON ICON IMPS").unwrap();
assert_eq!(bytes, vec![0x48, 0x65, 0x6C, 0x6C, 0x6F]);

// Encode from hex string
let words = encode_hex("48656C6C6F").unwrap();
assert_eq!(words, "FILM HOLY ICON ICON IMPS");

// Decode to hex string
let hex = decode_to_hex("FILM HOLY ICON ICON IMPS").unwrap();
assert_eq!(hex, "48656c6c6f");

// Single byte/word conversion
assert_eq!(byte_to_word(0x00), "ABLE");
assert_eq!(word_to_byte("ZOOM"), Some(255));
```

## API

### `encode(bytes: &[u8]) -> String`

Encode a byte slice to space-separated mnemonic words.

### `encode_compressed(bytes: &[u8]) -> String`

Like `encode`, but collapses runs of **3 or more** identical bytes into the
word followed by a decimal repeat count (e.g. `ABLE 4`). Runs of 1 or 2 are
emitted verbatim. Output is fully interoperable with `decode`.

```rust
assert_eq!(encode_compressed(&[0, 0, 0, 0]), "ABLE 4");
assert_eq!(encode_compressed(&[0, 0]),       "ABLE ABLE");
```

### `decode(mnemonic: &str) -> Result<Vec<u8>, DecodeError>`

Decode mnemonic words back to bytes. Case-insensitive. Also accepts the
run-length form: a digit-only token immediately following a word repeats that
word the given number of times in total (e.g. `ABLE 4` → four `ABLE` bytes).

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
    UnknownWord(String),    // Unrecognized mnemonic word
    InvalidHex(String),     // Invalid hexadecimal input
    InvalidAddress(String), // Invalid IPv4/IPv6 input
    InvalidRepeat(String),  // Malformed run-length count
}
```

## License

MIT

