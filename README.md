# keyword_holder 🗣️

> **A Native Rust Sylheti Phonetic Dictionary & TTS Transliteration Engine**  
> Bridging Romanized Sylheti to Hindi Devanagari (optimized for Vocalizer TTS synthesizers), Bengali script, and historical Sylheti Nagri (`U+A800..U+A82F`).

---

## 🎯 The Problem & Our Solution

Most native speakers of Sylheti (across the Surma-Barak Valley of Sylhet and South Assam — Cachar, Karimganj, Hailakandi — and the diaspora) communicate digitally in **Romanized Sylheti** (*"Kila aso?"*, *"Khani khaiso ni?"*, *"Ami bala asi"*).

* Standard text-to-speech engines (like Nuance Vocalizer, Google TTS, Windows SAPI) have no native Sylheti mode.
* Standard Bengali voices distort authentic Sylheti sounds (especially velar fricatives like `[x]`, labial `[f]`, and `[z]`).
* **The Solution:** By mapping Romanized Sylheti phonetically to **Hindi Devanagari with Nuktas (`ख़`, `फ़`, `ज़`)**, high-quality Hindi Vocalizer voices synthesize **authentic native Sylheti speech** right out of the box with zero heavy AI overhead!

---

## 📦 Features

1. **Curated Sylheti Dictionary (`SYLHETI_WORDS`):**
   * Curated vocabulary covering everyday conversations, food, time, verbs, family, and regional Barak Valley expressions.
   * Maps canonical Roman spellings, informal chat variations, Hindi Devanagari phonetic targets, Bengali script, and Sylheti Nagri Unicode.
2. **Deterministic Phonetic G2P Fallback (`rules.rs`):**
   * Even for out-of-vocabulary words, applies Sylheti phonology (`kh` $\to$ `ख़`, `f`/`ph` $\to$ `फ़`, `z`/`j` $\to$ `ज़`, `ch` $\to$ `च`, etc.).
3. **Cross-Script Outputs:**
   * `to_hindi_phonetic(text)`: Output ready to pass directly to Hindi Vocalizer / SAPI / Android TTS.
   * `to_bengali(text)`: Standard Bengali script.
   * `to_nagri(text)`: Historical Sylheti Nagri Unicode script (`U+A800..U+A82F`).
4. **Zero-Cost, Portable Rust Library:**
   * Pure standard library (no bloated runtime dependencies).
   * Blazing fast: converts full paragraphs in microseconds.
   * Ready to compile to `.dll` / `.so` for NVDA screen reader, `.aar` for Android via JNI, or `.wasm` for web browsers.

---

## 🚀 Quick Start

Add `keyword_holder` to your `Cargo.toml`:

```toml
[dependencies]
keyword_holder = { path = "E:/ashu/git/keyword_holder" }
```

### In Rust Code

```rust
use keyword_holder::{to_hindi_phonetic, to_bengali, to_nagri, find_word};

fn main() {
    let input = "Kila aso bhai? Khani khaiso ni?";

    // 1. Send this output directly into your Hindi Vocalizer TTS synthesizer
    let hindi_tts = to_hindi_phonetic(input);
    println!("Hindi TTS: {}", hindi_tts);
    // Output: "कीला आसो भाइ? ख़ानी ख़ाइसो नी?"

    // 2. Convert to Bengali script
    let bengali = to_bengali(input);
    println!("Bengali: {}", bengali);
    // Output: "কিলা আসো ভাই? খানি খাইসো নি?"

    // 3. Convert to Sylheti Nagri script
    let nagri = to_nagri(input);
    println!("Nagri: {}", nagri);
    // Output: "ꠇꠤꠟꠣ ꠀꠍꠧ ꠜꠣꠁ? ꠈꠣꠘꠤ ꠈꠣꠁꠍꠧ ꠘꠤ?"
}
```

---

## 🧪 Run the Demo

Run the interactive demo included in the repository:

```bash
cargo run --example demo
```

Run test suite:

```bash
cargo test
```

---

## 📚 References & Research

* **End-to-end variational speech synthesis for the endangered Sylheti Nagri language** (Ataullha, Jisun, Rahman, *Systems and Soft Computing*, 2026, [DOI: 10.1016/j.sasc.2026.200496](https://doi.org/10.1016/j.sasc.2026.200496))
* **SOAS Sylheti Project** — Descriptive Lexical Database & Nagri Script revival (SOAS University of London)
* **Vashantor Benchmark Dataset** — Large-scale Regional Dialect Benchmark (Mukaffi et al.)
