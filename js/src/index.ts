/**
 * Byte Mnemonic - Like the NATO alphabet but for binary data.
 * Each byte (0-255) maps to a unique four-letter word.
 */

/** The 256 mnemonic words, indexed by byte value */
export const WORDS: readonly string[] = [
  "ABLE", "ACID", "AERO", "AGED", "ALSO", "APEX", "ARCH", "AREA",
  "ARMS", "ATOM", "AUTO", "AWAY", "AXIS", "BABY", "BACK", "BAND",
  "BASH", "BEAN", "BEES", "BILL", "BLUE", "BOAT", "BOSS", "BOZO",
  "BREW", "BULB", "BURN", "BUTT", "CARE", "CHAT", "CITY", "CLAM",
  "CLOT", "CLUB", "COAX", "COLA", "COLD", "COMA", "COST", "CUTE",
  "DARK", "DATE", "DECK", "DELI", "DEMO", "DIET", "DISK", "DOCK",
  "DODO", "DOGS", "DRAW", "DRIP", "DRUM", "DUCT", "DUET", "DULL",
  "DUMB", "EACH", "ECHO", "EDIT", "ELSE", "EMIT", "EVEN", "EXAM",
  "EXIT", "EYES", "FALL", "FARM", "FAST", "FAWN", "FEET", "FEUD",
  "FILM", "FISH", "FLEX", "FOLK", "FOUL", "FOXY", "FROG", "FUNK",
  "FUZZ", "GAME", "GARB", "GAZE", "GERM", "GIFT", "GIVE", "GLAD",
  "GOOD", "GRAB", "GROW", "HALO", "HANK", "HARD", "HAVE", "HEAP",
  "HEEL", "HELP", "HIGH", "HIKE", "HOBO", "HOLY", "HOPE", "HOUR",
  "HUGE", "HUNG", "HURT", "HYMN", "ICON", "IDEA", "IDOL", "IMPS",
  "INCH", "INTO", "IRIS", "IRON", "ISLE", "ITEM", "JAZZ", "JINX",
  "JOIN", "JOKE", "JUMP", "JUST", "KALE", "KEPT", "KEYS", "KICK",
  "KILO", "KIND", "KITE", "KIWI", "LAMP", "LAVA", "LIMO", "LINT",
  "LION", "LOAF", "LONG", "LOOK", "LOUD", "LUCK", "MAIN", "MELT",
  "MILK", "MIST", "MOMS", "MONK", "MOVE", "MUCH", "MYTH", "NEON",
  "NERD", "NEWS", "NICE", "NORM", "NOUN", "NOVA", "NUKE", "NUTS",
  "OATS", "OBEY", "ODOR", "OILS", "ONLY", "OOZE", "OPEN", "ORCA",
  "OVAL", "OVER", "OWLS", "OWNS", "OXEN", "PAID", "PALM", "PAPA",
  "PIGS", "PLAN", "POEM", "POLO", "PONY", "POOR", "PUFF", "PUMA",
  "PYRO", "QUID", "RAYS", "REDS", "RENO", "REST", "RIDE", "RIFF",
  "RIMS", "ROAD", "ROMP", "ROSE", "RUIN", "RUSH", "SAGS", "SALT",
  "SCAB", "SEAT", "SEER", "SHIP", "SLUM", "SNOW", "SOAP", "SODA",
  "SOME", "SPIT", "STAR", "STOP", "SURE", "SWAB", "SWIM", "TACO",
  "TAIL", "TAKE", "TALK", "TAXI", "THEN", "TIME", "TOES", "TOFU",
  "TOMB", "TOOL", "TRAP", "TREE", "TRIM", "TUBA", "TYPE", "TYPO",
  "UGLY", "USED", "VEIL", "VERY", "VIBE", "VOID", "VOTE", "WAIT",
  "WASP", "WAXY", "WELL", "WIFE", "WILD", "WINK", "WOLF", "WORK",
  "YEAR", "YETI", "YOGA", "ZAPS", "ZEAL", "ZINC", "ZONE", "ZOOM",
] as const;

/** Reverse lookup map: word -> byte value */
const WORD_TO_BYTE: Map<string, number> = new Map(
  WORDS.map((word, index) => [word, index])
);

/**
 * Encode a byte array to a space-separated string of mnemonic words.
 *
 * @param bytes - The bytes to encode
 * @returns Space-separated mnemonic words
 *
 * @example
 * encode(new Uint8Array([0x48, 0x65, 0x6C, 0x6C, 0x6F]))
 * // => "FILM HOLY ICON ICON IMPS"
 */
export function encode(bytes: Uint8Array | number[]): string {
  const words: string[] = [];
  for (const byte of bytes) {
    if (byte < 0 || byte > 255) {
      throw new RangeError(`Byte value out of range: ${byte}`);
    }
    words.push(WORDS[byte]);
  }
  return words.join(" ");
}

/**
 * Encode a byte array using the run-length compression extension.
 *
 * Any run of **3 or more** identical bytes is written as the word followed by
 * a decimal repeat count (e.g. `ABLE 4`). Runs of 1 or 2 are written
 * verbatim. The output is fully interoperable with {@link decode}, which
 * always understands both forms.
 *
 * @param bytes - The bytes to encode
 * @returns Space-separated mnemonic words with run-length compression
 *
 * @example
 * encodeCompressed(new Uint8Array([0, 0, 0, 0]))
 * // => "ABLE 4"
 *
 * encodeCompressed(new Uint8Array([0, 0]))
 * // => "ABLE ABLE"  (runs of 2 stay verbatim)
 *
 * encodeCompressed(new Uint8Array([0, 0, 0, 1, 1, 2]))
 * // => "ABLE 3 ACID ACID AERO"
 */
export function encodeCompressed(bytes: Uint8Array | number[]): string {
  const arr = bytes instanceof Uint8Array ? bytes : Uint8Array.from(bytes);
  const out: string[] = [];
  let i = 0;
  while (i < arr.length) {
    const b = arr[i];
    if (b < 0 || b > 255) {
      throw new RangeError(`Byte value out of range: ${b}`);
    }
    let run = 1;
    while (i + run < arr.length && arr[i + run] === b) {
      run++;
    }
    const word = WORDS[b];
    if (run > 2) {
      out.push(`${word} ${run}`);
    } else {
      for (let k = 0; k < run; k++) out.push(word);
    }
    i += run;
  }
  return out.join(" ");
}

/**
 * Decode a space-separated string of mnemonic words to a byte array.
 *
 * Decoding is case-insensitive. The run-length extension is always supported:
 * a non-negative integer token following a word repeats that word the given
 * number of times in total (e.g. `ABLE 4` expands to four `ABLE` bytes).
 *
 * @param mnemonic - Space-separated mnemonic words (case-insensitive)
 * @returns The decoded bytes
 * @throws Error if an unknown word is encountered, or if a repeat count is
 *   malformed, zero, has no preceding word, or directly follows another count.
 *
 * @example
 * decode("FILM HOLY ICON ICON IMPS")
 * // => Uint8Array([0x48, 0x65, 0x6C, 0x6C, 0x6F])
 *
 * decode("ABLE 4")
 * // => Uint8Array([0, 0, 0, 0])
 */
export function decode(mnemonic: string): Uint8Array {
  const trimmed = mnemonic.trim();
  if (trimmed === "") {
    return new Uint8Array(0);
  }
  const tokens = trimmed.split(/\s+/);

  const bytes: number[] = [];
  // The most recently emitted byte, set only immediately after a word token.
  // Cleared after a repeat count is consumed so two counts cannot stack.
  let prev: number | null = null;

  for (const token of tokens) {
    if (/^\d+$/.test(token)) {
      const count = parseInt(token, 10);
      if (!Number.isFinite(count) || count < 1) {
        throw new Error(`Invalid repeat count: "${token}" must be at least 1`);
      }
      if (prev === null) {
        throw new Error(`Invalid repeat count: "${token}" has no preceding word`);
      }
      for (let k = 1; k < count; k++) {
        bytes.push(prev);
      }
      prev = null;
    } else {
      const upper = token.toUpperCase();
      const byte = WORD_TO_BYTE.get(upper);
      if (byte === undefined) {
        throw new Error(`Unknown mnemonic word: "${token}"`);
      }
      bytes.push(byte);
      prev = byte;
    }
  }
  return Uint8Array.from(bytes);
}

/**
 * Encode a hexadecimal string to mnemonic words.
 *
 * @param hex - Hexadecimal string (with or without 0x prefix, spaces allowed)
 * @returns Space-separated mnemonic words
 *
 * @example
 * encodeHex("48656C6C6F")
 * // => "FILM HOLY ICON ICON IMPS"
 */
export function encodeHex(hex: string): string {
  // Remove 0x prefix and spaces
  const cleaned = hex.replace(/^0x/i, "").replace(/\s/g, "");

  if (cleaned.length % 2 !== 0) {
    throw new Error("Hex string must have even length");
  }

  if (!/^[0-9a-fA-F]*$/.test(cleaned)) {
    throw new Error("Invalid hexadecimal string");
  }

  const bytes = new Uint8Array(cleaned.length / 2);
  for (let i = 0; i < cleaned.length; i += 2) {
    bytes[i / 2] = parseInt(cleaned.slice(i, i + 2), 16);
  }

  return encode(bytes);
}

/**
 * Decode mnemonic words to a hexadecimal string.
 *
 * @param mnemonic - Space-separated mnemonic words
 * @returns Lowercase hexadecimal string
 *
 * @example
 * decodeToHex("FILM HOLY ICON ICON IMPS")
 * // => "48656c6c6f"
 */
export function decodeToHex(mnemonic: string): string {
  const bytes = decode(mnemonic);
  return Array.from(bytes)
    .map((b) => b.toString(16).padStart(2, "0"))
    .join("");
}

/**
 * Get the mnemonic word for a single byte value.
 *
 * @param byte - Byte value (0-255)
 * @returns The mnemonic word
 *
 * @example
 * byteToWord(0x00) // => "ABLE"
 * byteToWord(255)  // => "ZOOM"
 */
export function byteToWord(byte: number): string {
  if (byte < 0 || byte > 255) {
    throw new RangeError(`Byte value out of range: ${byte}`);
  }
  return WORDS[byte];
}

/**
 * Get the byte value for a mnemonic word.
 *
 * @param word - The mnemonic word (case-insensitive, whitespace-trimmed)
 * @returns The byte value
 * @throws Error if the word is unknown
 *
 * @example
 * wordToByte("ABLE") // => 0
 * wordToByte("zoom") // => 255
 * wordToByte("  Kale  ") // => 124
 */
export function wordToByte(word: string): number {
  const byte = WORD_TO_BYTE.get(word.trim().toUpperCase());
  if (byte === undefined) {
    throw new Error(`Unknown mnemonic word: "${word}"`);
  }
  return byte;
}

/**
 * Encode an IPv4 address to mnemonic words (4 words).
 *
 * @param ip - Dotted-decimal IPv4 address (e.g. "192.168.1.1")
 * @returns Space-separated mnemonic words
 *
 * @example
 * encodeIPv4("192.168.1.1")
 * // => "RIMS OVAL ACID ACID"
 */
export function encodeIPv4(ip: string): string {
  const parts = ip.trim().split(".");
  if (parts.length !== 4) {
    throw new Error("Invalid IPv4 address: expected 4 octets");
  }
  const bytes = parts.map((p) => {
    const n = parseInt(p, 10);
    if (isNaN(n) || n < 0 || n > 255 || p.trim() === "" || String(n) !== p.trim()) {
      throw new Error(`Invalid IPv4 octet: "${p}"`);
    }
    return n;
  });
  return encode(new Uint8Array(bytes));
}

/**
 * Decode 4 mnemonic words to a dotted-decimal IPv4 address.
 *
 * @param mnemonic - Space-separated mnemonic words (exactly 4)
 * @returns Dotted-decimal IPv4 address
 *
 * @example
 * decodeToIPv4("RIMS OVAL ACID ACID")
 * // => "192.168.1.1"
 */
export function decodeToIPv4(mnemonic: string): string {
  const bytes = decode(mnemonic);
  if (bytes.length !== 4) {
    throw new Error(`Expected 4 words for IPv4, got ${bytes.length}`);
  }
  return Array.from(bytes).join(".");
}

function parseIPv6Bytes(ip: string): Uint8Array {
  const halves = ip.split("::");
  if (halves.length > 2) {
    throw new Error("Invalid IPv6: multiple '::'");
  }

  let groups: string[];
  if (halves.length === 2) {
    const left = halves[0] ? halves[0].split(":") : [];
    const right = halves[1] ? halves[1].split(":") : [];
    const missing = 8 - left.length - right.length;
    if (missing < 0) {
      throw new Error("Invalid IPv6: too many groups");
    }
    groups = [...left, ...Array(missing).fill("0"), ...right];
  } else {
    groups = ip.split(":");
    if (groups.length !== 8) {
      throw new Error("Invalid IPv6: expected 8 groups or use '::' for zero compression");
    }
  }

  const bytes = new Uint8Array(16);
  for (let i = 0; i < 8; i++) {
    if (!/^[0-9a-fA-F]{1,4}$/.test(groups[i])) {
      throw new Error(`Invalid IPv6 group: "${groups[i]}"`);
    }
    const val = parseInt(groups[i], 16);
    bytes[i * 2] = (val >> 8) & 0xff;
    bytes[i * 2 + 1] = val & 0xff;
  }
  return bytes;
}

function formatIPv6Bytes(bytes: Uint8Array): string {
  const groups: number[] = [];
  for (let i = 0; i < 16; i += 2) {
    groups.push((bytes[i] << 8) | bytes[i + 1]);
  }

  let bestStart = -1, bestLen = 0, curStart = -1, curLen = 0;
  for (let i = 0; i <= 8; i++) {
    if (i < 8 && groups[i] === 0) {
      if (curStart === -1) { curStart = i; curLen = 0; }
      curLen++;
    } else {
      if (curLen >= 2 && curLen > bestLen) { bestStart = curStart; bestLen = curLen; }
      curStart = -1; curLen = 0;
    }
  }

  if (bestStart === -1) {
    return groups.map((g) => g.toString(16)).join(":");
  }
  const left = groups.slice(0, bestStart).map((g) => g.toString(16)).join(":");
  const right = groups.slice(bestStart + bestLen).map((g) => g.toString(16)).join(":");
  return (left ? left + "::" : "::") + right;
}

/**
 * Encode an IPv6 address to mnemonic words (16 words).
 *
 * Accepts full and compressed IPv6 notation.
 *
 * @param ip - IPv6 address (e.g. "2001:db8::1" or "::1")
 * @returns Space-separated mnemonic words
 *
 * @example
 * encodeIPv6("::1")
 * // => "ABLE ABLE ABLE ABLE ABLE ABLE ABLE ABLE ABLE ABLE ABLE ABLE ABLE ABLE ABLE ACID"
 */
export function encodeIPv6(ip: string): string {
  const bytes = parseIPv6Bytes(ip.trim());
  return encode(bytes);
}

/**
 * Decode 16 mnemonic words to a compressed IPv6 address.
 *
 * @param mnemonic - Space-separated mnemonic words (exactly 16)
 * @returns Compressed IPv6 address string
 *
 * @example
 * decodeToIPv6("ABLE ABLE ABLE ABLE ABLE ABLE ABLE ABLE ABLE ABLE ABLE ABLE ABLE ABLE ABLE ACID")
 * // => "::1"
 */
export function decodeToIPv6(mnemonic: string): string {
  const bytes = decode(mnemonic);
  if (bytes.length !== 16) {
    throw new Error(`Expected 16 words for IPv6, got ${bytes.length}`);
  }
  return formatIPv6Bytes(bytes);
}

