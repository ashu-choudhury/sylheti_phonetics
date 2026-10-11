//! # Sylheti Phonetics (`sylheti_phonetics`)
//!
//! A high-performance, native Rust library for Sylheti language processing,
//! phonetic dictionary lookup, and speech synthesis transliteration.
//!
//! Specifically engineered for text-to-speech (TTS) engines and chat accessibility to bridge
//! Romanized Sylheti (as commonly typed in chats) into:
//! 1. **Hindi Devanagari (TTS Mode)**: Specially crafted for Hindi Vocalizer TTS engines
//!    to synthesize authentic native Sylheti speech with correct fricatives (`ख़`, `फ़`, `ज़`).
//! 2. **Standard Bengali Script**: For visual readers and formal orthography.
//! 3. **Sylheti Nagri Script**: For historical and Unicode-standard revitalization (`U+A800..U+A82F`).
//!
//! ### Dual Code-Switching Modes for Developers
//! - **Natural Mode (Default)**: Converts recognized English loanwords into natural Indian phonetic
//!   equivalents (e.g. `office` -> `ऑफ़िस`, `meeting` -> `मीटिंग`, `call` -> `कॉल`).
//!   *Why?* It eliminates awkward voice switching, stuttering, and unnatural pauses in TTS engines!
//! - **Strict Mode**: Preserves English words in raw Latin alphabet (e.g. `office-o` -> `office-ओ`,
//!   `please` -> `please`) for visual chat displays or multilingual engines with clean script switching.

pub mod dictionary;
pub mod english;
pub mod ffi;
pub mod rules;
pub mod types;

pub use dictionary::{find_word, SYLHETI_WORDS};
pub use english::{is_english, lookup_loanword, split_english_suffix};
pub use rules::{roman_to_bengali, roman_to_hindi_phonetic, roman_to_nagri};
pub use types::{ConversionOptions, EnglishMode, PartOfSpeech, TargetScript, WordEntry};

/// Converts Romanized Sylheti into Hindi Devanagari using **Natural Mode (Default)**.
///
/// Converts Sylheti words to authentic phonetics AND converts common English loanwords
/// (`office` -> `ऑफ़िस`, `meeting` -> `मीटिंग`, `call` -> `कॉल`) so the TTS voice speaks
/// in one unbroken, natural flow without voice-profile glitches.
///
/// # Example
/// ```rust
/// use sylheti_phonetics::to_hindi_phonetic;
///
/// let text = "Bhai, ami office-o meeting-e aisi, please call khoro.";
/// let output = to_hindi_phonetic(text);
/// assert!(output.contains("भाइ"));
/// assert!(output.contains("ऑफ़िस-ओ"));
/// assert!(output.contains("मीटिंग-ए"));
/// assert!(output.contains("कॉल"));
/// assert!(output.contains("ख़ोरो"));
/// ```
pub fn to_hindi_phonetic(text: &str) -> String {
    convert(
        text,
        &ConversionOptions {
            target: TargetScript::HindiVocalizer,
            english_mode: EnglishMode::Natural,
        },
    )
}

/// Converts Romanized Sylheti into Hindi Devanagari using **Strict Mode**.
///
/// Keeps recognized English words strictly in their Latin characters
/// (e.g., `office`, `meeting`, `please`) while converting Sylheti words to Devanagari.
///
/// # Example
/// ```rust
/// use sylheti_phonetics::to_hindi_phonetic_strict;
///
/// let text = "Bhai, ami office-o meeting-e aisi, please call khoro.";
/// let output = to_hindi_phonetic_strict(text);
/// assert!(output.contains("office-ओ"));
/// assert!(output.contains("please"));
/// ```
pub fn to_hindi_phonetic_strict(text: &str) -> String {
    convert(
        text,
        &ConversionOptions {
            target: TargetScript::HindiVocalizer,
            english_mode: EnglishMode::Strict,
        },
    )
}

/// Converts Romanized Sylheti text into Bengali script (Natural Mode).
pub fn to_bengali(text: &str) -> String {
    convert(
        text,
        &ConversionOptions {
            target: TargetScript::Bengali,
            english_mode: EnglishMode::Natural,
        },
    )
}

/// Converts Romanized Sylheti text into Bengali script (Strict Mode, preserving Latin English).
pub fn to_bengali_strict(text: &str) -> String {
    convert(
        text,
        &ConversionOptions {
            target: TargetScript::Bengali,
            english_mode: EnglishMode::Strict,
        },
    )
}

/// Converts Romanized Sylheti text into historical Sylheti Nagri script (Natural Mode).
pub fn to_nagri(text: &str) -> String {
    convert(
        text,
        &ConversionOptions {
            target: TargetScript::SylhetiNagri,
            english_mode: EnglishMode::Natural,
        },
    )
}

/// Converts Romanized Sylheti text into historical Sylheti Nagri script (Strict Mode).
pub fn to_nagri_strict(text: &str) -> String {
    convert(
        text,
        &ConversionOptions {
            target: TargetScript::SylhetiNagri,
            english_mode: EnglishMode::Strict,
        },
    )
}

/// Converts input text with fully customized developer options.
pub fn convert(text: &str, options: &ConversionOptions) -> String {
    let mut result = String::with_capacity(text.len() * 2);
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        // 1. Check for URL starting at i: http://, https://, ftp://, www.
        let remaining: String = chars[i..].iter().take(10).collect();
        if remaining.starts_with("http://")
            || remaining.starts_with("https://")
            || remaining.starts_with("ftp://")
            || remaining.starts_with("www.")
        {
            let mut j = i;
            while j < len && !chars[j].is_whitespace() && chars[j] != ')' && chars[j] != ']' && chars[j] != '>' {
                result.push(chars[j]);
                j += 1;
            }
            i = j;
            continue;
        }

        // 2. Check for Mention / Handle (@): e.g. @username
        if chars[i] == '@' {
            result.push('@');
            i += 1;
            while i < len && (chars[i].is_alphanumeric() || chars[i] == '.' || chars[i] == '_' || chars[i] == '-') {
                result.push(chars[i]);
                i += 1;
            }
            continue;
        }

        // 3. Check for Hashtag (#): e.g. #sylhet
        if chars[i] == '#' {
            result.push('#');
            i += 1;
            while i < len && (chars[i].is_alphanumeric() || chars[i] == '_') {
                result.push(chars[i]);
                i += 1;
            }
            continue;
        }

        // 4. Word collection (alphanumeric or hyphen)
        if chars[i].is_alphanumeric() || chars[i] == '-' {
            let mut token = String::new();
            while i < len && (chars[i].is_alphanumeric() || chars[i] == '-') {
                token.push(chars[i]);
                i += 1;
            }
            // Check if this token is part of an email address: e.g. "test@example.com"
            if i < len && chars[i] == '@' {
                result.push_str(&token);
                result.push('@');
                i += 1;
                while i < len && (chars[i].is_alphanumeric() || chars[i] == '.' || chars[i] == '_' || chars[i] == '-') {
                    result.push(chars[i]);
                    i += 1;
                }
            } else {
                result.push_str(&process_token(&token, options));
            }
            continue;
        }

        // 5. Any other character (punctuation, whitespace, emoji, symbols)
        result.push(chars[i]);
        i += 1;
    }

    result
}

/// Processes a single token (word or hyphenated compound).
fn process_token(token: &str, options: &ConversionOptions) -> String {
    // 0. Safety checks: pure hyphens, pure numbers, or URLs should not be transformed
    if token.chars().all(|ch| ch == '-') || token.chars().all(|ch| ch.is_numeric()) {
        return token.to_string();
    }
    if token.starts_with("http") || token.starts_with("www") {
        return token.to_string();
    }

    // 1. Direct Sylheti dictionary lookup
    if let Some(entry) = find_word(token) {
        return match options.target {
            TargetScript::HindiVocalizer => entry.hindi_phonetic.to_string(),
            TargetScript::Bengali => entry.bengali_script.to_string(),
            TargetScript::SylhetiNagri => entry.nagri_script.to_string(),
        };
    }

    // 2. English code-switching handling
    match options.english_mode {
        EnglishMode::Natural => {
            // Check direct loanword mapping
            if let Some(transcription) = lookup_loanword(token, options.target) {
                return transcription.to_string();
            }

            // Check English word with attached Sylheti suffix: e.g. "office-o", "meeting-e"
            if let Some((stem, suf)) = split_english_suffix(token) {
                let stem_converted = if let Some(tr) = lookup_loanword(stem, options.target) {
                    tr.to_string()
                } else if is_english(stem) {
                    stem.to_string()
                } else {
                    roman_to_hindi_phonetic(stem)
                };

                let suf_converted = format_suffix(suf, options.target);
                return format!("{}{}", stem_converted, suf_converted);
            }

            // General English word not in loanword map
            if is_english(token) {
                return token.to_string();
            }
        }
        EnglishMode::Strict => {
            // Strict mode preserves English words as Latin
            if is_english(token) {
                return token.to_string();
            }

            if let Some((stem, suf)) = split_english_suffix(token) {
                let suf_converted = format_suffix(suf, options.target);
                return format!("{}{}", stem, suf_converted);
            }
        }
        EnglishMode::RawPhonetic => {
            // Skips English detection completely
        }
    }

    // 3. Fallback to phonetic rule transformation
    match options.english_mode {
        EnglishMode::Strict => {
            // In Strict Mode, do not aggressively phoneticize arbitrary Latin words into Devanagari.
            // Only dictionary-matched words and explicit Sylheti suffixes are converted.
            token.to_string()
        }
        _ => match options.target {
            TargetScript::HindiVocalizer => roman_to_hindi_phonetic(token),
            TargetScript::Bengali => roman_to_bengali(token),
            TargetScript::SylhetiNagri => roman_to_nagri(token),
        },
    }
}

/// Formats a Sylheti grammatical suffix into the target script.
fn format_suffix(suf: &str, target: TargetScript) -> String {
    let clean = suf.trim_start_matches('-');
    match target {
        TargetScript::HindiVocalizer => match clean {
            "o" => "-ओ".to_string(),
            "e" => "-ए".to_string(),
            "ta" => "-टा".to_string(),
            "re" => "-रे".to_string(),
            "or" => "-ओर".to_string(),
            _ => format!("-{}", clean),
        },
        TargetScript::Bengali => match clean {
            "o" => "-ও".to_string(),
            "e" => "-এ".to_string(),
            "ta" => "-টা".to_string(),
            "re" => "-রে".to_string(),
            "or" => "-ওর".to_string(),
            _ => format!("-{}", clean),
        },
        TargetScript::SylhetiNagri => match clean {
            "o" => "-\u{A805}".to_string(),
            "e" => "-\u{A804}".to_string(),
            _ => format!("-{}", clean),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_greetings_hindi_phonetic() {
        let input = "Kila aso?";
        let output = to_hindi_phonetic(input);
        assert_eq!(output, "कीला आसो?");
    }

    #[test]
    fn test_fricatives_kh_and_f() {
        let input = "Khani khaiso ni? Fani khaiba ni?";
        let output = to_hindi_phonetic(input);
        assert_eq!(output, "ख़ानी ख़ाइसो नी? फ़ानी ख़ाइबा नी?");
    }

    #[test]
    fn test_z_sound() {
        let input = "Ami zairam";
        let output = to_hindi_phonetic(input);
        assert_eq!(output, "आमी ज़ाइराम");
    }

    #[test]
    fn test_bengali_script_conversion() {
        let input = "Kila aso?";
        let output = to_bengali(input);
        assert_eq!(output, "কিলা আসো?");
    }

    #[test]
    fn test_nagri_script_conversion() {
        let input = "Kila aso?";
        let output = to_nagri(input);
        assert_eq!(output, "ꠇꠤꠟꠣ ꠀꠍꠧ?");
    }

    #[test]
    fn test_barak_valley_phrase() {
        let input = "Ami bala asi";
        let output = to_hindi_phonetic(input);
        assert_eq!(output, "आमी भाला आसी");
    }

    #[test]
    fn test_natural_mode_loanword_conversion() {
        let input = "Bhai, ami office-o meeting-e aisi, please call khoro.";
        let output = to_hindi_phonetic(input);

        // In Natural mode, loanwords are converted to seamless Devanagari for unbroken TTS audio!
        assert!(output.contains("ऑफ़िस-ओ"));
        assert!(output.contains("मीटिंग-ए"));
        assert!(output.contains("प्लीज़"));
        assert!(output.contains("कॉल"));
        assert!(output.contains("ख़ोरो"));
    }

    #[test]
    fn test_strict_mode_preservation() {
        let input = "Bhai, ami office-o meeting-e aisi, please call khoro.";
        let output = to_hindi_phonetic_strict(input);

        // In Strict mode, English words remain in Latin
        assert!(output.contains("office-ओ"));
        assert!(output.contains("meeting-ए"));
        assert!(output.contains("please"));
        assert!(output.contains("call"));
        assert!(output.contains("ख़ोरो"));
    }
}
