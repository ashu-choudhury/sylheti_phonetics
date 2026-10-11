//! Core types for the Sylheti keyword and phonetic dictionary.

/// Represents the part of speech or grammatical category of a word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartOfSpeech {
    Noun,
    Pronoun,
    Verb,
    Adjective,
    Adverb,
    Greeting,
    Interrogative,
    Conjunction,
    Postposition,
    Interjection,
    Number,
    Other,
}

/// A structured entry in the Sylheti dictionary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WordEntry {
    /// Canonical romanized spelling (e.g., "kila", "bala", "khani")
    pub canonical_roman: &'static str,
    /// Common alternative Romanized variations people text (e.g., &["keela", "killa"])
    pub variations: &'static [&'static str],
    /// Phonetic Hindi Devanagari representation specifically crafted for Hindi Vocalizer TTS engines
    /// (e.g., "कीला", "भाला", "ख़ानी", "फ़ानी", "ज़ाइराम")
    pub hindi_phonetic: &'static str,
    /// Standard Bengali script equivalent (e.g., "কিলা" / "কেমন")
    pub bengali_script: &'static str,
    /// Sylheti Nagri script equivalent (Unicode U+A800..U+A82F) (e.g., "ꠇꠤꠟꠣ")
    pub nagri_script: &'static str,
    /// English definition or translation
    pub english_meaning: &'static str,
    /// Part of speech
    pub pos: PartOfSpeech,
}

/// Target script or output representation for transliteration / synthesis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetScript {
    /// Hindi Devanagari tuned for Hindi Vocalizer TTS audio generation
    HindiVocalizer,
    /// Bengali script for visual / text readers
    Bengali,
    /// Historical Sylheti Nagri script (Unicode)
    SylhetiNagri,
}

/// How English loanwords and words within bilingual chats should be handled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EnglishMode {
    /// **Natural Mode (Default)**: Automatically converts recognized English loanwords
    /// into natural Indian phonetic Devanagari (e.g. "office" -> "ऑफ़िस", "meeting" -> "मीटिंग", "call" -> "कॉल").
    ///
    /// *Why this is default*: When TTS engines encounter mixed English Latin text, they
    /// often glitch, pause, or awkwardly switch voices. Converting loanwords to matching Devanagari
    /// maintains a smooth, natural, uninterrupted human speech flow.
    #[default]
    Natural,

    /// **Strict Mode**: Leaves English loanwords entirely in Latin alphabet
    /// (e.g. "office-o" -> "office-ओ", "meeting-e" -> "meeting-ए", "please" -> "please").
    ///
    /// Useful for visual chat displays or multilingual engines configured for seamless bilingual Latin switching.
    Strict,

    /// **Raw Phonetic Mode**: Disables English word detection and treats every Latin sequence
    /// strictly through Sylheti phonetic rules.
    RawPhonetic,
}

/// Full control options passed by developers to customize text conversion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConversionOptions {
    /// Target script for Sylheti words
    pub target: TargetScript,
    /// How to handle English loanwords and code-switching
    pub english_mode: EnglishMode,
}

impl Default for ConversionOptions {
    fn default() -> Self {
        Self {
            target: TargetScript::HindiVocalizer,
            english_mode: EnglishMode::Natural,
        }
    }
}
