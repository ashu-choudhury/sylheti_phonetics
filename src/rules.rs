//! Phonetic rule-based transliterator engine for out-of-vocabulary words and suffixes.
//!
//! Applies Sylheti-specific phonetic mappings into:
//! 1. Hindi Devanagari (optimized for Hindi Vocalizer TTS speech synthesis)
//! 2. Bengali script
//! 3. Sylheti Nagri script (Unicode range U+A800..U+A82F)

/// Phonetically transliterates a single Romanized Sylheti token to Hindi Devanagari.
pub fn roman_to_hindi_phonetic(token: &str) -> String {
    let lower = token.to_lowercase();
    let chars: Vec<char> = lower.chars().collect();
    let len = chars.len();
    let mut out = String::new();
    let mut i = 0;

    while i < len {
        // Multi-character sequences first
        if i + 3 <= len {
            let slice: String = chars[i..i + 3].iter().collect();
            match slice.as_str() {
                "oid" => {
                    out.push_str("ोइद");
                    i += 3;
                    continue;
                }
                "oig" => {
                    out.push_str("ोइग");
                    i += 3;
                    continue;
                }
                _ => {}
            }
        }

        if i + 2 <= len {
            let slice: String = chars[i..i + 2].iter().collect();
            match slice.as_str() {
                // Sylheti velar fricative [x]
                "kh" => {
                    out.push_str("ख़");
                    i += 2;
                    continue;
                }
                // Sylheti labial fricative [f] / [ɸ]
                "ph" => {
                    out.push_str("फ़");
                    i += 2;
                    continue;
                }
                "bh" => {
                    out.push('भ');
                    i += 2;
                    continue;
                }
                "ch" => {
                    out.push('च');
                    i += 2;
                    continue;
                }
                "sh" => {
                    out.push('श');
                    i += 2;
                    continue;
                }
                "th" => {
                    out.push('थ');
                    i += 2;
                    continue;
                }
                "dh" => {
                    out.push('ध');
                    i += 2;
                    continue;
                }
                "ng" => {
                    out.push_str("ंग");
                    i += 2;
                    continue;
                }
                "ai" => {
                    if i == 0 {
                        out.push_str("आइ");
                    } else {
                        out.push_str("ाइ");
                    }
                    i += 2;
                    continue;
                }
                "oi" => {
                    if i == 0 {
                        out.push_str("ओइ");
                    } else {
                        out.push_str("ोइ");
                    }
                    i += 2;
                    continue;
                }
                "ee" => {
                    if i == 0 {
                        out.push('ई');
                    } else {
                        out.push('ी');
                    }
                    i += 2;
                    continue;
                }
                "oo" => {
                    if i == 0 {
                        out.push('ऊ');
                    } else {
                        out.push('ू');
                    }
                    i += 2;
                    continue;
                }
                _ => {}
            }
        }

        // Single character rules
        let c = chars[i];
        let is_start = i == 0;

        match c {
            // Vowels
            'a' => {
                if is_start {
                    out.push('आ');
                } else {
                    out.push('ा');
                }
            }
            'i' => {
                if is_start {
                    out.push('इ');
                } else {
                    out.push('ी');
                }
            }
            'u' => {
                if is_start {
                    out.push('उ');
                } else {
                    out.push('ु');
                }
            }
            'e' => {
                if is_start {
                    out.push('ए');
                } else {
                    out.push('े');
                }
            }
            'o' => {
                if is_start {
                    out.push('ओ');
                } else {
                    out.push('ो');
                }
            }
            // Consonants with Sylheti mapping
            'f' => out.push_str("फ़"),
            'z' | 'j' => out.push_str("ज़"),
            'k' => out.push('क'),
            'g' => out.push('ग'),
            'c' => out.push('च'),
            't' => out.push('त'),
            'd' => out.push('द'),
            'n' => out.push('न'),
            'p' => out.push('प'),
            'b' => out.push('ब'),
            'm' => out.push('म'),
            'r' => out.push('र'),
            'l' => out.push('ल'),
            's' => out.push('स'),
            'h' => out.push('ह'),
            'y' => out.push('य'),
            'w' => out.push('व'),
            other => out.push(other),
        }
        i += 1;
    }

    out
}

/// Transliterates a token to Bengali script fallback.
pub fn roman_to_bengali(token: &str) -> String {
    let lower = token.to_lowercase();
    let chars: Vec<char> = lower.chars().collect();
    let len = chars.len();
    let mut out = String::new();
    let mut i = 0;

    while i < len {
        if i + 2 <= len {
            let slice: String = chars[i..i + 2].iter().collect();
            match slice.as_str() {
                "kh" => {
                    out.push('খ');
                    i += 2;
                    continue;
                }
                "ph" => {
                    out.push('ফ');
                    i += 2;
                    continue;
                }
                "bh" => {
                    out.push('ভ');
                    i += 2;
                    continue;
                }
                "ch" => {
                    out.push('চ');
                    i += 2;
                    continue;
                }
                "sh" => {
                    out.push('শ');
                    i += 2;
                    continue;
                }
                "th" => {
                    out.push('থ');
                    i += 2;
                    continue;
                }
                "dh" => {
                    out.push('ধ');
                    i += 2;
                    continue;
                }
                "ai" => {
                    if i == 0 {
                        out.push_str("আই");
                    } else {
                        out.push_str("াই");
                    }
                    i += 2;
                    continue;
                }
                "oi" => {
                    if i == 0 {
                        out.push_str("অই");
                    } else {
                        out.push('ই');
                    }
                    i += 2;
                    continue;
                }
                _ => {}
            }
        }

        let c = chars[i];
        let is_start = i == 0;
        match c {
            'a' => {
                if is_start {
                    out.push('আ');
                } else {
                    out.push('া');
                }
            }
            'i' => {
                if is_start {
                    out.push('ই');
                } else {
                    out.push('ি');
                }
            }
            'u' => {
                if is_start {
                    out.push('উ');
                } else {
                    out.push('ু');
                }
            }
            'e' => {
                if is_start {
                    out.push('এ');
                } else {
                    out.push('ে');
                }
            }
            'o' => {
                if is_start {
                    out.push('অ');
                } else {
                    out.push('ো');
                }
            }
            'k' => out.push('ক'),
            'g' => out.push('গ'),
            'j' | 'z' => out.push('য'),
            't' => out.push('ত'),
            'd' => out.push('দ'),
            'n' => out.push('ন'),
            'p' => out.push('প'),
            'f' => out.push('ফ'),
            'b' => out.push('ব'),
            'm' => out.push('ম'),
            'r' => out.push('র'),
            'l' => out.push('ল'),
            's' => out.push('স'),
            'h' => out.push('হ'),
            'y' => out.push('য়'),
            other => out.push(other),
        }
        i += 1;
    }
    out
}

/// Transliterates a token to Sylheti Nagri script (Unicode block U+A800..U+A82F).
pub fn roman_to_nagri(token: &str) -> String {
    let lower = token.to_lowercase();
    let chars: Vec<char> = lower.chars().collect();
    let len = chars.len();
    let mut out = String::new();
    let mut i = 0;

    while i < len {
        if i + 2 <= len {
            let slice: String = chars[i..i + 2].iter().collect();
            match slice.as_str() {
                "kh" => {
                    out.push('\u{A808}'); // ꠈ
                    i += 2;
                    continue;
                }
                "gh" => {
                    out.push('\u{A80A}'); // ꠊ
                    i += 2;
                    continue;
                }
                "ch" => {
                    out.push('\u{A80C}'); // ꠌ
                    i += 2;
                    continue;
                }
                "th" => {
                    out.push('\u{A814}'); // ꠔ
                    i += 2;
                    continue;
                }
                "dh" => {
                    out.push('\u{A817}'); // ꠗ
                    i += 2;
                    continue;
                }
                "bh" => {
                    out.push('\u{A81C}'); // ꠜ
                    i += 2;
                    continue;
                }
                "sh" => {
                    out.push('\u{A821}'); // ꠡ
                    i += 2;
                    continue;
                }
                "ai" => {
                    out.push_str("\u{A823}\u{A801}"); // ꠣꠁ
                    i += 2;
                    continue;
                }
                _ => {}
            }
        }

        let c = chars[i];
        let is_start = i == 0;
        match c {
            'a' => {
                if is_start {
                    out.push('\u{A800}'); // ꠀ independent A
                } else {
                    out.push('\u{A823}'); // ꠣ vowel sign AA
                }
            }
            'i' => {
                if is_start {
                    out.push('\u{A801}'); // ꠁ independent I
                } else {
                    out.push('\u{A824}'); // ꠤ vowel sign I
                }
            }
            'u' => {
                if is_start {
                    out.push('\u{A803}'); // ꠃ independent U
                } else {
                    out.push('\u{A825}'); // ꠥ vowel sign U
                }
            }
            'e' => {
                if is_start {
                    out.push('\u{A804}'); // ꠄ independent E
                } else {
                    out.push('\u{A826}'); // ꠦ vowel sign E
                }
            }
            'o' => {
                if is_start {
                    out.push('\u{A805}'); // ꠅ independent O
                } else {
                    out.push('\u{A827}'); // ꠧ vowel sign O
                }
            }
            'k' => out.push('\u{A807}'),      // ꠇ
            'g' => out.push('\u{A809}'),      // ꠉ
            'j' | 'z' => out.push('\u{A80E}'), // ꠎ
            't' => out.push('\u{A813}'),      // ꠔ
            'd' => out.push('\u{A816}'),      // ꠖ
            'n' => out.push('\u{A818}'),      // ꠘ
            'p' => out.push('\u{A819}'),      // ꠙ
            'f' => out.push('\u{A819}'),      // ꠙ (Sylheti uses po for f/p)
            'b' => out.push('\u{A81B}'),      // ꠛ
            'm' => out.push('\u{A81D}'),      // ꠝ
            'r' => out.push('\u{A81E}'),      // ꠞ
            'l' => out.push('\u{A81F}'),      // ꠟ
            's' => out.push('\u{A80D}'),      // ꠍ
            'h' => out.push('\u{A822}'),      // ꠢ
            other => out.push(other),
        }
        i += 1;
    }
    out
}
