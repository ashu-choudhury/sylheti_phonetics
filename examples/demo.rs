//! Demo application showing how to use `sylheti_phonetics` in any Rust project.
//!
//! Run with:
//!     cargo run --example demo

use sylheti_phonetics::{find_word, to_bengali, to_hindi_phonetic, to_nagri, SYLHETI_WORDS};

fn main() {
    println!("============================================================");
    println!("  Sylheti Phonetics: TTS & Transliteration Engine Demo      ");
    println!("============================================================\n");

    println!("Total Curated Sylheti Words: {}", SYLHETI_WORDS.len());
    println!("------------------------------------------------------------");

    // Sentences including everyday chat and bilingual code-switching
    let test_sentences = [
        "Kila aso bhai? Khani khaiso ni?",
        "Ami bala asi. Fani khaiba ni?",
        "Aji shonjebela baire bairono lagbo, shonge aiba ni?",
        "Bhai, ami office-o meeting-e aisi, please later call khoro.",
        "Mobile-o message check khoriya dekho, important update ase.",
    ];

    for (i, sentence) in test_sentences.iter().enumerate() {
        println!("\n[Case {}]", i + 1);
        println!("  Input (WhatsApp Roman) : {}", sentence);
        println!("  Vocalizer Hindi TTS    : {}", to_hindi_phonetic(sentence));
        println!("  Bengali Script         : {}", to_bengali(sentence));
        println!("  Sylheti Nagri          : {}", to_nagri(sentence));
    }

    println!("\n------------------------------------------------------------");
    println!("Bilingual Code-Switching Demo:");
    println!("Input: 'Bhai, ami office-o meeting-e aisi, please call khoro.'");
    println!(
        "TTS Output: {}",
        to_hindi_phonetic("Bhai, ami office-o meeting-e aisi, please call khoro.")
    );
    println!("(Notice: 'office', 'meeting', 'please', 'call' remain untouched English,");
    println!(" while Sylheti words convert to authentic Devanagari phonetics!)");

    println!("\n------------------------------------------------------------");
    println!("Sample Dictionary Lookups:");
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
    println!("Ready to publish to crates.io and integrate with Android/NVDA!");
    println!("============================================================");
}
