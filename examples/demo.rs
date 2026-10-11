//! Demo application showing how to use `keyword_holder` in any Rust project.
//!
//! Run with:
//!     cargo run --example demo

use keyword_holder::{find_word, to_bengali, to_hindi_phonetic, to_nagri, SYLHETI_WORDS};

fn main() {
    println!("============================================================");
    println!("  Keyword Holder: Sylheti Phonetic & TTS Conversion Engine  ");
    println!("============================================================\n");

    println!("Total Curated Dictionary Words: {}", SYLHETI_WORDS.len());
    println!("------------------------------------------------------------");

    // Quick test phrases
    let test_sentences = [
        "Kila aso bhai? Khani khaiso ni?",
        "Ami bala asi. Fani khaiba ni?",
        "Aji shonjebela baire bairono lagbo, shonge aiba ni?",
        "Ami ghoro boiya aisi, Maa-e khosoba cha bonaiso.",
        "Kono zairay? Ami office-o zairam.",
    ];

    for (i, sentence) in test_sentences.iter().enumerate() {
        println!("\n[Case {}]", i + 1);
        println!("  Romanized Input : {}", sentence);
        println!("  Hindi TTS Voice : {}", to_hindi_phonetic(sentence));
        println!("  Bengali Script  : {}", to_bengali(sentence));
        println!("  Sylheti Nagri   : {}", to_nagri(sentence));
    }

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
    println!("Ready for integration with Android JNI, NVDA, or WebAssembly!");
    println!("============================================================");
}
