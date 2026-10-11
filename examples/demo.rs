//! Demo application showing both Natural and Strict modes of `sylheti_phonetics`.
//!
//! Run with:
//!     cargo run --example demo

use sylheti_phonetics::{
    find_word, to_bengali, to_hindi_phonetic, to_hindi_phonetic_strict, to_nagri, SYLHETI_WORDS,
};

fn main() {
    println!("============================================================");
    println!("  sylheti_phonetics: Natural & Strict Modes Demo            ");
    println!("============================================================\n");

    println!("Total Curated Sylheti Words: {}", SYLHETI_WORDS.len());
    println!("------------------------------------------------------------");

    let chat = "Bhai, ami office-o meeting-e aisi, please call khoro.";
    println!("\n[Original WhatsApp Message]");
    println!("  \"{}\"", chat);

    println!("\n[Mode 1: Natural Mode (Default)]");
    println!("  -> {}", to_hindi_phonetic(chat));
    println!("  (Notice: 'ऑफ़िस-ओ', 'मीटिंग-ए', 'प्लीज़', 'कॉल' are phonetically integrated");
    println!("   so the TTS voice reads the entire sentence in one continuous, natural breath!)");

    println!("\n[Mode 2: Strict Mode]");
    println!("  -> {}", to_hindi_phonetic_strict(chat));
    println!("  (Notice: 'office-ओ', 'meeting-ए', 'please', 'call' stay in Latin characters");
    println!("   for developers who prefer strict visual bilingual separation.)");

    println!("\n------------------------------------------------------------");
    println!("Cross-Script Conversions:");
    let greeting = "Kila aso bhai? Khani khaiso ni?";
    println!("  Romanized: {}", greeting);
    println!("  Hindi TTS: {}", to_hindi_phonetic(greeting));
    println!("  Bengali:   {}", to_bengali(greeting));
    println!("  Nagri:     {}", to_nagri(greeting));

    println!("\n------------------------------------------------------------");
    println!("Dictionary Lookups:");
    let sample_queries = ["kila", "khani", "fani", "zairam", "khosoba"];
    for q in sample_queries {
        if let Some(entry) = find_word(q) {
            println!(
                "  • '{}' -> Hindi: '{}', Bangla: '{}', Nagri: '{}' ({})",
                entry.canonical_roman,
                entry.hindi_phonetic,
                entry.bengali_script,
                entry.nagri_script,
                entry.english_meaning
            );
        }
    }

    println!("\n============================================================");
    println!("Ready for crates.io publication, Android JNI, and NVDA!");
    println!("============================================================");
}
