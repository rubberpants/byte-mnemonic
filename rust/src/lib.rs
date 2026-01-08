//! Byte Mnemonic - Like the NATO alphabet but for binary data.
//!
//! Each byte (0-255) maps to a unique four-letter word, enabling humans to
//! read, speak, and transcribe binary data with minimal errors.
//!
//! # Examples
//!
//! ```
//! use byte_mnemonic::{encode, decode, encode_hex, decode_to_hex};
//!
//! // Encode bytes to words
//! let words = encode(&[0x48, 0x65, 0x6C, 0x6C, 0x6F]);
//! assert_eq!(words, "FILM EXIT LOAF LOAF ONLY");
//!
//! // Decode words back to bytes
//! let bytes = decode("FILM EXIT LOAF LOAF ONLY").unwrap();
//! assert_eq!(bytes, vec![0x48, 0x65, 0x6C, 0x6C, 0x6F]);
//!
//! // Encode from hex string
//! let words = encode_hex("48656C6C6F").unwrap();
//! assert_eq!(words, "FILM EXIT LOAF LOAF ONLY");
//!
//! // Decode to hex string
//! let hex = decode_to_hex("FILM EXIT LOAF LOAF ONLY").unwrap();
//! assert_eq!(hex, "48656c6c6f");
//! ```

use std::collections::HashMap;
use std::sync::LazyLock;

/// The 256 mnemonic words, indexed by byte value.
pub const WORDS: [&str; 256] = [
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
];

/// Reverse lookup map: word -> byte value
static WORD_TO_BYTE: LazyLock<HashMap<&'static str, u8>> = LazyLock::new(|| {
    WORDS
        .iter()
        .enumerate()
        .map(|(i, &word)| (word, i as u8))
        .collect()
});

/// Error type for decoding operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeError {
    /// An unknown word was encountered during decoding.
    UnknownWord(String),
    /// Invalid hexadecimal input.
    InvalidHex(String),
}

impl std::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecodeError::UnknownWord(word) => write!(f, "unknown mnemonic word: \"{}\"", word),
            DecodeError::InvalidHex(msg) => write!(f, "invalid hex: {}", msg),
        }
    }
}

impl std::error::Error for DecodeError {}

/// Encode a byte slice to a space-separated string of mnemonic words.
///
/// # Examples
///
/// ```
/// use byte_mnemonic::encode;
///
/// let words = encode(&[0x48, 0x65, 0x6C, 0x6C, 0x6F]);
/// assert_eq!(words, "FILM EXIT LOAF LOAF ONLY");
///
/// let empty = encode(&[]);
/// assert_eq!(empty, "");
/// ```
pub fn encode(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|&b| WORDS[b as usize])
        .collect::<Vec<_>>()
        .join(" ")
}

/// Decode a space-separated string of mnemonic words to a byte vector.
///
/// Decoding is case-insensitive.
///
/// # Examples
///
/// ```
/// use byte_mnemonic::decode;
///
/// let bytes = decode("FILM EXIT LOAF LOAF ONLY").unwrap();
/// assert_eq!(bytes, vec![0x48, 0x65, 0x6C, 0x6C, 0x6F]);
///
/// // Case-insensitive
/// let bytes = decode("film exit loaf loaf only").unwrap();
/// assert_eq!(bytes, vec![0x48, 0x65, 0x6C, 0x6C, 0x6F]);
/// ```
///
/// # Errors
///
/// Returns `DecodeError::UnknownWord` if an unrecognized word is encountered.
pub fn decode(mnemonic: &str) -> Result<Vec<u8>, DecodeError> {
    let trimmed = mnemonic.trim();
    if trimmed.is_empty() {
        return Ok(Vec::new());
    }

    trimmed
        .split_whitespace()
        .map(|word| {
            let upper = word.to_uppercase();
            WORD_TO_BYTE
                .get(upper.as_str())
                .copied()
                .ok_or_else(|| DecodeError::UnknownWord(word.to_string()))
        })
        .collect()
}

/// Encode a hexadecimal string to mnemonic words.
///
/// Accepts hex strings with or without `0x` prefix, and ignores whitespace.
///
/// # Examples
///
/// ```
/// use byte_mnemonic::encode_hex;
///
/// let words = encode_hex("48656C6C6F").unwrap();
/// assert_eq!(words, "FILM EXIT LOAF LOAF ONLY");
///
/// // With 0x prefix
/// let words = encode_hex("0x48656C6C6F").unwrap();
/// assert_eq!(words, "FILM EXIT LOAF LOAF ONLY");
/// ```
///
/// # Errors
///
/// Returns `DecodeError::InvalidHex` if the input is not valid hexadecimal.
pub fn encode_hex(hex: &str) -> Result<String, DecodeError> {
    let cleaned: String = hex
        .strip_prefix("0x")
        .or_else(|| hex.strip_prefix("0X"))
        .unwrap_or(hex)
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();

    if cleaned.len() % 2 != 0 {
        return Err(DecodeError::InvalidHex(
            "hex string must have even length".to_string(),
        ));
    }

    let bytes: Result<Vec<u8>, _> = (0..cleaned.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&cleaned[i..i + 2], 16)
                .map_err(|_| DecodeError::InvalidHex("invalid hex characters".to_string()))
        })
        .collect();

    Ok(encode(&bytes?))
}

/// Decode mnemonic words to a hexadecimal string.
///
/// # Examples
///
/// ```
/// use byte_mnemonic::decode_to_hex;
///
/// let hex = decode_to_hex("FILM EXIT LOAF LOAF ONLY").unwrap();
/// assert_eq!(hex, "48656c6c6f");
/// ```
///
/// # Errors
///
/// Returns `DecodeError::UnknownWord` if an unrecognized word is encountered.
pub fn decode_to_hex(mnemonic: &str) -> Result<String, DecodeError> {
    let bytes = decode(mnemonic)?;
    Ok(bytes.iter().map(|b| format!("{:02x}", b)).collect())
}

/// Get the mnemonic word for a single byte value.
///
/// # Examples
///
/// ```
/// use byte_mnemonic::byte_to_word;
///
/// assert_eq!(byte_to_word(0x00), "ABLE");
/// assert_eq!(byte_to_word(0xFF), "ZOOM");
/// ```
#[inline]
pub fn byte_to_word(byte: u8) -> &'static str {
    WORDS[byte as usize]
}

/// Get the byte value for a mnemonic word.
///
/// Lookup is case-insensitive and trims surrounding whitespace.
///
/// # Examples
///
/// ```
/// use byte_mnemonic::word_to_byte;
///
/// assert_eq!(word_to_byte("ABLE"), Some(0x00));
/// assert_eq!(word_to_byte("zoom"), Some(0xFF));
/// assert_eq!(word_to_byte("  Kale  "), Some(0x7C));
/// assert_eq!(word_to_byte("INVALID"), None);
/// ```
pub fn word_to_byte(word: &str) -> Option<u8> {
    let upper = word.trim().to_uppercase();
    WORD_TO_BYTE.get(upper.as_str()).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode_roundtrip() {
        let original: Vec<u8> = (0..=255).collect();
        let encoded = encode(&original);
        let decoded = decode(&encoded).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_encode_hello() {
        let hello = b"Hello";
        let encoded = encode(hello);
        assert_eq!(encoded, "FILM EXIT LOAF LOAF ONLY");
    }

    #[test]
    fn test_decode_case_insensitive() {
        let upper = decode("FILM EXIT").unwrap();
        let lower = decode("film exit").unwrap();
        let mixed = decode("Film Exit").unwrap();
        assert_eq!(upper, lower);
        assert_eq!(upper, mixed);
    }

    #[test]
    fn test_encode_hex() {
        let words = encode_hex("48656C6C6F").unwrap();
        assert_eq!(words, "FILM EXIT LOAF LOAF ONLY");
    }

    #[test]
    fn test_decode_to_hex() {
        let hex = decode_to_hex("FILM EXIT LOAF LOAF ONLY").unwrap();
        assert_eq!(hex, "48656c6c6f");
    }

    #[test]
    fn test_byte_to_word() {
        assert_eq!(byte_to_word(0), "ABLE");
        assert_eq!(byte_to_word(255), "ZOOM");
    }

    #[test]
    fn test_word_to_byte() {
        assert_eq!(word_to_byte("ABLE"), Some(0));
        assert_eq!(word_to_byte("ZOOM"), Some(255));
        assert_eq!(word_to_byte("INVALID"), None);
    }

    #[test]
    fn test_empty() {
        assert_eq!(encode(&[]), "");
        assert_eq!(decode("").unwrap(), Vec::<u8>::new());
        assert_eq!(decode("   ").unwrap(), Vec::<u8>::new());
    }

    #[test]
    fn test_unknown_word_error() {
        let result = decode("FILM INVALID EXIT");
        assert!(matches!(result, Err(DecodeError::UnknownWord(_))));
    }

    #[test]
    fn test_invalid_hex_error() {
        let result = encode_hex("48G");
        assert!(matches!(result, Err(DecodeError::InvalidHex(_))));

        let result = encode_hex("123"); // odd length
        assert!(matches!(result, Err(DecodeError::InvalidHex(_))));
    }
}

