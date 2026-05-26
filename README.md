# Byte Mnemonic

Like the NATO phonetic alphabet, but for binary data. Each byte (0–255) maps to a unique four-letter word, enabling humans to read, speak, and transcribe binary data with minimal errors. Useful for writing and communicating long encryption keys or hashes.

## Web Converter

**[Try it live →](https://cofergus.github.io/byte-mnemonic/)** — bidirectional converter, works entirely in your browser with no backend.

To enable for your own fork: go to **Settings → Pages → Source** and set it to deploy from the `main` branch, root folder.

## Why?

Binary data is hard for humans to work with. Hexadecimal helps, but `4A 7F B2` is still easy to mishear or mistype. The NATO alphabet solved this for letters—"Alpha Bravo Charlie" is unambiguous over radio. Byte Mnemonic does the same for bytes.

**Example:** The bytes `[0x00, 0x48, 0x65, 0x6C, 0x6C, 0x6F]` become:

```
ABLE DOCK EXIT LOAF LOAF ONLY
```

## Word Selection Criteria

The 256 words were selected to:

- Be exactly four letters long
- Be common English words (easy to remember)
- Have distinct pronunciations (minimize phonetic collisions)
- Start with diverse consonants and vowels
- Avoid homophones and similar-sounding pairs
- Avoid silent letters

## The Word List

| Byte | Word | Byte | Word | Byte | Word | Byte | Word |
|------|------|------|------|------|------|------|------|
| 0x00 | ABLE | 0x40 | EXIT | 0x80 | FUNK | 0xC0 | RUSH |
| 0x01 | ACID | 0x41 | EYES | 0x81 | FUZZ | 0xC1 | SAGS |
| 0x02 | AERO | 0x42 | FALL | 0x82 | GAME | 0xC2 | SALT |
| 0x03 | AGED | 0x43 | FARM | 0x83 | GARB | 0xC3 | SCAB |
| 0x04 | ALSO | 0x44 | FAST | 0x84 | GAZE | 0xC4 | SEAT |
| 0x05 | APEX | 0x45 | FAWN | 0x85 | GERM | 0xC5 | SEER |
| 0x06 | ARCH | 0x46 | FEET | 0x86 | GIFT | 0xC6 | SHIP |
| 0x07 | AREA | 0x47 | FEUD | 0x87 | GIVE | 0xC7 | SLUM |
| 0x08 | ARMS | 0x48 | FILM | 0x88 | GLAD | 0xC8 | SNOW |
| 0x09 | ATOM | 0x49 | FISH | 0x89 | GOOD | 0xC9 | SOAP |
| 0x0A | AUTO | 0x4A | FLEX | 0x8A | GRAB | 0xCA | SODA |
| 0x0B | AWAY | 0x4B | FOLK | 0x8B | GROW | 0xCB | SOME |
| 0x0C | AXIS | 0x4C | FOUL | 0x8C | HALO | 0xCC | SPIT |
| 0x0D | BABY | 0x4D | FOXY | 0x8D | HANK | 0xCD | STAR |
| 0x0E | BACK | 0x4E | FROG | 0x8E | HARD | 0xCE | STOP |
| 0x0F | BAND | 0x4F | FUNK | 0x8F | HAVE | 0xCF | SURE |
| ... | ... | ... | ... | ... | ... | ... | ... |

See [`words.txt`](./words.txt) for the complete list (one word per line, line N = byte N-1).

## Usage

### JavaScript

```bash
npm install byte-mnemonic
```

```javascript
import { encode, decode, encodeHex, decodeToHex } from 'byte-mnemonic';

// Encode bytes to words
const bytes = new Uint8Array([0x48, 0x65, 0x6C, 0x6C, 0x6F]);
const words = encode(bytes);
// => "FILM EXIT LOAF LOAF ONLY"

// Decode words back to bytes
const decoded = decode("FILM EXIT LOAF LOAF ONLY");
// => Uint8Array([0x48, 0x65, 0x6C, 0x6C, 0x6F])

// Encode from hex string
const wordsFromHex = encodeHex("48656C6C6F");
// => "FILM EXIT LOAF LOAF ONLY"

// Decode to hex string
const hex = decodeToHex("FILM EXIT LOAF LOAF ONLY");
// => "48656c6c6f"
```

### Rust

```toml
[dependencies]
byte-mnemonic = "0.1"
```

```rust
use byte_mnemonic::{encode, decode, encode_hex, decode_to_hex};

// Encode bytes to words
let bytes = vec![0x48, 0x65, 0x6C, 0x6C, 0x6F];
let words = encode(&bytes);
// => "FILM EXIT LOAF LOAF ONLY"

// Decode words back to bytes
let decoded = decode("FILM EXIT LOAF LOAF ONLY").unwrap();
// => vec![0x48, 0x65, 0x6C, 0x6C, 0x6F]

// Encode from hex string
let words_from_hex = encode_hex("48656C6C6F").unwrap();
// => "FILM EXIT LOAF LOAF ONLY"

// Decode to hex string
let hex = decode_to_hex("FILM EXIT LOAF LOAF ONLY").unwrap();
// => "48656c6c6f"
```

## Encoding Format

- Words are separated by a single space
- Words are uppercase (decoding is case-insensitive)
- One word per byte, in order

## Applications

- **Verification codes**: "Your code is APEX FILM GROW MILK"
- **Reading hashes aloud**: "SHA starts with ATOM DUCK IRON..."
- **Phone support**: Spell out binary identifiers unambiguously
- **Air-gapped transfers**: Read data across systems verbally
- **Seed phrases**: Human-readable backup of cryptographic keys

## License

MIT
