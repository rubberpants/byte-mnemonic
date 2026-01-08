# byte-mnemonic (JavaScript)

Like the NATO alphabet but for binary data. Each byte gets a unique four-letter word.

## Installation

```bash
npm install byte-mnemonic
```

## Usage

```javascript
import { encode, decode, encodeHex, decodeToHex, byteToWord, wordToByte } from 'byte-mnemonic';

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

// Single byte/word conversion
byteToWord(0x00);  // => "ABLE"
wordToByte("ZOOM"); // => 255
```

## API

### `encode(bytes: Uint8Array | number[]): string`

Encode a byte array to space-separated mnemonic words.

### `decode(mnemonic: string): Uint8Array`

Decode mnemonic words back to bytes. Case-insensitive.

### `encodeHex(hex: string): string`

Encode a hex string to mnemonic words. Accepts `0x` prefix.

### `decodeToHex(mnemonic: string): string`

Decode mnemonic words to a lowercase hex string.

### `byteToWord(byte: number): string`

Get the mnemonic word for a single byte (0-255).

### `wordToByte(word: string): number`

Get the byte value for a mnemonic word. Case-insensitive, trims whitespace.

### `WORDS: readonly string[]`

The complete word list (256 words, indexed by byte value).

## License

MIT

