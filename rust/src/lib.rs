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
//! assert_eq!(words, "FILM HOLY ICON ICON IMPS");
//!
//! // Decode words back to bytes
//! let bytes = decode("FILM HOLY ICON ICON IMPS").unwrap();
//! assert_eq!(bytes, vec![0x48, 0x65, 0x6C, 0x6C, 0x6F]);
//!
//! // Encode from hex string
//! let words = encode_hex("48656C6C6F").unwrap();
//! assert_eq!(words, "FILM HOLY ICON ICON IMPS");
//!
//! // Decode to hex string
//! let hex = decode_to_hex("FILM HOLY ICON ICON IMPS").unwrap();
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
    /// Invalid IP address input.
    InvalidAddress(String),
    /// Invalid run-length repeat count (e.g. zero, leading, or duplicated).
    InvalidRepeat(String),
}

impl std::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecodeError::UnknownWord(word) => write!(f, "unknown mnemonic word: \"{}\"", word),
            DecodeError::InvalidHex(msg) => write!(f, "invalid hex: {}", msg),
            DecodeError::InvalidAddress(msg) => write!(f, "invalid address: {}", msg),
            DecodeError::InvalidRepeat(msg) => write!(f, "invalid repeat count: {}", msg),
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
/// assert_eq!(words, "FILM HOLY ICON ICON IMPS");
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

/// Encode a byte slice using the run-length compression extension.
///
/// Any run of **3 or more** identical bytes is written as the word followed by
/// a decimal repeat count (e.g. `ABLE 4`). Runs of 1 or 2 are written
/// verbatim. The output is fully interoperable with [`decode`], which always
/// understands both forms.
///
/// # Examples
///
/// ```
/// use byte_mnemonic::{encode_compressed, decode};
///
/// // Three or more repeats collapse to "WORD N"
/// assert_eq!(encode_compressed(&[0, 0, 0, 0]), "ABLE 4");
///
/// // Two repeats are written verbatim
/// assert_eq!(encode_compressed(&[0, 0]), "ABLE ABLE");
///
/// // Mixed runs
/// assert_eq!(
///     encode_compressed(&[0, 0, 0, 1, 1, 2]),
///     "ABLE 3 ACID ACID AERO"
/// );
///
/// // Round-trips through decode
/// let bytes: Vec<u8> = vec![0, 0, 0, 0, 1, 2, 2, 2];
/// assert_eq!(decode(&encode_compressed(&bytes)).unwrap(), bytes);
/// ```
pub fn encode_compressed(bytes: &[u8]) -> String {
    let mut out: Vec<String> = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        let mut run = 1;
        while i + run < bytes.len() && bytes[i + run] == b {
            run += 1;
        }
        let word = WORDS[b as usize];
        if run > 2 {
            out.push(format!("{} {}", word, run));
        } else {
            for _ in 0..run {
                out.push(word.to_string());
            }
        }
        i += run;
    }
    out.join(" ")
}

/// Decode a space-separated string of mnemonic words to a byte vector.
///
/// Decoding is case-insensitive. The run-length extension is always supported:
/// a non-negative integer token following a word repeats that word the given
/// number of times in total (e.g. `ABLE 4` expands to four `ABLE` bytes).
///
/// # Examples
///
/// ```
/// use byte_mnemonic::decode;
///
/// let bytes = decode("FILM HOLY ICON ICON IMPS").unwrap();
/// assert_eq!(bytes, vec![0x48, 0x65, 0x6C, 0x6C, 0x6F]);
///
/// // Case-insensitive
/// let bytes = decode("film holy icon icon imps").unwrap();
/// assert_eq!(bytes, vec![0x48, 0x65, 0x6C, 0x6C, 0x6F]);
///
/// // Run-length extension
/// assert_eq!(decode("ABLE 4").unwrap(), vec![0, 0, 0, 0]);
/// assert_eq!(decode("ABLE 3 ACID").unwrap(), vec![0, 0, 0, 1]);
/// ```
///
/// # Errors
///
/// - `DecodeError::UnknownWord` if an unrecognized word is encountered.
/// - `DecodeError::InvalidRepeat` if a repeat count is malformed, zero, has no
///   preceding word, or directly follows another repeat count.
pub fn decode(mnemonic: &str) -> Result<Vec<u8>, DecodeError> {
    let trimmed = mnemonic.trim();
    if trimmed.is_empty() {
        return Ok(Vec::new());
    }

    let mut bytes: Vec<u8> = Vec::new();
    // The most recently emitted byte, only set immediately after a word token.
    // Cleared after a repeat count is consumed so two counts cannot stack.
    let mut prev: Option<u8> = None;

    for token in trimmed.split_whitespace() {
        if token.chars().all(|c| c.is_ascii_digit()) {
            let count: usize = token.parse().map_err(|_| {
                DecodeError::InvalidRepeat(format!("\"{}\" is not a valid count", token))
            })?;
            if count == 0 {
                return Err(DecodeError::InvalidRepeat(
                    "count must be at least 1".to_string(),
                ));
            }
            let byte = prev.ok_or_else(|| {
                DecodeError::InvalidRepeat(format!(
                    "count \"{}\" has no preceding word",
                    token
                ))
            })?;
            // We already pushed one copy when we saw the word; add count - 1 more.
            for _ in 1..count {
                bytes.push(byte);
            }
            prev = None;
        } else {
            let upper = token.to_uppercase();
            let byte = WORD_TO_BYTE
                .get(upper.as_str())
                .copied()
                .ok_or_else(|| DecodeError::UnknownWord(token.to_string()))?;
            bytes.push(byte);
            prev = Some(byte);
        }
    }

    Ok(bytes)
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
/// assert_eq!(words, "FILM HOLY ICON ICON IMPS");
///
/// // With 0x prefix
/// let words = encode_hex("0x48656C6C6F").unwrap();
/// assert_eq!(words, "FILM HOLY ICON ICON IMPS");
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
/// let hex = decode_to_hex("FILM HOLY ICON ICON IMPS").unwrap();
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

/// Encode an IPv4 address to mnemonic words (4 words).
///
/// # Examples
///
/// ```
/// use byte_mnemonic::encode_ipv4;
///
/// let words = encode_ipv4("192.168.1.1").unwrap();
/// assert_eq!(words, "RIMS OVAL ACID ACID");
/// ```
///
/// # Errors
///
/// Returns `DecodeError::InvalidAddress` if the input is not a valid IPv4 address.
pub fn encode_ipv4(ip: &str) -> Result<String, DecodeError> {
    let parts: Vec<&str> = ip.trim().split('.').collect();
    if parts.len() != 4 {
        return Err(DecodeError::InvalidAddress(
            "expected 4 octets".to_string(),
        ));
    }
    let bytes: Result<Vec<u8>, _> = parts.iter().map(|p| {
        p.parse::<u8>().map_err(|_| {
            DecodeError::InvalidAddress(format!("invalid octet: \"{}\"", p))
        })
    }).collect();
    Ok(encode(&bytes?))
}

/// Decode 4 mnemonic words to a dotted-decimal IPv4 address.
///
/// # Examples
///
/// ```
/// use byte_mnemonic::decode_to_ipv4;
///
/// let ip = decode_to_ipv4("RIMS OVAL ACID ACID").unwrap();
/// assert_eq!(ip, "192.168.1.1");
/// ```
///
/// # Errors
///
/// Returns an error if the mnemonic does not decode to exactly 4 bytes.
pub fn decode_to_ipv4(mnemonic: &str) -> Result<String, DecodeError> {
    let bytes = decode(mnemonic)?;
    if bytes.len() != 4 {
        return Err(DecodeError::InvalidAddress(format!(
            "expected 4 words for IPv4, got {}",
            bytes.len()
        )));
    }
    Ok(format!("{}.{}.{}.{}", bytes[0], bytes[1], bytes[2], bytes[3]))
}

fn parse_ipv6_bytes(ip: &str) -> Result<Vec<u8>, DecodeError> {
    let inv = |msg: &str| DecodeError::InvalidAddress(msg.to_string());

    let halves: Vec<&str> = ip.splitn(3, "::").collect();
    if halves.len() > 2 {
        return Err(inv("multiple '::'"));
    }

    let groups: Vec<&str> = if halves.len() == 2 {
        let left: Vec<&str> = if halves[0].is_empty() { vec![] } else { halves[0].split(':').collect() };
        let right: Vec<&str> = if halves[1].is_empty() { vec![] } else { halves[1].split(':').collect() };
        let missing = 8usize.checked_sub(left.len() + right.len())
            .ok_or_else(|| inv("too many groups"))?;
        // Build owned strings for the zero groups then collect refs — work with indices instead
        let total = left.len() + missing + right.len();
        let _ = total; // used below via a different approach
        let mut g: Vec<String> = Vec::with_capacity(8);
        g.extend(left.iter().map(|s| s.to_string()));
        g.extend(std::iter::repeat("0".to_string()).take(missing));
        g.extend(right.iter().map(|s| s.to_string()));
        return parse_ipv6_groups(&g);
    } else {
        ip.split(':').collect()
    };

    if groups.len() != 8 {
        return Err(inv("expected 8 groups or use '::' for zero compression"));
    }
    let owned: Vec<String> = groups.iter().map(|s| s.to_string()).collect();
    parse_ipv6_groups(&owned)
}

fn parse_ipv6_groups(groups: &[String]) -> Result<Vec<u8>, DecodeError> {
    let inv = |msg: String| DecodeError::InvalidAddress(msg);
    if groups.len() != 8 {
        return Err(inv(format!("expected 8 groups, got {}", groups.len())));
    }
    let mut bytes = vec![0u8; 16];
    for (i, g) in groups.iter().enumerate() {
        if g.is_empty() || g.len() > 4 || !g.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(inv(format!("invalid IPv6 group: \"{}\"", g)));
        }
        let val = u16::from_str_radix(g, 16)
            .map_err(|_| inv(format!("invalid IPv6 group: \"{}\"", g)))?;
        bytes[i * 2] = (val >> 8) as u8;
        bytes[i * 2 + 1] = (val & 0xff) as u8;
    }
    Ok(bytes)
}

fn format_ipv6_bytes(bytes: &[u8]) -> String {
    let groups: Vec<u16> = (0..8)
        .map(|i| ((bytes[i * 2] as u16) << 8) | bytes[i * 2 + 1] as u16)
        .collect();

    // Find the longest run of consecutive zero groups (min 2) for :: compression
    let (mut best_start, mut best_len) = (usize::MAX, 0usize);
    let (mut cur_start, mut cur_len) = (usize::MAX, 0usize);
    for i in 0..=8 {
        if i < 8 && groups[i] == 0 {
            if cur_start == usize::MAX { cur_start = i; cur_len = 0; }
            cur_len += 1;
        } else {
            if cur_len >= 2 && cur_len > best_len {
                best_start = cur_start;
                best_len = cur_len;
            }
            cur_start = usize::MAX;
            cur_len = 0;
        }
    }

    if best_len == 0 {
        return groups.iter().map(|g| format!("{:x}", g)).collect::<Vec<_>>().join(":");
    }

    let left: Vec<String> = groups[..best_start].iter().map(|g| format!("{:x}", g)).collect();
    let right: Vec<String> = groups[best_start + best_len..].iter().map(|g| format!("{:x}", g)).collect();
    let left_str = left.join(":");
    let right_str = right.join(":");
    if left_str.is_empty() {
        format!("::{}", right_str)
    } else if right_str.is_empty() {
        format!("{}::", left_str)
    } else {
        format!("{}::{}", left_str, right_str)
    }
}

/// Encode an IPv6 address to mnemonic words (16 words).
///
/// Accepts full and compressed IPv6 notation.
///
/// # Examples
///
/// ```
/// use byte_mnemonic::encode_ipv6;
///
/// let words = encode_ipv6("::1").unwrap();
/// // loopback: 15 zero bytes + 0x01
/// assert!(words.starts_with("ABLE ABLE ABLE"));
/// assert!(words.ends_with("ACID"));
/// ```
///
/// # Errors
///
/// Returns `DecodeError::InvalidAddress` if the input is not a valid IPv6 address.
pub fn encode_ipv6(ip: &str) -> Result<String, DecodeError> {
    let bytes = parse_ipv6_bytes(ip.trim())?;
    Ok(encode(&bytes))
}

/// Decode 16 mnemonic words to a compressed IPv6 address.
///
/// # Examples
///
/// ```
/// use byte_mnemonic::decode_to_ipv6;
///
/// // Round-trip the loopback address
/// use byte_mnemonic::encode_ipv6;
/// let words = encode_ipv6("::1").unwrap();
/// let ip = decode_to_ipv6(&words).unwrap();
/// assert_eq!(ip, "::1");
/// ```
///
/// # Errors
///
/// Returns an error if the mnemonic does not decode to exactly 16 bytes.
pub fn decode_to_ipv6(mnemonic: &str) -> Result<String, DecodeError> {
    let bytes = decode(mnemonic)?;
    if bytes.len() != 16 {
        return Err(DecodeError::InvalidAddress(format!(
            "expected 16 words for IPv6, got {}",
            bytes.len()
        )));
    }
    Ok(format_ipv6_bytes(&bytes))
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
        assert_eq!(encoded, "FILM HOLY ICON ICON IMPS");
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
        assert_eq!(words, "FILM HOLY ICON ICON IMPS");
    }

    #[test]
    fn test_decode_to_hex() {
        let hex = decode_to_hex("FILM HOLY ICON ICON IMPS").unwrap();
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

    #[test]
    fn test_encode_ipv4() {
        // 192=RIMS, 168=OVAL, 1=ACID, 1=ACID
        let words = encode_ipv4("192.168.1.1").unwrap();
        assert_eq!(words, "RIMS OVAL ACID ACID");
    }

    #[test]
    fn test_decode_to_ipv4() {
        let ip = decode_to_ipv4("RIMS OVAL ACID ACID").unwrap();
        assert_eq!(ip, "192.168.1.1");
    }

    #[test]
    fn test_ipv4_roundtrip() {
        for a in [0u8, 10, 127, 192, 255] {
            for b in [0u8, 1, 168, 255] {
                let ip = format!("{}.{}.0.1", a, b);
                let words = encode_ipv4(&ip).unwrap();
                let decoded = decode_to_ipv4(&words).unwrap();
                assert_eq!(ip, decoded);
            }
        }
    }

    #[test]
    fn test_encode_ipv6_loopback() {
        let words = encode_ipv6("::1").unwrap();
        assert!(words.ends_with("ACID"), "loopback last byte is 0x01 = ACID");
        let decoded = decode_to_ipv6(&words).unwrap();
        assert_eq!(decoded, "::1");
    }

    #[test]
    fn test_encode_ipv6_full() {
        let words = encode_ipv6("2001:0db8:85a3:0000:0000:8a2e:0370:7334").unwrap();
        let decoded = decode_to_ipv6(&words).unwrap();
        // Should round-trip (compressed form)
        let reencoded = encode_ipv6(&decoded).unwrap();
        assert_eq!(words, reencoded);
    }

    #[test]
    fn test_ipv6_all_zeros() {
        let words = encode_ipv6("::").unwrap();
        let decoded = decode_to_ipv6(&words).unwrap();
        assert_eq!(decoded, "::");
    }

    #[test]
    fn test_invalid_ipv4() {
        assert!(matches!(encode_ipv4("999.0.0.1"), Err(DecodeError::InvalidAddress(_))));
        assert!(matches!(encode_ipv4("1.2.3"), Err(DecodeError::InvalidAddress(_))));
        assert!(matches!(decode_to_ipv4("ABLE ABLE ABLE"), Err(DecodeError::InvalidAddress(_))));
    }

    #[test]
    fn test_invalid_ipv6() {
        assert!(matches!(encode_ipv6("not::valid::addr"), Err(DecodeError::InvalidAddress(_))));
        assert!(matches!(encode_ipv6("gggg::1"), Err(DecodeError::InvalidAddress(_))));
        assert!(matches!(decode_to_ipv6("ABLE ABLE"), Err(DecodeError::InvalidAddress(_))));
    }

    #[test]
    fn test_encode_compressed_basic() {
        assert_eq!(encode_compressed(&[]), "");
        assert_eq!(encode_compressed(&[0]), "ABLE");
        assert_eq!(encode_compressed(&[0, 0]), "ABLE ABLE");
        assert_eq!(encode_compressed(&[0, 0, 0]), "ABLE 3");
        assert_eq!(encode_compressed(&[0, 0, 0, 0]), "ABLE 4");
    }

    #[test]
    fn test_encode_compressed_mixed_runs() {
        // 1 ABLE, 2 ACID, 5 AERO, 1 AGED
        let bytes = [0, 1, 1, 2, 2, 2, 2, 2, 3];
        assert_eq!(encode_compressed(&bytes), "ABLE ACID ACID AERO 5 AGED");
    }

    #[test]
    fn test_encode_compressed_only_runs_over_two() {
        // Runs of exactly 2 stay verbatim
        let bytes = [5, 5, 6, 6, 6, 7, 7];
        assert_eq!(encode_compressed(&bytes), "APEX APEX ARCH 3 AREA AREA");
    }

    #[test]
    fn test_decode_run_length() {
        assert_eq!(decode("ABLE 4").unwrap(), vec![0, 0, 0, 0]);
        assert_eq!(decode("ABLE 1").unwrap(), vec![0]);
        assert_eq!(decode("ABLE 3 ACID").unwrap(), vec![0, 0, 0, 1]);
        assert_eq!(
            decode("ACID ABLE 3 ACID").unwrap(),
            vec![1, 0, 0, 0, 1]
        );
        // Case-insensitive word still works alongside counts
        assert_eq!(decode("able 5").unwrap(), vec![0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_decode_run_length_consecutive_words_after_count() {
        // After "ABLE 3", subsequent ABLE keeps adding bytes
        assert_eq!(
            decode("ABLE 3 ABLE").unwrap(),
            vec![0, 0, 0, 0]
        );
    }

    #[test]
    fn test_compressed_roundtrip_all_bytes() {
        let original: Vec<u8> = (0..=255).collect();
        let encoded = encode_compressed(&original);
        let decoded = decode(&encoded).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_compressed_roundtrip_with_runs() {
        let mut original: Vec<u8> = Vec::new();
        original.extend(std::iter::repeat(0u8).take(7));
        original.extend(std::iter::repeat(42u8).take(2));
        original.extend(std::iter::repeat(99u8).take(1));
        original.extend(std::iter::repeat(255u8).take(10));
        let encoded = encode_compressed(&original);
        assert!(encoded.contains(" 7"));
        assert!(encoded.contains(" 10"));
        let decoded = decode(&encoded).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_decode_leading_count_errors() {
        assert!(matches!(
            decode("3 ABLE"),
            Err(DecodeError::InvalidRepeat(_))
        ));
    }

    #[test]
    fn test_decode_zero_count_errors() {
        assert!(matches!(
            decode("ABLE 0"),
            Err(DecodeError::InvalidRepeat(_))
        ));
    }

    #[test]
    fn test_decode_double_count_errors() {
        assert!(matches!(
            decode("ABLE 3 2"),
            Err(DecodeError::InvalidRepeat(_))
        ));
    }

    #[test]
    fn test_decode_to_hex_supports_compression() {
        let hex = decode_to_hex("ABLE 4").unwrap();
        assert_eq!(hex, "00000000");
    }

    #[test]
    fn test_compressed_uses_extension_only_when_beneficial() {
        // The phrasing "more than 2 repeated words" — exactly 2 must stay verbatim
        assert_eq!(encode_compressed(&[0, 0]), "ABLE ABLE");
        assert_ne!(encode_compressed(&[0, 0]), "ABLE 2");
    }
}

