//! # Keyword Holder (`keyword_holder`)
//!
//! A high-performance, native Rust library for Sylheti language processing,
//! phonetic dictionary lookup, and transliteration.
//!
//! Specifically engineered to bridge Romanized Sylheti (as commonly typed on social media)
//! to:
//! 1. **Hindi Devanagari (Phonetic TTS mode)**: Tuned specifically for Hindi Vocalizer TTS voices
//!    to synthesize authentic native Sylheti speech with correct fricatives (`ख़`, `फ़`, `ज़`).
//! 2. **Standard Bengali Script**: For visual readers and standard orthography.
//! 3. **Sylheti Nagri Script**: For historical and Unicode-standard revitalization (`U+A800..U+A82F`).

pub mod dictionary;
pub mod rules;
pub mod types;

pub use dictionary::{find_word, SYLHETI_WORDS};
pub use rules::{roman_to_bengali, roman_to_hindi_phonetic, roman_to_nagri};
pub use types::{PartOfSpeech, TargetScript, WordEntry};

/// Converts a word or sentence of Romanized Sylheti into Hindi Devanagari phonetically
/// optimized for Hindi TTS synthesizers (like Nuance Vocalizer).
///
/// # Example
/// ```rust
/// use keyword_holder::to_hindi_phonetic;
///
/// let audio_text = to_hindi_phonetic("Kila aso bhai? Khani khaiso ni?");
/// assert!(audio_text.contains("कीला"));
/// assert!(audio_text.contains("ख़ानी"));
/// ```
pub fn to_hindi_phonetic(text: &str) -> String {
    convert(text, TargetScript::HindiVocalizer)
}

/// Converts Romanized Sylheti text into Bengali script.
pub fn to_bengali(text: &str) -> String {
    convert(text, TargetScript::Bengali)
}

/// Converts Romanized Sylheti text into historical Sylheti Nagri script.
pub fn to_nagri(text: &str) -> String {
    convert(text, TargetScript::SylhetiNagri)
}

/// Converts input text to the specified target script.
///
/// Preserves whitespace, punctuation, and handles compound/hyphenated tokens cleanly.
pub fn convert(text: &str, target: TargetScript) -> String {
    let mut result = String::with_capacity(text.len() * 2);
    let mut current_word = String::new();

    for c in text.chars() {
        if c.is_alphabetic() {
            current_word.push(c);
        } else {
            if !current_word.is_empty() {
                result.push_str(&convert_single_word(&current_word, target));
                current_word.clear();
            }
            result.push(c);
        }
    }

    if !current_word.is_empty() {
        result.push_str(&convert_single_word(&current_word, target));
    }

    result
}

/// Converts a single word token (without punctuation) using dictionary lookup first,
/// falling back to phonetic rules if unlisted.
pub fn convert_single_word(word: &str, target: TargetScript) -> String {
    if let Some(entry) = find_word(word) {
        match target {
            TargetScript::HindiVocalizer => entry.hindi_phonetic.to_string(),
            TargetScript::Bengali => entry.bengali_script.to_string(),
            TargetScript::SylhetiNagri => entry.nagri_script.to_string(),
        }
    } else {
        // Fallback to phonetic rule transformation
        match target {
            TargetScript::HindiVocalizer => roman_to_hindi_phonetic(word),
            TargetScript::Bengali => roman_to_bengali(word),
            TargetScript::SylhetiNagri => roman_to_nagri(word),
        }
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
}
