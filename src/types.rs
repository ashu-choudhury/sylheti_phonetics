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
