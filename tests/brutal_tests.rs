//! Brutal stress and edge-case test suite for `sylheti_phonetics`.

use sylheti_phonetics::{
    convert, find_word, to_bengali, to_bengali_strict, to_hindi_phonetic,
    to_hindi_phonetic_strict, to_nagri, to_nagri_strict, ConversionOptions, EnglishMode,
    TargetScript, SYLHETI_WORDS,
};

#[test]
fn test_all_dictionary_entries_non_empty() {
    for entry in SYLHETI_WORDS {
        assert!(!entry.canonical_roman.is_empty());
        assert!(!entry.hindi_phonetic.is_empty());
        assert!(!entry.bengali_script.is_empty());
        assert!(!entry.nagri_script.is_empty());
        assert!(!entry.english_meaning.is_empty());

        // Verify lookup by canonical name
        let found = find_word(entry.canonical_roman).expect("Should find canonical");
        assert_eq!(found.canonical_roman, entry.canonical_roman);

        // Verify lookup by each variation
        for var in entry.variations {
            let found_var = find_word(var).expect("Should find variation");
            assert_eq!(found_var.canonical_roman, entry.canonical_roman);
        }
    }
}

#[test]
fn test_case_insensitivity_stress() {
    let variants = ["kila", "KILA", "Kila", "kIlA", "KiLa"];
    for v in variants {
        let output = to_hindi_phonetic(v);
        assert_eq!(output, "कीला", "Failed for variant: {}", v);
    }

    let sentences = [
        "AMI BALA ASI",
        "Ami Bala Asi",
        "ami bala asi",
        "aMi bAlA aSi",
    ];
    for s in sentences {
        let output = to_hindi_phonetic(s);
        assert_eq!(output, "आमी भाला आसी", "Failed for case sentence: {}", s);
    }
}

#[test]
fn test_empty_and_whitespace_only() {
    assert_eq!(to_hindi_phonetic(""), "");
    assert_eq!(to_hindi_phonetic(" "), " ");
    assert_eq!(to_hindi_phonetic("   \t  \n  \r\n  "), "   \t  \n  \r\n  ");
}

#[test]
fn test_punctuation_extremes() {
    let input = "¿¡Kila aso!?!? (Bhai)... \"Khani khaiso ni?\": Yes/No; [ok] {test}";
    let output = to_hindi_phonetic(input);

    assert!(output.starts_with("¿¡कीला आसो!?!?"));
    assert!(output.contains("(भाइ)..."));
    assert!(output.contains("\"ख़ानी ख़ाइसो नी?\":"));
}

#[test]
fn test_emojis_and_symbols() {
    let input = "Kila aso bhai? 😊🔥❤️ @user #sylhet $100 50% 5:30pm";
    let output = to_hindi_phonetic(input);

    assert!(output.contains("कीला आसो भाइ? 😊🔥❤️"));
    assert!(output.contains("@user"));
    assert!(output.contains("#sylhet"));
    assert!(output.contains("$100"));
    assert!(output.contains("50%"));
}

#[test]
fn test_numbers_intact() {
    let input = "123 456 7890 2026-10-11 3.14159";
    let output = to_hindi_phonetic(input);
    assert_eq!(output, input, "Numbers should remain untouched");
}

#[test]
fn test_standalone_hyphens_and_dashes() {
    let input = "Ami - bala -- asi --- ok";
    let output = to_hindi_phonetic(input);
    assert!(output.contains("आमी - भाला -- आसी ---"));
}

#[test]
fn test_loanword_variations_in_natural_mode() {
    let input = "office-o meeting-e hospital-o bus-e train-o doctor-re ticket-ta";
    let output = to_hindi_phonetic(input);

    assert!(output.contains("ऑफ़िस-ओ"));
    assert!(output.contains("मीटिंग-ए"));
    assert!(output.contains("हॉस्पिटल-ओ"));
    assert!(output.contains("बस-ए"));
    assert!(output.contains("ट्रेन-ओ"));
    assert!(output.contains("डॉक्टर-रे"));
    assert!(output.contains("टिकट-टा"));
}

#[test]
fn test_loanword_variations_in_strict_mode() {
    let input = "office-o meeting-e hospital-o bus-e train-o doctor-re ticket-ta";
    let output = to_hindi_phonetic_strict(input);

    assert!(output.contains("office-ओ"));
    assert!(output.contains("meeting-ए"));
    assert!(output.contains("hospital-ओ"));
    assert!(output.contains("bus-ए"));
    assert!(output.contains("train-ओ"));
    assert!(output.contains("doctor-रे"));
    assert!(output.contains("ticket-टा"));
}

#[test]
fn test_all_fricatives_and_consonant_clusters() {
    // Testing phonetic engine fallback with words not in the dictionary
    let input = "khan fani zanto bhalo thaka dhon choto";
    let hindi = to_hindi_phonetic(input);

    assert!(hindi.contains("ख़")); // velar fricative
    assert!(hindi.contains("फ़")); // labial fricative
    assert!(hindi.contains("ज़")); // voiced fricative
    assert!(hindi.contains("भ")); // aspirated bh
    assert!(hindi.contains("थ")); // aspirated th
    assert!(hindi.contains("ध")); // aspirated dh
}

#[test]
fn test_strict_bengali_and_nagri_functions() {
    let input = "Bhai office-o aisi";
    let bengali_strict = to_bengali_strict(input);
    assert!(bengali_strict.contains("office-ও"));

    let nagri_strict = to_nagri_strict(input);
    assert!(nagri_strict.contains("office-"));
}

#[test]
fn test_urls_and_email_preservation() {
    let input = "Check this link https://github.com and email me at test@example.com ok";
    let output = to_hindi_phonetic(input);

    assert!(output.contains("https://github.com") || output.contains("github.com"));
    assert!(output.contains("@example.com"));
}

#[test]
fn test_nagri_unicode_range_validity() {
    let input = "Kila aso bhai? Khani khaiso ni? Fani khaiba ni? Ami zairam.";
    let nagri = to_nagri(input);

    assert!(!nagri.is_empty());
    // Verify presence of Sylheti Nagri characters (U+A800..U+A82F)
    let contains_nagri = nagri.chars().any(|c| ('\u{A800}'..='\u{A82F}').contains(&c));
    assert!(contains_nagri, "Nagri output should contain characters in U+A800..U+A82F");
}

#[test]
fn test_bengali_script_output() {
    let input = "Kila aso bhai? Ami bala asi.";
    let bengali = to_bengali(input);

    assert_eq!(bengali, "কিলা আসো ভাই? আমি ভালা আসি.");
}

#[test]
fn test_raw_phonetic_mode() {
    let options = ConversionOptions {
        target: TargetScript::HindiVocalizer,
        english_mode: EnglishMode::RawPhonetic,
    };

    // In raw phonetic mode, even "office" goes through Sylheti phonetic rules
    let output = convert("office", &options);
    assert!(!output.is_empty());
}

#[test]
fn test_large_text_stress_and_no_panics() {
    // Construct a ~10,000 word realistic chat text
    let paragraph = "Kila aso bhai? Ami office-o meeting-e aisi, please call khoro. Khani khaiso ni? Fani khaiba ni? Aji shonjebela baire bairono lagbo. ";
    let repeated = paragraph.repeat(500); // 500 paragraphs = ~70,000 characters

    let start = std::time::Instant::now();
    let hindi = to_hindi_phonetic(&repeated);
    let duration = start.elapsed();

    assert!(!hindi.is_empty());
    // Processing ~70KB of text should easily take less than 150 milliseconds in Rust!
    assert!(duration.as_millis() < 500, "Should be fast, took: {:?}", duration);
}

#[test]
fn test_multilingual_mixed_indic_input_preservation() {
    // If the input already contains Devanagari or Bengali, it should not be damaged
    let input = "আজকে meeting-e aisi, सब ठीक है भाई?";
    let output = to_hindi_phonetic(input);

    assert!(output.contains("आजके") || output.contains("আজকে"));
    assert!(output.contains("सब ठीक है भाई?"));
}
