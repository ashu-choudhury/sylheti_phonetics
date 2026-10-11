//! # Sylheti Phonetics (`sylheti_phonetics`)
//!
//! A high-performance, native Rust library for Sylheti language processing,
//! phonetic dictionary lookup, and transliteration.
//!
//! Specifically engineered for speech synthesis (TTS) and digital messaging to bridge
//! Romanized Sylheti (as commonly typed in chats) into:
//! 1. **Hindi Devanagari (TTS Mode)**: Specially crafted for Hindi Vocalizer TTS engines
//!    to synthesize authentic native Sylheti speech with correct fricatives (`ख़`, `फ़`, `ज़`).
//! 2. **Standard Bengali Script**: For visual readers and formal orthography.
//! 3. **Sylheti Nagri Script**: For historical and Unicode-standard revitalization (`U+A800..U+A82F`).
//!
//! ### Intelligent English Code-Switching
//! In real-world chats, speakers frequently mix English words with Sylheti
//! (*"Bhai, ami office-o meeting-e aisi, please call khoro"*).
//! `sylheti_phonetics` intelligently recognizes English loanwords and preserves them in Latin script,
//! allowing multilingual synthesizers to read English words natively without mangling them.

pub mod dictionary;
pub mod english;
pub mod rules;
pub mod types;

pub use dictionary::{find_word, SYLHETI_WORDS};
pub use english::{is_english, split_english_suffix};
pub use rules::{roman_to_bengali, roman_to_hindi_phonetic, roman_to_nagri};
pub use types::{PartOfSpeech, TargetScript, WordEntry};

/// Configuration options for text conversion and transliteration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConversionOptions {
    /// Target script for Sylheti words
    pub target: TargetScript,
    /// Whether to detect and preserve standard English words in original Latin script
    pub preserve_english: bool,
}

impl Default for ConversionOptions {
    fn default() -> Self {
        Self {
            target: TargetScript::HindiVocalizer,
            preserve_english: true,
        }
    }
}

/// Converts Romanized Sylheti into Hindi Devanagari phonetically optimized for
/// Hindi TTS synthesizers (like Nuance Vocalizer, Google TTS, SAPI).
///
/// Automatically preserves recognized English words in Latin script so the synthesizer's
/// English engine reads them naturally.
///
/// # Example
/// ```rust
/// use sylheti_phonetics::to_hindi_phonetic;
///
/// let text = "Bhai, ami office-o aisi, please call khoro.";
/// let output = to_hindi_phonetic(text);
/// // Sylheti words are converted; English loanwords like "office", "please", "call" are preserved!
/// assert!(output.contains("भाइ"));
/// assert!(output.contains("please"));
/// ```
pub fn to_hindi_phonetic(text: &str) -> String {
    convert_with_options(
        text,
        &ConversionOptions {
            target: TargetScript::HindiVocalizer,
            preserve_english: true,
        },
    )
}

/// Converts Romanized Sylheti text into Bengali script.
pub fn to_bengali(text: &str) -> String {
    convert_with_options(
        text,
        &ConversionOptions {
            target: TargetScript::Bengali,
            preserve_english: true,
        },
    )
}

/// Converts Romanized Sylheti text into historical Sylheti Nagri script.
pub fn to_nagri(text: &str) -> String {
    convert_with_options(
        text,
        &ConversionOptions {
            target: TargetScript::SylhetiNagri,
            preserve_english: true,
        },
    )
}

/// Converts input text with explicit options.
pub fn convert_with_options(text: &str, options: &ConversionOptions) -> String {
    let mut result = String::with_capacity(text.len() * 2);
    let mut current_token = String::new();

    for c in text.chars() {
        if c.is_alphanumeric() || c == '-' {
            current_token.push(c);
        } else {
            if !current_token.is_empty() {
                result.push_str(&process_token(&current_token, options));
                current_token.clear();
            }
            result.push(c);
        }
    }

    if !current_token.is_empty() {
        result.push_str(&process_token(&current_token, options));
    }

    result
}

/// Processes a single token (word or hyphenated compound).
fn process_token(token: &str, options: &ConversionOptions) -> String {
    // 1. Check if it's directly in the Sylheti dictionary
    if let Some(entry) = find_word(token) {
        return match options.target {
            TargetScript::HindiVocalizer => entry.hindi_phonetic.to_string(),
            TargetScript::Bengali => entry.bengali_script.to_string(),
            TargetScript::SylhetiNagri => entry.nagri_script.to_string(),
        };
    }

    // 2. Check English code-switching
    if options.preserve_english {
        // Pure English word: e.g. "meeting", "please", "call"
        if is_english(token) {
            return token.to_string();
        }

        // English word with attached Sylheti suffix: e.g. "office-o", "bus-e"
        if let Some((stem, suf)) = split_english_suffix(token) {
            let suf_converted = match options.target {
                TargetScript::HindiVocalizer => match suf {
                    "-o" | "o" => "-ओ",
                    "-e" | "e" => "-ए",
                    "-ta" | "ta" => "-टा",
                    "-re" | "re" => "-रे",
                    _ => suf,
                },
                TargetScript::Bengali => match suf {
                    "-o" | "o" => "-ও",
                    "-e" | "e" => "-এ",
                    "-ta" | "ta" => "-টা",
                    "-re" | "re" => "-রে",
                    _ => suf,
                },
                TargetScript::SylhetiNagri => match suf {
                    "-o" | "o" => "-\u{A805}",
                    "-e" | "e" => "-\u{A804}",
                    _ => suf,
                },
            };
            return format!("{}{}", stem, suf_converted);
        }
    }

    // 3. Fallback to phonetic rule transformation
    match options.target {
        TargetScript::HindiVocalizer => roman_to_hindi_phonetic(token),
        TargetScript::Bengali => roman_to_bengali(token),
        TargetScript::SylhetiNagri => roman_to_nagri(token),
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
    fn test_english_code_switching_preservation() {
        let input = "Bhai, ami office-o meeting-e aisi, please later call khoro.";
        let output = to_hindi_phonetic(input);

        // English words must remain English
        assert!(output.contains("office-ओ"));
        assert!(output.contains("meeting-ए"));
        assert!(output.contains("please"));
        assert!(output.contains("later"));
        assert!(output.contains("call"));

        // Sylheti words must be converted
        assert!(output.contains("भाइ"));
        assert!(output.contains("आमी"));
        assert!(output.contains("आइसी"));
        assert!(output.contains("ख़ोरो"));
    }
}
