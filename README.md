# sylheti_phonetics 🗣️

[![Crates.io](https://img.shields.io/crates/v/sylheti_phonetics.svg)](https://crates.io/crates/sylheti_phonetics)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Build Status](https://img.shields.io/badge/tests-passing-brightgreen.svg)]()

> **High-performance Sylheti phonetic dictionary and transliteration engine.**  
> Bridges Romanized Sylheti chat text to **Vocalizer-tuned Hindi Devanagari (TTS mode)**, **Bengali script**, and historical **Sylheti Nagri (`U+A800..U+A82F`)**, with built-in **bilingual English code-switching**.

---

## 🌟 The Vision

Sylheti is spoken by millions across the Surma-Barak Valley (Sylhet, Bangladesh and South Assam's Cachar, Karimganj, Hailakandi) and global diaspora communities. Today, native speakers overwhelmingly communicate on WhatsApp, Facebook, and messaging apps using **Romanized Sylheti** (*"Kila aso?"*, *"Khani khaiso ni?"*, *"Ami bala asi"*).

However:
1. **Existing TTS engines** (Nuance Vocalizer, Google TTS, Microsoft Speech) lack native Sylheti models.
2. **Standard Bengali voices** distort authentic Sylheti sounds, missing characteristic velar fricatives `[x]`, labial fricatives `[f]`, and `[z]`.
3. **Pure transliteration engines mangle English words** when people write mixed bilingual messages (*"Bhai, ami office-o meeting-e aisi, please call khoro"*).

`sylheti_phonetics` solves all three challenges:
* **For TTS Synthesis:** Maps Sylheti words to Hindi Devanagari using Nuktas (`ख़`, `फ़`, `ज़`), allowing Hindi Vocalizer engines (e.g. *Lekha*, *Neel*) to synthesize **authentic, natural native Sylheti speech** out of the box.
* **For Bilingual Chats:** Intelligently recognizes English words (`office`, `meeting`, `please`, `call`) and preserves them in Latin script so multilingual synthesizers pronounce them cleanly.
* **For Heritage & Revitalization:** Translates text directly into standard **Bengali script** and historical **Sylheti Nagri script** (`U+A800..U+A82F`).

---

## ⚡ Features

* **Zero Runtime Dependencies:** Pure, safe Rust standard library.
* **Microsecond Speed:** Processes entire chat paragraphs in microseconds with negligible RAM footprint.
* **Curated Word Database (`SYLHETI_WORDS`):** High-frequency Sylheti greetings, pronouns, verbs, regional Barak Valley idioms, and food terms.
* **Phonetic Rule-Based Fallback (`rules.rs`):** Converts out-of-vocabulary words deterministically.
* **Morphological Suffix Parsing:** Understands Sylheti suffixes attached to English roots (e.g., `office-o` $\to$ `office-ओ`).
* **Deploy Anywhere:** Ready to compile into:
  * Rust applications (`cargo add sylheti_phonetics`)
  * Desktop C shared libraries (`.dll` / `.so`) for NVDA screen reader plugins
  * Android (`.aar` via JNI)
  * WebAssembly (`.wasm`) for web browsers

---

## 🚀 Installation

Add `sylheti_phonetics` to your `Cargo.toml`:

```toml
[dependencies]
sylheti_phonetics = "0.1.0"
```

Or via Cargo CLI:

```bash
cargo add sylheti_phonetics
```

---

## 📖 Usage Examples

### 1. Generating Audio-Ready Text for Vocalizer TTS

```rust
use sylheti_phonetics::to_hindi_phonetic;

fn main() {
    let input = "Kila aso bhai? Khani khaiso ni?";
    let tts_text = to_hindi_phonetic(input);
    println!("{}", tts_text);
    // Output: "कीला आसो भाइ? ख़ानी ख़ाइसो नी?"
    // (Pass this output directly to your Hindi Vocalizer voice!)
}
```

### 2. Mixed English + Sylheti Chat (Code-Switching)

```rust
use sylheti_phonetics::to_hindi_phonetic;

fn main() {
    let chat = "Bhai, ami office-o meeting-e aisi, please later call khoro.";
    let tts_text = to_hindi_phonetic(chat);
    println!("{}", tts_text);
    // Output: "भाइ, आमी office-ओ meeting-ए आइसी, please later call ख़ोरो."
    // English words remain intact; Sylheti words get authentic phonetics!
}
```

### 3. Converting to Bengali and Sylheti Nagri Scripts

```rust
use sylheti_phonetics::{to_bengali, to_nagri};

fn main() {
    let input = "Kila aso?";

    // Standard Bengali Script
    let bangla = to_bengali(input);
    println!("Bengali: {}", bangla); // "কিলা আসো?"

    // Historical Sylheti Nagri Unicode (U+A800..U+A82F)
    let nagri = to_nagri(input);
    println!("Nagri: {}", nagri);     // "ꠇꠤꠟꠣ ꠀꠍꠧ?"
}
```

### 4. Dictionary Lookup

```rust
use sylheti_phonetics::find_word;

fn main() {
    if let Some(entry) = find_word("khani") {
        println!("Canonical: {}", entry.canonical_roman);
        println!("Hindi TTS : {}", entry.hindi_phonetic); // ख़ानी
        println!("Bengali   : {}", entry.bengali_script);  // খানি
        println!("Nagri     : {}", entry.nagri_script);    // ꠈꠣꠘꠤ
        println!("Meaning   : {}", entry.english_meaning); // food / meal
    }
}
```

---

## 🧪 Testing & Demo

Run the test suite:

```bash
cargo test
```

Run the interactive demonstration:

```bash
cargo run --example demo
```

---

## 🔬 Research & Citations

1. **End-to-end variational speech synthesis for the endangered Sylheti Nagri language** (Md. Ataullha, Soumik Paul Jisun, M. Shahidur Rahman, *Systems and Soft Computing*, 2026, [DOI: 10.1016/j.sasc.2026.200496](https://doi.org/10.1016/j.sasc.2026.200496))
2. **SOAS Sylheti Project** — Descriptive Lexicon & Nagri revival (SOAS University of London)
3. **Vashantor Benchmark Dataset** — Large-scale Regional Dialect Benchmark (Mukaffi et al.)

---

## 📄 License

Licensed under either of:

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
