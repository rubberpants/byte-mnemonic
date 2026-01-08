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
 * // => "FILM EXIT LOAF LOAF ONLY"
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
 * Decode a space-separated string of mnemonic words to a byte array.
 *
 * @param mnemonic - Space-separated mnemonic words (case-insensitive)
 * @returns The decoded bytes
 * @throws Error if an unknown word is encountered
 *
 * @example
 * decode("FILM EXIT LOAF LOAF ONLY")
 * // => Uint8Array([0x48, 0x65, 0x6C, 0x6C, 0x6F])
 */
export function decode(mnemonic: string): Uint8Array {
  const words = mnemonic.trim().split(/\s+/);
  if (words.length === 1 && words[0] === "") {
    return new Uint8Array(0);
  }

  const bytes = new Uint8Array(words.length);
  for (let i = 0; i < words.length; i++) {
    const word = words[i].toUpperCase();
    const byte = WORD_TO_BYTE.get(word);
    if (byte === undefined) {
      throw new Error(`Unknown mnemonic word: "${words[i]}"`);
    }
    bytes[i] = byte;
  }
  return bytes;
}

/**
 * Encode a hexadecimal string to mnemonic words.
 *
 * @param hex - Hexadecimal string (with or without 0x prefix, spaces allowed)
 * @returns Space-separated mnemonic words
 *
 * @example
 * encodeHex("48656C6C6F")
 * // => "FILM EXIT LOAF LOAF ONLY"
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
 * decodeToHex("FILM EXIT LOAF LOAF ONLY")
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

